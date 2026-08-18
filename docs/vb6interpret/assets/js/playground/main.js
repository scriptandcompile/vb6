import init, { build_debug_trace, clear_files, dump_env, dump_files, dump_settings, install_file, install_setting, interpret_vb6_code, init_panic_hook, remove_env, remove_setting, set_env } from "../../wasm/vb6interpret.js";
import { getDefaultExample, getExample } from "./examples.js";
import * as Editor from "./editor.js";
import { createZip } from "./zip.js";

const state = {
    wasmReady: false,
    activeTab: "output",
    isResizing: false,
    initialCode: "",
    debugTrace: null,
    debugTraceIndex: -1,
    lastDebugSource: "",
    hasExecutionState: false,
    sessionComplete: false,
    activeFilePath: null,
};

const elements = {
    fileType: document.getElementById("file-type"),
    examples: document.getElementById("examples"),
    runButton: document.getElementById("run-btn"),
    panelRunButton: document.getElementById("panel-run-btn"),
    stepButton: document.getElementById("step-btn"),
    resetButton: document.getElementById("reset-btn"),
    shareButton: document.getElementById("share-btn"),
    clearButton: document.getElementById("clear-btn"),
    stdout: document.getElementById("stdout"),
    errorBox: document.getElementById("error-box"),
    wasmStatus: document.getElementById("wasm-status"),
    runStatus: document.getElementById("run-status"),
    resizer: document.getElementById("resizer"),
    editorPanel: document.querySelector(".editor-panel"),
    outputPanel: document.querySelector(".output-panel"),
    debugSteps: document.getElementById("debug-steps"),
    debugPosition: document.getElementById("debug-position"),
    debugProcedure: document.getElementById("debug-procedure"),
    debugStackDepth: document.getElementById("debug-stack-depth"),
    debugGlobals: document.getElementById("debug-globals"),
    debugLocals: document.getElementById("debug-locals"),
    environmentTab: document.getElementById("environment-tab"),
    environmentStatus: document.getElementById("environment-status"),
    envTable: document.getElementById("env-table"),
    envAddForm: document.getElementById("env-add-form"),
    envName: document.getElementById("env-name"),
    envValue: document.getElementById("env-value"),
    settingsTable: document.getElementById("settings-table"),
    settingAddForm: document.getElementById("setting-add-form"),
    settingAppname: document.getElementById("setting-appname"),
    settingSection: document.getElementById("setting-section"),
    settingKey: document.getElementById("setting-key"),
    settingValue: document.getElementById("setting-value"),
    filesCurrentDir: document.getElementById("files-current-dir"),
    filesCurrentDrive: document.getElementById("files-current-drive"),
    filesOpenNumbers: document.getElementById("files-open-numbers"),
    filesListNav: document.getElementById("files-list-nav"),
    filesDetail: document.getElementById("files-detail"),
    filesSaveButton: document.getElementById("files-save-btn"),
    filesLoadButton: document.getElementById("files-load-btn"),
    filesDownloadButton: document.getElementById("files-download-btn"),
    filesClearButton: document.getElementById("files-clear-btn"),
    tabButtons: Array.from(document.querySelectorAll(".tab-btn")),
    tabPanes: Array.from(document.querySelectorAll(".tab-pane")),
};

/// Key prefix under which settings are persisted in `localStorage`, mirroring
/// the registry-style `<appname>/<section>/<key>` hierarchy the runtime uses
/// on disk. Each stored entry is a single localStorage key.
const SETTINGS_STORAGE_PREFIX = "vb6interpret-settings:";

/// Key prefix under which environment variables are persisted in
/// `localStorage`. The browser has no OS environment, so this snapshot stands
/// in for it.
const ENV_STORAGE_PREFIX = "vb6interpret-env:";

/// Key prefix under which memory-backend files are persisted in
/// `localStorage` (base64-encoded content, one key per file path), saved and
/// restored explicitly via the Files tab's Save/Load buttons.
const FILES_STORAGE_PREFIX = "vb6interpret-files:";

