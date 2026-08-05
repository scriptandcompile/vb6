/**
 * VB6Semantic Playground - Analyzer Module
 *
 * Handles WASM module loading and provides a wrapper around the semantic
 * analyzer. This is the bridge between the editor and the WASM analyzer.
 */

import init, { analyze_vb6_code, init_panic_hook } from "../../wasm/vb6semantic.js";

let wasmInitialized = false;

/**
 * Initialize the WASM module
 * This should be called on page load
 *
 * @returns {Promise<boolean>} True if initialization succeeded
 */
export async function initWasm() {
    try {
        await init();
        init_panic_hook();
        wasmInitialized = true;
        return true;
    } catch (error) {
        console.error('Failed to initialize WASM:', error);
        return false;
    }
}

/**
 * Check if WASM is initialized
 * @returns {boolean}
 */
export function isWasmReady() {
    return wasmInitialized;
}

/**
 * Analyze VB6 code and return the semantic analysis output.
 *
 * The result contains:
 * - scopes: hierarchical scopes with their symbols (name, kind, type, visibility, location)
 * - errors: semantic errors with category, message, and location
 * - warnings: analyzer warnings
 * - stats: scope/symbol/error counts and analysis time
 *
 * @param {string} code - VB6 source code
 * @param {string} fileType - 'module'/'bas', 'class'/'cls', or 'form'/'frm'
 * @returns {object} Analysis output object
 */
export function analyzeCode(code, fileType) {
    if (!wasmInitialized) {
        throw new Error('WASM module not initialized');
    }

    const startTime = performance.now();

    try {
        const result = analyze_vb6_code(code, fileType);
        const elapsed = performance.now() - startTime;

        // The Rust side may report 0 for timer purposes; prefer the measured wall time.
        if (!result.analyze_time_ms || result.analyze_time_ms <= 0) {
            result.analyze_time_ms = elapsed;
        }

        console.log(`✅ Analyzed ${fileType} in ${elapsed.toFixed(2)}ms`);
        return result;
    } catch (error) {
        console.error('Analysis error:', error);
        throw new Error(`Failed to analyze ${fileType}: ${error.message}`);
    }
}

/**
 * Type definitions (for documentation)
 *
 * @typedef {Object} LocationInfo
 * @property {string} file - Source file name
 * @property {number} line - Line number (1-based)
 * @property {number} column - Column number (1-based)
 *
 * @typedef {Object} SymbolInfo
 * @property {string} name - Symbol name
 * @property {string} kind - Symbol kind (Variable, Function, Class, ...)
 * @property {Object} type_info - Structured type information
 * @property {string} type_display - Human-readable type
 * @property {string} visibility - Public, Private, Friend, or Global
 * @property {LocationInfo} location - Definition location
 * @property {number} scope_id - Containing scope ID
 * @property {Array<[string,string]>} attributes - Extra symbol attributes
 *
 * @typedef {Object} ScopeInfo
 * @property {number} id - Unique scope ID
 * @property {string} kind - Global, Class, Procedure, Property, Block, Type, Enum
 * @property {number|null} parent - Parent scope ID
 * @property {number[]} children - Child scope IDs
 * @property {string} name - Scope name
 * @property {SymbolInfo[]} symbols - Symbols defined in this scope
 *
 * @typedef {Object} SemanticErrorInfo
 * @property {string} type - Error category (UndefinedSymbol, TypeMismatch, ...)
 * @property {string} message - Human-readable description
 * @property {LocationInfo|null} location - Error location, if available
 *
 * @typedef {Object} AnalysisOutput
 * @property {ScopeInfo[]} scopes - All scopes in the final scope manager
 * @property {SemanticErrorInfo[]} errors - Semantic errors found
 * @property {string[]} warnings - Analyzer warnings
 * @property {boolean} successful - Whether analysis completed without errors
 * @property {number} error_count - Total errors
 * @property {number} warning_count - Total warnings
 * @property {number} symbol_count - Total symbols across all scopes
 * @property {number} scope_count - Total scopes
 * @property {number} analyze_time_ms - Analysis time in milliseconds
 */

export default {
    initWasm,
    isWasmReady,
    analyzeCode
};
