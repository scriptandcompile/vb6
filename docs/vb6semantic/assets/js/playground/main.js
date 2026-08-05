/**
 * VB6Semantic Playground - Main Application Module
 *
 * Entry point that coordinates all other modules:
 * - Initializes WASM module
 * - Sets up editor
 * - Handles UI events
 * - Coordinates analysis and rendering
 */

import { getExample } from './examples.js';
import * as Analyzer from './analyzer.js';
import * as Editor from './editor.js';
import * as Renderer from './renderer.js';

// Application state
const state = {
    currentFileType: 'module',
    autoParse: true,
    parseTimeout: null,
    lastResult: null,
    isInitialized: false,
    activeTab: 'scopes'
};

/**
 * Main initialization function
 * Called when DOM is ready
 */
async function init() {
    console.log('🚀 Initializing VB6Semantic Playground...');

    try {
        showLoading('Initializing WASM module...');

        const wasmOk = await Analyzer.initWasm();
        if (!wasmOk) {
            throw new Error('Failed to initialize WASM module');
        }

        await Editor.initEditor('editor-container');

        // Expose the editor model so the renderer can read line/char stats
        const editorInstance = Editor.getEditor();
        if (editorInstance) {
            window.__vb6semanticEditorModel = editorInstance.getModel();
        }

        setupEventListeners();
        loadFromLocalStorage();

        hideLoading();
        state.isInitialized = true;
        console.log('✅ Playground initialized successfully');

    } catch (error) {
        console.error('❌ Initialization failed:', error);
        showError(`Failed to initialize playground: ${error.message}`);
        hideLoading();
    }
}

/**
 * Set up all event listeners
 */
function setupEventListeners() {
    document.getElementById('file-type')?.addEventListener('change', handleFileTypeChange);
    document.getElementById('examples')?.addEventListener('change', handleExampleChange);
    document.getElementById('analyze-btn')?.addEventListener('click', handleAnalyze);
    document.getElementById('share-btn')?.addEventListener('click', handleShare);
    document.getElementById('clear-btn')?.addEventListener('click', handleClear);
    document.getElementById('auto-parse')?.addEventListener('change', (e) => {
        state.autoParse = e.target.checked;
    });

    document.querySelectorAll('.tab-btn').forEach(btn => {
        btn.addEventListener('click', () => handleTabChange(btn.dataset.tab));
    });

    document.getElementById('scopes-expand')?.addEventListener('click', handleScopesExpandAll);
    document.getElementById('scopes-collapse')?.addEventListener('click', handleScopesCollapseAll);

    // Navigate from scope/symbol/error rows -> editor
    document.addEventListener('semanticNavigate', handleSemanticNavigate);

    // Editor content change (for auto-analyze)
    document.addEventListener('editorContentChanged', handleEditorChange);

    // Editor cursor change -> highlight symbol in scopes/symbols
    document.addEventListener('editorCursorPositionChange', handleEditorCursorChange);

    // Theme toggle (inherited from main site)
    document.getElementById('theme-toggle')?.addEventListener('click', handleThemeToggle);

    setupResizer();
    window.addEventListener('resize', handleWindowResize);

    console.log('✅ Event listeners set up');
}

/**
 * Handle file type change
 */
function handleFileTypeChange(e) {
    state.currentFileType = e.target.value;
    Editor.setFileType(state.currentFileType);

    if (state.autoParse) {
        debouncedAnalyze();
    }
}

/**
 * Handle example selection
 */
function handleExampleChange(e) {
    const exampleId = e.target.value;
    if (!exampleId) return;

    const example = getExample(exampleId);
    if (!example) {
        console.error(`Example ${exampleId} not found`);
        return;
    }

    document.getElementById('file-type').value = example.fileType;
    state.currentFileType = example.fileType;

    Editor.setEditorContent(example.code);

    if (state.autoParse) {
        handleAnalyze();
    }

    e.target.value = '';
    console.log(`📝 Loaded example: ${example.name}`);
}

/**
 * Handle analyze button click
 */
async function handleAnalyze() {
    if (!state.isInitialized) {
        showError('Playground not initialized yet');
        return;
    }

    const code = Editor.getEditorContent();
    if (!code || code.trim().length === 0) {
        return;
    }

    try {
        console.log(`🔍 Analyzing ${state.currentFileType}...`);

        const result = Analyzer.analyzeCode(code, state.currentFileType);
        state.lastResult = result;

        Renderer.renderOutput(result);

        console.log(`✅ Analysis complete: ${result.scope_count ?? 0} scopes, ${result.symbol_count ?? 0} symbols, ${result.error_count ?? 0} errors`);
    } catch (error) {
        console.error('❌ Analysis failed:', error);
        showError(`Analysis failed: ${error.message}`);
    }
}

