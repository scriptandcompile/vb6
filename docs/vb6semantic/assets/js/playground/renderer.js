/**
 * VB6Semantic Playground - Renderer Module
 *
 * Renders semantic analysis output into the output tabs:
 * - Scopes: collapsible scope tree with symbols and their types
 * - Symbols: flat, searchable table of every symbol
 * - Errors: semantic errors with locations
 * - Warnings: analyzer warnings
 * - Info: analysis statistics
 */

let nodeIdCounter = 0;
let symbolRowByKey = new Map();
let symbolTableRows = new Map();

/**
 * Render all output tabs from an analysis result
 * @param {object} result - Analysis output from analyzer.js
 */
export function renderOutput(result) {
    renderScopesTab(result.scopes || []);
    renderSymbolsTab(result.scopes || []);
    renderErrorsTab(result.errors || []);
    renderWarningsTab(result.warnings || []);
    renderInfoTab(result);
}

/**
 * Normalize a location (handle missing/partial locations)
 * @param {object} location - Location object
 * @returns {{line:number,column:number}|null} 1-based line/column or null
 */
function normalizeLocation(location) {
    if (!location) return null;
    const line = Number(location.line);
    const column = Number(location.column);
    if (!Number.isFinite(line) || line < 1) return null;
    return { line, column: Number.isFinite(column) && column > 0 ? column : 1 };
}

/**
 * Emit a navigate event so main.js can move the editor cursor.
 * @param {object} location - Location object
 */
function navigateTo(location) {
    const normalized = normalizeLocation(location);
    if (!normalized) return;
    document.dispatchEvent(new CustomEvent('semanticNavigate', {
        detail: normalized
    }));
}

/**
 * Map a symbol kind (Debug string) to a CSS badge class
 * @param {string} kind - Symbol kind string
 * @returns {string} CSS class suffix
 */
function symbolKindClass(kind) {
    if (!kind) return '';
    const k = kind.toLowerCase();
    if (k.startsWith('property')) return 'prop';
    if (k === 'variable') return 'var';
    if (k === 'constant') return 'const';
    if (k === 'subprocedure' || k === 'sub') return 'sub';
    if (k === 'function') return 'func';
    if (k === 'class') return 'class';
    if (k === 'module') return 'module';
    if (k === 'form') return 'form';
    if (k === 'control') return 'control';
    if (k.startsWith('enum')) return 'enum';
    if (k === 'usertype' || k === 'typemember') return 'type';
    if (k === 'parameter' || k === 'param') return 'param';
    if (k === 'label') return 'label';
    return '';
}

/**
 * Render a compact display for a symbol kind
 * @param {string} kind - Symbol kind string
 * @returns {string} Short label
 */
function shortKind(kind) {
    if (!kind) return kind || '';
    const k = kind.toLowerCase();
    if (k === 'subprocedure') return 'Sub';
    if (k === 'propertyget') return 'Prop Get';
    if (k === 'propertylet') return 'Prop Let';
    if (k === 'propertyset') return 'Prop Set';
    if (k === 'usertype') return 'Type';
    if (k === 'typemember') return 'Member';
    if (k === 'enummember') return 'EnumVal';
    return kind.replace(/([a-z])([A-Z])/g, '$1 $2');
}

/**
 * Render the Scopes tab
 * @param {ScopeInfo[]} scopes - All scopes from the analysis output
 */
export function renderScopesTab(scopes) {
    const container = document.getElementById('scopes-content');
    if (!container) return;

    container.innerHTML = '';
    symbolRowByKey = new Map();
    symbolRowByLine = new Map();
    nodeIdCounter = 0;

    if (!scopes.length) {
        container.innerHTML = '<div class="placeholder"><p>No scopes were produced. Analyze code to build the scope tree.</p></div>';
        return;
    }

    const scopeById = new Map();
    scopes.forEach(scope => scopeById.set(scope.id, scope));

    // Render the root scopes (those without a parent)
    const roots = scopes.filter(scope => scope.parent === null || scope.parent === undefined);
    const tree = document.createElement('div');
    if (roots.length === 0 && scopes.length > 0) {
        // No explicit root: treat all scopes as roots
        scopes.forEach(scope => tree.appendChild(renderScopeNode(scope, scopeById)));
    } else {
        roots.forEach(scope => tree.appendChild(renderScopeNode(scope, scopeById)));
    }
    container.appendChild(tree);
}

/**
 * Render a single scope node recursively
 * @param {ScopeInfo} scope - Scope to render
 * @param {Map<number,ScopeInfo>} scopeById - Scope lookup
 * @returns {HTMLElement} Rendered node
 */
