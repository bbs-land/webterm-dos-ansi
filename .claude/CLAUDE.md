# BBS.land webterm-dos-ansi

## Project Overview

A terminal emulator library written in Rust (compiled to WebAssembly) for
rendering DOS CP437 and extended ANSI escape sequences in web browsers. Primary
use case is connecting to BBS (Bulletin Board System) servers via WebSocket to
raw TCP connections.

## Architecture

### Core Components

- **Rust/WASM Core**: Contains the majority of application logic including:
  - Terminal emulation engine (ANSI parsing, CP437 decoding, screen buffer
    management)
  - Canvas rendering logic (character-to-pixel rendering with 3x4 scaling)
  - DOM manipulation (canvas creation, element management)
  - Connection state management and UI logic
  - Keyboard event handling
- **JavaScript Glue**: Minimal JavaScript glue code for:
  - WASM module loading and initialization
  - WebSocket connectivity (bridge between JS WebSocket API and WASM)
  - Browser event forwarding to WASM
- **Rendering Engine**: Canvas-based renderer using authentic EGA font (8x14
  pixels) with proper aspect ratio correction

### Technology Stack

- **Language**: Rust + WebAssembly
- **Build System**:
  - Rust: `cargo` with `wasm-pack`
  - JavaScript/TypeScript: Deno with Vite
- **Testing**: Unit tests (Rust), integration tests, visual regression, E2E
  browser tests

## Key Features

### Phase 1 (Initial Implementation)

- DOS CP437 character set support (full 256-character codepage)
- Extended ANSI escape sequence parsing (colors, cursor control, attributes)
- VT-100/VT-102 escape sequences commonly used by DOS-era BBSes
- WebSocket connectivity for raw TCP connections to BBS systems
- Basic terminal emulation (cursor movement, scrolling, colors)
- Connection UI with optional splash ANSI and connect image overlay
- User-initiated connection flow (click to connect)
- `renderAnsi()` function for testing and static ANSI content display
- Baud rate simulation for realistic retro experience

### Phase 2 (Future)

- Advanced ANSI features (extended colors, terminal queries)
- Terminal type emulation (ANSI-BBS, xterm extensions)
- Keyboard input handling (special keys, function keys)
- Copy/paste support
- Configuration options (font, colors, baud rate emulation)

## Development Guidelines

### Code Organization

```
/
├── projects/            # Project workspace
│   ├── lib/             # Main WASM library
│   │   ├── src/
│   │   │   ├── lib.rs       # Main library entry with initWebTerm() and renderAnsi()
│   │   │   ├── parser.rs    # ANSI escape sequence parser
│   │   │   ├── screen.rs    # Screen buffer management (80x25 buffer)
│   │   │   ├── cp437.rs     # CP437 codec
│   │   │   ├── renderer.rs  # Canvas rendering logic (1920x1400px, 3x4 scaling)
│   │   │   ├── dom.rs       # DOM manipulation (canvas creation, element handling)
│   │   │   ├── connect_ui.rs # Connection UI logic (splash ANSI, connect overlay)
│   │   │   ├── keyboard.rs  # Keyboard event handling
│   │   │   ├── websocket.rs # WebSocket state management
│   │   │   └── ...
│   │   ├── tests/       # Rust unit and integration tests
│   │   ├── js/          # Minimal JavaScript glue code
│   │   │   ├── index.js     # WASM loader and initialization
│   │   │   └── websocket.js # WebSocket bridge to WASM
│   │   ├── fonts/       # EGA font data (embedded in WASM)
│   │   │   └── ega-8x14.bin # EGA 8x14 font bitmap
│   │   ├── Cargo.toml
│   │   ├── deno.json    # Deno configuration
│   │   └── vite.config.ts # Vite build configuration
│   │
│   ├── websocket-connect/   # WebSocket bridge server (Rust + Axum)
│   │   ├── src/
│   │   │   ├── main.rs      # Axum server with WebSocket bridge
│   │   │   ├── websocket.rs # WebSocket to raw TCP bridge logic
│   │   │   └── ...
│   │   ├── static/          # Static web assets
│   │   │   ├── index.html   # Terminal connection page
│   │   │   ├── webterm-dos-ansi.js  # WASM library (copied from lib/dist)
│   │   │   └── webterm-dos-ansi_bg.wasm
│   │   └── Cargo.toml
│   │
│   └── ansi-view/       # ANSI file viewer application
│       ├── index.html   # Viewer UI with file picker and BPS dropdown
│       ├── app.js       # File loading and viewer control logic
│       ├── styles.css   # Viewer styles
│       ├── webterm-dos-ansi.js  # WASM library (copied from lib/dist)
│       ├── webterm-dos-ansi_bg.wasm
│       └── sample/      # Sample ANSI files for testing
│           └── *.ans
│
├── tests/               # E2E and visual regression tests
└── examples/            # Example usage and demos
```

