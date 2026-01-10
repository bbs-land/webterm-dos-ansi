# ANSI Art Viewer

A standalone viewer for CP437 ANSI art files using the WebTerm WASM library.

## Quick Start

1. **Build the WASM library** (if not already done):
   ```bash
   cd ../lib
   wasm-pack build --target web
   cp pkg/webterm_dos_ansi.js pkg/webterm_dos_ansi_bg.wasm ../ansi-view/
   ```

2. **Serve the viewer**:
   ```bash
   # Using Python
   python3 -m http.server 8080

   # Or using Deno
   deno serve --port 8080 .
   ```

3. **Open in browser**:
   - Main viewer: http://localhost:8080
   - Simple test: http://localhost:8080/test.html

## Features

- **File Picker**: Load local .ANS, .ASC, or .TXT files with ANSI codes
- **Baud Rate Simulation**: Experience retro modem speeds (300-57600 bps)
- **Sample Files**: Pre-loaded ANSI art for testing
  - `simple.ans` - Basic colors and ASCII art
  - `welcome.ans` - Welcome screen with CP437 box drawing
  - `test.ans` - Comprehensive test pattern

## Sample Files

The viewer includes three sample ANSI files in the `sample/` directory:

### simple.ans
Basic ANSI test with:
- Primary colors (red, green, yellow, blue, magenta, cyan, white)
- Bold text
- Reverse video
- Simple ASCII box drawing

### welcome.ans
Welcome screen featuring:
- CP437 box-drawing characters (╔═╗ ║ ╚═╝)
- Multiple color combinations
- Background colors
- Bold and normal text mixing

### test.ans
Comprehensive test pattern with:
- Various box-drawing styles (single, double, mixed)
- Shade characters (░ ▒ ▓ █)
- Block elements (▀ ▄ ▌ ▐)
- Cursor positioning
- Text attributes (bold, dim, underline, blink, reverse)

## Creating Your Own ANSI Files

ANSI files use escape sequences for colors and formatting:

```bash
# Basic color codes
ESC[30m - Black foreground
ESC[31m - Red foreground
ESC[32m - Green foreground
ESC[33m - Yellow foreground
ESC[34m - Blue foreground
ESC[35m - Magenta foreground
ESC[36m - Cyan foreground
ESC[37m - White foreground

ESC[40-47m - Background colors (same order)
ESC[1m - Bold/bright
ESC[0m - Reset all attributes

# Example: printf '\033[1;32mGreen Bold Text\033[0m\n'
```

For more complex ANSI art, use dedicated tools:
- TheDraw (DOS)
- PabloDraw (Windows)
- Moebius (Cross-platform)

## Baud Rate Simulation

The viewer can simulate various modem speeds to recreate the authentic BBS experience:

- **300 bps** - Very slow (original 300 baud modems)
- **1200 bps** - Slow (early 1980s standard)
- **2400 bps** - Normal BBS speed (mid 1980s)
- **9600 bps** - Fast BBS speed (late 1980s)
- **14400+ bps** - Very fast (1990s modems)
- **Instant** - No delay, render immediately

## Technical Details

- **Character encoding**: CP437 (DOS)
- **Screen size**: 80 columns × 25 rows
- **Font**: EGA 8×14 (scaled 3×4 for aspect ratio)
- **Canvas size**: 1920×1400 pixels
- **Color palette**: 16 ANSI colors (8 normal + 8 bright)

## Troubleshooting

### WASM module not loading
Make sure you've built and copied the WASM files:
```bash
ls -l webterm_dos_ansi.js webterm_dos_ansi_bg.wasm
```

### CORS errors
Serve the files over HTTP, don't open `file://` URLs directly.

### Sample files not loading
Check that the `sample/` directory exists and contains the .ans files.

### Colors not showing
Verify the ANSI file has actual escape codes (ESC = 0x1B byte), not literal text like `[32m`.