function renderScopeNode(scope, scopeById) {
    const node = document.createElement('div');
    node.className = 'scope-node';
    node.dataset.scopeId = scope.id;

    const header = document.createElement('div');
    header.className = 'scope-header';

    const hasChildren = scope.children && scope.children.length > 0;

    const toggle = document.createElement('span');
    toggle.className = 'scope-toggle';
    if (hasChildren) {
        toggle.addEventListener('click', (e) => {
            e.stopPropagation();
            node.classList.toggle('collapsed');
        });
    } else {
        toggle.style.visibility = 'hidden';
    }
    header.appendChild(toggle);

    const name = document.createElement('span');
    name.className = 'scope-name';
    name.textContent = scope.name || `Scope ${scope.id}`;
    header.appendChild(name);

    const kind = document.createElement('span');
    kind.className = 'sem-badge scope-kind';
    kind.textContent = shortKind(scope.kind) || 'Scope';
    header.appendChild(kind);

    const count = document.createElement('span');
    count.className = 'symbol-location';
    count.textContent = `${(scope.symbols || []).length} symbol(s)`;
    header.appendChild(count);

    header.addEventListener('click', () => {
        node.classList.toggle('collapsed');
    });

    node.appendChild(header);

    // Symbols in this scope
    const symbolsDiv = document.createElement('div');
    symbolsDiv.className = 'scope-symbols';
    (scope.symbols || []).forEach(symbol => {
        const row = renderSymbolRow(symbol, scope.id);
        symbolsDiv.appendChild(row);
        symbolRowByKey.set(`${scope.id}:${symbol.name}`, row);
        const loc = normalizeLocation(symbol.location);
        if (loc) {
            if (!symbolRowByLine.has(loc.line)) symbolRowByLine.set(loc.line, []);
            symbolRowByLine.get(loc.line).push(row);
        }
    });
    node.appendChild(symbolsDiv);

    // Child scopes
    if (hasChildren) {
        const childrenDiv = document.createElement('div');
        childrenDiv.className = 'scope-children';
        scope.children.forEach(childId => {
            const child = scopeById.get(childId);
            if (child) {
                childrenDiv.appendChild(renderScopeNode(child, scopeById));
            }
        });
        node.appendChild(childrenDiv);
    }

    return node;
}

/**
 * Render a single symbol row (used in the scope tree)
 * @param {SymbolInfo} symbol - Symbol to render
 * @param {number} scopeId - Containing scope ID
 * @returns {HTMLElement} Rendered row
 */
function renderSymbolRow(symbol, scopeId) {
    const row = document.createElement('div');
    row.className = 'symbol-row';
    row.dataset.symbolKey = `${scopeId}:${symbol.name}`;

    const kind = document.createElement('span');
    kind.className = `sem-badge symbol-kind ${symbolKindClass(symbol.kind)}`;
    kind.textContent = shortKind(symbol.kind);
    row.appendChild(kind);

    const name = document.createElement('span');
    name.className = 'symbol-name';
    name.textContent = symbol.name;
    row.appendChild(name);

    const type = document.createElement('span');
    type.className = 'symbol-type';
    type.textContent = symbol.type_display ? `As ${symbol.type_display}` : '';
    row.appendChild(type);

    const visibility = document.createElement('span');
    visibility.className = `sem-badge visibility ${String(symbol.visibility || '').toLowerCase()}`;
    visibility.textContent = symbol.visibility || '';
    row.appendChild(visibility);

    const location = document.createElement('span');
    location.className = 'symbol-location';
    location.textContent = formatLocation(symbol.location);
    row.appendChild(location);

    row.addEventListener('click', () => navigateTo(symbol.location));

    return row;
}

/**
 * Render the Symbols tab (flat table)
 * @param {ScopeInfo[]} scopes - All scopes
 */
