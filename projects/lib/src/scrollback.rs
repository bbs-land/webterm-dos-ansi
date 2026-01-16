/// Scrollback buffer for terminal history.
///
/// Stores terminal history in CGA-compatible format (2 bytes per character)
/// and manages scrollback viewing mode.

use crate::screen::{Cell, Screen};

/// Default maximum number of lines to retain in scrollback history.
pub const DEFAULT_MAX_LINES: usize = 5000;

/// Width of the terminal in characters.
const SCREEN_WIDTH: usize = 80;

/// Height of the terminal in characters.
const SCREEN_HEIGHT: usize = 25;

/// Bytes per line in CGA format (80 chars * 2 bytes).
const LINE_BYTES: usize = SCREEN_WIDTH * 2;

/// Scrollback buffer storing terminal history in CGA format.
///
/// ## CGA Format
/// Each character is stored as 2 bytes:
/// - Byte 0: Character code (CP437)
/// - Byte 1: Attribute byte
///   - Bits 7-4: Background color (0-15)
///   - Bits 3-0: Foreground color (0-15)
pub struct ScrollbackBuffer {
    /// History buffer: Vec of lines, each line is 160 bytes (80 chars * 2 bytes)
    history: Vec<[u8; LINE_BYTES]>,
    /// Maximum history lines to retain
    max_lines: usize,
    /// Whether scrollback mode is active
    active: bool,
    /// Whether scrollback was entered via keyboard (Alt+K) - prevents auto-exit on scroll to bottom
    keyboard_entry: bool,
    /// For mouse mode: offset from the END of virtual buffer (0 = live screen)
    /// For keyboard mode: absolute index into history where viewport starts
    viewport_position: usize,
    /// Whether we're animating scroll-to-bottom exit
    animating_exit: bool,
    /// Viewer mode: like keyboard mode but hides indicators and never auto-exits
    /// Used for instant render (no BPS) to show content from the top
    viewer_mode: bool,
}

impl ScrollbackBuffer {
    /// Create a new scrollback buffer with the default maximum lines.
    pub fn new() -> Self {
        Self::with_max_lines(DEFAULT_MAX_LINES)
    }

    /// Create a new scrollback buffer with a specified maximum lines.
    pub fn with_max_lines(max_lines: usize) -> Self {
        ScrollbackBuffer {
            history: Vec::new(),
            max_lines,
            active: false,
            keyboard_entry: false,
            viewport_position: 0,
            animating_exit: false,
            viewer_mode: false,
        }
    }

    /// Convert a Cell to CGA format bytes.
    #[inline]
    fn cell_to_cga(cell: &Cell) -> [u8; 2] {
        // Byte 0: character
        // Byte 1: attribute (bg in high nibble, fg in low nibble)
        let attr = ((cell.bg & 0x0F) << 4) | (cell.fg & 0x0F);
        [cell.ch, attr]
    }

    /// Convert CGA format bytes back to a Cell.
    #[cfg(test)]
    pub fn cga_to_cell(cga: [u8; 2]) -> Cell {
        Cell {
            ch: cga[0],
            fg: cga[1] & 0x0F,
            bg: (cga[1] >> 4) & 0x0F,
        }
    }

    /// Push a single line to the history buffer.
    ///
    /// The line should be exactly SCREEN_WIDTH cells.
    pub fn push_line(&mut self, cells: &[Cell]) {
        let mut line = [0u8; LINE_BYTES];
        for (i, cell) in cells.iter().take(SCREEN_WIDTH).enumerate() {
            let cga = Self::cell_to_cga(cell);
            line[i * 2] = cga[0];
            line[i * 2 + 1] = cga[1];
        }
        self.history.push(line);

        // For mouse mode with offset > 0 (not at bottom), increment offset to keep view sticky
        // This makes the viewport stay at the same position as new content comes in
        // But NOT during exit animation - we want to scroll towards the bottom
        if self.active && !self.keyboard_entry && !self.animating_exit && self.viewport_position > 0 {
            self.viewport_position += 1;
        }

        // Trim if over max_lines
        if self.history.len() > self.max_lines {
            self.history.remove(0);
            // For keyboard mode, adjust absolute position when history is trimmed
            if self.keyboard_entry && self.viewport_position > 0 {
                self.viewport_position = self.viewport_position.saturating_sub(1);
            }
            // For mouse mode (not animating), adjust offset when history is trimmed
            // (offset is from end, but we removed from start, so content shifted)
            // During animation, don't adjust - animation is scrolling toward live anyway
            if !self.keyboard_entry && !self.animating_exit && self.viewport_position > 0 {
                self.viewport_position = self.viewport_position.saturating_sub(1);
            }
        }
    }

