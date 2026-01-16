/* tslint:disable */
/* eslint-disable */

/**
 * Options for rendering ANSI content.
 */
export class RenderOptions {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Create new render options with required selector.
     */
    constructor(selector: string);
    /**
     * Set baud rate for rendering simulation.
     */
    setBps(bps: number): RenderOptions;
    /**
     * Set color palette ("CGA" or "VGA").
     */
    setPalette(palette: string): RenderOptions;
    /**
     * Set scrollback buffer size.
     */
    setScrollbackLines(lines: number): RenderOptions;
}

/**
 * Initialize WebTerm terminals on the page.
 *
 * Scans the DOM for elements with `data-term-url` attribute and initializes
 * terminal instances for each one.
 *
 * Supported data attributes:
 * - `data-term-url`: WebSocket URL (required)
 * - `data-term-palette`: Color palette ("CGA" or "VGA", default: "VGA")
 * - `data-term-scrollback-lines`: Scrollback buffer size (default: 5000)
 */
export function initWebTerm(): void;

/**
 * Render CP437 ANSI content to a container element.
 *
 * # Arguments
 * * `content` - CP437 ANSI content as bytes
 * * `options` - Render options (selector, bps, palette, scrollback_lines)
 *
 * # Example (JavaScript)
 * ```javascript
 * const options = new RenderOptions("#terminal")
 *     .setBps(9600)
 *     .setPalette("CGA")
 *     .setScrollbackLines(10000);
 * renderAnsi(content, options);
 * ```
 */
export function renderAnsi(content: Uint8Array, options: RenderOptions): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_renderoptions_free: (a: number, b: number) => void;
    readonly initWebTerm: () => void;
    readonly renderAnsi: (a: number, b: number, c: number) => void;
    readonly renderoptions_new: (a: number, b: number) => number;
    readonly renderoptions_setBps: (a: number, b: number) => number;
    readonly renderoptions_setPalette: (a: number, b: number, c: number) => number;
    readonly renderoptions_setScrollbackLines: (a: number, b: number) => number;
    readonly __wasm_bindgen_func_elem_107: (a: number, b: number) => void;
    readonly __wasm_bindgen_func_elem_913: (a: number, b: number) => void;
    readonly __wasm_bindgen_func_elem_1007: (a: number, b: number, c: number, d: number) => void;
    readonly __wasm_bindgen_func_elem_145: (a: number, b: number, c: number) => void;
    readonly __wasm_bindgen_func_elem_928: (a: number, b: number, c: number) => void;
    readonly __wasm_bindgen_func_elem_144: (a: number, b: number) => void;
    readonly __wbindgen_export: (a: number, b: number) => number;
    readonly __wbindgen_export2: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_export3: (a: number) => void;
    readonly __wbindgen_export4: (a: number, b: number, c: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
