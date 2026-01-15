/// WebGL post-processing for CRT effects.
///
/// Applies gaussian blur and scanline effects to the rendered terminal output.

use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{
    HtmlCanvasElement, WebGlBuffer, WebGlFramebuffer, WebGlProgram, WebGlRenderingContext,
    WebGlShader, WebGlTexture, WebGlUniformLocation,
};

use crate::renderer::{CANVAS_HEIGHT, CANVAS_WIDTH};

/// Vertex shader source (shared by all passes)
const VERTEX_SHADER: &str = r#"
    attribute vec2 a_position;
    attribute vec2 a_texcoord;
    varying vec2 v_texcoord;
    void main() {
        gl_Position = vec4(a_position, 0.0, 1.0);
        v_texcoord = a_texcoord;
    }
"#;

/// Gaussian blur fragment shader (separable - run twice for H and V)
const BLUR_FRAGMENT_SHADER: &str = r#"
    precision mediump float;
    uniform sampler2D u_texture;
    uniform vec2 u_resolution;
    uniform vec2 u_direction;
    varying vec2 v_texcoord;

    void main() {
        vec2 texel = 1.0 / u_resolution;
        vec4 color = vec4(0.0);

        // 5-tap gaussian kernel
        color += texture2D(u_texture, v_texcoord - 2.0 * texel * u_direction) * 0.06;
        color += texture2D(u_texture, v_texcoord - 1.0 * texel * u_direction) * 0.24;
        color += texture2D(u_texture, v_texcoord) * 0.40;
        color += texture2D(u_texture, v_texcoord + 1.0 * texel * u_direction) * 0.24;
        color += texture2D(u_texture, v_texcoord + 2.0 * texel * u_direction) * 0.06;

        gl_FragColor = color;
    }
"#;

/// Passthrough fragment shader (just copies texture)
const PASSTHROUGH_FRAGMENT_SHADER: &str = r#"
    precision mediump float;
    uniform sampler2D u_texture;
    varying vec2 v_texcoord;

    void main() {
        gl_FragColor = texture2D(u_texture, v_texcoord);
    }
"#;

/// WebGL post-processor for blur effects.
pub struct PostProcessor {
    gl: WebGlRenderingContext,
    blur_program: WebGlProgram,
    passthrough_program: WebGlProgram,
    source_texture: WebGlTexture,
    intermediate_texture: WebGlTexture,
    framebuffer: WebGlFramebuffer,
    /// Vertex buffer - kept alive for WebGL state, accessed via GL context not Rust
    _quad_buffer: WebGlBuffer,
    // Uniform locations for blur program
    blur_texture_loc: WebGlUniformLocation,
    blur_resolution_loc: WebGlUniformLocation,
    blur_direction_loc: WebGlUniformLocation,
    // Uniform locations for passthrough program
    passthrough_texture_loc: WebGlUniformLocation,
}

impl PostProcessor {
    /// Create a new post-processor for the given display canvas.
    pub fn new(canvas: &HtmlCanvasElement) -> Result<Self, JsValue> {
        let gl = canvas
            .get_context("webgl")?
            .ok_or("Failed to get WebGL context")?
            .dyn_into::<WebGlRenderingContext>()?;

        // Compile shaders and create programs
        let blur_program = create_program(&gl, VERTEX_SHADER, BLUR_FRAGMENT_SHADER)?;
        let passthrough_program = create_program(&gl, VERTEX_SHADER, PASSTHROUGH_FRAGMENT_SHADER)?;

        // Get uniform locations for blur program
        let blur_texture_loc = gl
            .get_uniform_location(&blur_program, "u_texture")
            .ok_or("Failed to get u_texture location for blur")?;
        let blur_resolution_loc = gl
            .get_uniform_location(&blur_program, "u_resolution")
            .ok_or("Failed to get u_resolution location for blur")?;
        let blur_direction_loc = gl
            .get_uniform_location(&blur_program, "u_direction")
            .ok_or("Failed to get u_direction location for blur")?;

        // Get uniform locations for passthrough program
        let passthrough_texture_loc = gl
            .get_uniform_location(&passthrough_program, "u_texture")
            .ok_or("Failed to get u_texture location for passthrough")?;

        // Create textures
        let source_texture = create_texture(&gl)?;
        let intermediate_texture = create_texture(&gl)?;

        // Initialize intermediate texture with correct size
        gl.bind_texture(WebGlRenderingContext::TEXTURE_2D, Some(&intermediate_texture));
        gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
            WebGlRenderingContext::TEXTURE_2D,
            0,
            WebGlRenderingContext::RGBA as i32,
            CANVAS_WIDTH as i32,
            CANVAS_HEIGHT as i32,
            0,
            WebGlRenderingContext::RGBA,
            WebGlRenderingContext::UNSIGNED_BYTE,
            None,
        )?;