/**
 * Handle editor content change (for auto-analyze)
 */
function handleEditorChange() {
    if (state.autoParse) {
        debouncedAnalyze();
    }
    saveToLocalStorage();
}

/**
 * Debounced analyze (500ms delay)
 */
function debouncedAnalyze() {
    if (state.parseTimeout) {
        clearTimeout(state.parseTimeout);
    }
    state.parseTimeout = setTimeout(() => {
        handleAnalyze();
    }, 500);
}

/**
 * Handle navigation event from a scope/symbol/error row
 */
function handleSemanticNavigate(e) {
    const { line, column } = e.detail;
    Editor.highlightLine(line);
    Editor.setCursorToPosition(line, column);
}

/**
 * Handle editor cursor change (for symbol highlighting)
 */
function handleEditorCursorChange(e) {
    if (!state.lastResult) return;
    if (state.activeTab === 'scopes' || state.activeTab === 'symbols') {
        Renderer.highlightSymbolAtLine(e.detail.lineNumber);
    }
}

/**
 * Handle share button click
 */
function handleShare() {
    const code = Editor.getEditorContent();
    if (!code) return;

    try {
        const encoded = btoa(encodeURIComponent(code));
        const url = `${window.location.origin}${window.location.pathname}?code=${encoded}&type=${state.currentFileType}`;

        if (navigator.clipboard && navigator.clipboard.writeText) {
            navigator.clipboard.writeText(url).then(() => {
                showToast('Share link copied to clipboard');
            });
        } else {
            prompt('Copy this share link:', url);
        }
    } catch (error) {
        console.error('Failed to generate share link:', error);
        showError('Share link generation failed');
    }
}

/**
 * Handle clear button click
 */
function handleClear() {
    if (confirm('Clear editor and output?')) {
        Editor.clearEditor();
        Renderer.clearOutput();
        state.lastResult = null;
        console.log('🗑️ Cleared editor and output');
    }
}

/**
 * Handle tab change
 */
function handleTabChange(tabId) {
    state.activeTab = tabId;

    document.querySelectorAll('.tab-btn').forEach(btn => {
        btn.classList.toggle('active', btn.dataset.tab === tabId);
    });

    document.querySelectorAll('.tab-pane').forEach(pane => {
        pane.classList.toggle('active', pane.id === `${tabId}-tab`);
    });

    if (state.lastResult) {
        Renderer.renderScopesTab(state.lastResult.scopes || []);
        Renderer.renderSymbolsTab(state.lastResult.scopes || []);
    }

    console.log(`📑 Switched to ${tabId} tab`);
}

/**
 * Handle scope tree expand all
 */
function handleScopesExpandAll() {
    document.querySelectorAll('.scope-node.collapsed').forEach(node => {
        node.classList.remove('collapsed');
    });
}

/**
 * Handle scope tree collapse all
 */
function handleScopesCollapseAll() {
    document.querySelectorAll('.scope-node').forEach(node => {
        if (node.querySelector('.scope-children')) {
            node.classList.add('collapsed');
        }
    });
}

/**
 * Handle theme toggle
 */
function handleThemeToggle() {
    Editor.updateEditorTheme();
}

/**
 * Set up split panel resizer
 */
function setupResizer() {
    const resizer = document.getElementById('resizer');
    const leftPanel = document.querySelector('.editor-panel');
    const rightPanel = document.querySelector('.output-panel');

    if (!resizer || !leftPanel || !rightPanel) return;

    let isResizing = false;
    let startX = 0;
    let startLeftWidth = 0;

    resizer.addEventListener('mousedown', (e) => {
        isResizing = true;
        startX = e.clientX;
        startLeftWidth = leftPanel.offsetWidth;
        document.body.style.cursor = 'col-resize';
        e.preventDefault();
    });

    document.addEventListener('mousemove', (e) => {
        if (!isResizing) return;

        const deltaX = e.clientX - startX;
        const newLeftWidth = startLeftWidth + deltaX;
        const minWidth = 300;
        const maxWidth = window.innerWidth - 300 - 8;

        if (newLeftWidth >= minWidth && newLeftWidth <= maxWidth) {
            leftPanel.style.width = `${newLeftWidth}px`;
            leftPanel.style.flex = 'none';
        }
    });

    document.addEventListener('mouseup', () => {
        if (isResizing) {
            isResizing = false;
            document.body.style.cursor = '';
        }
    });
}