    /// Push all lines of the current screen to history.
    ///
    /// Called before a screen clear to preserve the display.
    pub fn push_screen(&mut self, screen: &Screen) {
        for y in 0..SCREEN_HEIGHT {
            let mut cells = Vec::with_capacity(SCREEN_WIDTH);
            for x in 0..SCREEN_WIDTH {
                if let Some(cell) = screen.get_cell(x, y) {
                    cells.push(*cell);
                }
            }
            self.push_line(&cells);
        }
    }

    /// Enter scrollback viewing mode via mouse wheel.
    ///
    /// Mouse mode: viewport_position is offset from END of virtual buffer.
    /// 0 = live screen, >0 = scrolled back N lines from end.
    pub fn enter_scrollback(&mut self) {
        if !self.active && !self.history.is_empty() {
            self.active = true;
            self.keyboard_entry = false;
            // Start at offset 0 from end (viewing live screen)
            self.viewport_position = 0;
        }
    }

    /// Enter scrollback viewing mode via keyboard (Alt+K).
    ///
    /// Keyboard mode: viewport_position is absolute index into history.
    /// Position stays fixed as new content comes in (content appears to scroll below).
    pub fn enter_scrollback_keyboard(&mut self) {
        if !self.active && !self.history.is_empty() {
            self.active = true;
            self.keyboard_entry = true;
            // Start showing the current screen (not scrolled back).
            // Virtual buffer is [history...][current_screen_25_lines].
            // viewport_position = history.len() means viewport starts at the current screen.
            self.viewport_position = self.history.len();
        }
    }

    /// Enter viewer mode - scrollback at the top of content with no indicators.
    ///
    /// Used for instant render (no BPS) to show content from the beginning.
    /// In viewer mode:
    /// - No "SCROLLBACK" indicators are shown
    /// - Scrolling to bottom does NOT auto-exit
    /// - Viewport starts at the top (position 0)
    pub fn enter_viewer_mode(&mut self) {
        if !self.history.is_empty() {
            self.active = true;
            self.keyboard_entry = true; // Use keyboard-style absolute positioning
            self.viewer_mode = true;
            // Start at the very top of history
            self.viewport_position = 0;
        }
    }

    /// Check if currently in viewer mode.
    ///
    /// Viewer mode is used for instant render - click should not exit.
    pub fn is_viewer_mode(&self) -> bool {
        self.viewer_mode
    }

    /// Check if scrollback indicators should be shown.
    ///
    /// Returns true if in scrollback mode but NOT in viewer mode.
    pub fn should_show_indicators(&self) -> bool {
        self.active && !self.viewer_mode
    }

    /// Exit scrollback viewing mode immediately (no animation).
    pub fn exit_scrollback(&mut self) {
        self.active = false;
        self.keyboard_entry = false;
        self.viewport_position = 0;
        self.animating_exit = false;
        self.viewer_mode = false;
    }

    /// Start animated exit - scrolls to bottom over time.
    /// Returns true if animation started, false if already at bottom or not active.
    pub fn start_animated_exit(&mut self) -> bool {
        if !self.active {
            return false;
        }

        // Calculate current offset from bottom for animation
        let offset_from_bottom = if self.keyboard_entry {
            // Keyboard mode: convert absolute position to offset from end
            self.history.len().saturating_sub(self.viewport_position)
        } else {
            self.viewport_position
        };

        if offset_from_bottom == 0 {
            // Already at bottom, just exit
            self.exit_scrollback();
            return false;
        }

        // Convert to mouse-style offset for animation
        self.keyboard_entry = false;
        self.viewport_position = offset_from_bottom;
        self.animating_exit = true;
        true
    }

