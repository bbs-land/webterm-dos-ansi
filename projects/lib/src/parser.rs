/// ANSI escape sequence parser for VT-100/VT-102 sequences.
///
/// Parses ANSI escape sequences commonly used by DOS-era BBS systems.

use crate::screen::{Cell, Screen};

/// ANSI parser state machine.
pub struct AnsiParser {
    state: ParserState,
    params: Vec<u32>,
    current_param: String,
    current_fg: u8,
    current_bg: u8,
    bold: bool,
    blink: bool,
    reverse: bool,
}

#[derive(Debug, PartialEq)]
enum ParserState {
    Normal,
    Escape,
    Csi,  // Control Sequence Introducer (ESC[)
}

impl AnsiParser {
    pub fn new() -> Self {
        AnsiParser {
            state: ParserState::Normal,
            params: Vec::new(),
            current_param: String::new(),
            current_fg: 7,  // White
            current_bg: 0,  // Black
            bold: false,
            blink: false,
            reverse: false,
        }
    }

    /// Process a single byte and update the screen.
    pub fn process_byte(&mut self, byte: u8, screen: &mut Screen) {
        match self.state {
            ParserState::Normal => {
                if byte == 0x1B {  // ESC
                    self.state = ParserState::Escape;
                } else if byte == b'\n' {
                    self.handle_newline(screen);
                } else if byte == b'\r' {
                    self.handle_carriage_return(screen);
                } else if byte >= 32 {  // Printable characters
                    self.write_char(byte, screen);
                }
            }
            ParserState::Escape => {
                if byte == b'[' {
                    self.state = ParserState::Csi;
                    self.params.clear();
                    self.current_param.clear();
                } else {
                    // Unknown escape sequence, return to normal
                    self.state = ParserState::Normal;
                }
            }
            ParserState::Csi => {
                if byte.is_ascii_digit() {
                    self.current_param.push(byte as char);
                } else if byte == b';' {
                    self.push_param();
                } else {
                    // Command byte
                    self.push_param();
                    self.handle_csi_command(byte, screen);
                    self.state = ParserState::Normal;
                }
            }
        }
    }

    fn push_param(&mut self) {
        if !self.current_param.is_empty() {
            if let Ok(val) = self.current_param.parse() {
                self.params.push(val);
            }
            self.current_param.clear();
        }
    }

    fn handle_csi_command(&mut self, cmd: u8, screen: &mut Screen) {
        match cmd {
            b'H' | b'f' => self.handle_cursor_position(screen),  // Cursor position
            b'A' => self.handle_cursor_up(screen),               // Cursor up
            b'B' => self.handle_cursor_down(screen),             // Cursor down
            b'C' => self.handle_cursor_forward(screen),          // Cursor forward
            b'D' => self.handle_cursor_backward(screen),         // Cursor backward
            b'J' => self.handle_erase_display(screen),           // Erase display
            b'K' => self.handle_erase_line(screen),              // Erase line
            b'm' => self.handle_sgr(),                           // Select Graphic Rendition
            _ => {}  // Unknown command
        }
    }

    fn handle_cursor_position(&self, screen: &mut Screen) {
        let row = self.params.get(0).copied().unwrap_or(1).saturating_sub(1) as usize;
        let col = self.params.get(1).copied().unwrap_or(1).saturating_sub(1) as usize;
        screen.set_cursor(col, row);
    }

    fn handle_cursor_up(&self, screen: &mut Screen) {
        let n = self.params.get(0).copied().unwrap_or(1) as usize;
        let (x, y) = screen.cursor_pos();
        screen.set_cursor(x, y.saturating_sub(n));
    }

    fn handle_cursor_down(&self, screen: &mut Screen) {
        let n = self.params.get(0).copied().unwrap_or(1) as usize;
        let (x, y) = screen.cursor_pos();
        screen.set_cursor(x, y + n);
    }

    fn handle_cursor_forward(&self, screen: &mut Screen) {
        let n = self.params.get(0).copied().unwrap_or(1) as usize;
        let (x, y) = screen.cursor_pos();
        screen.set_cursor(x + n, y);
    }

    fn handle_cursor_backward(&self, screen: &mut Screen) {
        let n = self.params.get(0).copied().unwrap_or(1) as usize;
        let (x, y) = screen.cursor_pos();
        screen.set_cursor(x.saturating_sub(n), y);
    }

    fn handle_erase_display(&self, screen: &mut Screen) {
        let mode = self.params.get(0).copied().unwrap_or(0);
        match mode {
            2 => screen.clear_with_bg(self.effective_bg()),  // Clear entire screen with current bg
            _ => {}  // TODO: Implement other erase modes
        }
    }

    fn handle_erase_line(&self, _screen: &mut Screen) {
        // TODO: Implement line erasing
    }

    fn handle_sgr(&mut self) {
        if self.params.is_empty() {
            // Reset all attributes
            self.current_fg = 7;
            self.current_bg = 0;
            self.bold = false;
            self.blink = false;
            self.reverse = false;
            return;
        }

        for &param in &self.params {
            match param {
                0 => {
                    // Reset
                    self.current_fg = 7;
                    self.current_bg = 0;
                    self.bold = false;
                    self.blink = false;
                    self.reverse = false;
                }
                1 => self.bold = true,
                5 => self.blink = true,
                7 => self.reverse = true,
                30..=37 => self.current_fg = (param - 30) as u8,  // Foreground colors
                40..=47 => self.current_bg = (param - 40) as u8,  // Background colors
                90..=97 => self.current_fg = (param - 90 + 8) as u8,  // Bright foreground
                100..=107 => self.current_bg = (param - 100 + 8) as u8,  // Bright background
                _ => {}
            }
        }
    }

    /// Get the effective foreground color (applying bold and reverse)
    fn effective_fg(&self) -> u8 {
        let fg = if self.reverse { self.current_bg } else { self.current_fg };
        // Bold makes foreground bright (add 8 if not already bright)
        if self.bold && fg < 8 { fg + 8 } else { fg }
    }

    /// Get the effective background color (applying blink and reverse)
    fn effective_bg(&self) -> u8 {
        let bg = if self.reverse { self.current_fg } else { self.current_bg };
        // Blink makes background bright (DOS behavior - no actual blinking)
        if self.blink && bg < 8 { bg + 8 } else { bg }
    }

    fn write_char(&self, ch: u8, screen: &mut Screen) {
        let (x, y) = screen.cursor_pos();
        let cell = Cell {
            ch,
            fg: self.effective_fg(),
            bg: self.effective_bg(),
        };
        screen.set_cell(x, y, cell);

        // Move cursor forward
        let (width, height) = screen.dimensions();
        if x + 1 < width {
            screen.set_cursor(x + 1, y);
        } else {
            // Line wrap: move to start of next line
            if y + 1 < height {
                screen.set_cursor(0, y + 1);
            } else {
                // At bottom of screen, scroll up
                screen.scroll_up();
                screen.set_cursor(0, y);
            }
        }
    }

    fn handle_newline(&self, screen: &mut Screen) {
        let (_, y) = screen.cursor_pos();
        let (_, height) = screen.dimensions();
        if y + 1 < height {
            screen.set_cursor(0, y + 1);
        } else {
            // At bottom of screen, scroll up
            screen.scroll_up();
            screen.set_cursor(0, y);
        }
    }

    fn handle_carriage_return(&self, screen: &mut Screen) {
        let (_, y) = screen.cursor_pos();
        screen.set_cursor(0, y);
    }
}
