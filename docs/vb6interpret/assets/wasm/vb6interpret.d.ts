/* tslint:disable */
/* eslint-disable */

/**
 * Chroma subsampling format
 */
export enum ChromaSampling {
    /**
     * Both vertically and horizontally subsampled.
     */
    Cs420 = 0,
    /**
     * Horizontally subsampled.
     */
    Cs422 = 1,
    /**
     * Not subsampled.
     */
    Cs444 = 2,
    /**
     * Monochrome.
     */
    Cs400 = 3,
}

/**
 * A VB6 project loaded from JS-provided byte maps for multi-file VBP-style
 * projects in the browser.
 *
 * The JS layer constructs a `WasmProject` by populating the `forms`,
 * `modules`, and `classes` fields with [`JsMap`] instances whose keys are
 * file names and whose values are `Uint8Array` (raw file bytes).  The
 * `startup` field holds the startup object name (form name or module name).
 *
 * # Examples
 *
 * Constructed on the JS side from a file picker or a build tool:
 *
 * ```js
 * const project = new WasmProject();
 * project.forms.set("Form1.frm", form1Bytes);
 * project.modules.set("Module1.bas", module1Bytes);
 * project.startup = "Sub Main";
 * ```
 */
export class WasmProject {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Create a new, empty `WasmProject`.
     *
     * Callers on the JS side should populate `forms`, `modules`, and
     * `classes` with [`JsMap`] instances and set `startup` before passing
     * the project to a WASM function.
     */
    constructor();
    /**
     * The startup object name (form name or module name).
     */
    startup: string;
}

/**
 * Build a full statement-boundary execution trace that the browser can use
 * for true resume-from-current-state stepping.
 */
export function build_debug_trace(code: string): any;

/**
 * Call a Sub procedure by name within a running project session.
 */
export function call_sub(state_handle: number, name: string): any;

/**
 * Close every open file and wipe the in-memory file backend, restoring it to
 * an empty filesystem.
 */
export function clear_files(): void;

/**
 * Execute a single VB6 module up to `pause_after_steps` statements and return
 * a snapshot suitable for debugger-style stepping.
 */
export function debug_vb6_code(code: string, pause_after_steps: number): any;

/**
 * Dispose a session and free its resources.
 */
export function dispose_state(state_handle: number): boolean;

/**
 * The current mock clock date and time, for the Clock section of the
 * Environment tab. Displayed in the system's local time zone, matching
 * what VB6's `Now`/`Date`/`Time` functions would report.
 *
 * `time` is 24-hour (`HH:MM:SS`) so it round-trips with [`set_clock`] and
 * native `<input type="time">` elements; the browser formats it for
 * display (e.g. 12-hour with AM/PM).
 */
export function dump_clock(): any;

/**
 * Every environment variable currently in the snapshot, for display and
 * persisting back to `localStorage`.
 */
export function dump_env(): any;

/**
 * Snapshot of the memory file backend for the Files tab.
 */
export function dump_files(): any;

/**
 * Every setting currently in the store, for persisting back to `localStorage`.
 */
export function dump_settings(): any;

/**
 * Get event procedure bindings for a loaded form.
 *
 * Returns a JSON array of bindings in the same format as the Tauri
 * `form_event_bindings` command. Each binding contains:
 * - `node_id`: The unique node ID of the control in the layout tree.
 * - `control`: The VB6 control name (e.g. `"cmdOK"`).
 * - `event`: The VB6 event name (e.g. `"Click"`).
 * - `procedure`: The full procedure name (e.g. `"cmdOK_Click"`).
 *
 * # Arguments
 *
 * * `form_handle` — The handle returned by [`show_form`].
 *
 * # Errors
 *
 * Returns a `JsValue` error if the form handle is unknown.
 */
export function get_form_procedures(form_handle: number): any;

/**
 * Get all captured output for a session.
 */
export function get_output(state_handle: number): string[];

/**
 * Hide a loaded form by handle.
 *
 * Sets the form's visibility to false and re-renders the DOM.
 * Uses the same container as the original [`show_form`] call.
 *
 * # Note
 *
 * This function uses the default container `"vb6-container"` for
 * backward compatibility. For multi-container support, use
 * [`hide_form_with_container`].
 */
export function hide_form(handle: number): void;

/**
 * Hide a loaded form by handle, targeting a specific container.
 *
 * Sets the form's visibility to false and re-renders the DOM in the
 * container identified by `container_id`.
 */
export function hide_form_with_container(handle: number, container_id: string): void;

/**
 * Initializes the panic hook for better error messages in the browser console.
 */
export function init_panic_hook(): void;

/**
 * Create or replace the file at `path` with raw `content`, bypassing
 * `Open`/`Close`. Used to restore a snapshot saved from the Files tab.
 */
export function install_file(path: string, content: Uint8Array): void;

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
 * Remove environment variable `name` from the snapshot, if present.
 */
export function remove_env(name: string): void;

/**
 * Remove the setting `(appname, section, key)`, if present.
 */
export function remove_setting(appname: string, section: string, key: string): void;

/**
 * Run a project (forms + modules + classes) and return a state handle.
 *
 * The handle can be used with [`call_sub`] and [`get_output`] to interact
 * with the running interpreter session. Returns a handle that JS must
 * pass to subsequent calls.
 */
export function run_project(form_bytes: any, module_bytes: any, class_bytes: any, startup: string): any;

