# Setup Instructions

## Prerequisites Installation

### 1. Install Rustup (Rust Toolchain Manager)

Since Rust is currently installed via Homebrew, you'll need rustup for WASM development:

```bash
# Install rustup (this will manage Rust toolchains)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Follow the prompts and restart your shell

# Add WASM target
rustup target add wasm32-unknown-unknown
```

### 2. Install wasm-pack

```bash
# Install wasm-pack for building WASM modules
cargo install wasm-pack
```

### 3. Verify Deno Installation

Deno should already be installed. Verify with:

```bash
deno --version
```

If not installed:

```bash
# macOS/Linux
curl -fsSL https://deno.land/install.sh | sh
```

## Building the Project

### Build the Core WASM Library

```bash
cd projects/lib

# Build WASM module
wasm-pack build --target web

# This creates the pkg/ directory with:
# - webterm_dos_ansi.js
# - webterm_dos_ansi_bg.wasm
# - webterm_dos_ansi.d.ts
```

### Build JavaScript Wrapper (Optional)

```bash
cd projects/lib

# Build with Vite
deno task build
```

## Next Steps

After installing the prerequisites:

1. Build the WASM library: `cd projects/lib && wasm-pack build --target web`
2. Set up the WebSocket server project
3. Set up the ANSI viewer project
4. Test with sample ANSI files

## Troubleshooting

### Rust installed via Homebrew conflicts with rustup

If you have Rust via Homebrew and want to use rustup:

```bash
# Remove Homebrew Rust (optional)
brew uninstall rust

# Or ensure rustup's binaries are first in PATH
export PATH="$HOME/.cargo/bin:$PATH"
```

### WASM target not found

```bash
rustup target add wasm32-unknown-unknown
```

### wasm-pack not found

```bash
cargo install wasm-pack
```
