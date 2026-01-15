/// Canvas renderer for terminal output.
///
/// Renders the terminal screen buffer to an HTML5 canvas with EGA font
/// and proper aspect ratio correction (6x8 pixel scaling for 2x resolution).

use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

use crate::font;
use crate::screen::Screen;

/// Canvas dimensions (3x4 scaling per EGA pixel)
pub const CANVAS_WIDTH: u32 = 1920;   // 80 * 8 * 3
pub const CANVAS_HEIGHT: u32 = 1400;  // 25 * 14 * 4

/// Color palette type
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum Palette {
    /// IBM 5153-accurate CGA colors
    Cga,
    /// Standard EGA/VGA colors (default)
    #[default]
    Vga,
}

impl Palette {
    /// Parse palette from string (case-insensitive)
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "cga" => Palette::Cga,
            _ => Palette::Vga,
        }
    }
}

/// Terminal renderer that draws to a canvas.
pub struct Renderer {
    context: CanvasRenderingContext2d,
    palette: Palette,
}

impl Renderer {
    /// Create a new renderer for the given canvas with default (CGA) palette.
    pub fn new(canvas: &HtmlCanvasElement) -> Result<Self, JsValue> {
        Self::with_palette(canvas, Palette::default())
    }

    /// Create a new renderer for the given canvas with specified palette.
    pub fn with_palette(canvas: &HtmlCanvasElement, palette: Palette) -> Result<Self, JsValue> {
        let context = canvas
            .get_context("2d")?
            .ok_or("Failed to get 2d context")?
            .dyn_into::<CanvasRenderingContext2d>()?;

        Ok(Renderer { context, palette })
    }

    /// Render the screen buffer to the canvas.
    pub fn render(&self, screen: &Screen) -> Result<(), JsValue> {
        // Clear canvas to black
        self.context.set_fill_style_str("#000000");
        self.context.fill_rect(0.0, 0.0, CANVAS_WIDTH as f64, CANVAS_HEIGHT as f64);

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
        // Each character is 8x14 pixels, scaled to 24x56 (3x4) for aspect ratio correction
        const SCALE_X: usize = 3;
        const SCALE_Y: usize = 4;
        const CELL_WIDTH: usize = font::FONT_WIDTH * SCALE_X;   // 24
        const CELL_HEIGHT: usize = font::FONT_HEIGHT * SCALE_Y; // 56

        let px = x * CELL_WIDTH;
        let py = y * CELL_HEIGHT;

        // Get font bitmap for this character
        let bitmap = font::get_char_bitmap(cell.ch);

        // Get foreground and background colors as RGB tuples
        let fg_rgb = ansi_color_to_rgb_tuple(cell.fg, self.palette);
        let bg_rgb = ansi_color_to_rgb_tuple(cell.bg, self.palette);

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
fn ansi_color_to_rgb_tuple(color: u8, palette: Palette) -> (u8, u8, u8) {
    match palette {
        Palette::Cga => ansi_color_cga(color),
        Palette::Vga => ansi_color_vga(color),
    }
}

/// IBM 5153-accurate CGA colors for authentic CRT appearance.
fn ansi_color_cga(color: u8) -> (u8, u8, u8) {
    match color {
        0 => (0x00, 0x00, 0x00),  // Black
        1 => (0xC4, 0x00, 0x00),  // Red
        2 => (0x00, 0xC4, 0x00),  // Green
        3 => (0xC4, 0x7E, 0x00),  // Brown
        4 => (0x00, 0x00, 0xC4),  // Blue
        5 => (0xC4, 0x00, 0xC4),  // Magenta
        6 => (0x00, 0xC4, 0xC4),  // Cyan
        7 => (0xC4, 0xC4, 0xC4),  // Light Gray
        8 => (0x4E, 0x4E, 0x4E),  // Dark Gray
        9 => (0xDC, 0x4E, 0x4E),  // Light Red
        10 => (0x4E, 0xDC, 0x4E), // Light Green
        11 => (0xF3, 0xF3, 0x4E), // Yellow
        12 => (0x4E, 0x4E, 0xDC), // Light Blue
        13 => (0xF3, 0x4E, 0xF3), // Light Magenta
        14 => (0x4E, 0xF3, 0xF3), // Light Cyan
        15 => (0xFF, 0xFF, 0xFF), // White
        _ => (0xC4, 0xC4, 0xC4),  // Default to light gray
    }
}

/// Standard VGA colors.
fn ansi_color_vga(color: u8) -> (u8, u8, u8) {
    match color {
        0 => (0x00, 0x00, 0x00),  // Black
        1 => (0xAA, 0x00, 0x00),  // Red
        2 => (0x00, 0xAA, 0x00),  // Green
        3 => (0xAA, 0x55, 0x00),  // Brown/Yellow
        4 => (0x00, 0x00, 0xAA),  // Blue
        5 => (0xAA, 0x00, 0xAA),  // Magenta
        6 => (0x00, 0xAA, 0xAA),  // Cyan
        7 => (0xAA, 0xAA, 0xAA),  // Light Gray
        8 => (0x55, 0x55, 0x55),  // Dark Gray
        9 => (0xFF, 0x55, 0x55),  // Light Red
        10 => (0x55, 0xFF, 0x55), // Light Green
        11 => (0xFF, 0xFF, 0x55), // Yellow
        12 => (0x55, 0x55, 0xFF), // Light Blue
        13 => (0xFF, 0x55, 0xFF), // Light Magenta
        14 => (0x55, 0xFF, 0xFF), // Light Cyan
        15 => (0xFF, 0xFF, 0xFF), // White
        _ => (0xAA, 0xAA, 0xAA),  // Default to light gray
    }
}