function settingsStorageKey(appname, section, key) {
    return `${SETTINGS_STORAGE_PREFIX}${appname}/${section}/${key}`;
}

/// Seed the in-memory settings store from `localStorage` so `GetSetting`
/// sees previously persisted values. The webassembly host has no filesystem,
/// so localStorage stands in for the store root.
function loadSettingsFromLocalStorage() {
    for (let index = 0; index < window.localStorage.length; index += 1) {
        const storageKey = window.localStorage.key(index);
        if (!storageKey || !storageKey.startsWith(SETTINGS_STORAGE_PREFIX)) {
            continue;
        }
        const [appname, section, key] = storageKey
            .slice(SETTINGS_STORAGE_PREFIX.length)
            .split("/");
        if (appname && section && key) {
            install_setting(appname, section, key, window.localStorage.getItem(storageKey));
        }
    }
}

/// Write every current setting back to `localStorage` (replacing the whole
/// block) so changes made during a run, such as a future `SaveSetting`,
/// survive a page reload.
function persistSettingsToLocalStorage() {
    const settings = dump_settings();
    const staleKeys = [];
    for (let index = 0; index < window.localStorage.length; index += 1) {
        const storageKey = window.localStorage.key(index);
        if (storageKey && storageKey.startsWith(SETTINGS_STORAGE_PREFIX)) {
            staleKeys.push(storageKey);
        }
    }
    staleKeys.forEach((storageKey) => window.localStorage.removeItem(storageKey));
    settings.forEach((setting) => {
        window.localStorage.setItem(
            settingsStorageKey(setting.appname, setting.section, setting.key),
            setting.value,
        );
    });
}

/// Seed the environment snapshot from `localStorage` so `Environ$` sees
/// previously persisted variables.
function loadEnvFromLocalStorage() {
    for (let index = 0; index < window.localStorage.length; index += 1) {
        const storageKey = window.localStorage.key(index);
        if (!storageKey || !storageKey.startsWith(ENV_STORAGE_PREFIX)) {
            continue;
        }
        const name = storageKey.slice(ENV_STORAGE_PREFIX.length);
        if (name) {
            set_env(name, window.localStorage.getItem(storageKey));
        }
    }
}

/// Write every current environment variable back to `localStorage` so edits
/// made in the Environment tab survive a page reload.
function persistEnvToLocalStorage() {
    const entries = dump_env();
    const staleKeys = [];
    for (let index = 0; index < window.localStorage.length; index += 1) {
        const storageKey = window.localStorage.key(index);
        if (storageKey && storageKey.startsWith(ENV_STORAGE_PREFIX)) {
            staleKeys.push(storageKey);
        }
    }
    staleKeys.forEach((storageKey) => window.localStorage.removeItem(storageKey));
    entries.forEach((entry) => {
        window.localStorage.setItem(`${ENV_STORAGE_PREFIX}${entry.name}`, entry.value);
    });
}

async function initPlayground() {
    state.initialCode = loadInitialCode();
    bindEvents();

    try {
        await Editor.initEditor("editor-container", state.initialCode);
        await init(buildWasmUrl());
        init_panic_hook();
        state.wasmReady = true;
        loadSettingsFromLocalStorage();
        loadEnvFromLocalStorage();
        elements.wasmStatus.textContent = "WebAssembly ready";
        setStatus("Ready", "success");
        renderEnvironment();
        renderFiles();
        syncExecutionControls();
    } catch (error) {
        console.error("Failed to initialize wasm", error);
        elements.wasmStatus.textContent = "WebAssembly failed to load";
        setStatus("WASM error", "error");
        renderError({ message: `Failed to initialize WASM: ${error.message}` });
    }
}

function buildWasmUrl() {
    const wasmUrl = new URL("../../wasm/vb6interpret_bg.wasm", import.meta.url);
    wasmUrl.searchParams.set("t", String(Date.now()));
    return wasmUrl.href;
}