    /// Advance the exit animation by one frame.
    /// Returns true if animation is still running, false if complete.
    /// Call this at ~60fps, it will scroll at 360 lines/sec (6 lines per frame).
    pub fn animate_exit_frame(&mut self) -> bool {
        if !self.animating_exit || !self.active {
            return false;
        }

        // 360 lines per second at 60fps = 6 lines per frame
        let lines_per_frame = 6;

        if self.viewport_position <= lines_per_frame {
            // Animation complete
            self.exit_scrollback();
            false
        } else {
            self.viewport_position -= lines_per_frame;
            true
        }
    }

    /// Check if currently animating exit.
    pub fn is_animating_exit(&self) -> bool {
        self.animating_exit
    }

    /// Toggle scrollback viewing mode (keyboard-initiated).
    pub fn toggle_scrollback(&mut self) {
        // If animating exit, cancel and stay at current position
        if self.animating_exit {
            self.animating_exit = false;
            // Switch to keyboard mode at current position
            // Convert mouse-style offset to keyboard-style absolute position
            let total_virtual_lines = self.history.len() + SCREEN_HEIGHT;
            let view_start = total_virtual_lines.saturating_sub(SCREEN_HEIGHT + self.viewport_position);
            self.viewport_position = view_start;
            self.keyboard_entry = true;
            return;
        }

        if self.active {
            self.start_animated_exit();
        } else {
            self.enter_scrollback_keyboard();
        }
    }

    /// Check if scrollback mode is active.
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Get the current viewport position.
    #[cfg(test)]
    pub fn viewport_position(&self) -> usize {
        self.viewport_position
    }

    /// Get the total number of lines in history.
    #[cfg(test)]
    pub fn history_len(&self) -> usize {
        self.history.len()
    }

    /// Scroll up (back in history) by the specified number of lines.
    ///
    /// Entering scrollback mode if not already active.
    /// If animating exit, cancels animation and continues scrolling from current position.
    pub fn scroll_up(&mut self, lines: usize) {
        // If animating exit, cancel and stay in mouse mode at current position
        if self.animating_exit {
            self.animating_exit = false;
            // Already in mouse mode with correct viewport_position, just continue scrolling
            self.viewport_position = (self.viewport_position + lines).min(self.history.len());
            return;
        }

        if !self.active {
            self.enter_scrollback();
        }
        if self.active {
            if self.keyboard_entry {
                // Keyboard mode: decrease absolute position (move earlier in history)
                self.viewport_position = self.viewport_position.saturating_sub(lines);
            } else {
                // Mouse mode: increase offset from end
                self.viewport_position = (self.viewport_position + lines).min(self.history.len());
            }
        }
    }

    /// Scroll down (toward present) by the specified number of lines.
    ///
    /// Exits scrollback mode if scrolling to the bottom (only for mouse-initiated scrollback).
    pub fn scroll_down(&mut self, lines: usize) {
        if self.active {
            if self.keyboard_entry {
                // Keyboard mode: increase absolute position (move later in history)
                // Max is history.len() (can view up to current content)
                self.viewport_position = (self.viewport_position + lines).min(self.history.len());
            } else {
                // Mouse mode: decrease offset from end
                if self.viewport_position <= lines {
                    // At bottom - auto-exit for mouse mode
                    self.exit_scrollback();
                } else {
                    self.viewport_position -= lines;
                }
            }
        }
    }

    /// Scroll up by one full page (SCREEN_HEIGHT lines).
    pub fn page_up(&mut self) {
        self.scroll_up(SCREEN_HEIGHT);
    }

    /// Scroll down by one full page (SCREEN_HEIGHT lines).
    pub fn page_down(&mut self) {
        self.scroll_down(SCREEN_HEIGHT);
    }

    /// Get the line to display at a given screen row.
    ///
    /// Returns the CGA-format line bytes for the given row, taking into account
    /// the current viewport position. Returns None if the row is out of bounds.
    ///
    /// Virtual buffer model: [history...][current_screen_25_lines]
    /// - Mouse mode: viewport_position is offset from END (0 = live screen)
    /// - Keyboard mode: viewport_position is absolute index (fixed position as content grows)
    pub fn get_display_line(&self, y: usize, screen: &Screen) -> Option<[u8; LINE_BYTES]> {
        if y >= SCREEN_HEIGHT {
            return None;
        }

        if !self.active {
            // Not in scrollback mode - show current screen
            return self.screen_line_to_cga(screen, y);
        }

        // Calculate the absolute line index to display
        let line_index = if self.keyboard_entry {
            // Keyboard mode: viewport_position is absolute start index
            self.viewport_position + y
        } else {
            // Mouse mode: viewport_position is offset from end
            if self.viewport_position == 0 {
                // At bottom - show live screen
                return self.screen_line_to_cga(screen, y);
            }
            // Virtual buffer: history + screen (25 lines)
            let total_virtual_lines = self.history.len() + SCREEN_HEIGHT;
            let view_start = total_virtual_lines.saturating_sub(SCREEN_HEIGHT + self.viewport_position);
            view_start + y
        };

        // Fetch from history or current screen
        if line_index < self.history.len() {
            Some(self.history[line_index])
        } else {
            let screen_y = line_index - self.history.len();
            self.screen_line_to_cga(screen, screen_y)
        }
    }

