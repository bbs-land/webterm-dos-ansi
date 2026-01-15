/// Screen buffer for 80x25 text mode terminal.
///
/// Represents the complete terminal state including character cells,
/// colors, and attributes.
pub struct Screen {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
    cursor_x: usize,
    cursor_y: usize,
}

/// A single character cell in the terminal.
#[derive(Clone, Copy)]
pub struct Cell {
    pub ch: u8,        // CP437 character code
    pub fg: u8,        // Foreground color (0-15)
    pub bg: u8,        // Background color (0-15)
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            ch: b' ',
            fg: 7,  // White
            bg: 0,  // Black
        }
    }
}

impl Screen {
    /// Create a new 80x25 screen buffer.
    pub fn new() -> Self {
        Screen {
            width: 80,
            height: 25,
            cells: vec![Cell::default(); 80 * 25],
            cursor_x: 0,
            cursor_y: 0,
        }
    }

    /// Get a cell at the specified position.
    pub fn get_cell(&self, x: usize, y: usize) -> Option<&Cell> {
        if x < self.width && y < self.height {
            Some(&self.cells[y * self.width + x])
        } else {
            None
        }
    }

    /// Set a cell at the specified position.
    pub fn set_cell(&mut self, x: usize, y: usize, cell: Cell) {
        if x < self.width && y < self.height {
            self.cells[y * self.width + x] = cell;
        }
    }

    /// Get cursor position.
    pub fn cursor_pos(&self) -> (usize, usize) {
        (self.cursor_x, self.cursor_y)
    }

    /// Set cursor position.
    pub fn set_cursor(&mut self, x: usize, y: usize) {
        self.cursor_x = x.min(self.width - 1);
        self.cursor_y = y.min(self.height - 1);
    }

    /// Clear the screen with default colors.
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.clear_with_bg(0);
    }

    /// Clear the screen with specified background color.
    pub fn clear_with_bg(&mut self, bg: u8) {
        for cell in &mut self.cells {
            cell.ch = b' ';
            cell.fg = 7;
            cell.bg = bg;
        }
        self.cursor_x = 0;
        self.cursor_y = 0;
    }

    /// Get screen dimensions.
    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    /// Scroll the screen up by one line.
    /// The top line is discarded and a new blank line is added at the bottom.
    pub fn scroll_up(&mut self) {
        // Move all lines up by one
        for y in 0..(self.height - 1) {
            for x in 0..self.width {
                let src_idx = (y + 1) * self.width + x;
                let dst_idx = y * self.width + x;
                self.cells[dst_idx] = self.cells[src_idx];
            }
        }
        // Clear the bottom line
        let bottom_start = (self.height - 1) * self.width;
        for x in 0..self.width {
            self.cells[bottom_start + x] = Cell::default();
        }
    }
}