function bindEvents() {
    document.addEventListener("editorContentChanged", () => {
        resetDebugProgress();
        Editor.clearExecutionHighlight();
        saveToLocalStorage();
    });

    elements.examples.addEventListener("change", (event) => {
        const example = getExample(event.target.value);
        if (!example) {
            return;
        }

        Editor.setEditorContent(example.code);
        saveToLocalStorage();
        event.target.value = "";
    });

    elements.runButton.addEventListener("click", runModule);
    elements.panelRunButton.addEventListener("click", runModule);
    elements.stepButton.addEventListener("click", stepModule);
    elements.resetButton.addEventListener("click", resetExecutionSession);
    elements.shareButton.addEventListener("click", shareCode);
    elements.clearButton.addEventListener("click", clearEditor);
    elements.envAddForm.addEventListener("submit", addEnvironmentVariable);
    elements.settingAddForm.addEventListener("submit", addSetting);
    elements.filesSaveButton.addEventListener("click", persistFilesToLocalStorage);
    elements.filesLoadButton.addEventListener("click", loadFilesFromLocalStorage);
    elements.filesDownloadButton.addEventListener("click", downloadFilesAsZip);
    elements.filesClearButton.addEventListener("click", clearFiles);

    elements.tabButtons.forEach((button) => {
        button.addEventListener("click", () => setActiveTab(button.dataset.tab));
    });

    document.getElementById("theme-toggle")?.addEventListener("click", handleThemeToggle);

    setupResizer();
}

function loadInitialCode() {
    const fromQuery = loadFromQuery();
    if (fromQuery) {
        return fromQuery;
    }

    const fromStorage = window.localStorage.getItem("vb6interpret-playground-code");
    if (fromStorage) {
        return fromStorage;
    }

    return getDefaultExample().code;
}

function saveToLocalStorage() {
    window.localStorage.setItem("vb6interpret-playground-code", Editor.getEditorContent());
}

async function runModule() {
    if (!state.wasmReady) {
        renderError({ message: "WebAssembly is not ready yet." });
        return;
    }

    const code = Editor.getEditorContent().trim();
    if (!code) {
        renderOutput({
            successful: false,
            output_text: "",
            output_lines: [],
            steps: 0,
            terminated: false,
            error: { message: "Enter a VB6 module before running it." },
        });
        return;
    }

    const canResume = state.debugTrace && state.lastDebugSource === Editor.getEditorContent();
    setStatus("Running", "pending");
    state.hasExecutionState = true;

    try {
        const result = canResume
            ? state.debugTrace.snapshots[state.debugTrace.snapshots.length - 1]
            : interpret_vb6_code(Editor.getEditorContent());

        if (canResume) {
            state.debugTraceIndex = state.debugTrace.snapshots.length - 1;
        } else {
            resetDebugProgress();
            persistSettingsToLocalStorage();
        }

        updateSessionCompletion(result);
        renderOutput(result);
    } catch (error) {
        console.error("Interpreter execution failed", error);
        updateSessionCompletion({ paused: false });
        renderOutput({
            successful: false,
            output_text: "",
            output_lines: [],
            steps: 0,
            terminated: false,
            error: { message: error.message ?? "Execution failed." },
        });
    }
}

