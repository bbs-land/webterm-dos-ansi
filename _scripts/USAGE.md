# Quick Usage Guide

## Build Everything

```bash
./_scripts/build-all.sh
```

This creates a `dist/` directory with all build artifacts.

## Run the ANSI Viewer

```bash
cd dist/ansi-view
python3 -m http.server 8080
```

Then open http://localhost:8080

### Try the Sample Files

1. Click "Simple" - Basic CP437 test
2. Click "Welcome" - Full welcome screen with box drawing
3. Click "Test Pattern" - Comprehensive ANSI test

### Try Different Speeds

Select a baud rate from the dropdown to simulate retro modem speeds:
- 300 bps - Very slow (early modems)
- 2400 bps - Typical BBS speed
- 9600 bps - Fast BBS speed
- Instant - No delay

### Load Your Own ANSI Files

Click "Choose File" to load local .ANS or .ASC files.

## Run the WebSocket Server

```bash
cd dist/websocket-connect
./websocket-connect
```

Then open http://localhost:3000

## Development

### Rebuild After Changes

```bash
# Rebuild just the WASM
cd projects/lib
wasm-pack build --target web

# Rebuild everything
cd ../..
./_scripts/build-all.sh
```

### Watch for Changes

```bash
# In one terminal - rebuild on changes
cd projects/lib
cargo watch -x 'build --target wasm32-unknown-unknown'

# In another terminal - serve viewer
cd projects/ansi-view
python3 -m http.server 8080
```

## File Sizes

- WASM binary: 33 KB
- JS bindings: 20 KB
- Total bundle: ~53 KB (uncompressed)

## Browser Console

Open browser DevTools console to see:
- WASM initialization messages
- Rendering debug info
- Any errors

## Troubleshooting

### WASM not loading
- Check browser console for errors
- Ensure you're serving over HTTP, not file://
- Verify WASM files are in the directory

### Sample files not loading
- Check that sample/ directory exists
- Verify files have proper .ans extension
- Check browser console for 404 errors

### No output displayed
- Ensure WASM module loaded successfully
- Check that renderAnsi() was called
- Verify canvas was created in container

## Tips

- Use Chrome/Edge for best WASM performance
- Firefox works but may be slightly slower
- Safari also supported (WebAssembly available)
- Try different sample files to see various ANSI features
- Experiment with baud rates for authentic retro feel
