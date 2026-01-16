/**
 * ANSI Viewer Application
 *
 * Loads ANSI files and renders them using the WebTerm WASM library.
 */

import init, { renderAnsi, RenderOptions } from './lib/mod.js';

// Get DOM elements
const filePicker = document.getElementById('file-picker');
const bpsSelector = document.getElementById('bps-selector');
const paletteSelector = document.getElementById('palette-selector');
const sizeSelector = document.getElementById('size-selector');
const sampleSelector = document.getElementById('sample-selector');
const clearBtn = document.getElementById('clear-btn');
const viewerContainer = document.getElementById('viewer-container');

// LocalStorage keys
const STORAGE_KEYS = {
    bps: 'ansiview-bps',
    palette: 'ansiview-palette',
    size: 'ansiview-size',
};

// Current state
let currentFile = null;
let currentBps = null;
let currentPalette = null;
let currentSize = null;

// Apply size class to container
function applySize() {
    // Remove all size classes
    viewerContainer.classList.remove('size-640', 'size-960', 'size-1280');
    // Add the selected size class if not "Fit"
    if (currentSize) {
        viewerContainer.classList.add(`size-${currentSize}`);
    }
}

// Load saved settings from localStorage and apply them
function loadSavedSettings() {
    const savedBps = localStorage.getItem(STORAGE_KEYS.bps);
    const savedPalette = localStorage.getItem(STORAGE_KEYS.palette);
    const savedSize = localStorage.getItem(STORAGE_KEYS.size);

    if (savedBps !== null) {
        bpsSelector.value = savedBps;
    }
    if (savedPalette !== null) {
        paletteSelector.value = savedPalette;
    }
    if (savedSize !== null) {
        sizeSelector.value = savedSize;
    }

    // Update current state from (possibly restored) selector values
    currentBps = parseInt(bpsSelector.value) || null;
    currentPalette = paletteSelector.value;
    currentSize = sizeSelector.value;

    // Apply initial size
    applySize();
}

// Load settings on startup
loadSavedSettings();

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
    localStorage.setItem(STORAGE_KEYS.bps, e.target.value);
    if (currentFile) {
        renderCurrentFile();
    }
});

// Palette selector handler
paletteSelector.addEventListener('change', (e) => {
    currentPalette = e.target.value;
    localStorage.setItem(STORAGE_KEYS.palette, e.target.value);
    if (currentFile) {
        renderCurrentFile();
    }
});

// Size selector handler
sizeSelector.addEventListener('change', (e) => {
    currentSize = e.target.value;
    localStorage.setItem(STORAGE_KEYS.size, e.target.value);
    applySize();
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

    // Apply current size
    applySize();

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
