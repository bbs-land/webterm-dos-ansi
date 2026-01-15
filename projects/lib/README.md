# @bbs/webterm-dos-ansi

WebAssembly terminal emulator library for rendering DOS CP437 ANSI art and connecting to BBS systems.

## Installation

### From NPM (when published)

#### npm:
```bash
npm install @bbs/webterm-dos-ansi
```

#### deno:
```bash
deno install jsr:@bbs/webterm-dos-ansi
# or
deno install npm:@bbs/webterm-dos-ansi
```

## Usage

### Basic ANSI Rendering

```javascript
import init, { renderAnsi } from '@bbs/webterm-dos-ansi';

// Initialize WASM module
await init();

// Render ANSI content to a container
const ansiContent = new TextEncoder().encode(
  '\x1b[1;32mHello, ANSI World!\x1b[0m'
);

renderAnsi('#terminal-container', ansiContent);
```

### With Baud Rate Simulation

```javascript
import init, { renderAnsi } from '@bbs/webterm-dos-ansi';

await init();

// Simulate 2400 bps modem speed
const ansiFile = await fetch('/path/to/file.ans')
  .then(r => r.arrayBuffer())
  .then(b => new Uint8Array(b));

renderAnsi('#terminal-container', ansiFile, 2400);
```

### Auto-Initialize Terminals

```html
<!-- Terminal container with data attributes -->
<div
  data-term-url="wss://bbs.example.com/ws"
  data-term-connect-button="/assets/connect.png"
  data-term-preconnect-screen="Click to connect..."
></div>

<script type="module">
  import init, { initWebTerm } from '@bbs/webterm-dos-ansi';

  await init();
  initWebTerm(); // Scans for all [data-term-url] elements
</script>
```

## API

### `init()`

Initialize the WASM module. Must be called before using other functions.

**Returns:** `Promise<void>`

```javascript
await init();
```

### `renderAnsi(selector, content, bps?)`

Render CP437 ANSI content to a container element.

**Parameters:**
- `selector` (string) - CSS selector for container element
- `content` (Uint8Array) - CP437 ANSI content as bytes
- `bps` (number, optional) - Baud rate for simulation (300-57600)

**Example:**

```javascript
renderAnsi('#viewer', ansiBytes, 9600);
```

### `initWebTerm()`

Auto-discover and initialize terminal elements with `data-term-url` attribute.

**Example:**

```html
<div data-term-url="wss://bbs.example.com/ws"></div>

<script type="module">
  import init, { initWebTerm } from '@bbs/webterm-dos-ansi';
  await init();
  initWebTerm();
</script>
```

## Data Attributes

### `data-term-url` (required)

WebSocket URL to connect to.

```html
<div data-term-url="wss://bbs.example.com/ws"></div>
```

### `data-term-connect-button` (optional)

URL to PNG image for connect button overlay.

```html
<div
  data-term-url="wss://bbs.example.com/ws"
  data-term-connect-button="/images/connect.png"
></div>
```

### `data-term-preconnect-screen` (optional)

CP437 ANSI text to display before connection.

```html
<div
  data-term-url="wss://bbs.example.com/ws"
  data-term-preconnect-screen="Welcome! Click to connect..."
></div>
```

## Terminal Specifications

- **Screen Size:** 80 columns × 25 rows
- **Canvas Size:** 1920×1400 pixels
- **Font:** EGA 8×14 pixels (scaled 3×4 for aspect ratio)
- **Colors:** 16 ANSI colors (8 standard + 8 bright)
- **Character Encoding:** CP437 (DOS codepage)
- **Escape Sequences:** VT-100/VT-102 compatible

## Supported ANSI Sequences

### Cursor Control
- `ESC[H` - Home
- `ESC[{row};{col}H` - Position
- `ESC[{n}A/B/C/D` - Up/Down/Forward/Backward

### Colors
- `ESC[30-37m` - Foreground colors
- `ESC[40-47m` - Background colors
- `ESC[90-97m` - Bright foreground
- `ESC[100-107m` - Bright background

### Attributes
- `ESC[0m` - Reset
- `ESC[1m` - Bold/bright
- `ESC[5m` - Blink
- `ESC[7m` - Reverse video

### Display
- `ESC[2J` - Clear screen
- `ESC[K` - Clear line

## Browser Support

- Chrome/Edge 90+
- Firefox 89+
- Safari 15+

Requires WebAssembly and ES modules support.

## File Sizes

- WASM binary: ~33 KB
- JS bindings: ~20 KB
- Total: ~53 KB (uncompressed)

## Examples

### Load Local File

```javascript
import init, { renderAnsi } from '@bbs/webterm-dos-ansi';

await init();

document.getElementById('file-input').addEventListener('change', async (e) => {
  const file = e.target.files[0];
  const buffer = await file.arrayBuffer();
  renderAnsi('#terminal', new Uint8Array(buffer));
});
```

### Fetch Remote ANSI

```javascript
import init, { renderAnsi } from '@bbs/webterm-dos-ansi';

await init();

const response = await fetch('/ansi/welcome.ans');
const buffer = await response.arrayBuffer();
renderAnsi('#terminal', new Uint8Array(buffer), 9600);
```

### Multiple Terminals

```html
<div id="term1" data-term-url="wss://bbs1.example.com/ws"></div>
<div id="term2" data-term-url="wss://bbs2.example.com/ws"></div>

<script type="module">
  import init, { initWebTerm } from '@bbs/webterm-dos-ansi';
  await init();
  initWebTerm(); // Initializes both terminals
</script>
```

## TypeScript Support

TypeScript definitions are included:

```typescript
import init, { renderAnsi, initWebTerm } from '@bbs/webterm-dos-ansi';

await init();

const content: Uint8Array = new TextEncoder().encode('\x1b[32mGreen\x1b[0m');
renderAnsi('#terminal', content, 2400);
```

## License

ISC License - Copyright © 2026 BBS.land

## Links

- [GitHub Repository](https://github.com/bbs-land/webterm-dos-ansi)
- [Documentation](https://github.com/bbs-land/webterm-dos-ansi/blob/main/.claude/CLAUDE.md)
- [Examples](https://github.com/bbs-land/webterm-dos-ansi/tree/main/projects/ansi-view)

## Support

For issues and questions:
- GitHub Issues: https://github.com/bbs-land/webterm-dos-ansi/issues
- Website: https://bbs.land