export function renderSymbolsTab(scopes) {
    const container = document.getElementById('symbols-content');
    if (!container) return;

    container.innerHTML = '';
    symbolTableRows = new Map();
    symbolTableRowsByLine = new Map();

    // Flatten all symbols with scope context
    const allSymbols = [];
    scopes.forEach(scope => {
        (scope.symbols || []).forEach(symbol => {
            allSymbols.push({ symbol, scope });
        });
    });

    if (!allSymbols.length) {
        container.innerHTML = '<div class="placeholder"><p>No symbols were produced. Analyze code to build the symbol table.</p></div>';
        return;
    }

    const table = document.createElement('table');
    table.className = 'symbols-table';
    table.id = 'symbols-table';

    const thead = document.createElement('thead');
    thead.innerHTML = `
        <tr>
            <th>Name</th>
            <th>Kind</th>
            <th>Type</th>
            <th>Visibility</th>
            <th>Scope</th>
            <th>Location</th>
        </tr>
    `;
    table.appendChild(thead);

    const tbody = document.createElement('tbody');
    allSymbols.forEach(({ symbol, scope }) => {
        const row = document.createElement('tr');
        row.dataset.symbolKey = `${scope.id}:${symbol.name}`;

        const nameCell = document.createElement('td');
        nameCell.className = 'symbol-name-cell';
        nameCell.textContent = symbol.name;
        row.appendChild(nameCell);

        const kindCell = document.createElement('td');
        const kindBadge = document.createElement('span');
        kindBadge.className = `sem-badge symbol-kind ${symbolKindClass(symbol.kind)}`;
        kindBadge.textContent = shortKind(symbol.kind);
        kindCell.appendChild(kindBadge);
        row.appendChild(kindCell);

        const typeCell = document.createElement('td');
        typeCell.className = 'symbol-type-cell';
        typeCell.textContent = symbol.type_display || '';
        row.appendChild(typeCell);

        const visibilityCell = document.createElement('td');
        const visibilityBadge = document.createElement('span');
        visibilityBadge.className = `sem-badge visibility ${String(symbol.visibility || '').toLowerCase()}`;
        visibilityBadge.textContent = symbol.visibility || '';
        visibilityCell.appendChild(visibilityBadge);
        row.appendChild(visibilityCell);

        const scopeCell = document.createElement('td');
        scopeCell.textContent = scope.name || `Scope ${scope.id}`;
        scopeCell.style.color = 'var(--placeholder-color)';
        row.appendChild(scopeCell);

        const locationCell = document.createElement('td');
        locationCell.className = 'symbol-loc-cell';
        locationCell.textContent = formatLocation(symbol.location);
        row.appendChild(locationCell);

        row.addEventListener('click', () => navigateTo(symbol.location));

        tbody.appendChild(row);
        symbolTableRows.set(`${scope.id}:${symbol.name}`, row);
        const loc = normalizeLocation(symbol.location);
        if (loc) {
            if (!symbolTableRowsByLine.has(loc.line)) symbolTableRowsByLine.set(loc.line, []);
            symbolTableRowsByLine.get(loc.line).push(row);
        }
    });

    table.appendChild(tbody);
    container.appendChild(table);
}

/**
 * Render the Errors tab
 * @param {SemanticErrorInfo[]} errors - Semantic errors
 */
export function renderErrorsTab(errors) {
    const container = document.getElementById('errors-content');
    const empty = document.getElementById('errors-empty');
    if (!container) return;

    container.querySelectorAll('.errors-list').forEach(el => el.remove());

    if (!errors.length) {
        if (empty) empty.classList.remove('hidden');
        return;
    }
    if (empty) empty.classList.add('hidden');

    const list = document.createElement('div');
    list.className = 'errors-list';

    errors.forEach(error => {
        const item = document.createElement('div');
        item.className = 'error-item';

        const header = document.createElement('div');
        header.className = 'error-header';

        const type = document.createElement('span');
        type.className = 'error-type';
        type.textContent = error.type || 'Semantic Error';
        header.appendChild(type);

        const location = document.createElement('span');
        location.className = 'error-location';
        location.textContent = formatLocation(error.location);
        header.appendChild(location);

        item.appendChild(header);

        const message = document.createElement('div');
        message.className = 'error-message';
        message.textContent = error.message || '';
        item.appendChild(message);

        item.addEventListener('click', () => navigateTo(error.location));

        list.appendChild(item);
    });

    container.appendChild(list);
}

/**
 * Render the Warnings tab
 * @param {string[]} warnings - Analyzer warnings
 */
export function renderWarningsTab(warnings) {
    const container = document.getElementById('warnings-content');
    const empty = document.getElementById('warnings-empty');
    if (!container) return;

    container.querySelectorAll('.warning-list').forEach(el => el.remove());

    if (!warnings || !warnings.length) {
        if (empty) empty.classList.remove('hidden');
        return;
    }
    if (empty) empty.classList.add('hidden');

    const list = document.createElement('div');
    list.className = 'warnings-list';

    warnings.forEach(warning => {
        const item = document.createElement('div');
        item.className = 'warning-item';

        const message = document.createElement('div');
        message.className = 'warning-message';
        message.textContent = warning;
        item.appendChild(message);

        list.appendChild(item);
    });

    container.appendChild(list);
}

/**
 * Render the Info tab
 * @param {object} result - Analysis output
 */
