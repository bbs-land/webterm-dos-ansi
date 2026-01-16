# Getting Started with BBS.land webterm-dos-ansi

This guide will help you set up and start developing the WebTerm DOS ANSI project.

## Initial Project Setup Complete

The following has been set up:

### Core Library (`projects/lib/`)
- ✅ Rust/WASM project structure with Cargo.toml
- ✅ Core modules: lib.rs, screen.rs, parser.rs, renderer.rs, cp437.rs, dom.rs
- ✅ Basic ANSI escape sequence parser
- ✅ Screen buffer (80×25 character grid)
- ✅ Canvas renderer with color support
- ✅ DOM manipulation utilities
- ✅ Deno configuration for build tasks

### WebSocket Bridge Server (`projects/websocket-connect/`)
- ✅ Axum-based WebSocket server
- ✅ Basic WebSocket handler (echo mode for now)
- ✅ HTML terminal connection page
- ✅ Server configuration with static file serving

### ANSI Viewer (`projects/ansiview/`)
- ✅ HTML viewer interface
- ✅ File picker and BPS rate selector
- ✅ Sample ANSI files (welcome.ans, test.ans)
- ✅ Styling and layout

## Required Tools Installation

Before you can build and run the project, install these prerequisites:

### 1. Install Rustup (Rust Toolchain Manager)

You currently have Rust installed via Homebrew. For WASM development, you need rustup:

```bash
# Install rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Restart your shell or run:
source $HOME/.cargo/env

# Add WASM compilation target
rustup target add wasm32-unknown-unknown
```

### 2. Install wasm-pack

```bash
cargo install wasm-pack
```

### 3. Verify Deno

Deno should already be installed. Check with:

```bash
deno --version
```

## Building the Project

### Build the WASM Library

```bash
# From project root
run/lib-build

# This creates projects/lib/pkg/ with:
# - webterm_dos_ansi.js
# - webterm_dos_ansi_bg.wasm
# - webterm_dos_ansi.d.ts
# - package.json (npm package: @bbs/webterm-dos-ansi)
```

### Test the ANSI Viewer

```bash
# Copy WASM files to ansiview
cp projects/lib/pkg/webterm_dos_ansi.js projects/ansiview/
cp projects/lib/pkg/webterm_dos_ansi_bg.wasm projects/ansiview/

# Serve the ansiview directory
cd projects/ansiview
python3 -m http.server 8080

# Open http://localhost:8080 in your browser
```

### Test the WebSocket Server

```bash
# Copy WASM files to websocket-connect
cp projects/lib/pkg/webterm_dos_ansi.js projects/websocket-connect/static/
cp projects/lib/pkg/webterm_dos_ansi_bg.wasm projects/websocket-connect/static/

# Build and run the server
cd projects/websocket-connect
cargo run

# Open http://localhost:3000 in your browser
```

## Development Workflow

### Working on the WASM Library

```bash
cd projects/lib

# Build in development mode (faster, larger)
wasm-pack build --target web --dev

# Build in release mode (slower, optimized)
wasm-pack build --target web

# Run Rust tests
cargo test
```

### Working on the WebSocket Server

```bash
cd projects/websocket-connect

# Run in development mode (with logging)
RUST_LOG=debug cargo run

# Build release version
cargo build --release
./target/release/websocket-connect
```

### Working on the ANSI Viewer

After building the WASM library and copying files:

```bash
cd projects/ansiview

# Serve with Deno
deno serve --port 8080 .

# Or use Python
python3 -m http.server 8080

# Or use any static file server
```

## Next Development Steps

### Immediate Priorities

1. **Install Prerequisites** (see above)
   - Install rustup and add wasm32 target
   - Install wasm-pack

2. **Test the Build**
   - Build the WASM library
   - Test ANSI viewer with sample files
   - Verify basic rendering works

3. **Implement EGA Font**
   - Acquire or create 8×14 EGA font bitmap
   - Embed font in WASM binary
   - Update renderer to use actual font glyphs

4. **Enhance ANSI Parser**
   - Add missing escape sequences
   - Test with real BBS ANSI files
   - Handle edge cases

5. **Implement Connection UI**
   - Connect button overlay
   - Pre-connect splash screen
   - WebSocket connection handling

### Medium-term Goals

1. **Complete WebSocket Bridge**
   - Implement TCP connection
   - Bridge WebSocket ↔ TCP
   - Handle connection lifecycle

2. **Keyboard Input**
   - Capture keyboard events
   - Send to WebSocket
   - Handle special keys

3. **Baud Rate Simulation**
   - Character-by-character rendering
   - Accurate timing based on BPS

4. **Testing**
   - Unit tests for parser
   - Integration tests
   - Visual regression tests

## Project Structure Reference

```
webterm-dos-ansi/
├── projects/
│   ├── lib/                    # Main WASM library
│   │   ├── src/
│   │   │   ├── lib.rs          # initWebTerm(), renderAnsi()
│   │   │   ├── screen.rs       # 80×25 screen buffer
│   │   │   ├── parser.rs       # ANSI parser
│   │   │   ├── renderer.rs     # Canvas rendering
│   │   │   ├── cp437.rs        # CP437 codec
│   │   │   └── dom.rs          # DOM utilities
│   │   └── Cargo.toml
│   │
│   ├── websocket-connect/      # WebSocket bridge server
│   │   ├── src/
│   │   │   ├── main.rs
│   │   │   └── websocket.rs
│   │   ├── static/
│   │   │   └── index.html
│   │   └── Cargo.toml
│   │
│   └── ansiview/              # ANSI file viewer
│       ├── index.html
│       ├── styles.css
│       ├── app.js
│       └── sample/             # Sample ANSI files
│           ├── welcome.ans
│           └── test.ans
│
├── .gitignore
├── LICENSE.md
├── README.md
├── SETUP.md
├── GETTING_STARTED.md          # This file
└── .claude/
    └── CLAUDE.md               # Detailed documentation
```

## Troubleshooting

### "wasm-pack not found"
```bash
cargo install wasm-pack
```

### "can't find crate for `core`"
```bash
rustup target add wasm32-unknown-unknown
```

### "Rust installed via Homebrew"
You need rustup for WASM development. Follow the rustup installation steps above.

### WASM file not loading in browser
1. Check that you copied the WASM files to the correct directory
2. Verify the file paths in the JavaScript imports
3. Check browser console for errors
4. Ensure you're serving the files (not opening file:// URLs)

## Resources

- [CLAUDE.md](.claude/CLAUDE.md) - Detailed project documentation
- [README.md](README.md) - Project overview
- [SETUP.md](SETUP.md) - Detailed setup instructions
- [Rust and WebAssembly Book](https://rustwasm.github.io/book/)
- [wasm-pack Documentation](https://rustwasm.github.io/wasm-pack/)

## Getting Help

If you encounter issues:

1. Check the troubleshooting section above
2. Review the detailed documentation in [.claude/CLAUDE.md](.claude/CLAUDE.md)
3. Check that all prerequisites are installed correctly
4. Verify you're in the correct directory when running commands

Happy coding! 🚀
