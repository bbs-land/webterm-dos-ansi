# Build Scripts

Build automation scripts for the BBS.land WebTerm DOS ANSI project.

## build-all.sh

Complete build script that:
1. Cleans the `dist/` directory
2. Builds the WASM library with wasm-pack
3. Builds the WebSocket server (if cargo is available)
4. Copies all artifacts to `dist/` directory
5. Generates build information

### Usage

```bash
# From project root
./_scripts/build-all.sh
```

### Output Structure

```
dist/
├── BUILD_INFO.txt              # Build metadata
├── lib/                        # WASM library
│   ├── webterm_dos_ansi.js
│   ├── webterm_dos_ansi_bg.wasm
│   ├── webterm_dos_ansi.d.ts
│   └── package.json
├── ansiview/                  # Viewer application
│   ├── index.html
│   ├── app.js
│   ├── styles.css
│   ├── test.html
│   ├── webterm_dos_ansi.js     # (copied from lib/)
│   ├── webterm_dos_ansi_bg.wasm
│   └── sample/
│       ├── simple.ans
│       ├── welcome.ans
│       ├── test.ans
│       └── CP437_REFERENCE.md
└── websocket-connect/          # Server (optional)
    ├── websocket-connect       # Binary
    └── static/
        ├── index.html
        ├── webterm_dos_ansi.js
        └── webterm_dos_ansi_bg.wasm
```

### Requirements

- **wasm-pack** - Required for building WASM
- **cargo** - Optional, for building WebSocket server
- **bash** - Script interpreter
- **git** - Optional, for commit hash in build info

### Testing the Build

After building, test the ANSI viewer:

```bash
cd dist/ansiview
python3 -m http.server 8080
# Open http://localhost:8080
```

Or test the WebSocket server:

```bash
cd dist/websocket-connect
./websocket-connect
# Open http://localhost:3000
```

## Environment Variables

None required. The script auto-detects the workspace root.

## Exit Codes

- **0** - Success
- **1** - WASM build failed
- **Other** - Script error

## Notes

- The script uses `set -e` to exit on any error
- WASM build is required; WebSocket server build is optional
- All paths are relative to workspace root
- The `dist/` directory is completely cleaned before each build
