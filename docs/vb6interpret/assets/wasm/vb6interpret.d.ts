/* tslint:disable */
/* eslint-disable */

/**
 * Build a full statement-boundary execution trace that the browser can use
 * for true resume-from-current-state stepping.
 */
export function build_debug_trace(code: string): any;

/**
 * Execute a single VB6 module up to `pause_after_steps` statements and return
 * a snapshot suitable for debugger-style stepping.
 */
export function debug_vb6_code(code: string, pause_after_steps: number): any;

/**
 * Every setting currently in the store, for persisting back to `localStorage`.
 */
export function dump_settings(): any;

/**
 * Initializes the panic hook for better error messages in the browser console.
 */
export function init_panic_hook(): void;

/**
 * Install or overwrite the setting `(appname, section, key)` with `value`.
 *
 * The webassembly host has no filesystem, so `localStorage` takes the role
 * of the settings store root: the host calls [`install_setting`] once per
 * persisted entry before running a module, and persists [`dump_settings`]
 * afterwards. `GetSetting` reads whatever is installed.
 */
export function install_setting(appname: string, section: string, key: string, value: string): void;

/**
 * Execute a single VB6 module and return captured output plus runtime status.
 *
 * The interpreter playground currently supports module input only.
 */
export function interpret_vb6_code(code: string): any;

/**
 * Parses VB6 code and returns a `PlaygroundOutput` object containing tokens, CST, and errors.
 *
 * # Errors
 *
 * So far we do not correctly handle errors and failures and just panic but this must eventually
 * be converted into an error value.
 *
 * # Panics
 *
 * Currently, we are doing minimal error recovery and checking for the playground as this
 * is an attempt to get the system up and working well enough to demonstrate the possibilities.
 * As is, we can produce a panic if the input can not be tokenized.
 */
export function parse_vb6_code(code: string, _file_type: string): any;

/**
 * Remove the setting `(appname, section, key)`, if present.
 */
export function remove_setting(appname: string, section: string, key: string): void;

/**
 * Tokenizes VB6 code and returns a list of `TokenInfo` objects for quick preview.
 *
 * # Errors
 *
 * So far we do not correctly handle errors and failures and just panic but this must eventually
 * be converted into an error value.
 *
 * # Panics
 *
 * Currently, we are doing minimal error recovery and checking for the playground as this
 * is an attempt to get the system up and working well enough to demonstrate the possibilities.
 * As is, we can produce a panic if the input can not be tokenized.
 */
export function tokenize_vb6_code(code: string): any;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly build_debug_trace: (a: number, b: number, c: number) => void;
    readonly debug_vb6_code: (a: number, b: number, c: number, d: number) => void;
    readonly dump_settings: (a: number) => void;
    readonly install_setting: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => void;
    readonly interpret_vb6_code: (a: number, b: number, c: number) => void;
    readonly remove_setting: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
    readonly parse_vb6_code: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly tokenize_vb6_code: (a: number, b: number, c: number) => void;
    readonly init_panic_hook: () => void;
    readonly __wbindgen_export: (a: number, b: number) => number;
    readonly __wbindgen_export2: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_export3: (a: number, b: number, c: number) => void;
    readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
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
