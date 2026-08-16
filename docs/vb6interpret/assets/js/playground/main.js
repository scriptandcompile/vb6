import init, { build_debug_trace, dump_env, dump_settings, install_setting, interpret_vb6_code, init_panic_hook, remove_env, remove_setting, set_env } from "../../wasm/vb6interpret.js";
import { getDefaultExample, getExample } from "./examples.js";
import * as Editor from "./editor.js";

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