async function stepModule() {
    if (!state.wasmReady) {
        renderError({ message: "WebAssembly is not ready yet." });
        return;
    }

    const source = Editor.getEditorContent();
    if (!source.trim()) {
        renderOutput({
            successful: false,
            output_text: "",
            output_lines: [],
            steps: 0,
            terminated: false,
            paused: false,
            error: { message: "Enter a VB6 module before stepping it." },
            debug: emptyDebugState(),
        });
        return;
    }

    if (!state.debugTrace || state.lastDebugSource !== source) {
        setStatus("Preparing debug session", "pending");
        state.hasExecutionState = true;
        try {
            state.debugTrace = build_debug_trace(source);
            state.lastDebugSource = source;
            state.debugTraceIndex = -1;
            persistSettingsToLocalStorage();
        } catch (error) {
            console.error("Failed to create debug trace", error);
            renderOutput({
                successful: false,
                output_text: "",
                output_lines: [],
                steps: 0,
                terminated: false,
                paused: false,
                error: { message: error.message ?? "Failed to create debug trace." },
                debug: emptyDebugState(),
            });
            return;
        }
    }

    if (!state.debugTrace.snapshots || state.debugTrace.snapshots.length === 0) {
        renderOutput({
            successful: false,
            output_text: "",
            output_lines: [],
            steps: 0,
            terminated: false,
            paused: false,
            error: state.debugTrace.error ?? { message: "No debug trace available." },
            debug: emptyDebugState(),
        });
        setActiveTab("errors");
        return;
    }

    if (state.debugTraceIndex < state.debugTrace.snapshots.length - 1) {
        state.debugTraceIndex += 1;
    }

    setStatus(`Stepping (${state.debugTraceIndex + 1})`, "pending");

    try {
        const result = state.debugTrace.snapshots[state.debugTraceIndex];
        updateSessionCompletion(result);
        renderOutput(result);
        setActiveTab("debug");
    } catch (error) {
        console.error("Interpreter stepping failed", error);
        updateSessionCompletion({ paused: false });
        renderOutput({
            successful: false,
            output_text: "",
            output_lines: [],
            steps: 0,
            terminated: false,
            paused: false,
            error: { message: error.message ?? "Stepping failed." },
            debug: emptyDebugState(),
        });
    }
}

function renderOutput(result) {
    syncExecutionControls();
    renderEnvironment();
    renderFiles();
    elements.stdout.textContent = result.output_text || "No output.";
    renderDebugState(
        result.debug ?? emptyDebugState(),
        Boolean(result.debug?.current_line) && Boolean(result.paused || result.error || (result.steps ?? 0) > 0),
    );

    if (result.error && !result.error.is_debug_pause) {
        renderError(result.error);
        setStatus("Error", "error");
    } else if (result.paused) {
        elements.errorBox.textContent = "Paused before the next statement.";
        elements.errorBox.className = "error-box empty";
        setStatus("Paused", "pending");
    } else {
        elements.errorBox.textContent = "No runtime or parse errors.";
        elements.errorBox.className = "error-box empty";
        setStatus("Completed", "success");
    }
}

function updateSessionCompletion(result) {
    state.sessionComplete = !result?.paused;
}

function syncExecutionControls() {
    const resetEnabled = state.hasExecutionState;
    const runEnabled = !state.sessionComplete;
    const stepEnabled = !state.sessionComplete;

    elements.resetButton.disabled = !resetEnabled;
    elements.runButton.disabled = !runEnabled;
    elements.panelRunButton.disabled = !runEnabled;
    elements.stepButton.disabled = !stepEnabled;
}

function renderDebugState(debug, highlightLine = false) {
    elements.debugSteps.textContent = String(debug.current_steps ?? 0);
    elements.debugPosition.textContent = `Line ${debug.current_line ?? 1}`;
    elements.debugProcedure.textContent = debug.current_procedure || "Module";
    elements.debugStackDepth.textContent = String(debug.stack_depth ?? 0);
    elements.debugGlobals.textContent = formatScope(debug.globals, "No globals yet.");
    elements.debugLocals.textContent = formatScope(debug.locals, "No active local scope.");

    if (highlightLine) {
        if (debug.cursor) {
            Editor.highlightExecutionRange(debug.cursor);
        } else {
            Editor.highlightExecutionLine(debug.current_line);
        }
    } else {
        Editor.clearExecutionHighlight();
    }
}

function formatScope(variables, emptyLabel) {
    if (!variables || variables.length === 0) {
        return emptyLabel;
    }

    return variables
        .map((variable) => `${variable.name} As ${variable.type_name} = ${variable.value}`)
        .join("\n");
}

function emptyDebugState() {
    return {
        current_steps: 0,
        current_line: 1,
        current_procedure: null,
        stack_depth: 0,
        globals: [],
        locals: [],
        cursor: null,
    };
}

