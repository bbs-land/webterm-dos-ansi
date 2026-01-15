# BBS.land webterm-dos-ansi

A WebAssembly-based terminal emulator library for rendering DOS CP437 ANSI art
and connecting to BBS (Bulletin Board System) servers via WebSocket. Built with
Rust and compiled to WASM for high-performance rendering in modern web browsers.

## Features

- **CP437 Character Set**: Full support for the classic DOS codepage including
  box drawing characters
- **ANSI/VT-100 Escape Sequences**: Comprehensive support for DOS-era BBS
  terminal sequences
- **EGA Font Rendering**: Authentic 8x14 pixel EGA font with proper aspect ratio
  correction (3×4 pixel scaling)
- **WebSocket Terminal**: Connect to BBS systems through WebSocket-to-TCP
  bridges
- **ANSI Art Viewer**: Standalone viewer for rendering .ANS files with baud rate
  simulation
- **Retro Experience**: Optional baud rate simulation (300-57600 bps) for
  authentic modem feel

## Project Structure

- **`projects/lib/`** - Main WASM library (Rust + wasm-pack)
- **`projects/websocket-connect/`** - WebSocket-to-TCP bridge server (Rust +
  Axum)
- **`projects/ansi-view/`** - Standalone ANSI file viewer application

## Quick Start

### Building

```bash
# Build the WASM library
run/lib-build

# Output is in projects/lib/pkg/:
# - webterm_dos_ansi.js
# - webterm_dos_ansi_bg.wasm
# - webterm_dos_ansi.d.ts
```

### Usage

```javascript
import init, { initWebTerm, renderAnsi } from '@bbs/webterm-dos-ansi';

// Initialize WASM module
await init();

// Auto-discover terminals with data-term-url attribute
initWebTerm();

// Or render ANSI content directly
renderAnsi('#container', ansiBytes, 9600); // optional baud rate
```

### Documentation

- [GETTING_STARTED.md](GETTING_STARTED.md) - Setup and build instructions
- [.claude/CLAUDE.md](.claude/CLAUDE.md) - Complete project documentation
- [projects/lib/README.md](projects/lib/README.md) - Library API documentation

## License

Copyright © 2026 BBS.land

This project is licensed under the ISC License - see the
[LICENSE.md](LICENSE.md) file for details.
