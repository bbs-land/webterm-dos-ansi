use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use std::rc::Rc;
use std::cell::RefCell;

mod dom;
mod font;
mod parser;
mod renderer;
mod screen;

use parser::AnsiParser;
use renderer::Renderer;
use screen::Screen;

/// Initialize WebTerm terminals on the page.
///
/// Scans the DOM for elements with `data-term-url` attribute and initializes
/// terminal instances for each one.
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

    web_sys::console::log_1(&format!("WebTerm: Initializing terminal for {}", term_url).into());

    // Create canvas
    let canvas = dom::create_canvas(1920, 1400)?;

    // Append canvas to container
    container.append_child(&canvas)?;

    // Create screen and renderer
    let screen = Screen::new();
    let renderer = Renderer::new(canvas)?;

    // Initial render
    renderer.render(&screen)?;

    // TODO: Handle connect button and pre-connect screen
    // TODO: Set up WebSocket connection on click

    Ok(())
}

/// Render CP437 ANSI content to a container element.
///
/// # Arguments
/// * `selector` - CSS selector for the container element
/// * `content` - CP437 ANSI content as bytes
/// * `bps` - Optional baud rate for rendering simulation (e.g., 2400, 9600)
#[wasm_bindgen(js_name = renderAnsi)]
pub fn render_ansi(selector: &str, content: &[u8], bps: Option<u32>) {
    web_sys::console::log_1(&format!("WebTerm: Rendering ANSI to {} (bps: {:?})", selector, bps).into());

    // Clone data for the async closure
    let selector = selector.to_string();
    let content = content.to_vec();

    spawn_local(async move {
        match render_ansi_async(&selector, &content, bps).await {
            Ok(_) => web_sys::console::log_1(&"WebTerm: ANSI rendering complete".into()),
            Err(e) => web_sys::console::error_1(&format!("Failed to render ANSI: {:?}", e).into()),
        }
    });
}

async fn render_ansi_async(selector: &str, content: &[u8], bps: Option<u32>) -> Result<(), JsValue> {
    // Find container element
    let container = dom::query_selector(selector)?
        .ok_or_else(|| JsValue::from_str("Container not found"))?;

    // Create canvas
    let canvas = dom::create_canvas(1920, 1400)?;
    container.append_child(&canvas)?;

    // Create screen, parser, and renderer wrapped in Rc<RefCell<>> for async access
    let screen = Rc::new(RefCell::new(Screen::new()));
    let parser = Rc::new(RefCell::new(AnsiParser::new()));
    let renderer = Renderer::new(canvas)?;

    match bps {
        Some(bps) if bps > 0 => {
            // BPS simulation: render in chunks with delays
            // Calculate delay: 8 bits per byte, so bytes_per_second = bps / 8
            // We'll render in chunks and delay between them
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
                    let mut parser = parser.borrow_mut();
                    let mut screen = screen.borrow_mut();
                    for &byte in &content[offset..chunk_end] {
                        parser.process_byte(byte, &mut screen);
                    }
                }

                // Render current state
                renderer.render(&screen.borrow())?;

                offset = chunk_end;

                // Wait before next frame (unless we're done)
                if offset < content.len() {
                    sleep_ms(frame_delay_ms).await;
                }
            }
        }
        _ => {
            // No BPS - render immediately
            let mut parser = parser.borrow_mut();
            let mut screen = screen.borrow_mut();
            for &byte in content {
                parser.process_byte(byte, &mut screen);
            }
            drop(parser);
            renderer.render(&screen)?;
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