function renderError(error) {
    if (error.pretty_report) {
        elements.errorBox.textContent = error.pretty_report;
        elements.errorBox.className = "error-box error";
        return;
    }

    const parts = [error.message];

    if (error.line) {
        parts.push(`Line: ${error.line}`);
    }

    if (error.procedure) {
        parts.push(`Procedure: ${error.procedure}`);
    }

    if (error.error_number) {
        parts.push(`Err.Number: ${error.error_number}`);
    }

    elements.errorBox.textContent = parts.join("\n");
    elements.errorBox.className = "error-box error";
}

function setStatus(label, kind) {
    elements.runStatus.textContent = label;
    elements.runStatus.className = `status-${kind}`;
}

async function shareCode() {
    const encoded = btoa(encodeURIComponent(Editor.getEditorContent()));
    const url = `${window.location.origin}${window.location.pathname}?code=${encoded}`;

    try {
        if (navigator.clipboard?.writeText) {
            await navigator.clipboard.writeText(url);
            setStatus("Link copied", "success");
        } else {
            window.prompt("Copy this URL", url);
        }
    } catch (error) {
        console.error("Failed to copy share URL", error);
        window.prompt("Copy this URL", url);
    }
}

function clearEditor() {
    Editor.clearEditor();
    resetExecutionSession();
    saveToLocalStorage();
}

function resetDebugProgress() {
    state.debugTrace = null;
    state.debugTraceIndex = -1;
    state.lastDebugSource = Editor.getEditorContent();
}

function resetExecutionSession() {
    resetDebugProgress();
    state.hasExecutionState = false;
    state.sessionComplete = false;
    Editor.clearExecutionHighlight();
    syncExecutionControls();
    // Each run installs a fresh memory file backend, so a session reset
    // should wipe it too and refresh the Files tab to match.
    if (state.wasmReady) {
        clear_files();
        state.activeFilePath = null;
    }
    renderOutput({
        successful: true,
        output_text: "Run a module to see Debug.Print output.",
        output_lines: [],
        steps: 0,
        terminated: false,
        paused: false,
        error: null,
        debug: emptyDebugState(),
    });
    setStatus("Idle", "pending");
}

function renderEnvironment() {
    if (!state.wasmReady) {
        return;
    }
    renderEnvVars();
    renderSettings();
    syncEnvironmentControls();
}

function renderEnvVars() {
    const entries = dump_env();
    const container = elements.envTable;
    container.innerHTML = "";

    if (!entries || entries.length === 0) {
        const empty = document.createElement("div");
        empty.className = "env-empty";
        empty.textContent = "No environment variables set.";
        container.appendChild(empty);
        return;
    }

    entries.forEach((entry) => {
        container.appendChild(
            buildEnvRow(entry.name, entry.value, () => removeEnvironmentVariable(entry.name)),
        );
    });
}

function renderSettings() {
    const settings = dump_settings();
    const container = elements.settingsTable;
    container.innerHTML = "";

    if (!settings || settings.length === 0) {
        const empty = document.createElement("div");
        empty.className = "env-empty";
        empty.textContent = "No settings.";
        container.appendChild(empty);
        return;
    }

    settings.forEach((setting) => {
        const key = `${setting.appname}/${setting.section}/${setting.key}`;
        container.appendChild(buildEnvRow(key, setting.value, () => {
            removeSetting(setting.appname, setting.section, setting.key);
        }));
    });
}

function buildEnvRow(key, value, onRemove) {
    const row = document.createElement("div");
    row.className = "env-row";

    const keyEl = document.createElement("span");
    keyEl.className = "env-key";
    keyEl.textContent = key;
    keyEl.title = key;

    const valueEl = document.createElement("span");
    valueEl.className = "env-value";
    valueEl.textContent = value;
    valueEl.title = value;

    const remove = document.createElement("button");
    remove.type = "button";
    remove.className = "btn btn-secondary btn-sm env-remove";
    remove.textContent = "Remove";
    remove.addEventListener("click", onRemove);

    row.append(keyEl, valueEl, remove);
    return row;
}