        // Create framebuffer for intermediate rendering
        let framebuffer = gl
            .create_framebuffer()
            .ok_or("Failed to create framebuffer")?;

        // Create fullscreen quad buffer
        let quad_buffer = create_quad_buffer(&gl)?;

        // Set up vertex attributes (position at location 0, texcoord at location 1)
        setup_vertex_attributes(&gl, &blur_program)?;

        Ok(PostProcessor {
            gl,
            blur_program,
            passthrough_program,
            source_texture,
            intermediate_texture,
            framebuffer,
            _quad_buffer: quad_buffer,
            blur_texture_loc,
            blur_resolution_loc,
            blur_direction_loc,
            passthrough_texture_loc,
        })
    }

    /// Apply post-processing effects to the source canvas and render to display.
    pub fn process(&self, source_canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
        let gl = &self.gl;

        // Upload source canvas to texture
        gl.bind_texture(
            WebGlRenderingContext::TEXTURE_2D,
            Some(&self.source_texture),
        );
        gl.tex_image_2d_with_u32_and_u32_and_canvas(
            WebGlRenderingContext::TEXTURE_2D,
            0,
            WebGlRenderingContext::RGBA as i32,
            WebGlRenderingContext::RGBA,
            WebGlRenderingContext::UNSIGNED_BYTE,
            source_canvas,
        )?;

        // Pass 1: Horizontal blur (source -> intermediate)
        gl.bind_framebuffer(
            WebGlRenderingContext::FRAMEBUFFER,
            Some(&self.framebuffer),
        );
        gl.framebuffer_texture_2d(
            WebGlRenderingContext::FRAMEBUFFER,
            WebGlRenderingContext::COLOR_ATTACHMENT0,
            WebGlRenderingContext::TEXTURE_2D,
            Some(&self.intermediate_texture),
            0,
        );

        gl.viewport(0, 0, CANVAS_WIDTH as i32, CANVAS_HEIGHT as i32);
        gl.use_program(Some(&self.blur_program));

        gl.active_texture(WebGlRenderingContext::TEXTURE0);
        gl.bind_texture(
            WebGlRenderingContext::TEXTURE_2D,
            Some(&self.source_texture),
        );

        gl.uniform1i(Some(&self.blur_texture_loc), 0);
        gl.uniform2f(
            Some(&self.blur_resolution_loc),
            CANVAS_WIDTH as f32,
            CANVAS_HEIGHT as f32,
        );
        gl.uniform2f(Some(&self.blur_direction_loc), 1.0, 0.0); // Horizontal

        setup_vertex_attributes(gl, &self.blur_program)?;
        gl.draw_arrays(WebGlRenderingContext::TRIANGLE_STRIP, 0, 4);

        // Pass 2: Vertical blur (intermediate -> source texture, reusing it)
        gl.framebuffer_texture_2d(
            WebGlRenderingContext::FRAMEBUFFER,
            WebGlRenderingContext::COLOR_ATTACHMENT0,
            WebGlRenderingContext::TEXTURE_2D,
            Some(&self.source_texture),
            0,
        );

        gl.bind_texture(
            WebGlRenderingContext::TEXTURE_2D,
            Some(&self.intermediate_texture),
        );

        gl.uniform2f(Some(&self.blur_direction_loc), 0.0, 1.0); // Vertical

        gl.draw_arrays(WebGlRenderingContext::TRIANGLE_STRIP, 0, 4);

        // Pass 3: Passthrough (source -> screen)
        gl.bind_framebuffer(WebGlRenderingContext::FRAMEBUFFER, None);

        gl.viewport(0, 0, CANVAS_WIDTH as i32, CANVAS_HEIGHT as i32);
        gl.use_program(Some(&self.passthrough_program));

        gl.bind_texture(
            WebGlRenderingContext::TEXTURE_2D,
            Some(&self.source_texture),
        );

        gl.uniform1i(Some(&self.passthrough_texture_loc), 0);

        setup_vertex_attributes(gl, &self.passthrough_program)?;
        gl.draw_arrays(WebGlRenderingContext::TRIANGLE_STRIP, 0, 4);

        Ok(())
    }
}

/// Compile a shader from source.
fn compile_shader(
    gl: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, JsValue> {
    let shader = gl
        .create_shader(shader_type)
        .ok_or("Failed to create shader")?;

    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        let log = gl
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| "Unknown error".to_string());
        gl.delete_shader(Some(&shader));
        Err(JsValue::from_str(&format!("Shader compile error: {}", log)))
    }
}

