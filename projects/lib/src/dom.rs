/// DOM manipulation utilities for creating and managing terminal elements.

use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{Document, Element, HtmlCanvasElement, Window};

/// Get the window object.
pub fn window() -> Result<Window, JsValue> {
    web_sys::window().ok_or_else(|| JsValue::from_str("No window object"))
}

/// Get the document object.
pub fn document() -> Result<Document, JsValue> {
    window()?
        .document()
        .ok_or_else(|| JsValue::from_str("No document object"))
}

/// Create a canvas element with the specified dimensions and display styles.
pub fn create_canvas(width: u32, height: u32) -> Result<HtmlCanvasElement, JsValue> {
    let doc = document()?;
    let canvas = doc
        .create_element("canvas")?
        .dyn_into::<HtmlCanvasElement>()?;

    canvas.set_width(width);
    canvas.set_height(height);

    // Generate a unique class name for this canvas instance
    let class_name = format!("webterm-canvas-{}", js_sys::Math::random().to_bits());
    canvas.set_class_name(&class_name);

    // Inject fullscreen CSS rules for this canvas
    inject_fullscreen_css(&doc, &class_name, width, height)?;

    // Apply CSS for responsive scaling
    let style = canvas.style();
    style.set_property("max-width", "100%")?;
    style.set_property("max-height", "100%")?;
    style.set_property("width", "100%")?;
    style.set_property("display", "block")?;
    style.set_property("image-rendering", "pixelated")?;
    style.set_property("image-rendering", "crisp-edges")?;

    Ok(canvas)
}

/// Inject CSS rules for fullscreen mode.
///
/// In fullscreen, the canvas should:
/// - Fill the screen height (unless aspect ratio would overflow width)
/// - Maintain native aspect ratio
/// - Be centered with black background
fn inject_fullscreen_css(doc: &Document, class_name: &str, width: u32, height: u32) -> Result<(), JsValue> {
    // Create style element with fullscreen rules
    let style = doc.create_element("style")?;

    // CSS for fullscreen mode:
    // - Black background fills the screen
    // - Canvas uses object-fit: contain to maintain aspect ratio
    // - Media queries select height or width constraint based on screen aspect ratio
    let css = format!(
        r#"
        .{class_name}:fullscreen {{
            background-color: black;
            object-fit: contain;
            max-width: none;
            max-height: none;
        }}

        @media (min-aspect-ratio: {width}/{height}) {{
            .{class_name}:fullscreen {{
                width: auto;
                height: 100vh;
            }}
        }}

        @media (max-aspect-ratio: {width}/{height}) {{
            .{class_name}:fullscreen {{
                width: 100vw;
                height: auto;
            }}
        }}
        "#,
        class_name = class_name,
        width = width,
        height = height
    );

    style.set_text_content(Some(&css));

    // Append to document head
    if let Some(head) = doc.head() {
        head.append_child(&style)?;
    }

    Ok(())
}

/// Create an offscreen canvas element (no styles, not attached to DOM).
pub fn create_offscreen_canvas(width: u32, height: u32) -> Result<HtmlCanvasElement, JsValue> {
    let doc = document()?;
    let canvas = doc
        .create_element("canvas")?
        .dyn_into::<HtmlCanvasElement>()?;

    canvas.set_width(width);
    canvas.set_height(height);

    Ok(canvas)
}

/// Find an element by CSS selector.
pub fn query_selector(selector: &str) -> Result<Option<Element>, JsValue> {
    document()?.query_selector(selector)
}

/// Find all elements with a specific attribute.
pub fn query_selector_all(selector: &str) -> Result<Vec<Element>, JsValue> {
    let doc = document()?;
    let node_list = doc.query_selector_all(selector)?;
    let mut elements = Vec::new();

    for i in 0..node_list.length() {
        if let Some(node) = node_list.get(i) {
            if let Some(element) = node.dyn_ref::<Element>() {
                elements.push(element.clone());
            }
        }
    }

    Ok(elements)
}

/// Get the value of a data attribute from an element.
pub fn get_data_attribute(element: &Element, attr: &str) -> Option<String> {
    element.get_attribute(&format!("data-{}", attr))
}