/**
 * Handle window resize
 */
function handleWindowResize() {
    // Monaco uses automaticLayout, nothing else to do here.
}

/**
 * Show loading overlay
 */
function showLoading(message = 'Loading...') {
    const overlay = document.getElementById('loading-overlay');
    if (overlay) {
        const text = overlay.querySelector('p');
        if (text) text.textContent = message;
        overlay.classList.remove('hidden');
    }
}

/**
 * Hide loading overlay
 */
function hideLoading() {
    const overlay = document.getElementById('loading-overlay');
    if (overlay) {
        overlay.classList.add('hidden');
    }
}

/**
 * Show error modal
 */
function showError(message) {
    const modal = document.getElementById('error-modal');
    const messageEl = document.getElementById('error-message');

    if (modal && messageEl) {
        messageEl.textContent = message;
        modal.classList.remove('hidden');
    }

    console.error('❌ Error:', message);
}

/**
 * Hide error modal
 */
function hideError() {
    const modal = document.getElementById('error-modal');
    if (modal) {
        modal.classList.add('hidden');
    }
}

/**
 * Show a transient toast message
 */
function showToast(message) {
    let toast = document.getElementById('toast');
    if (!toast) {
        toast = document.createElement('div');
        toast.id = 'toast';
        toast.style.cssText = `
            position: fixed;
            bottom: 20px;
            left: 50%;
            transform: translateX(-50%);
            background: var(--success-color, #28a745);
            color: white;
            padding: 10px 20px;
            border-radius: 6px;
            z-index: 3000;
            box-shadow: 0 4px 6px rgba(0, 0, 0, 0.3);
            font-size: 0.9rem;
        `;
        document.body.appendChild(toast);
    }
    toast.textContent = message;
    toast.style.display = 'block';

    setTimeout(() => {
        toast.style.display = 'none';
    }, 2000);
}

// Error modal close button
document.querySelector('.modal-close')?.addEventListener('click', hideError);
document.getElementById('error-modal')?.addEventListener('click', (e) => {
    if (e.target.id === 'error-modal') {
        hideError();
    }
});

/**
 * Save state to localStorage
 */
function saveToLocalStorage() {
    try {
        localStorage.setItem('vb6semantic-playground-code', Editor.getEditorContent());
        localStorage.setItem('vb6semantic-playground-filetype', state.currentFileType);
        localStorage.setItem('vb6semantic-playground-autoparse', String(state.autoParse));
    } catch (error) {
        console.warn('Failed to save to localStorage:', error);
    }
}

/**
 * Load state from localStorage
 */
function loadFromLocalStorage() {
    try {
        const code = localStorage.getItem('vb6semantic-playground-code');
        const fileType = localStorage.getItem('vb6semantic-playground-filetype');
        const autoParse = localStorage.getItem('vb6semantic-playground-autoparse');

        if (code) {
            Editor.setEditorContent(code);
        }

        if (fileType) {
            state.currentFileType = fileType;
            document.getElementById('file-type').value = fileType;
        }

        if (autoParse !== null) {
            state.autoParse = autoParse === 'true';
            const toggle = document.getElementById('auto-parse');
            if (toggle) toggle.checked = state.autoParse;
        }

        // Load code from URL parameter if present (overrides localStorage)
        loadFromUrl();

        console.log('📂 Loaded state from localStorage');
    } catch (error) {
        console.warn('Failed to load from localStorage:', error);
    }
}

/**
 * Load code from URL parameter
 */
function loadFromUrl() {
    const params = new URLSearchParams(window.location.search);
    const encodedCode = params.get('code');
    const fileType = params.get('type');

    if (encodedCode) {
        try {
            const code = decodeURIComponent(atob(encodedCode));
            Editor.setEditorContent(code);
            console.log('🔗 Loaded code from URL');
        } catch (error) {
            console.error('Failed to decode URL code:', error);
        }
    }

    if (fileType && ['module', 'class', 'form'].includes(fileType)) {
        state.currentFileType = fileType;
        document.getElementById('file-type').value = fileType;
    }
}

// Initialize when DOM is ready
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', init);
} else {
    init();
}

export default {
    init,
    state
};