/// Create a shader program from vertex and fragment shader sources.
fn create_program(
    gl: &WebGlRenderingContext,
    vertex_src: &str,
    fragment_src: &str,
) -> Result<WebGlProgram, JsValue> {
    let vertex_shader = compile_shader(gl, WebGlRenderingContext::VERTEX_SHADER, vertex_src)?;
    let fragment_shader = compile_shader(gl, WebGlRenderingContext::FRAGMENT_SHADER, fragment_src)?;

    let program = gl.create_program().ok_or("Failed to create program")?;

    gl.attach_shader(&program, &vertex_shader);
    gl.attach_shader(&program, &fragment_shader);
    gl.link_program(&program);

    // Clean up shaders (they're now part of the program)
    gl.delete_shader(Some(&vertex_shader));
    gl.delete_shader(Some(&fragment_shader));

    if gl
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        let log = gl
            .get_program_info_log(&program)
            .unwrap_or_else(|| "Unknown error".to_string());
        gl.delete_program(Some(&program));
        Err(JsValue::from_str(&format!("Program link error: {}", log)))
    }
}

/// Create a texture with appropriate filtering settings.
fn create_texture(gl: &WebGlRenderingContext) -> Result<WebGlTexture, JsValue> {
    let texture = gl.create_texture().ok_or("Failed to create texture")?;

    gl.bind_texture(WebGlRenderingContext::TEXTURE_2D, Some(&texture));

    // Set texture parameters for linear filtering (important for blur)
    gl.tex_parameteri(
        WebGlRenderingContext::TEXTURE_2D,
        WebGlRenderingContext::TEXTURE_WRAP_S,
        WebGlRenderingContext::CLAMP_TO_EDGE as i32,
    );
    gl.tex_parameteri(
        WebGlRenderingContext::TEXTURE_2D,
        WebGlRenderingContext::TEXTURE_WRAP_T,
        WebGlRenderingContext::CLAMP_TO_EDGE as i32,
    );
    gl.tex_parameteri(
        WebGlRenderingContext::TEXTURE_2D,
        WebGlRenderingContext::TEXTURE_MIN_FILTER,
        WebGlRenderingContext::LINEAR as i32,
    );
    gl.tex_parameteri(
        WebGlRenderingContext::TEXTURE_2D,
        WebGlRenderingContext::TEXTURE_MAG_FILTER,
        WebGlRenderingContext::LINEAR as i32,
    );

    Ok(texture)
}

/// Create a buffer with fullscreen quad vertices.
fn create_quad_buffer(gl: &WebGlRenderingContext) -> Result<WebGlBuffer, JsValue> {
    let buffer = gl.create_buffer().ok_or("Failed to create buffer")?;

    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

    // Fullscreen quad: position (x, y) and texcoord (u, v)
    // Using triangle strip: 4 vertices
    // V coordinates are flipped (1-v) because canvas Y=0 is top, but GL Y=0 is bottom
    #[rustfmt::skip]
    let vertices: [f32; 16] = [
        // x,    y,    u,    v
        -1.0, -1.0,  0.0,  1.0,  // Bottom-left  (screen) -> top of texture
         1.0, -1.0,  1.0,  1.0,  // Bottom-right (screen) -> top of texture
        -1.0,  1.0,  0.0,  0.0,  // Top-left     (screen) -> bottom of texture
         1.0,  1.0,  1.0,  0.0,  // Top-right    (screen) -> bottom of texture
    ];

    // Convert to bytes for WebGL
    let vertices_array = unsafe {
        js_sys::Float32Array::view(&vertices)
    };

    gl.buffer_data_with_array_buffer_view(
        WebGlRenderingContext::ARRAY_BUFFER,
        &vertices_array,
        WebGlRenderingContext::STATIC_DRAW,
    );

    Ok(buffer)
}

/// Set up vertex attributes for the current program.
fn setup_vertex_attributes(
    gl: &WebGlRenderingContext,
    program: &WebGlProgram,
) -> Result<(), JsValue> {
    let position_loc = gl.get_attrib_location(program, "a_position") as u32;
    let texcoord_loc = gl.get_attrib_location(program, "a_texcoord") as u32;

    // Stride: 4 floats per vertex (x, y, u, v) = 16 bytes
    let stride = 4 * 4;

    gl.enable_vertex_attrib_array(position_loc);
    gl.vertex_attrib_pointer_with_i32(
        position_loc,
        2,                                    // 2 components (x, y)
        WebGlRenderingContext::FLOAT,
        false,
        stride,
        0,                                    // offset 0
    );

    gl.enable_vertex_attrib_array(texcoord_loc);
    gl.vertex_attrib_pointer_with_i32(
        texcoord_loc,
        2,                                    // 2 components (u, v)
        WebGlRenderingContext::FLOAT,
        false,
        stride,
        2 * 4,                                // offset 8 bytes (after x, y)
    );

    Ok(())
}