### Coding Standards

#### Rust

- Follow standard Rust conventions (rustfmt, clippy)
- Use `#![deny(unsafe_code)]` unless absolutely necessary
- Comprehensive error handling with `Result<T, E>`
- Document public APIs with doc comments (`///`)
- Keep WASM binary size minimal (use `wasm-opt`)
- **Maximize logic in WASM**: All application logic, rendering, DOM
  manipulation, and state management should be in Rust/WASM
- Use `web-sys` and `wasm-bindgen` for DOM and browser API access

#### JavaScript/TypeScript

- **Minimal glue code only**: JavaScript should only handle WASM loading and
  WebSocket bridging
- Use TypeScript for type safety where beneficial
- Modern ES6+ syntax
- No business logic in JavaScript (belongs in WASM)
- Thin wrapper around WebSocket API to forward messages to/from WASM
- Use Deno for development tooling and package management

### Deno and Vite Configuration

#### deno.json

```json
{
  "tasks": {
    "build": "deno run -A npm:vite build",
    "dev": "deno run -A npm:vite",
    "preview": "deno run -A npm:vite preview",
    "test": "deno test",
    "test:e2e": "deno test --allow-all tests/e2e/",
    "test:visual": "deno test --allow-all tests/visual/"
  },
  "compilerOptions": {
    "lib": ["deno.window", "dom", "dom.iterable"],
    "types": ["./pkg/webterm-dos-ansi.d.ts"]
  }
}
```

#### vite.config.ts

```typescript
import { defineConfig } from "npm:vite@^5";

export default defineConfig({
  build: {
    lib: {
      entry: "./js/index.js",
      name: "WebTermDosAnsi",
      fileName: "webterm-dos-ansi",
      formats: ["es"],
    },
    outDir: "dist",
    target: "esnext",
  },
  server: {
    port: 5173,
  },
});
```

### Testing Strategy

1. **Rust Unit Tests**: Test individual components (parser, CP437 decoder,
   screen buffer)
2. **Integration Tests**: Test WASM module integration with JS
3. **Visual Regression**: Screenshot comparison for rendering accuracy (using
   Deno)
4. **E2E Tests**: Full browser tests with actual ANSI sequences (using Deno +
   Puppeteer)
5. **Manual Testing**: Connect to real BBS systems during development

### Display Specifications

#### EGA Font and Screen Layout

- **Font**: EGA 8x14 pixel font (standard DOS text mode font)
- **Screen Dimensions**: 80 columns × 25 rows (2000 characters)
- **Character Cell**: 8×14 pixels (font glyph size)
- **Pixel Aspect Ratio Correction**: Each font pixel rendered as 3×4 canvas
  pixels
  - Accounts for non-square EGA pixels (1:1.35 aspect ratio)
  - Rendered character cell: 24×56 canvas pixels (8×3 wide, 14×4 tall)
  - Total canvas size: 1920×1400 pixels (80×24 wide, 25×56 tall)

#### Canvas Sizing and Scaling

- **Canvas Element**: Created with fixed dimensions of 1920×1400 pixels (canvas
  width/height attributes)
- **CSS Scaling**: Apply CSS to fit canvas within container element
  - Use `max-width: 100%` and `max-height: 100%` to ensure canvas fits in
    viewport
  - Use `width: 100%` or calculated dimensions to fill container while
    maintaining aspect ratio
  - Container should constrain canvas to visible area (no scrolling required)
  - CSS handles responsive scaling; canvas pixel dimensions remain fixed at
    1920×1400