export function renderInfoTab(result) {
    const container = document.getElementById('info-content');
    if (!container) return;

    // Remove the placeholder
    const placeholder = container.querySelector('.placeholder');
    if (placeholder) placeholder.classList.add('hidden');

    const statsSection = document.getElementById('info-stats');
    if (statsSection) statsSection.classList.remove('hidden');

    const set = (id, value) => {
        const el = document.getElementById(id);
        if (el) el.textContent = value;
    };

    set('stat-scopes', (result.scope_count ?? (result.scopes || []).length).toLocaleString());
    set('stat-symbols', (result.symbol_count ?? countSymbols(result.scopes)).toLocaleString());
    set('stat-errors', (result.error_count ?? (result.errors || []).length).toLocaleString());
    set('stat-warnings', (result.warning_count ?? (result.warnings || []).length).toLocaleString());
    set('stat-analyze-time', `${(result.analyze_time_ms || 0).toFixed(2)}ms`);
    set('stat-file-type', document.getElementById('file-type')?.value || 'module');

    const model = window.__vb6semanticEditorModel;
    if (model) {
        set('stat-lines', model.getLineCount().toLocaleString());
        set('stat-chars', model.getValueLength().toLocaleString());
    }
}

/**
 * Count symbols across scopes
 * @param {ScopeInfo[]} scopes - All scopes
 * @returns {number} Total symbol count
 */
function countSymbols(scopes) {
    return (scopes || []).reduce((sum, scope) => sum + (scope.symbols || []).length, 0);
}

/**
 * Format a location for display
 * @param {object} location - Location object
 * @returns {string} "file:line:column" or "--"
 */
function formatLocation(location) {
    if (!location) return '--';
    const file = location.file || '';
    const line = location.line ?? '?';
    const column = location.column ?? '?';
    return `${file}:${line}:${column}`;
}

/**
 * Highlight the symbol rows for a given line in both the scope tree and table
 * @param {number} line - Editor line (1-based)
 */
export function highlightSymbolAtLine(line) {
    clearSymbolHighlights();

    const rows = symbolRowByLine.get(line);
    if (rows) {
        rows.forEach(row => {
            row.classList.add('highlighted');
            row.scrollIntoView({ behavior: 'smooth', block: 'nearest' });
        });
    }

    const tableRows = symbolTableRowsByLine.get(line);
    if (tableRows) {
        tableRows.forEach(row => {
            row.classList.add('highlighted');
            row.scrollIntoView({ behavior: 'smooth', block: 'nearest' });
        });
    }
}

/**
 * Clear all symbol/scope highlights
 */
export function clearSymbolHighlights() {
    document.querySelectorAll('.symbol-row.highlighted, .symbols-table tr.highlighted').forEach(el => {
        el.classList.remove('highlighted');
    });
}

// Maps built during rendering: line -> DOM elements
let symbolRowByLine = new Map();
let symbolTableRowsByLine = new Map();

/**
 * Clear all output tabs
 */
export function clearOutput() {
    const scopesContent = document.getElementById('scopes-content');
    if (scopesContent) {
        scopesContent.innerHTML = '<div class="placeholder"><p>👈 Enter VB6 code and click Analyze to see scopes and symbols</p></div>';
    }

    const symbolsContent = document.getElementById('symbols-content');
    if (symbolsContent) {
        symbolsContent.innerHTML = '<div class="placeholder"><p>👈 Analyze code to see the symbol table</p></div>';
    }

    const errorsContent = document.getElementById('errors-content');
    if (errorsContent) errorsContent.querySelectorAll('.errors-list').forEach(el => el.remove());

    const errorsEmpty = document.getElementById('errors-empty');
    if (errorsEmpty) errorsEmpty.classList.remove('hidden');

    const warningsContent = document.getElementById('warnings-content');
    if (warningsContent) warningsContent.querySelectorAll('.warning-list').forEach(el => el.remove());

    const warningsEmpty = document.getElementById('warnings-empty');
    if (warningsEmpty) warningsEmpty.classList.remove('hidden');

    const infoStats = document.getElementById('info-stats');
    if (infoStats) infoStats.classList.add('hidden');

    const infoContent = document.getElementById('info-content');
    const placeholder = infoContent?.querySelector('.placeholder');
    if (placeholder) placeholder.classList.remove('hidden');

    symbolRowByKey = new Map();
    symbolTableRows = new Map();
    symbolRowByLine = new Map();
    symbolTableRowsByLine = new Map();
}

export default {
    renderOutput,
    renderScopesTab,
    renderSymbolsTab,
    renderErrorsTab,
    renderWarningsTab,
    renderInfoTab,
    highlightSymbolAtLine,
    clearSymbolHighlights,
    clearOutput
};
