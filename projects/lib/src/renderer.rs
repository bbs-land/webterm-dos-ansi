/// Canvas renderer for terminal output.
///
/// Renders the terminal screen buffer to an HTML5 canvas with EGA font
/// and proper aspect ratio correction (3x4 pixel scaling).

use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

use crate::font;
use crate::screen::Screen;

/// Terminal renderer that draws to a canvas.
pub struct Renderer {
    context: CanvasRenderingContext2d,
}

impl Renderer {
    /// Create a new renderer for the given canvas.
    pub fn new(canvas: HtmlCanvasElement) -> Result<Self, JsValue> {
        let context = canvas
            .get_context("2d")?
            .ok_or("Failed to get 2d context")?
            .dyn_into::<CanvasRenderingContext2d>()?;

        Ok(Renderer { context })
    }

    /// Render the screen buffer to the canvas.
    pub fn render(&self, screen: &Screen) -> Result<(), JsValue> {
        // Clear canvas to black
        self.context.set_fill_style_str("#000000");
        self.context.fill_rect(0.0, 0.0, 1920.0, 1400.0);

        // Render each character cell
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
        const SCALE_X: usize = 3;
        const SCALE_Y: usize = 4;
        const CELL_WIDTH: usize = font::FONT_WIDTH * SCALE_X;  // 24
        const CELL_HEIGHT: usize = font::FONT_HEIGHT * SCALE_Y; // 56

        let px = x * CELL_WIDTH;
        let py = y * CELL_HEIGHT;

        // Get font bitmap for this character
        let bitmap = font::get_char_bitmap(cell.ch);

        // Get foreground and background colors as RGB tuples
        let fg_rgb = ansi_color_to_rgb_tuple(cell.fg);
        let bg_rgb = ansi_color_to_rgb_tuple(cell.bg);

        // Create ImageData for this cell (24x56 pixels, RGBA format)
        let mut pixel_data: Vec<u8> = Vec::with_capacity(CELL_WIDTH * CELL_HEIGHT * 4);

        // Render each scanline of the character
        for font_y in 0..font::FONT_HEIGHT {
            let scanline = bitmap[font_y];

            // Each font scanline is rendered SCALE_Y times vertically
            for _ in 0..SCALE_Y {
                // Render each pixel in the scanline
                for font_x in 0..font::FONT_WIDTH {
                    let is_set = font::is_pixel_set(scanline, font_x as u8);
                    let (r, g, b) = if is_set { fg_rgb } else { bg_rgb };

                    // Each font pixel is rendered SCALE_X times horizontally
                    for _ in 0..SCALE_X {
                        pixel_data.push(r);
                        pixel_data.push(g);
                        pixel_data.push(b);
                        pixel_data.push(255); // Alpha
                    }
                }
            }
        }

        // Create ImageData and put it on the canvas
        let image_data = ImageData::new_with_u8_clamped_array_and_sh(
            wasm_bindgen::Clamped(&pixel_data),
            CELL_WIDTH as u32,
            CELL_HEIGHT as u32,
        )?;

        self.context.put_image_data(&image_data, px as f64, py as f64)?;

        Ok(())
    }
}

/// Convert ANSI color code (0-15) to RGB tuple (r, g, b).
fn ansi_color_to_rgb_tuple(color: u8) -> (u8, u8, u8) {
    match color {
        0 => (0x00, 0x00, 0x00),  // Black
        1 => (0xAA, 0x00, 0x00),  // Red
        2 => (0x00, 0xAA, 0x00),  // Green
        3 => (0xAA, 0x55, 0x00),  // Yellow
        4 => (0x00, 0x00, 0xAA),  // Blue
        5 => (0xAA, 0x00, 0xAA),  // Magenta
        6 => (0x00, 0xAA, 0xAA),  // Cyan
        7 => (0xAA, 0xAA, 0xAA),  // White
        8 => (0x55, 0x55, 0x55),  // Bright Black (Gray)
        9 => (0xFF, 0x55, 0x55),  // Bright Red
        10 => (0x55, 0xFF, 0x55), // Bright Green
        11 => (0xFF, 0xFF, 0x55), // Bright Yellow
        12 => (0x55, 0x55, 0xFF), // Bright Blue
        13 => (0xFF, 0x55, 0xFF), // Bright Magenta
        14 => (0x55, 0xFF, 0xFF), // Bright Cyan
        15 => (0xFF, 0xFF, 0xFF), // Bright White
        _ => (0xAA, 0xAA, 0xAA),  // Default to white
    }
}