/**
 * Execute a multi-file VB6 project loaded from JS-provided byte maps.
 *
 * The `form_bytes`, `module_bytes`, and `class_bytes` arguments are JS
 * `Map<string, Uint8Array>` instances.  Each map key is a file name and
 * each value is the raw file contents as bytes.
 *
 * The `startup` field selects which procedure to run after all
 * module-level statements have executed:
 *
 * * If a loaded form has a matching `VB_Name`, `Form_Load` is invoked.
 * * If `startup` equals `"Sub Main"` or `"Main"`, the `Main` sub is called.
 * * If `startup` matches a module name, `ModuleName.Main` is called.
 * * If `startup` is empty, all module-level statements run but no entry
 *   procedure is invoked.
 *
 * Returns a [`WasmRunOutput`] serialised as JSON.
 */
export function run_wasm_project(form_bytes: any, module_bytes: any, class_bytes: any, startup: string): any;

/**
 * Set the in-memory clock to `date` (`YYYY-MM-DD`) and `time` (`HH:MM:SS`)
 * in the system's local time zone.
 *
 * This rewrites the memory clock backend directly; it never touches (and is
 * never echoed back to) the real system clock.
 */
export function set_clock(date: string, time: string): void;

/**
 * Set (or replace) the value of environment variable `name` in the snapshot.
 *
 * The webassembly host has no process environment, so the snapshot starts
 * empty and is seeded from `localStorage` before a run; `Environ$` reads
 * whatever is installed here.
 */
export function set_env(name: string, value: string): void;

/**
 * Parse form bytes (as produced by a VB6 `.frm` file), load into the layout
 * engine, and render the DOM tree into the container identified by `container_id`.
 *
 * Returns a handle that can be used with [`hide_form`], [`show_form_by_handle`],
 * [`unload_form`], and [`update_form`].
 *
 * # Arguments
 *
 * * `form_bytes` — Raw bytes of a VB6 `.frm` file.
 * * `container_id` — The `id` attribute of the DOM element to render into.
 *   Defaults to `"vb6-container"` if empty.
 *
 * # Errors
 *
 * Returns a `JsValue` error string if:
 * - The bytes cannot be parsed as a VB6 Form file.
 * - The container element does not exist in the DOM.
 * - The form cannot be retrieved from the layout store after loading.
 */
export function show_form(form_bytes: Uint8Array, container_id: string): any;

/**
 * Show (unhide) a previously hidden form by handle.
 *
 * Sets the form's visibility to true and re-renders the DOM.
 * Uses the same container as the original [`show_form`] call.
 */
export function show_form_by_handle(handle: number): void;

/**
 * Show (unhide) a previously hidden form by handle, targeting a specific container.
 *
 * Sets the form's visibility to true and re-renders the DOM in the
 * container identified by `container_id`.
 */
export function show_form_by_handle_with_container(handle: number, container_id: string): void;

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

/**
 * Remove a form from the layout store and clear the DOM container.
 *
 * After calling this, the handle is no longer valid.
 *
 * # Arguments
 *
 * * `handle` — The form handle returned by [`show_form`].
 * * `container_id` — The container to clear. Defaults to `"vb6-container"` if empty.
 */
export function unload_form(handle: number, container_id: string): void;

/**
 * Re-render a loaded form after state changes.
 *
 * Clears the DOM container and re-renders the form from the current
 * layout model state. This should be called after mutating form properties
 * via [`layout::get_form_mut`].
 *
 * Uses the same container as the original [`show_form`] call.
 */
export function update_form(handle: number): void;

/**
 * Re-render a loaded form after state changes, targeting a specific container.
 *
 * Clears the DOM container identified by `container_id` and re-renders
 * the form from the current layout model state.
 */
export function update_form_with_container(handle: number, container_id: string): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_get_wasmproject_startup: (a: number, b: number) => void;
    readonly __wbg_set_wasmproject_startup: (a: number, b: number, c: number) => void;
    readonly __wbg_wasmproject_free: (a: number, b: number) => void;
    readonly build_debug_trace: (a: number, b: number, c: number) => void;
    readonly call_sub: (a: number, b: number, c: number, d: number) => void;
    readonly clear_files: () => void;
    readonly debug_vb6_code: (a: number, b: number, c: number, d: number) => void;
    readonly dispose_state: (a: number) => number;
    readonly dump_clock: (a: number) => void;
    readonly dump_env: (a: number) => void;
    readonly dump_files: (a: number) => void;
    readonly dump_settings: (a: number) => void;
    readonly get_form_procedures: (a: number, b: number) => void;
    readonly get_output: (a: number, b: number) => void;
    readonly hide_form: (a: number, b: number) => void;
    readonly hide_form_with_container: (a: number, b: number, c: number, d: number) => void;
    readonly init_panic_hook: () => void;
    readonly install_file: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly install_setting: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => void;
    readonly interpret_vb6_code: (a: number, b: number, c: number) => void;
    readonly parse_vb6_code: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly remove_env: (a: number, b: number) => void;
    readonly remove_setting: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
    readonly run_project: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
    readonly run_wasm_project: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
    readonly set_clock: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly set_env: (a: number, b: number, c: number, d: number) => void;
    readonly show_form: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly show_form_by_handle: (a: number, b: number) => void;
    readonly show_form_by_handle_with_container: (a: number, b: number, c: number, d: number) => void;
    readonly tokenize_vb6_code: (a: number, b: number, c: number) => void;
    readonly unload_form: (a: number, b: number, c: number, d: number) => void;
    readonly update_form: (a: number, b: number) => void;
    readonly update_form_with_container: (a: number, b: number, c: number, d: number) => void;
    readonly wasmproject_new: () => number;
    readonly __wbindgen_export: (a: number, b: number) => number;
    readonly __wbindgen_export2: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_export3: (a: number) => void;
    readonly __wbindgen_export4: (a: number, b: number, c: number) => void;
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
