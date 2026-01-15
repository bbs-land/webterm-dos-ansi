/**
 * ANSI Viewer Application
 *
 * Loads ANSI files and renders them using the WebTerm WASM library.
 */

import init, { renderAnsi, RenderOptions } from '/assets/lib/mod.js';

// Get DOM elements
const filePicker = document.getElementById('file-picker');
const bpsSelector = document.getElementById('bps-selector');
const paletteSelector = document.getElementById('palette-selector');
const sampleSelector = document.getElementById('sample-selector');
const clearBtn = document.getElementById('clear-btn');
const viewerContainer = document.getElementById('viewer-container');

// Current state
let currentFile = null;
let currentBps = parseInt(bpsSelector.value) || null;
let currentPalette = paletteSelector.value;

// File picker handler
filePicker.addEventListener('change', async (e) => {
    const file = e.target.files[0];
    if (!file) return;

    try {
        const arrayBuffer = await file.arrayBuffer();
        currentFile = new Uint8Array(arrayBuffer);
        sampleSelector.value = ''; // Clear sample selection
        renderCurrentFile();
    } catch (error) {
        console.error('Error reading file:', error);
        alert('Failed to read file');
    }
});

// BPS selector handler
bpsSelector.addEventListener('change', (e) => {
    currentBps = parseInt(e.target.value) || null;
    if (currentFile) {
        renderCurrentFile();
    }
});

// Palette selector handler
paletteSelector.addEventListener('change', (e) => {
    currentPalette = e.target.value;
    if (currentFile) {
        renderCurrentFile();
    }
});

// Sample selector handler
sampleSelector.addEventListener('change', async (e) => {
    const sampleName = e.target.value;
    if (!sampleName) return;

    try {
        const response = await fetch(`sample/${sampleName}`);
        if (!response.ok) {
            throw new Error(`HTTP ${response.status}: ${sampleName}`);
        }
        const arrayBuffer = await response.arrayBuffer();
        currentFile = new Uint8Array(arrayBuffer);
        filePicker.value = ''; // Clear file picker
        renderCurrentFile();
    } catch (error) {
        console.error('Error loading sample:', error);
        alert(`Failed to load sample: ${sampleName}`);
    }
});

// Clear button handler
clearBtn.addEventListener('click', () => {
    viewerContainer.innerHTML = '';
    currentFile = null;
    filePicker.value = '';
    sampleSelector.value = '';
});

// Render the current file
function renderCurrentFile() {
    if (!currentFile) return;

    // Clear previous render
    viewerContainer.innerHTML = '';

    // Build render options
    let options = new RenderOptions('#viewer-container');
    if (currentBps) {
        options = options.setBps(currentBps);
    }
    if (currentPalette) {
        options = options.setPalette(currentPalette);
    }

    // Render with current settings
    renderAnsi(currentFile, options);
}

// Initialize WASM module
async function initWasm() {
    try {
        await init();
        console.log('WASM module loaded successfully');
    } catch (error) {
        console.error('Failed to load WASM:', error);
    }
}

// Initialize on load
initWasm();