    /// Convert a screen line to CGA format.
    fn screen_line_to_cga(&self, screen: &Screen, y: usize) -> Option<[u8; LINE_BYTES]> {
        if y >= SCREEN_HEIGHT {
            return None;
        }
        let mut line = [0u8; LINE_BYTES];
        for x in 0..SCREEN_WIDTH {
            if let Some(cell) = screen.get_cell(x, y) {
                let cga = Self::cell_to_cga(cell);
                line[x * 2] = cga[0];
                line[x * 2 + 1] = cga[1];
            }
        }
        Some(line)
    }

    /// Get the "SCROLLBACK" indicator text as CGA-format bytes.
    ///
    /// Returns 10 characters (20 bytes) with yellow on red coloring.
    pub fn scrollback_indicator() -> [u8; 20] {
        const TEXT: &[u8] = b"SCROLLBACK";
        // Yellow (14) foreground, Red (4) background
        // Attribute: bg=4, fg=14 -> (4 << 4) | 14 = 0x4E
        const ATTR: u8 = 0x4E;

        let mut result = [0u8; 20];
        for (i, &ch) in TEXT.iter().enumerate() {
            result[i * 2] = ch;
            result[i * 2 + 1] = ATTR;
        }
        result
    }
}

impl Default for ScrollbackBuffer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_to_cga_roundtrip() {
        let cell = Cell { ch: b'A', fg: 14, bg: 4 };
        let cga = ScrollbackBuffer::cell_to_cga(&cell);
        let result = ScrollbackBuffer::cga_to_cell(cga);
        assert_eq!(result.ch, cell.ch);
        assert_eq!(result.fg, cell.fg);
        assert_eq!(result.bg, cell.bg);
    }

    #[test]
    fn test_scrollback_indicator() {
        let indicator = ScrollbackBuffer::scrollback_indicator();
        assert_eq!(indicator[0], b'S');
        assert_eq!(indicator[1], 0x4E); // Yellow on red
        assert_eq!(indicator[18], b'K');
        assert_eq!(indicator[19], 0x4E);
    }

    #[test]
    fn test_scroll_navigation_mouse() {
        let mut buffer = ScrollbackBuffer::new();

        // Push some test lines
        let cell = Cell { ch: b' ', fg: 7, bg: 0 };
        let line = vec![cell; 80];
        for _ in 0..50 {
            buffer.push_line(&line);
        }

        // Not active initially
        assert!(!buffer.is_active());

        // Scroll up activates and moves offset (mouse mode)
        buffer.scroll_up(1);
        assert!(buffer.is_active());
        assert!(!buffer.keyboard_entry);
        assert_eq!(buffer.viewport_position(), 1);

        // Scroll up more
        buffer.scroll_up(10);
        assert_eq!(buffer.viewport_position(), 11);

        // Scroll down
        buffer.scroll_down(5);
        assert_eq!(buffer.viewport_position(), 6);

        // Scroll down to bottom exits (mouse mode only)
        buffer.scroll_down(10);
        assert!(!buffer.is_active());
        assert_eq!(buffer.viewport_position(), 0);
    }

    #[test]
    fn test_max_lines_trimming() {
        let mut buffer = ScrollbackBuffer::with_max_lines(10);
        let cell = Cell { ch: b'X', fg: 7, bg: 0 };
        let line = vec![cell; 80];

        // Add 15 lines
        for _ in 0..15 {
            buffer.push_line(&line);
        }

        // Should be trimmed to 10
        assert_eq!(buffer.history_len(), 10);
    }
}
