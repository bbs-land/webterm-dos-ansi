use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use std::rc::Rc;
use std::cell::RefCell;

mod dom;
mod font;
mod parser;
mod postprocess;
mod renderer;
mod screen;
mod scrollback;
mod terminal;

use postprocess::PostProcessor;
use renderer::{Palette, Renderer, CANVAS_HEIGHT, CANVAS_WIDTH};
use scrollback::DEFAULT_MAX_LINES;
use terminal::{setup_scrollback_events, Terminal};

/// Options for rendering ANSI content.
#[wasm_bindgen]
pub struct RenderOptions {
    /// CSS selector for the container element
    selector: String,
    /// Baud rate for rendering simulation (e.g., 2400, 9600). None for instant.
    bps: Option<u32>,
    /// Color palette: "CGA" or "VGA" (default)
    palette: Option<String>,
    /// Scrollback buffer size (default: 5000)
    scrollback_lines: Option<u32>,
}

#[wasm_bindgen]
impl RenderOptions {
    /// Create new render options with required selector.
    #[wasm_bindgen(constructor)]
    pub fn new(selector: String) -> Self {
        RenderOptions {
            selector,
            bps: None,
            palette: None,
            scrollback_lines: None,
        }
    }

    /// Set baud rate for rendering simulation.
    #[wasm_bindgen(js_name = setBps)]
    pub fn set_bps(mut self, bps: u32) -> Self {
        self.bps = Some(bps);
        self
    }

    /// Set color palette ("CGA" or "VGA").
    #[wasm_bindgen(js_name = setPalette)]
    pub fn set_palette(mut self, palette: String) -> Self {
        self.palette = Some(palette);
        self
    }

    /// Set scrollback buffer size.
    #[wasm_bindgen(js_name = setScrollbackLines)]
    pub fn set_scrollback_lines(mut self, lines: u32) -> Self {
        self.scrollback_lines = Some(lines);
        self
    }
}

/// Initialize WebTerm terminals on the page.
///
/// Scans the DOM for elements with `data-term-url` attribute and initializes
/// terminal instances for each one.
///
/// Supported data attributes:
/// - `data-term-url`: WebSocket URL (required)
/// - `data-term-palette`: Color palette ("CGA" or "VGA", default: "VGA")
/// - `data-term-scrollback-lines`: Scrollback buffer size (default: 5000)
#[wasm_bindgen(js_name = initWebTerm)]
pub fn init_web_term() {
    // Set panic hook for better error messages in the browser console
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    // Log initialization
    web_sys::console::log_1(&"WebTerm: Initializing terminals...".into());

    // Scan DOM for elements with data-term-url
    match dom::query_selector_all("[data-term-url]") {
        Ok(elements) => {
            web_sys::console::log_1(&format!("WebTerm: Found {} terminal(s)", elements.len()).into());

            for element in elements {
                if let Err(e) = init_terminal(&element) {
                    web_sys::console::error_1(&format!("Failed to initialize terminal: {:?}", e).into());
                }
            }
        }
        Err(e) => {
            web_sys::console::error_1(&format!("Failed to query terminals: {:?}", e).into());
        }
    }

    web_sys::console::log_1(&"WebTerm: Initialization complete".into());
}

/// Initialize a single terminal instance.
fn init_terminal(container: &web_sys::Element) -> Result<(), JsValue> {
    let term_url = dom::get_data_attribute(container, "term-url")
        .ok_or_else(|| JsValue::from_str("Missing data-term-url"))?;

    // Get palette configuration (default: VGA)
    let palette_str = dom::get_data_attribute(container, "term-palette")
        .unwrap_or_else(|| "VGA".to_string());
    let palette = Palette::from_str(&palette_str);

    // Get scrollback lines configuration (default: 5000)
    let scrollback_lines = dom::get_data_attribute(container, "term-scrollback-lines")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(DEFAULT_MAX_LINES);

    web_sys::console::log_1(&format!(
        "WebTerm: Initializing terminal for {} (palette: {}, scrollback: {} lines)",
        term_url, palette_str, scrollback_lines
    ).into());

    // Create offscreen canvas for 2D rendering
    let offscreen_canvas = Rc::new(dom::create_offscreen_canvas(CANVAS_WIDTH, CANVAS_HEIGHT)?);

    // Create display canvas with WebGL for post-processing
    let display_canvas = dom::create_canvas(CANVAS_WIDTH, CANVAS_HEIGHT)?;
    container.append_child(&display_canvas)?;

    // Create terminal with scrollback
    let terminal = Rc::new(RefCell::new(Terminal::with_scrollback_lines(scrollback_lines)));

    // Create renderer with specified palette
    let renderer = Rc::new(Renderer::with_palette(&offscreen_canvas, palette)?);

    // Create post-processor (renders to display canvas)
    let post_processor = Rc::new(PostProcessor::new(&display_canvas)?);

    // Set up scrollback event listeners
    setup_scrollback_events(
        &display_canvas,
        terminal.clone(),
        renderer.clone(),
        offscreen_canvas.clone(),
        post_processor.clone(),
    )?;

    // Initial render with post-processing
    {
        let term = terminal.borrow();
        renderer.render_with_scrollback(&term.screen, &term.scrollback)?;
        post_processor.process(&offscreen_canvas)?;
    }

    // TODO: Handle connect button and pre-connect screen
    // TODO: Set up WebSocket connection on click

    Ok(())
}