- **Aspect Ratio**: Canvas has ~1.37:1 aspect ratio (1920:1400)
  - CSS should maintain this ratio when scaling to fit container

#### Rendering Details

- Use HTML5 Canvas 2D context
- Font bitmap embedded or loaded as resource
- Each character rendered by copying font glyph pixels to canvas with 3×4
  scaling
- Support foreground and background colors per character cell
- Handle text attributes (bold, blink, reverse video) by modifying colors or
  glyph data
- Canvas rendering happens at native 1920×1400 resolution; browser handles
  display scaling via CSS

### Performance Considerations

- Optimize for 60 FPS rendering
- Minimize WASM ↔ JS boundary crossings
- Use efficient buffer management for screen updates
- Implement dirty-region rendering (only redraw changed characters)
- Pre-render font glyphs in all color combinations (optional optimization)
- Consider using ImageData for direct pixel manipulation

## Building and Running

### Prerequisites

- Rust toolchain (rustup)
- wasm-pack
- Deno runtime (for development tooling and build)
- Modern browser with WebAssembly support

### Build Commands

#### Building the WASM Library

```bash
# Build the core WASM library
cd projects/lib

# Build WASM with wasm-pack
wasm-pack build --target web

# This generates in pkg/:
# - webterm-dos-ansi.js (JavaScript glue)
# - webterm-dos-ansi_bg.wasm (WebAssembly binary)
# - webterm-dos-ansi.d.ts (TypeScript definitions)

# Optional: Build JavaScript wrapper with Vite (if needed)
deno task build
```

#### Building the WebSocket Bridge Server

```bash
# Build and run the WebSocket server
cd projects/websocket-connect

# Copy WASM library to static directory
cp ../lib/pkg/webterm-dos-ansi.js static/
cp ../lib/pkg/webterm-dos-ansi_bg.wasm static/

# Build and run
cargo build --release
cargo run

# Server starts on http://localhost:3000
# Serves terminal page and bridges WebSocket to TCP
```

#### Setting up the ANSI Viewer

```bash
# Copy WASM library to ansi-view directory
cd projects/ansi-view
cp ../lib/pkg/webterm-dos-ansi.js ./
cp ../lib/pkg/webterm-dos-ansi_bg.wasm ./

# Serve with Deno or other static file server
deno serve --port 8080 .
# Or use: python3 -m http.server 8080
# Or use: cargo install simple-http-server && simple-http-server

# Open http://localhost:8080 in browser
```

### Running Tests

```bash
# Rust unit tests (in lib directory)
cd projects/lib
cargo test

# E2E and visual regression tests (from project root)
deno task test

# Or run specific test suites
deno task test:e2e
deno task test:visual
```

## ANSI/DOS Compatibility

### Supported Features

- CP437 character set (box drawing, extended ASCII)
- Standard ANSI colors (16 colors: 8 standard + 8 bright)
- Extended colors (256-color mode, RGB/truecolor)
- Cursor positioning and movement
- Text attributes (bold, italic, underline, blink, reverse)
- Screen clearing and scrolling
- Common BBS-specific sequences

### Terminal Type

Target terminal type: `ANSI-BBS` with xterm extensions

### VT-x Escape Sequences (DOS BBS Era)

The terminal must support the most common VT-100/VT-102 sequences used by
DOS-era BBSes:

#### Cursor Control

- `ESC[H` - Cursor home (move to 0,0)
- `ESC[{row};{col}H` - Cursor position (direct addressing)
- `ESC[{row};{col}f` - Horizontal and vertical position (same as above)
- `ESC[{n}A` - Cursor up n lines
- `ESC[{n}B` - Cursor down n lines
- `ESC[{n}C` - Cursor forward n columns
- `ESC[{n}D` - Cursor backward n columns
- `ESC[s` - Save cursor position
- `ESC[u` - Restore cursor position

#### Erasing