/// Encode raw bytes as base64, chunked to avoid blowing the call stack on
/// `String.fromCharCode.apply` for large files.
function bytesToBase64(bytes) {
    let binary = "";
    const chunkSize = 0x8000;
    for (let i = 0; i < bytes.length; i += chunkSize) {
        binary += String.fromCharCode.apply(null, bytes.subarray(i, i + chunkSize));
    }
    return window.btoa(binary);
}

function base64ToBytes(base64) {
    const binary = window.atob(base64);
    const bytes = new Uint8Array(binary.length);
    for (let i = 0; i < binary.length; i += 1) {
        bytes[i] = binary.charCodeAt(i);
    }
    return bytes;
}

/// Get the raw content bytes of a `dump_files` entry, decoding whichever of
/// `content_text`/`content_base64` is populated.
function fileBytes(file) {
    if (file.content_base64 != null) {
        return base64ToBytes(file.content_base64);
    }
    return new TextEncoder().encode(file.content_text ?? "");
}

/// Briefly replace a button's label to confirm an action, then restore it.
function flashButtonLabel(button, message) {
    const original = button.innerHTML;
    button.textContent = message;
    window.setTimeout(() => {
        button.innerHTML = original;
    }, 1200);
}

/// Whether any file was previously saved with [`persistFilesToLocalStorage`].
function hasSavedFiles() {
    for (let index = 0; index < window.localStorage.length; index += 1) {
        if (window.localStorage.key(index)?.startsWith(FILES_STORAGE_PREFIX)) {
            return true;
        }
    }
    return false;
}

/// Save/Download/Clear only make sense with files in the memory backend;
/// Load only makes sense when a saved snapshot exists in `localStorage`.
function updateFileActionButtons(fileCount) {
    const noFiles = fileCount === 0;
    elements.filesSaveButton.disabled = noFiles;
    elements.filesDownloadButton.disabled = noFiles;
    elements.filesClearButton.disabled = noFiles;
    elements.filesLoadButton.disabled = !hasSavedFiles();
}

/// Save every file currently in the memory backend to `localStorage`
/// (base64-encoded), replacing whatever was previously saved there.
function persistFilesToLocalStorage() {
    if (!state.wasmReady) {
        return;
    }
    const staleKeys = [];
    for (let index = 0; index < window.localStorage.length; index += 1) {
        const storageKey = window.localStorage.key(index);
        if (storageKey && storageKey.startsWith(FILES_STORAGE_PREFIX)) {
            staleKeys.push(storageKey);
        }
    }
    staleKeys.forEach((storageKey) => window.localStorage.removeItem(storageKey));

    const files = dump_files().files || [];
    files.forEach((file) => {
        window.localStorage.setItem(
            `${FILES_STORAGE_PREFIX}${file.path}`,
            bytesToBase64(fileBytes(file)),
        );
    });
    flashButtonLabel(elements.filesSaveButton, `Saved ${files.length}`);
    updateFileActionButtons(files.length);
}

/// Restore every file previously saved with [`persistFilesToLocalStorage`]
/// into the memory backend, then refresh the Files tab.
function loadFilesFromLocalStorage() {
    if (!state.wasmReady) {
        return;
    }
    let loaded = 0;
    for (let index = 0; index < window.localStorage.length; index += 1) {
        const storageKey = window.localStorage.key(index);
        if (!storageKey || !storageKey.startsWith(FILES_STORAGE_PREFIX)) {
            continue;
        }
        const path = storageKey.slice(FILES_STORAGE_PREFIX.length);
        if (!path) {
            continue;
        }
        try {
            install_file(path, base64ToBytes(window.localStorage.getItem(storageKey)));
            loaded += 1;
        } catch (error) {
            console.error(`Failed to restore file ${path}`, error);
        }
    }
    flashButtonLabel(elements.filesLoadButton, `Loaded ${loaded}`);
    renderFiles();
}

/// Close all open files and wipe the in-memory file backend back to empty.
function clearFiles() {
    if (!state.wasmReady) {
        return;
    }
    clear_files();
    state.activeFilePath = null;
    flashButtonLabel(elements.filesClearButton, "Cleared");
    renderFiles();
}

