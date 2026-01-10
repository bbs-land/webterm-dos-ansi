/// Canvas renderer for terminal output.
///
/// Renders the terminal screen buffer to an HTML5 canvas with EGA font
/// and proper aspect ratio correction (3x4 pixel scaling).

use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use crate::screen::Screen;

/// Terminal renderer that draws to a canvas.
pub struct Renderer {
    canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
}

impl Renderer {
    /// Create a new renderer for the given canvas.
    pub fn new(canvas: HtmlCanvasElement) -> Result<Self, JsValue> {
        let context = canvas
            .get_context("2d")?
            .ok_or("Failed to get 2d context")?
            .dyn_into::<CanvasRenderingContext2d>()?;

        Ok(Renderer { canvas, context })
    }

    /// Render the screen buffer to the canvas.
    pub fn render(&self, screen: &Screen) -> Result<(), JsValue> {
        // Clear canvas
        self.context.set_fill_style_str("#000000");
        self.context.fill_rect(0.0, 0.0, 1920.0, 1400.0);

        // TODO: Render each character cell
        // For now, just draw a test pattern
        self.context.set_fill_style_str("#00FF00");
        self.context.fill_rect(10.0, 10.0, 100.0, 100.0);

        let (width, height) = screen.dimensions();
        for y in 0..height {
            for x in 0..width {
                if let Some(cell) = screen.get_cell(x, y) {
                    self.render_cell(x, y, cell)?;
                }
            }
        }

        Ok(())
    }

    /// Render a single character cell.
    fn render_cell(&self, x: usize, y: usize, cell: &crate::screen::Cell) -> Result<(), JsValue> {
        // Each character is 8x14 pixels, scaled to 24x56 (3x4)
        let cell_width = 24.0;
        let cell_height = 56.0;
        let px = x as f64 * cell_width;
        let py = y as f64 * cell_height;

        // Draw background
        let bg_color = ansi_color_to_rgb(cell.bg);
        self.context.set_fill_style_str(&bg_color);
        self.context.fill_rect(px, py, cell_width, cell_height);

        // TODO: Draw character glyph from EGA font
        // For now, draw a placeholder rectangle for non-space characters
        if cell.ch != b' ' {
            let fg_color = ansi_color_to_rgb(cell.fg);
            self.context.set_fill_style_str(&fg_color);
            self.context.fill_rect(px + 4.0, py + 8.0, 16.0, 40.0);
        }

        Ok(())
    }
}

/// Convert ANSI color code (0-15) to RGB hex string.
fn ansi_color_to_rgb(color: u8) -> String {
    match color {
        0 => "#000000".to_string(),  // Black
        1 => "#AA0000".to_string(),  // Red
        2 => "#00AA00".to_string(),  // Green
        3 => "#AA5500".to_string(),  // Yellow
        4 => "#0000AA".to_string(),  // Blue
        5 => "#AA00AA".to_string(),  // Magenta
        6 => "#00AAAA".to_string(),  // Cyan
        7 => "#AAAAAA".to_string(),  // White
        8 => "#555555".to_string(),  // Bright Black (Gray)
        9 => "#FF5555".to_string(),  // Bright Red
        10 => "#55FF55".to_string(), // Bright Green
        11 => "#FFFF55".to_string(), // Bright Yellow
        12 => "#5555FF".to_string(), // Bright Blue
        13 => "#FF55FF".to_string(), // Bright Magenta
        14 => "#55FFFF".to_string(), // Bright Cyan
        15 => "#FFFFFF".to_string(), // Bright White
        _ => "#AAAAAA".to_string(),  // Default to white
    }
}
