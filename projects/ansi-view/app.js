/**
 * ANSI Viewer Application
 *
 * Loads ANSI files and renders them using the WebTerm WASM library.
 */

import init, { renderAnsi } from './webterm_dos_ansi.js';

let wasmLoaded = false;

// Get DOM elements
const filePicker = document.getElementById('file-picker');
const bpsSelector = document.getElementById('bps-selector');
const clearBtn = document.getElementById('clear-btn');
const viewerContainer = document.getElementById('viewer-container');
const sampleButtons = document.querySelectorAll('.sample-btn');

// Current state
let currentFile = null;
let currentBps = parseInt(bpsSelector.value) || null;

// File picker handler
filePicker.addEventListener('change', async (e) => {
    const file = e.target.files[0];
    if (!file) return;

    try {
        const arrayBuffer = await file.arrayBuffer();
        currentFile = new Uint8Array(arrayBuffer);
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

// Clear button handler
clearBtn.addEventListener('click', () => {
    viewerContainer.innerHTML = '';
    currentFile = null;
    filePicker.value = '';
});

// Sample file buttons
sampleButtons.forEach(btn => {
    btn.addEventListener('click', async (e) => {
        const sampleName = e.target.dataset.sample;
        try {
            const response = await fetch(`sample/${sampleName}`);
            if (!response.ok) {
                throw new Error(`Sample file not found: ${sampleName}`);
            }
            const arrayBuffer = await response.arrayBuffer();
            currentFile = new Uint8Array(arrayBuffer);
            renderCurrentFile();
        } catch (error) {
            console.error('Error loading sample:', error);
            alert(`Sample file not available: ${sampleName}`);
        }
    });
});

// Render the current file
function renderCurrentFile() {
    if (!currentFile) return;

    // Clear previous render
    viewerContainer.innerHTML = '';

    // Render with current BPS setting
    renderAnsi('#viewer-container', currentFile, currentBps);
}

// Initialize WASM module
async function initWasm() {
    try {
        await init();
        wasmLoaded = true;
        console.log('WASM module loaded successfully');
    } catch (error) {
        console.error('Failed to load WASM:', error);
    }
}

// Initialize on load
initWasm();
