/**
 * VB6Parse Playground - Parser Module
 * 
 * Handles WASM module loading and provides wrapper functions for parsing VB6 code.
 * This is the bridge between the editor and the WASM parser.
 */

import init, { tokenize_vb6_code, parse_vb6_code } from "../../wasm/vb6parse.js";

let wasmInitialized = false;

/**
 * Initialize the WASM module from the bundled asset.
 */
export async function initWasm() {
    try {
        console.log('Loading vb6parse WASM from bundled asset...');
        await init();
        wasmInitialized = true;
        console.log('vb6parse WASM initialized');
        return true;
    } catch (error) {
        console.error('Failed to initialize vb6parse WASM:', error);
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
 * Parse VB6 code and return full parse result
 */
export async function parseCode(code, fileType) {
    if (!wasmInitialized) {
        throw new Error('WASM module not initialized');
    }

    const startTime = performance.now();

    try {
        const parseResult = parse_vb6_code(code, fileType);
        const parseTime = performance.now() - startTime;
        parseResult.parseTimeMs = parseTime;

        console.log(`Parsed ${fileType} in ${parseTime.toFixed(2)}ms`);
        return parseResult;

    } catch (error) {
        console.error('Parse error:', error);
        throw new Error(`Failed to parse ${fileType}: ${error.message}`);
    }
}

/**
 * Tokenize VB6 code (faster than full parse)
 */
export async function tokenizeCode(code) {
    if (!wasmInitialized) {
        throw new Error('WASM module not initialized');
    }

    try {
        return await tokenize_vb6_code(code);

    } catch (error) {
        console.error('Tokenize error:', error);
        throw error;
    }
}

/**
 * Create mock CST for testing UI
 */
function createMockCst(code, fileType) {
    return {
        kind: 'CompilationUnit',
        range: [0, code.length],
        children: [
            {
                kind: 'VersionStatement',
                range: [0, 20],
                children: [
                    { kind: 'Keyword', value: 'VERSION', range: [0, 7] },
                    { kind: 'Whitespace', value: ' ', range: [7, 8] },
                    { kind: 'Number', value: '1.0', range: [8, 11] }
                ]
            },
            {
                kind: 'OptionStatement',
                range: [21, 36],
                children: [
                    { kind: 'Keyword', value: 'Option', range: [21, 27] },
                    { kind: 'Whitespace', value: ' ', range: [27, 28] },
                    { kind: 'Keyword', value: 'Explicit', range: [28, 36] }
                ]
            }
        ]
    };
}

export default {
    initWasm,
    isWasmReady,
    parseCode,
    tokenizeCode
};