/// Bundle every file currently in the memory backend into a ZIP archive and
/// trigger a browser download.
function downloadFilesAsZip() {
    if (!state.wasmReady) {
        return;
    }
    const files = dump_files().files || [];
    if (files.length === 0) {
        flashButtonLabel(elements.filesDownloadButton, "No files");
        return;
    }

    const entries = files.map((file) => ({
        name: file.path.replace(/^\/+/, ""),
        data: fileBytes(file),
    }));
    const blob = createZip(entries);
    const url = URL.createObjectURL(blob);
    const link = document.createElement("a");
    link.href = url;
    link.download = "vb6interpret-files.zip";
    document.body.appendChild(link);
    link.click();
    link.remove();
    URL.revokeObjectURL(url);
}

/// Refresh the Files tab from the memory file backend: the current
/// directory/drive, open file numbers, and every known file's attributes and
/// content (text or base64-encoded binary).
function renderFiles() {
    if (!state.wasmReady) {
        return;
    }

    const snapshot = dump_files();
    elements.filesCurrentDir.textContent = snapshot.current_dir || "/";
    elements.filesCurrentDrive.textContent = snapshot.current_drive || "C";
    elements.filesOpenNumbers.textContent = snapshot.open_file_numbers?.length
        ? snapshot.open_file_numbers.join(", ")
        : "None";

    const files = snapshot.files || [];
    if (files.length === 0) {
        state.activeFilePath = null;
    } else if (!files.some((file) => file.path === state.activeFilePath)) {
        state.activeFilePath = files[0].path;
    }

    renderFileListNav(files);
    renderFileDetail(files);
    updateFileActionButtons(files.length);
}

/// Drop the leading '/' for root-level files (e.g. "/foo.txt" -> "foo.txt");
/// files in a subdirectory keep their full path (e.g. "/sub/foo.txt").
function formatFilePath(path) {
    if (path.startsWith("/") && !path.slice(1).includes("/")) {
        return path.slice(1);
    }
    return path;
}

function renderFileListNav(files) {
    const container = elements.filesListNav;
    container.innerHTML = "";

    if (files.length === 0) {
        const empty = document.createElement("div");
        empty.className = "env-empty";
        empty.textContent = "No files in the memory backend.";
        container.appendChild(empty);
        return;
    }

    files.forEach((file) => {
        const button = document.createElement("button");
        button.type = "button";
        button.className = "file-tab-btn";
        button.classList.toggle("active", file.path === state.activeFilePath);
        const displayPath = formatFilePath(file.path);
        button.textContent = file.number > 0 ? `#${file.number} ${displayPath}` : displayPath;
        button.addEventListener("click", () => {
            state.activeFilePath = file.path;
            renderFileListNav(files);
            renderFileDetail(files);
        });
        container.appendChild(button);
    });
}

function renderFileDetail(files) {
    const container = elements.filesDetail;
    container.innerHTML = "";

    const file = files.find((candidate) => candidate.path === state.activeFilePath);
    if (!file) {
        const empty = document.createElement("div");
        empty.className = "env-empty";
        empty.textContent = "Select a file to inspect its contents.";
        container.appendChild(empty);
        return;
    }

    const isBinary = file.content_base64 != null;
    const attrs = document.createElement("div");
    attrs.className = "stats-grid";
    attrs.append(
        buildStatItem("File Name", formatFilePath(file.path)),
        buildStatItem("File Number", file.number > 0 ? String(file.number) : "Closed"),
        buildStatItem("Mode", file.mode),
        buildStatItem("Content Type", isBinary ? "Binary" : "Text"),
    );
    container.appendChild(attrs);

    const contentBox = document.createElement("div");
    contentBox.className = "debug-scope-box file-content-box";
    if (isBinary) {
        contentBox.textContent = file.content_base64;
    } else if (file.content_text != null) {
        contentBox.textContent = file.content_text;
    } else {
        contentBox.textContent = "(empty)";
    }
    container.appendChild(contentBox);
}

