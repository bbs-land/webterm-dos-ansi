use wasm_bindgen::prelude::*;

mod cp437;
mod dom;
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

    match render_ansi_impl(selector, content, bps) {
        Ok(_) => web_sys::console::log_1(&"WebTerm: ANSI rendering complete".into()),
        Err(e) => web_sys::console::error_1(&format!("Failed to render ANSI: {:?}", e).into()),
    }
}

fn render_ansi_impl(selector: &str, content: &[u8], _bps: Option<u32>) -> Result<(), JsValue> {
    // Find container element
    let container = dom::query_selector(selector)?
        .ok_or_else(|| JsValue::from_str("Container not found"))?;

    // Create canvas
    let canvas = dom::create_canvas(1920, 1400)?;
    container.append_child(&canvas)?;

    // Create screen, parser, and renderer
    let mut screen = Screen::new();
    let mut parser = AnsiParser::new();
    let renderer = Renderer::new(canvas)?;

    // Parse ANSI content
    for &byte in content {
        parser.process_byte(byte, &mut screen);
    }

    // Render to canvas
    renderer.render(&screen)?;

    // TODO: Implement baud rate simulation

    Ok(())
}
