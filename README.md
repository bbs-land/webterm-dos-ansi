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

See [.claude/CLAUDE.md](.claude/CLAUDE.md) for complete documentation including
architecture, API reference, build instructions, and development guidelines.

## License

Copyright © 2026 BBS.land

This project is licensed under the ISC License - see the
[LICENSE.md](LICENSE.md) file for details.
