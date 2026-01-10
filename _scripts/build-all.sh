#!/usr/bin/env bash
set -e  # Exit on error

# Build script for BBS.land webterm-dos-ansi
# Builds WASM library and copies artifacts to dist directory

echo "🚀 BBS.land WebTerm - Build All"
echo "================================"
echo ""

# Get the workspace root (parent of _scripts)
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
WORKSPACE_ROOT="$( cd "$SCRIPT_DIR/.." && pwd )"
DIST_DIR="$WORKSPACE_ROOT/dist"

echo "📁 Workspace: $WORKSPACE_ROOT"
echo "📁 Dist: $DIST_DIR"
echo ""

# Clean dist directory
echo "🧹 Cleaning dist directory..."
if [ -d "$DIST_DIR" ]; then
    rm -rf "$DIST_DIR"
fi
mkdir -p "$DIST_DIR"
echo "   ✓ Cleaned"
echo ""

# Build WASM library
echo "🔨 Building WASM library..."
cd "$WORKSPACE_ROOT/projects/lib"
wasm-pack build --target web --scope bbs
if [ $? -ne 0 ]; then
    echo "   ✗ WASM build failed!"
    exit 1
fi
echo "   ✓ WASM build complete"
echo ""

# Create dist subdirectories
mkdir -p "$DIST_DIR/lib"
mkdir -p "$DIST_DIR/ansi-view"
mkdir -p "$DIST_DIR/ansi-view/sample"
mkdir -p "$DIST_DIR/websocket-connect/static"

# Copy WASM library artifacts
echo "📦 Copying WASM library to dist/lib..."
cp pkg/webterm_dos_ansi.js "$DIST_DIR/lib/"
cp pkg/webterm_dos_ansi_bg.wasm "$DIST_DIR/lib/"
cp pkg/webterm_dos_ansi.d.ts "$DIST_DIR/lib/"
cp pkg/package.json "$DIST_DIR/lib/"
cp LIB_README.md "$DIST_DIR/lib/README.md"

# Update package.json to include README.md in files array
if command -v node &> /dev/null; then
    node -e "
    const fs = require('fs');
    const pkg = JSON.parse(fs.readFileSync('$DIST_DIR/lib/package.json', 'utf8'));
    if (!pkg.files.includes('README.md')) {
        pkg.files.push('README.md');
    }
    fs.writeFileSync('$DIST_DIR/lib/package.json', JSON.stringify(pkg, null, 2) + '\n');
    "
else
    # Fallback: use sed if node is not available
    sed -i.bak 's/"webterm_dos_ansi.d.ts"/"webterm_dos_ansi.d.ts",\n    "README.md"/' "$DIST_DIR/lib/package.json"
    rm -f "$DIST_DIR/lib/package.json.bak"
fi

echo "   ✓ WASM library copied"
echo ""

# Copy ANSI viewer
echo "📦 Copying ANSI viewer to dist/ansi-view..."
cd "$WORKSPACE_ROOT/projects/ansi-view"
cp index.html "$DIST_DIR/ansi-view/"
cp styles.css "$DIST_DIR/ansi-view/"
cp app.js "$DIST_DIR/ansi-view/"
cp test.html "$DIST_DIR/ansi-view/"
cp README.md "$DIST_DIR/ansi-view/"
cp sample/*.ans "$DIST_DIR/ansi-view/sample/"
cp sample/CP437_REFERENCE.md "$DIST_DIR/ansi-view/sample/"
# Copy WASM files to ansi-view
cp "$DIST_DIR/lib/webterm_dos_ansi.js" "$DIST_DIR/ansi-view/"
cp "$DIST_DIR/lib/webterm_dos_ansi_bg.wasm" "$DIST_DIR/ansi-view/"
echo "   ✓ ANSI viewer copied"
echo ""

# Build WebSocket server (if cargo is available)
if command -v cargo &> /dev/null; then
    echo "🔨 Building WebSocket server..."
    cd "$WORKSPACE_ROOT/projects/websocket-connect"
    cargo build --release
    if [ $? -eq 0 ]; then
        # Copy binary
        cp target/release/websocket-connect "$DIST_DIR/websocket-connect/"
        # Copy static files
        cp static/index.html "$DIST_DIR/websocket-connect/static/"
        # Copy WASM files to websocket-connect static
        cp "$DIST_DIR/lib/webterm_dos_ansi.js" "$DIST_DIR/websocket-connect/static/"
        cp "$DIST_DIR/lib/webterm_dos_ansi_bg.wasm" "$DIST_DIR/websocket-connect/static/"
        echo "   ✓ WebSocket server built"
    else
        echo "   ⚠ WebSocket server build failed (non-critical)"
    fi
else
    echo "   ⚠ Cargo not found, skipping WebSocket server build"
fi
echo ""

# Generate build info
echo "📝 Generating build info..."
BUILD_DATE=$(date -u +"%Y-%m-%d %H:%M:%S UTC")
BUILD_COMMIT=$(git rev-parse --short HEAD 2>/dev/null || echo "unknown")

cat > "$DIST_DIR/BUILD_INFO.txt" <<EOF
BBS.land WebTerm DOS ANSI
=========================

Build Date: $BUILD_DATE
Git Commit: $BUILD_COMMIT

Contents:
---------
lib/                    - WASM library artifacts
  webterm_dos_ansi.js   - JavaScript bindings (ES module)
  webterm_dos_ansi_bg.wasm - WebAssembly binary
  webterm_dos_ansi.d.ts - TypeScript definitions

ansi-view/              - ANSI art viewer application
  index.html            - Main viewer page
  test.html             - Simple test page
  app.js                - Viewer application logic
  styles.css            - Viewer styles
  sample/*.ans          - Sample ANSI files (CP437 encoded)

websocket-connect/      - WebSocket to TCP bridge server (optional)
  websocket-connect     - Server binary (release build)
  static/               - Web interface files

Usage:
------
ANSI Viewer:
  cd ansi-view
  python3 -m http.server 8080
  # Open http://localhost:8080

WebSocket Server:
  cd websocket-connect
  ./websocket-connect
  # Server runs on http://localhost:3000
EOF

echo "   ✓ Build info generated"
echo ""

# Summary
echo "✅ Build complete!"
echo ""
echo "📊 Build Summary:"
WASM_SIZE=$(du -h "$DIST_DIR/lib/webterm_dos_ansi_bg.wasm" | cut -f1)
JS_SIZE=$(du -h "$DIST_DIR/lib/webterm_dos_ansi.js" | cut -f1)
echo "   WASM binary: $WASM_SIZE"
echo "   JS bindings: $JS_SIZE"
echo ""
echo "📁 Distribution directory: $DIST_DIR"
echo ""
echo "🎯 Next steps:"
echo "   cd dist/ansi-view"
echo "   python3 -m http.server 8080"
echo "   # Open http://localhost:8080"
echo ""
