/// ANSI escape sequence parser for VT-100/VT-102 sequences.
///
/// Parses ANSI escape sequences commonly used by DOS-era BBS systems.

use crate::screen::{Cell, Screen};

/// Actions that may occur during parsing that callers need to know about.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseAction {
    /// No special action occurred.
    None,
    /// The screen was cleared (ESC[2J).
    ScreenCleared,
    /// A line was scrolled off the top of the screen.
    LineScrolled,
}

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

    /// Check if the parser is in normal state (not processing an escape sequence).
    ///
    /// When in normal state, printable characters will be written to the screen.
    pub fn is_in_normal_state(&self) -> bool {
        self.state == ParserState::Normal
    }

    /// Check if the given byte will trigger a full screen clear (ESC[2J).
    ///
    /// This allows callers to capture the screen before it's cleared.
    pub fn will_clear_screen(&self, byte: u8) -> bool {
        // We're looking for ESC[2J - byte 'J' when in CSI state with param '2'
        if self.state != ParserState::Csi {
            return false;
        }
        if byte != b'J' {
            return false;
        }
        // Check if param is 2 (or will be 2 after pushing current_param)
        let param = if !self.current_param.is_empty() {
            self.current_param.parse::<u32>().unwrap_or(0)
        } else {
            self.params.first().copied().unwrap_or(0)
        };
        param == 2
    }

    /// Process a single byte and update the screen.
    ///
    /// Returns a `ParseAction` indicating if any special action occurred that
    /// the caller may need to handle (e.g., screen clear for scrollback capture).
    pub fn process_byte(&mut self, byte: u8, screen: &mut Screen) -> ParseAction {
        match self.state {
            ParserState::Normal => {
                if byte == 0x1B {  // ESC
                    self.state = ParserState::Escape;
                    ParseAction::None
                } else if byte == b'\n' {
                    self.handle_newline(screen)
                } else if byte == b'\r' {
                    self.handle_carriage_return(screen);
                    ParseAction::None
                } else if byte >= 32 {  // Printable characters
                    self.write_char(byte, screen)
                } else {
                    ParseAction::None
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
                ParseAction::None
            }
            ParserState::Csi => {
                if byte.is_ascii_digit() {
                    self.current_param.push(byte as char);
                    ParseAction::None
                } else if byte == b';' {
                    self.push_param();
                    ParseAction::None
                } else {
                    // Command byte
                    self.push_param();
                    let action = self.handle_csi_command(byte, screen);
                    self.state = ParserState::Normal;
                    action
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

    fn handle_csi_command(&mut self, cmd: u8, screen: &mut Screen) -> ParseAction {
        match cmd {
            b'H' | b'f' => { self.handle_cursor_position(screen); ParseAction::None }
            b'A' => { self.handle_cursor_up(screen); ParseAction::None }
            b'B' => { self.handle_cursor_down(screen); ParseAction::None }
            b'C' => { self.handle_cursor_forward(screen); ParseAction::None }
            b'D' => { self.handle_cursor_backward(screen); ParseAction::None }
            b'J' => self.handle_erase_display(screen),           // Erase display
            b'K' => { self.handle_erase_line(screen); ParseAction::None }
            b'm' => { self.handle_sgr(); ParseAction::None }
            _ => ParseAction::None  // Unknown command
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

    fn handle_erase_display(&self, screen: &mut Screen) -> ParseAction {
        let mode = self.params.get(0).copied().unwrap_or(0);
        match mode {
            2 => {
                screen.clear_with_bg(self.effective_bg());  // Clear entire screen with current bg
                ParseAction::ScreenCleared
            }
            _ => ParseAction::None  // TODO: Implement other erase modes
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

    fn write_char(&self, ch: u8, screen: &mut Screen) -> ParseAction {
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
            ParseAction::None
        } else {
            // Line wrap: move to start of next line
            if y + 1 < height {
                screen.set_cursor(0, y + 1);
                ParseAction::None
            } else {
                // At bottom of screen, scroll up
                screen.scroll_up();
                screen.set_cursor(0, y);
                ParseAction::LineScrolled
            }
        }
    }

    fn handle_newline(&self, screen: &mut Screen) -> ParseAction {
        let (_, y) = screen.cursor_pos();
        let (_, height) = screen.dimensions();
        if y + 1 < height {
            screen.set_cursor(0, y + 1);
            ParseAction::None
        } else {
            // At bottom of screen, scroll up
            screen.scroll_up();
            screen.set_cursor(0, y);
            ParseAction::LineScrolled
        }
    }

    fn handle_carriage_return(&self, screen: &mut Screen) {
        let (_, y) = screen.cursor_pos();
        screen.set_cursor(0, y);
    }
}
