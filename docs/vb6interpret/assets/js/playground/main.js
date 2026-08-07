import init, { interpret_vb6_code, init_panic_hook } from "../../wasm/vb6interpret.js";
import { getDefaultExample, getExample } from "./examples.js";
import * as Editor from "./editor.js";

const state = {
    wasmReady: false,
    activeTab: "output",
    isResizing: false,
    initialCode: "",
};

const elements = {
    fileType: document.getElementById("file-type"),
    examples: document.getElementById("examples"),
    runButton: document.getElementById("run-btn"),
    shareButton: document.getElementById("share-btn"),
    clearButton: document.getElementById("clear-btn"),
    stdout: document.getElementById("stdout"),
    errorBox: document.getElementById("error-box"),
    wasmStatus: document.getElementById("wasm-status"),
    runStatus: document.getElementById("run-status"),
    steps: document.getElementById("stat-steps"),
    terminated: document.getElementById("stat-terminated"),
    lines: document.getElementById("stat-lines"),
    resizer: document.getElementById("resizer"),
    editorPanel: document.querySelector(".editor-panel"),
    outputPanel: document.querySelector(".output-panel"),
    tabButtons: Array.from(document.querySelectorAll(".tab-btn")),
    tabPanes: Array.from(document.querySelectorAll(".tab-pane")),
};

async function initPlayground() {
    state.initialCode = loadInitialCode();
    bindEvents();

    try {
        await Editor.initEditor("editor-container", state.initialCode);
        await init();
        init_panic_hook();
        state.wasmReady = true;
        elements.wasmStatus.textContent = "WebAssembly ready";
        setStatus("Ready", "success");
    } catch (error) {
        console.error("Failed to initialize wasm", error);
        elements.wasmStatus.textContent = "WebAssembly failed to load";
        setStatus("WASM error", "error");
        renderError({ message: `Failed to initialize WASM: ${error.message}` });
    }
}

function bindEvents() {
    document.addEventListener("editorContentChanged", () => {
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
    elements.shareButton.addEventListener("click", shareCode);
    elements.clearButton.addEventListener("click", clearEditor);

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

    setStatus("Running", "pending");

    try {
        const result = interpret_vb6_code(Editor.getEditorContent());
        renderOutput(result);
    } catch (error) {
        console.error("Interpreter execution failed", error);
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

function renderOutput(result) {
    elements.stdout.textContent = result.output_text || "No output.";
    elements.steps.textContent = String(result.steps ?? 0);
    elements.terminated.textContent = result.terminated ? "Yes" : "No";
    elements.lines.textContent = String(result.output_lines?.length ?? 0);

    if (result.error) {
        renderError(result.error);
        setStatus("Error", "error");
    } else {
        elements.errorBox.textContent = "No runtime or parse errors.";
        elements.errorBox.className = "error-box empty";
        setStatus("Completed", "success");
    }
}

function renderError(error) {
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
    saveToLocalStorage();
    renderOutput({
        successful: true,
        output_text: "Run a module to see Debug.Print output.",
        output_lines: [],
        steps: 0,
        terminated: false,
        error: null,
    });
    setStatus("Idle", "pending");
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