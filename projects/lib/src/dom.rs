/// DOM manipulation utilities for creating and managing terminal elements.

use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{Document, Element, HtmlCanvasElement, HtmlElement, Window};

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

/// Create a canvas element with the specified dimensions.
pub fn create_canvas(width: u32, height: u32) -> Result<HtmlCanvasElement, JsValue> {
    let doc = document()?;
    let canvas = doc
        .create_element("canvas")?
        .dyn_into::<HtmlCanvasElement>()?;

    canvas.set_width(width);
    canvas.set_height(height);

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

/// Append a child element to a parent.
pub fn append_child(parent: &Element, child: &HtmlElement) -> Result<(), JsValue> {
    parent.append_child(child)?;
    Ok(())
}
