/// Terminal state management combining screen, parser, and scrollback.
///
/// This module provides a unified interface for terminal operations,
/// ensuring scrollback capture happens during scroll operations.

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::rc::Rc;
use std::cell::RefCell;
use web_sys::{HtmlCanvasElement, KeyboardEvent, MouseEvent, WheelEvent};

use crate::parser::AnsiParser;
use crate::postprocess::PostProcessor;
use crate::renderer::Renderer;
use crate::screen::Screen;
use crate::scrollback::ScrollbackBuffer;

/// Combined terminal state for coordinated updates.
pub struct Terminal {
    pub screen: Screen,
    pub parser: AnsiParser,
    pub scrollback: ScrollbackBuffer,
}

impl Terminal {
    /// Create a new terminal with default scrollback buffer size.
    pub fn new() -> Self {
        Self::with_scrollback_lines(crate::scrollback::DEFAULT_MAX_LINES)
    }

    /// Create a new terminal with specified scrollback buffer size.
    pub fn with_scrollback_lines(max_lines: usize) -> Self {
        Terminal {
            screen: Screen::new(),
            parser: AnsiParser::new(),
            scrollback: ScrollbackBuffer::with_max_lines(max_lines),
        }
    }

    /// Process a single byte, capturing any scrolled lines to scrollback.
    ///
    /// The parser returns a `ParseAction` indicating if scrolling or screen
    /// clearing occurred, allowing us to capture the appropriate content
    /// to the scrollback buffer.
    pub fn process_byte(&mut self, byte: u8) {
        // For screen clear, we need to capture BEFORE the clear happens.
        // We detect ESC[2J by checking if we're in CSI state with '2' param and 'J' command.
        // This is a bit of a hack but necessary since parser clears inline.
        let should_capture_screen = self.is_about_to_clear_screen(byte);

        if should_capture_screen {
            self.scrollback.push_screen(&self.screen);
        }

        // For line scroll, we need to capture BEFORE the scroll happens.
        // Detect conditions that will cause a scroll:
        let should_capture_line = self.is_about_to_scroll(byte);

        if should_capture_line {
            if let Some(line) = self.screen.get_line(0) {
                self.scrollback.push_line(&line);
            }
        }

        // Process the byte - ParseAction tells us what happened
        let _action = self.parser.process_byte(byte, &mut self.screen);
    }

    /// Check if the next byte will trigger a screen clear (ESC[2J).
    fn is_about_to_clear_screen(&self, byte: u8) -> bool {
        self.parser.will_clear_screen(byte)
    }

    /// Check if the next byte will cause a line scroll.
    fn is_about_to_scroll(&self, byte: u8) -> bool {
        // Only check when parser is in normal state - escape sequences don't directly scroll
        if !self.parser.is_in_normal_state() {
            return false;
        }

        let (_width, height) = self.screen.dimensions();
        let (cursor_x, cursor_y) = self.screen.cursor_pos();

        match byte {
            0x0A => cursor_y == height - 1, // Newline at bottom
            b if b >= 32 => cursor_y == height - 1 && cursor_x == 79, // Char at bottom-right
            _ => false,
        }
    }

    /// Process multiple bytes.
    pub fn process_bytes(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.process_byte(byte);
        }
    }

    /// Handle a wheel scroll event.
    ///
    /// Returns true if the event was handled and should not propagate.
    /// When in scrollback mode, all wheel events are captured.
    pub fn handle_wheel(&mut self, delta_y: f64) -> bool {
        let was_active = self.scrollback.is_active();

        // Convert wheel delta to line count
        // Typical wheel events are ~100-150 pixels per "click"
        // We want roughly 3 lines per wheel click for responsive scrolling
        let lines = ((delta_y.abs() / 40.0).ceil() as usize).max(1);

        if delta_y < 0.0 {
            // Scroll up (back in history)
            self.scrollback.scroll_up(lines);
            true
        } else if delta_y > 0.0 {
            // Scroll down (toward present)
            if was_active {
                self.scrollback.scroll_down(lines);
                true // Capture the event even if we just exited scrollback
            } else {
                false
            }
        } else {
            // Capture horizontal/no scroll only if in scrollback mode
            was_active
        }
    }

    /// Handle a keyboard event.
    ///
    /// Returns true if the event was handled and should NOT be sent to the host.
    /// When in scrollback mode, ALL keys are captured (not sent to host).
    pub fn handle_key(&mut self, key: &str, alt_key: bool) -> bool {
        // Alt+K toggles scrollback regardless of mode
        if (key == "k" || key == "K") && alt_key {
            self.scrollback.toggle_scrollback();
            return true;
        }

        // If in scrollback mode, handle navigation or block all other keys
        if self.scrollback.is_active() {
            match key {
                "Escape" => {
                    self.scrollback.start_animated_exit();
                }
                "ArrowUp" => {
                    self.scrollback.scroll_up(1);
                }
                "ArrowDown" => {
                    self.scrollback.scroll_down(1);
                }
                "PageUp" => {
                    self.scrollback.page_up();
                }
                "PageDown" => {
                    self.scrollback.page_down();
                }
                _ => {
                    // Block all other keys while in scrollback (don't send to host)
                }
            }
            // Always return true when in scrollback to prevent sending to host
            return true;
        }

        false
    }

    /// Handle a mouse click event.
    ///
    /// Returns true if the click was handled (exits scrollback mode).
    pub fn handle_click(&mut self) -> bool {
        if self.scrollback.is_active() {
            self.scrollback.start_animated_exit();
            true
        } else {
            false
        }
    }
}