/// Render CP437 ANSI content to a container element.
///
/// # Arguments
/// * `content` - CP437 ANSI content as bytes
/// * `options` - Render options (selector, bps, palette, scrollback_lines)
///
/// # Example (JavaScript)
/// ```javascript
/// const options = new RenderOptions("#terminal")
///     .setBps(9600)
///     .setPalette("CGA")
///     .setScrollbackLines(10000);
/// renderAnsi(content, options);
/// ```
#[wasm_bindgen(js_name = renderAnsi)]
pub fn render_ansi(content: &[u8], options: RenderOptions) {
    let palette_str = options.palette.as_deref().unwrap_or("VGA");
    let scrollback_size = options.scrollback_lines.map(|n| n as usize).unwrap_or(DEFAULT_MAX_LINES);

    web_sys::console::log_1(&format!(
        "WebTerm: Rendering ANSI to {} (bps: {:?}, palette: {}, scrollback: {} lines)",
        options.selector, options.bps, palette_str, scrollback_size
    ).into());

    // Clone data for the async closure
    let selector = options.selector.clone();
    let content = content.to_vec();
    let palette = Palette::from_str(palette_str);
    let bps = options.bps;

    spawn_local(async move {
        match render_ansi_async(&selector, &content, bps, palette, scrollback_size).await {
            Ok(_) => web_sys::console::log_1(&"WebTerm: ANSI rendering complete".into()),
            Err(e) => web_sys::console::error_1(&format!("Failed to render ANSI: {:?}", e).into()),
        }
    });
}

async fn render_ansi_async(
    selector: &str,
    content: &[u8],
    bps: Option<u32>,
    palette: Palette,
    scrollback_lines: usize,
) -> Result<(), JsValue> {
    // Find container element
    let container = dom::query_selector(selector)?
        .ok_or_else(|| JsValue::from_str("Container not found"))?;

    // Create offscreen canvas for 2D rendering
    let offscreen_canvas = Rc::new(dom::create_offscreen_canvas(CANVAS_WIDTH, CANVAS_HEIGHT)?);

    // Create display canvas with WebGL for post-processing
    let display_canvas = dom::create_canvas(CANVAS_WIDTH, CANVAS_HEIGHT)?;
    container.append_child(&display_canvas)?;

    // Create terminal with scrollback
    let terminal = Rc::new(RefCell::new(Terminal::with_scrollback_lines(scrollback_lines)));

    // Create renderer
    let renderer = Rc::new(Renderer::with_palette(&offscreen_canvas, palette)?);

    // Create post-processor for blur effects
    let post_processor = Rc::new(PostProcessor::new(&display_canvas)?);

    // Set up scrollback event listeners
    setup_scrollback_events(
        &display_canvas,
        terminal.clone(),
        renderer.clone(),
        offscreen_canvas.clone(),
        post_processor.clone(),
    )?;

    // Focus the canvas so it can receive keyboard events for scrollback
    let _ = display_canvas.focus();

    match bps {
        Some(bps) if bps > 0 => {
            // BPS simulation: render in chunks with delays
            let bytes_per_second = bps as f64 / 8.0;

            // Render approximately 30 frames per second for smooth animation
            let target_fps = 30.0;
            let bytes_per_frame = (bytes_per_second / target_fps).max(1.0) as usize;
            let frame_delay_ms = (1000.0 / target_fps) as i32;

            let mut offset = 0;
            while offset < content.len() {
                let chunk_end = (offset + bytes_per_frame).min(content.len());

                // Process this chunk
                {
                    let mut term = terminal.borrow_mut();
                    term.process_bytes(&content[offset..chunk_end]);
                }

                // Render the current view (scrollback position or live screen)
                {
                    let term = terminal.borrow();
                    renderer.render_with_scrollback(&term.screen, &term.scrollback)?;
                    post_processor.process(&offscreen_canvas)?;
                }

                offset = chunk_end;

                // Wait before next frame (unless we're done)
                if offset < content.len() {
                    sleep_ms(frame_delay_ms).await;
                }
            }
        }
        _ => {
            // No BPS - render immediately
            {
                let mut term = terminal.borrow_mut();
                term.process_bytes(content);
            }
            {
                let term = terminal.borrow();
                renderer.render_with_scrollback(&term.screen, &term.scrollback)?;
                post_processor.process(&offscreen_canvas)?;
            }
        }
    }

    Ok(())
}

/// Sleep for the specified number of milliseconds using JavaScript setTimeout
async fn sleep_ms(ms: i32) {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        let window = web_sys::window().unwrap();
        window.set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms).unwrap();
    });
    wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();
}