function buildStatItem(label, value) {
    const item = document.createElement("div");
    item.className = "stat-item";

    const labelEl = document.createElement("span");
    labelEl.className = "stat-label";
    labelEl.textContent = label;

    const valueEl = document.createElement("span");
    valueEl.className = "stat-value";
    valueEl.textContent = value;

    item.append(labelEl, valueEl);
    return item;
}

function addEnvironmentVariable(event) {
    event.preventDefault();
    if (state.hasExecutionState) {
        return;
    }
    const name = elements.envName.value.trim();
    if (!name) {
        return;
    }
    set_env(name, elements.envValue.value);
    elements.envName.value = "";
    elements.envValue.value = "";
    persistEnvToLocalStorage();
    renderEnvironment();
}

function removeEnvironmentVariable(name) {
    if (state.hasExecutionState) {
        return;
    }
    remove_env(name);
    persistEnvToLocalStorage();
    renderEnvironment();
}

function addSetting(event) {
    event.preventDefault();
    if (state.hasExecutionState) {
        return;
    }
    const appname = elements.settingAppname.value.trim();
    const section = elements.settingSection.value.trim();
    const key = elements.settingKey.value.trim();
    if (!appname || !section || !key) {
        return;
    }
    install_setting(appname, section, key, elements.settingValue.value);
    elements.settingAppname.value = "";
    elements.settingSection.value = "";
    elements.settingKey.value = "";
    elements.settingValue.value = "";
    persistSettingsToLocalStorage();
    renderEnvironment();
}

function removeSetting(appname, section, key) {
    if (state.hasExecutionState) {
        return;
    }
    remove_setting(appname, section, key);
    persistSettingsToLocalStorage();
    renderEnvironment();
}

/// Lock the Environment tab while the interpreter has run or is stepping so
/// its contents mirror the state the program observes; edits are allowed only
/// after a Reset or before the first run.
function syncEnvironmentControls() {
    const editable = !state.hasExecutionState;
    elements.environmentTab
        ?.querySelectorAll("input, button, select")
        .forEach((element) => {
            element.disabled = !editable;
        });
    if (elements.environmentStatus) {
        elements.environmentStatus.textContent = editable
            ? "Editable while the interpreter is idle. Environment variables and `GetSetting`/`SaveSetting` registry entries."
            : "Read-only while running or after a run. Press Reset to edit.";
        elements.environmentStatus.classList.toggle("status-edit-locked", !editable);
    }
}

function handleThemeToggle() {
    Editor.updateEditorTheme();
}

function loadFromQuery() {
    const params = new URLSearchParams(window.location.search);
    const encoded = params.get("code");
    if (!encoded) {
        return null;
    }

    try {
        return decodeURIComponent(atob(encoded));
    } catch (error) {
        console.error("Failed to decode shared code", error);
        return null;
    }
}

function setActiveTab(tabName) {
    state.activeTab = tabName;

    elements.tabButtons.forEach((button) => {
        button.classList.toggle("active", button.dataset.tab === tabName);
    });

    elements.tabPanes.forEach((pane) => {
        pane.classList.toggle("active", pane.id === `${tabName}-tab`);
    });
}

function setupResizer() {
    if (!elements.resizer || !elements.editorPanel || !elements.outputPanel) {
        return;
    }

    const stopResize = () => {
        state.isResizing = false;
        document.body.style.cursor = "";
        document.body.style.userSelect = "";
    };

    const resizePanels = (event) => {
        if (!state.isResizing || window.innerWidth <= 900) {
            return;
        }

        const container = document.querySelector(".playground-container");
        const rect = container.getBoundingClientRect();
        const nextWidth = ((event.clientX - rect.left) / rect.width) * 100;
        const clamped = Math.min(75, Math.max(25, nextWidth));
        elements.editorPanel.style.width = `${clamped}%`;
    };

    elements.resizer.addEventListener("mousedown", () => {
        state.isResizing = true;
        document.body.style.cursor = "col-resize";
        document.body.style.userSelect = "none";
    });

    window.addEventListener("mousemove", resizePanels);
    window.addEventListener("mouseup", stopResize);
}

initPlayground();