impl Default for Terminal {
    fn default() -> Self {
        Self::new()
    }
}

/// Set up event listeners for scrollback on a canvas element.
///
/// This attaches wheel, keyboard, and click events to the canvas container.
pub fn setup_scrollback_events(
    canvas: &HtmlCanvasElement,
    terminal: Rc<RefCell<Terminal>>,
    renderer: Rc<Renderer>,
    offscreen_canvas: Rc<HtmlCanvasElement>,
    post_processor: Rc<PostProcessor>,
) -> Result<(), JsValue> {
    // Make canvas focusable for keyboard events
    canvas.set_tab_index(0);

    // Create options for passive: false (required to preventDefault on wheel)
    let wheel_options = web_sys::AddEventListenerOptions::new();
    wheel_options.set_passive(false);

    // Set up wheel event listener with passive: false
    {
        let terminal = terminal.clone();
        let renderer = renderer.clone();
        let offscreen_canvas = offscreen_canvas.clone();
        let post_processor = post_processor.clone();

        let closure = Closure::<dyn Fn(WheelEvent)>::new(move |event: WheelEvent| {
            // Always capture wheel events on canvas to prevent page scrolling
            event.prevent_default();
            event.stop_propagation();

            let mut term = terminal.borrow_mut();
            let was_animating = term.scrollback.is_animating_exit();
            if term.handle_wheel(event.delta_y()) {
                // Re-render with scrollback
                let _ = renderer.render_with_scrollback(&term.screen, &term.scrollback);
                let _ = post_processor.process(&offscreen_canvas);

                // Start animation only if it just started (wasn't already running)
                if !was_animating && term.scrollback.is_animating_exit() {
                    drop(term); // Release borrow before starting animation
                    start_exit_animation(
                        terminal.clone(),
                        renderer.clone(),
                        offscreen_canvas.clone(),
                        post_processor.clone(),
                    );
                }
            }
        });

        canvas.add_event_listener_with_callback_and_add_event_listener_options(
            "wheel",
            closure.as_ref().unchecked_ref(),
            &wheel_options,
        )?;
        closure.forget(); // Keep the closure alive
    }

    // Set up keyboard event listener on the canvas itself (requires focus)
    {
        let terminal = terminal.clone();
        let renderer = renderer.clone();
        let offscreen_canvas = offscreen_canvas.clone();
        let post_processor = post_processor.clone();
        let canvas_for_fullscreen = canvas.clone();

        let closure = Closure::<dyn Fn(KeyboardEvent)>::new(move |event: KeyboardEvent| {
            // Handle Alt+Enter for fullscreen toggle
            if event.key() == "Enter" && event.alt_key() {
                event.prevent_default();
                event.stop_propagation();
                toggle_fullscreen(&canvas_for_fullscreen);
                return;
            }

            let mut term = terminal.borrow_mut();
            let was_animating = term.scrollback.is_animating_exit();
            if term.handle_key(&event.key(), event.alt_key()) {
                event.prevent_default();
                event.stop_propagation();

                // Re-render with scrollback
                let _ = renderer.render_with_scrollback(&term.screen, &term.scrollback);
                let _ = post_processor.process(&offscreen_canvas);

                // Start animation only if it just started (wasn't already running)
                if !was_animating && term.scrollback.is_animating_exit() {
                    drop(term); // Release borrow before starting animation
                    start_exit_animation(
                        terminal.clone(),
                        renderer.clone(),
                        offscreen_canvas.clone(),
                        post_processor.clone(),
                    );
                }
            }
        });

        // Add to canvas directly for keyboard events (canvas is now focusable)
        canvas.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // Set up click event listener for exiting scrollback and focusing canvas
    {
        let terminal = terminal.clone();
        let renderer = renderer.clone();
        let offscreen_canvas = offscreen_canvas.clone();
        let post_processor = post_processor.clone();
        let canvas_clone = canvas.clone();

        let closure = Closure::<dyn Fn(MouseEvent)>::new(move |event: MouseEvent| {
            // Focus the canvas on click so it receives keyboard events
            let _ = canvas_clone.focus();

            let mut term = terminal.borrow_mut();
            let was_animating = term.scrollback.is_animating_exit();
            if term.handle_click() {
                event.prevent_default();
                event.stop_propagation();

                // Re-render with scrollback
                let _ = renderer.render_with_scrollback(&term.screen, &term.scrollback);
                let _ = post_processor.process(&offscreen_canvas);

                // Start animation only if it just started (wasn't already running)
                if !was_animating && term.scrollback.is_animating_exit() {
                    drop(term); // Release borrow before starting animation
                    start_exit_animation(
                        terminal.clone(),
                        renderer.clone(),
                        offscreen_canvas.clone(),
                        post_processor.clone(),
                    );
                }
            }
        });

        canvas.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // Set up mousedown event listener to prevent bubbling (especially for middle-click scroll)
    {
        let closure = Closure::<dyn Fn(MouseEvent)>::new(move |event: MouseEvent| {
            // Prevent default for middle mouse button (scroll wheel click)
            // This stops the auto-scroll behavior in browsers
            if event.button() == 1 {
                event.prevent_default();
                event.stop_propagation();
            }
        });

        canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    Ok(())
}

/// Start the exit animation loop using requestAnimationFrame.
///
/// The animation loop will automatically stop if:
/// - Animation completes (scrolled to bottom)
/// - Animation is cancelled (user scrolled up or pressed Alt+K)
/// - Scrollback mode is exited
fn start_exit_animation(
    terminal: Rc<RefCell<Terminal>>,
    renderer: Rc<Renderer>,
    offscreen_canvas: Rc<HtmlCanvasElement>,
    post_processor: Rc<PostProcessor>,
) {
    let window = match web_sys::window() {
        Some(w) => w,
        None => return,
    };

    // Create the animation frame callback
    let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let g = f.clone();

    let terminal_clone = terminal.clone();
    let renderer_clone = renderer.clone();
    let offscreen_clone = offscreen_canvas.clone();
    let post_clone = post_processor.clone();

    *g.borrow_mut() = Some(Closure::new(move || {
        let mut term = terminal_clone.borrow_mut();

        // Check if animation was cancelled by user input (scroll wheel, Alt+K, etc.)
        // This happens when is_animating_exit() is false but scrollback is still active
        if !term.scrollback.is_animating_exit() {
            // Animation was cancelled or completed, stop the loop
            let _ = f.borrow_mut().take();
            return;
        }

        // Advance animation
        let still_animating = term.scrollback.animate_exit_frame();

        // Re-render
        let _ = renderer_clone.render_with_scrollback(&term.screen, &term.scrollback);
        let _ = post_clone.process(&offscreen_clone);

        // Continue animation if needed
        if still_animating && term.scrollback.is_animating_exit() {
            drop(term); // Release borrow
            if let Some(window) = web_sys::window() {
                let _ = window.request_animation_frame(
                    f.borrow().as_ref().unwrap().as_ref().unchecked_ref()
                );
            }
        } else {
            // Animation complete, clean up
            let _ = f.borrow_mut().take();
        }
    }));

    // Start the animation loop
    let _ = window.request_animation_frame(
        g.borrow().as_ref().unwrap().as_ref().unchecked_ref()
    );
}

/// Toggle fullscreen mode for the canvas element.
///
/// When entering fullscreen:
/// - The canvas fills the screen height (or width if aspect ratio requires)
/// - Black background fills any remaining space
/// - Canvas maintains its native aspect ratio (~1.37:1 for 1920x1400)
///
/// Uses the Fullscreen API with fallbacks for different browsers.
fn toggle_fullscreen(canvas: &HtmlCanvasElement) {
    let document = match web_sys::window().and_then(|w| w.document()) {
        Some(d) => d,
        None => return,
    };

    // Check if we're currently in fullscreen
    let fullscreen_element = document.fullscreen_element();

    if fullscreen_element.is_some() {
        // Exit fullscreen
        document.exit_fullscreen();
    } else {
        // Enter fullscreen - request on the canvas element
        // The canvas will be centered with black background automatically
        let _ = canvas.request_fullscreen();
    }
}