- `ESC[2J` - Clear entire screen
- `ESC[J` - Clear from cursor to end of screen
- `ESC[1J` - Clear from cursor to beginning of screen
- `ESC[K` - Clear from cursor to end of line
- `ESC[1K` - Clear from cursor to beginning of line
- `ESC[2K` - Clear entire line

#### Graphics Mode (SGR - Select Graphic Rendition)

- `ESC[0m` - Reset all attributes
- `ESC[1m` - Bold/bright
- `ESC[2m` - Dim (rarely used)
- `ESC[4m` - Underscore
- `ESC[5m` - Blink
- `ESC[7m` - Reverse video
- `ESC[8m` - Concealed (hidden)
- `ESC[30-37m` - Foreground colors (black, red, green, yellow, blue, magenta,
  cyan, white)
- `ESC[40-47m` - Background colors
- `ESC[90-97m` - Bright foreground colors (non-standard but common)
- `ESC[100-107m` - Bright background colors (non-standard but common)

#### Terminal Queries (may respond or ignore)

- `ESC[6n` - Device Status Report (cursor position) - should respond with
  `ESC[{row};{col}R`
- `ESC[c` or `ESC[0c` - Device Attributes - respond with VT-100 compatible
  string
- `ESC[5n` - Device Status Report - respond with `ESC[0n` (ready)

#### Special Sequences

- `ESC[?25h` - Show cursor
- `ESC[?25l` - Hide cursor
- `ESC[{n}m` - Multiple SGR parameters can be combined (e.g., `ESC[1;31;44m` =
  bold red on blue)

## Initialization and Connection UI

The terminal library uses a single WASM function `initWebTerm()` that
automatically discovers and initializes terminals on the page.

### HTML Data Attributes

The WASM module scans for HTML elements with the `data-term-url` attribute:

```html
<div
  id="terminal-container"
  data-term-url="wss://bbs.example.com/ws/node/1"
  data-term-connect-button="/assets/connect-button.png"
  data-term-preconnect-screen="Welcome to Example BBS!&#10;&#10;Click to connect..."
>
</div>
```

#### Attributes:

- **`data-term-url`** (required): WebSocket URL (ws:// or wss://) to connect to
- **`data-term-connect-button`** (optional): URL to PNG image for connect button
  overlay
  - If not provided, uses embedded default connect button
- **`data-term-preconnect-screen`** (optional): CP437 ANSI text to display
  before connection
  - If not provided, terminal starts with blank screen

### Initialization Flow

1. **Page Load**:
   - JavaScript loads WASM module
   - Calls `initWebTerm()` function (exposed from Rust)
   - WASM scans DOM for elements with `data-term-url`

2. **Terminal Setup** (per matching element):
   - WASM creates canvas element (1920x1400px) inside the container
   - Apply CSS to canvas for responsive scaling:
     - Set `max-width: 100%` and `max-height: 100%`
     - Set `width: 100%` to fill container width (maintains aspect ratio)
     - Ensures canvas fits within container without scrolling
   - If `data-term-preconnect-screen` exists, render CP437 ANSI text to terminal
   - If `data-term-connect-button` exists, load and display PNG centered over
     canvas
   - Otherwise, show default connect button
   - Attach click event listener to canvas

3. **Pre-Connection State**:
   - Terminal is in read-only mode
   - No WebSocket connection yet
   - Waiting for user click

4. **User Click**:
   - User clicks anywhere on the canvas
   - Clear terminal screen (reset to blank 80x25 buffer)
   - Remove connect button overlay
   - Initiate WebSocket connection via JavaScript bridge

5. **Connected State**:
   - Terminal becomes interactive
   - Receive and render data from WebSocket
   - Send keyboard input to WebSocket

6. **Disconnection Handling**:
   - On WebSocket disconnect/error:
     - Move cursor to start of bottom row (row 25, column 1)
     - Reset text attributes (normal white on black)
     - Display message: `"Server Disconnected"`
     - Disable all keyboard input
     - Terminal becomes read-only permanently (no reconnection)

### Example Usage

```html
<!DOCTYPE html>
<html>
  <head>
    <title>BBS Terminal</title>
    <script type="module" src="/dist/webterm-dos-ansi.js"></script>
    <style>
      /* Optional: style the container to control terminal size */
      .terminal-container {
        width: 100%;
        max-width: 1920px; /* Optional: limit maximum size */
        margin: 0 auto;
        background: #000;
      }
    </style>
  </head>
  <body>
    <!-- Terminal will be auto-discovered and initialized -->
    <!-- Canvas will be created inside and scaled to fit -->
    <div
      class="terminal-container"
      data-term-url="wss://bbs.example.com/ws/node/1"
    >
    </div>

    <!-- Terminal with custom connect button and splash screen -->
    <div
      class="terminal-container"
      data-term-url="wss://another-bbs.com/telnet"
      data-term-connect-button="/images/connect.png"
      data-term-preconnect-screen="&#27;[1;37;44m  Example BBS  &#27;[0m&#10;&#10;Click to connect..."
    >
    </div>
  </body>
</html>
```

## Manual Testing and ANSI Rendering Function

For testing and development purposes, the library exposes a `renderAnsi()`
function that allows direct rendering of CP437 ANSI content without WebSocket
connectivity.

### renderAnsi Function

```javascript
renderAnsi(selector, content, bps = null);
```

#### Parameters:

- **`selector`** (string, required): CSS selector for the container element
  where the terminal will be rendered
- **`content`** (string | Uint8Array, required): CP437 ANSI content to render
  - String: UTF-8 encoded ANSI sequences (will be converted to CP437)
  - Uint8Array: Raw bytes in CP437 encoding
- **`bps`** (number, optional): Bits per second to simulate (baud rate
  emulation)
  - If provided, simulates connection speed by throttling rendering
  - Common values: 300, 1200, 2400, 9600, 14400, 28800, 57600
  - If null/undefined, renders immediately without delay

#### Behavior:

- Creates a canvas element (1920x1400px) inside the target container
- Applies responsive CSS scaling (same as `initWebTerm`)
- Parses and renders CP437 ANSI content to the terminal
- If `bps` is specified, simulates serial connection speed by introducing
  character-by-character delays
- Terminal is read-only (no keyboard input)
- No WebSocket connection or interactive features

#### Use Cases:

- **Testing ANSI Art**: Quickly render and preview .ANS files
- **Development**: Test ANSI parser without running a BBS server
- **Demos**: Display static ANSI content on a page
- **Visual Regression**: Automated screenshot testing of ANSI rendering
- **Baud Rate Testing**: Verify rendering speed and animations at different
  connection speeds

### Example Usage

```html
<!DOCTYPE html>
<html>
  <head>
    <title>ANSI Renderer Test</title>
    <script type="module" src="/dist/webterm-dos-ansi.js"></script>
    <style>
      .ansi-container {
        width: 100%;
        max-width: 1920px;
        margin: 20px auto;
        background: #000;
      }
    </style>
  </head>
  <body>
    <h1>ANSI Test 1: Immediate Render</h1>
    <div id="test1" class="ansi-container"></div>

    <h1>ANSI Test 2: 2400 bps Simulation</h1>
    <div id="test2" class="ansi-container"></div>

    <script type="module">
      import {
        initWebTerm,
        renderAnsi,
      } from "/dist/webterm-dos-ansi.js";

      // Test 1: Render ANSI immediately
      const ansiString =
        "\x1b[1;37;44m  Test BBS  \x1b[0m\n\n\x1b[32mHello, World!\x1b[0m";
      renderAnsi("#test1", ansiString);

      // Test 2: Render with 2400 bps simulation (slower, like old modems)
      fetch("/assets/welcome.ans")
        .then((res) => res.arrayBuffer())
        .then((buffer) => {
          const data = new Uint8Array(buffer);
          renderAnsi("#test2", data, 2400);
        });

      // Also initialize any live terminals on the page
      initWebTerm();
    </script>
  </body>
</html>
```

### Implementation Notes

- Uses the same rendering engine as live terminals
- BPS simulation calculates character delay:
  `delay_ms = (8 bits/char) / (bps) * 1000`
  - Example: 2400 bps = ~3.33ms per character
  - Example: 9600 bps = ~0.83ms per character
- Rendering happens asynchronously if BPS is set (uses requestAnimationFrame or
  setTimeout)
- Each `renderAnsi` call creates an independent terminal instance

## Project Applications

### websocket-connect Server

A Rust-based WebSocket bridge server using Axum that allows browsers to connect
to raw TCP services (like BBS servers).

#### Features:

- **WebSocket to TCP Bridge**: Accepts WebSocket connections and bridges them to
  raw TCP sockets
- **Simple Web Interface**: Serves a terminal page with input fields for server
  and port
- **Automatic Connection**: Uses `initWebTerm()` with user-specified connection
  details
- **Static File Serving**: Serves the WASM library and terminal HTML

#### Usage:

1. Start the server: `cargo run` (default port 3000)
2. Navigate to `http://localhost:3000`
3. Enter target BBS server hostname and port
4. Terminal auto-initializes and presents connect button
5. Click to connect - WebSocket bridges to the specified TCP service

#### Server Configuration:

- Configurable listen port via environment variable or CLI argument
- CORS support for development
- WebSocket message format: binary passthrough (no protocol wrapping)

#### Terminal Page Structure:

```html
<!-- projects/websocket-connect/static/index.html -->
<form id="connection-form">
  <label>BBS Server: <input name="server" placeholder="bbs.example.com"></label>
  <label>Port: <input name="port" type="number" placeholder="23"></label>
  <button type="submit">Set Connection</button>
</form>

<div
  id="terminal"
  data-term-url=""
  data-term-preconnect-screen="Enter server details above to connect"
>
</div>

<script>
  // Form updates data-term-url attribute
  // Calls initWebTerm() to reinitialize terminal
</script>
```

### ansi-view Application

A standalone ANSI file viewer for testing, previewing, and demonstrating ANSI
art rendering.

#### Features:

- **File Picker**: Select local .ANS, .ASC, or binary ANSI files
- **BPS Rate Selector**: Dropdown with common baud rates (300, 1200, 2400, 9600,
  14400, 28800, 57600, instant)
- **Auto-Clear**: Clears previous render when file or BPS rate changes
- **Sample Files**: Includes sample ANSI art for immediate testing

#### UI Structure:

```html
<!-- projects/ansi-view/index.html -->
<div class="controls">
  <label>ANSI File: <input
      type="file"
      id="file-picker"
      accept=".ans,.asc"
    ></label>
  <label>Speed:
    <select id="bps-selector">
      <option value="">Instant</option>
      <option value="300">300 bps</option>
      <option value="1200">1200 bps</option>
      <option value="2400">2400 bps</option>
      <option value="9600" selected>9600 bps</option>
      <option value="14400">14400 bps</option>
      <option value="28800">28800 bps</option>
      <option value="57600">57600 bps</option>
    </select>
  </label>
</div>

<div id="viewer-container"></div>
```

#### Behavior:

1. User selects ANSI file via file picker
2. User optionally selects baud rate from dropdown
3. On file selection or BPS change:
   - Clear previous terminal instance (remove canvas from container)
   - Read file as Uint8Array
   - Call `renderAnsi('#viewer-container', fileData, bps)`
4. Terminal renders with selected speed simulation

#### Sample Files:

Include curated ANSI art samples demonstrating:

- Basic color and formatting
- Box drawing characters
- Complex ANSI art with multiple colors
- Cursor positioning sequences
- Screen clearing and scrolling

## WebSocket Protocol

- Connects to WebSocket server that bridges to raw TCP
- Binary mode for efficient data transfer
- Handle connection states (connecting, open, closed, error)
- **No automatic reconnection**: On disconnect, show "Server Disconnected" and
  lock terminal
- Send keyboard input as raw bytes
- Connection initiated by user interaction (click), not automatically
- JavaScript bridge forwards all WebSocket events to WASM:
  - `onopen` - Connection established
  - `onmessage` - Data received (binary)
  - `onerror` - Connection error
  - `onclose` - Connection closed

## Architecture Principles

### WASM-First Design

The majority of the application logic resides in the Rust/WASM module:

- **DOM Manipulation**: Canvas creation, element insertion, event attachment,
  CSS styling (via `web-sys`)
- **Rendering**: All pixel-level drawing operations
- **State Management**: Connection state, terminal state, keyboard state
- **Business Logic**: ANSI parsing, screen buffer, cursor management
- **UI Logic**: Connect button overlay, splash screen, disconnection message
- **CSS Management**: Apply responsive scaling styles to canvas element for
  proper display sizing

### JavaScript Role

JavaScript serves only as a thin bridge layer:

- **WASM Loading**: Load and initialize the WASM module
- **WebSocket Bridge**: Create WebSocket connections and forward events to WASM
  - WASM cannot directly create WebSocket connections
  - JS creates the socket and passes messages bidirectionally
- **Module Initialization**: Call `initWebTerm()` on page load

### Benefits of WASM-First Approach

- **Performance**: Rendering and parsing logic runs at near-native speed
- **Maintainability**: Single codebase for business logic (Rust)
- **Type Safety**: Rust's type system catches bugs at compile time
- **Testability**: Core logic testable without browser environment
- **Code Reuse**: Potential to reuse core logic in other contexts

## Known Limitations

- No SSH/Telnet protocol implementation (requires server-side bridge)
- Limited to terminal emulation (no local shell)
- WebSocket required (no direct TCP from browser)
- No reconnection on disconnect (terminal locks after disconnection)

## Contributing

This project is developed and maintained by **BBS.land**.

When adding features:

1. Start with Rust implementation and tests
2. Update WASM bindings
3. Update JS wrapper
4. Add visual/E2E tests
5. Update documentation and examples

## Resources

- [ANSI Escape Code Reference](https://en.wikipedia.org/wiki/ANSI_escape_code)
- [CP437 Character Set](https://en.wikipedia.org/wiki/Code_page_437)
- [BBS Terminal Standards](http://www.bbsdocumentary.com/library/PROGRAMS/GRAPHICS/)
- [WebAssembly Best Practices](https://rustwasm.github.io/book/)

## License

Copyright © 2026 BBS.land

This project is licensed under the ISC License - see the [LICENSE.md](LICENSE.md) file for details.

## Project Status

**Status**: Initial setup and architecture planning

**Current Phase**: Project initialization

**Next Steps**:

#### Core Library (projects/lib)

1. Set up Rust project with wasm-pack and web-sys/wasm-bindgen dependencies
2. Acquire/create EGA 8x14 font bitmap (embed in WASM binary)
3. Create default connect button PNG (embed in WASM binary)
4. Implement `initWebTerm()` function with DOM scanning for `data-term-url`
5. Implement `renderAnsi()` function for manual testing and static content
6. Implement DOM manipulation (canvas creation via web-sys)
7. Implement CSS styling for responsive canvas scaling (max-width, width, aspect
   ratio)
8. Implement basic CP437 decoder
9. Create ANSI/VT-x escape sequence parser (with DOS BBS era sequences)
10. Build screen buffer management (80x25 character grid)
11. Create Canvas renderer with 3x4 pixel scaling (in Rust)
12. Implement baud rate simulation (character delay throttling)
13. Implement connection UI logic (splash ANSI, connect image overlay)
14. Create JavaScript WebSocket bridge
15. Wire up WebSocket events to WASM handlers
16. Add keyboard input handling (in Rust, via web-sys events)
17. Implement disconnection handling ("Server Disconnected" message, terminal
    lock)

#### WebSocket Bridge Server (projects/websocket-connect)

18. Set up Axum server project
19. Implement WebSocket handler
20. Implement WebSocket to TCP bridge logic
21. Add server/port configuration from WebSocket connection
22. Create terminal connection page (static/index.html)
23. Add form for server/port input
24. Implement dynamic `data-term-url` update logic
25. Copy WASM library to static directory
26. Test end-to-end BBS connection

#### ANSI Viewer (projects/ansi-view)

27. Create HTML viewer page with file picker
28. Add BPS rate dropdown selector
29. Implement file loading (FileReader API)
30. Implement viewer logic (clear on change, call renderAnsi)
31. Add sample ANSI files for testing
32. Create basic CSS styling
33. Copy WASM library to ansi-view directory
34. Test with various ANSI files and baud rates
