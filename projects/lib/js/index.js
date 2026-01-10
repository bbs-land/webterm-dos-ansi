/**
 * WebTerm DOS ANSI - WASM Module Loader
 *
 * Minimal JavaScript glue code for loading the WASM module and
 * exposing the terminal functions.
 */

import init, { initWebTerm, renderAnsi } from '../pkg/webterm_dos_ansi.js';

// Initialize the WASM module
let wasmInitialized = false;

async function ensureWasmLoaded() {
  if (!wasmInitialized) {
    await init();
    wasmInitialized = true;
  }
}

// Auto-initialize on page load
if (typeof window !== 'undefined') {
  window.addEventListener('DOMContentLoaded', async () => {
    await ensureWasmLoaded();
    initWebTerm();
  });
}

// Export functions for manual use
export { initWebTerm, renderAnsi, ensureWasmLoaded };
