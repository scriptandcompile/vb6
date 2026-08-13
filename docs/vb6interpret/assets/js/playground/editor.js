let editor = null;
let themeObserver = null;
let executionLineDecorations = [];

export async function initEditor(containerId, initialValue = "") {
    return new Promise((resolve, reject) => {
        if (typeof require === "undefined") {
            reject(new Error("Monaco Editor loader not found"));
            return;
        }

        require.config({
            paths: {
                vs: "https://cdn.jsdelivr.net/npm/monaco-editor@0.45.0/min/vs",
            },
        });

        require(["vs/editor/editor.main"], () => {
            registerVB6Language();

            editor = monaco.editor.create(document.getElementById(containerId), {
                value: initialValue,
                language: "vb6",
                theme: getCurrentTheme(),
                automaticLayout: true,
                fontSize: 14,
                lineNumbers: "on",
                minimap: {
                    enabled: true,
                },
                scrollBeyondLastLine: false,
                wordWrap: "on",
                tabSize: 4,
                insertSpaces: true,
                matchBrackets: "always",
            });

            editor.onDidChangeModelContent(() => {
                updateEditorStats();
                document.dispatchEvent(new CustomEvent("editorContentChanged"));
            });

            editor.onDidChangeCursorPosition((event) => {
                updateCursorPosition(event.position);
            });

            startThemeObserver();
            updateEditorStats();
            updateCursorPosition(editor.getPosition());

            resolve(editor);
        });
    });
}

function registerVB6Language() {
    if (monaco.languages.getLanguages().some((language) => language.id === "vb6")) {
        return;
    }

    monaco.languages.register({ id: "vb6" });
    monaco.languages.setMonarchTokensProvider("vb6", {
        keywords: [
            "AddressOf", "Alias", "As", "Attribute", "Boolean", "ByRef", "ByVal", "Call",
            "Case", "Class", "Close", "Const", "Currency", "Date", "Declare", "Dim", "Do",
            "Double", "Each", "Else", "ElseIf", "End", "Enum", "Erase", "Error", "Event",
            "Exit", "Explicit", "False", "For", "Friend", "Function", "Get", "GoSub", "GoTo",
            "If", "Implements", "In", "Input", "Integer", "Is", "Let", "Lib", "Long", "Loop",
            "Me", "Mod", "New", "Next", "Nothing", "Null", "Object", "On", "Open", "Option",
            "Optional", "Preserve", "Print", "Private", "Property", "Public", "Put", "RaiseEvent",
            "ReDim", "Resume", "Return", "Select", "Set", "Single", "Static", "Step", "Stop",
            "String", "Sub", "Then", "To", "True", "Type", "Until", "Variant", "Wend",
            "While", "With", "WithEvents", "Write",
        ],
        ignoreCase: true,
        tokenizer: {
            root: [
                [/'.*$/, "comment"],
                [/^REM\s+.*$/, "comment"],
                [/"([^"\\]|\\.)*$/, "string.invalid"],
                [/"/, "string", "@string"],
                [/\b\d+\.?\d*[#!@%&]?\b/, "number"],
                [/&H[0-9A-Fa-f]+/, "number.hex"],
                [/&O[0-7]+/, "number.octal"],
                [/\b(?:Sub|Function|Property|End|If|Then|Else|ElseIf|Select|Case|For|Do|While|Loop|Next|Exit|GoTo|GoSub|On|Resume)\b/, "keyword.control"],
                [/@?[a-zA-Z_]\w*/, {
                    cases: {
                        "@keywords": "keyword",
                        "@default": "identifier",
                    },
                }],
                [/[=<>!+\-*\/\\^&]/, "operator"],
                [/_$/, "operator"],
                [/[()[\]]/, "delimiter.bracket"],
                [/[,.:;]/, "delimiter"],
            ],
            string: [
                [/[^\\"]+/, "string"],
                [/""/, "string.escape"],
                [/"/, "string", "@pop"],
            ],
        },
    });

    monaco.editor.defineTheme("vb6-dark", {
        base: "vs-dark",
        inherit: true,
        rules: [
            { token: "comment", foreground: "6A9955" },
            { token: "keyword", foreground: "569CD6" },
            { token: "keyword.control", foreground: "C586C0" },
            { token: "string", foreground: "CE9178" },
            { token: "number", foreground: "B5CEA8" },
            { token: "operator", foreground: "D4D4D4" },
        ],
        colors: {},
    });

    monaco.editor.defineTheme("vb6-light", {
        base: "vs",
        inherit: true,
        rules: [
            { token: "comment", foreground: "008000" },
            { token: "keyword", foreground: "0000FF" },
            { token: "keyword.control", foreground: "AF00DB" },
            { token: "string", foreground: "A31515" },
            { token: "number", foreground: "098658" },
            { token: "operator", foreground: "000000" },
        ],
        colors: {},
    });
}

function getCurrentTheme() {
    const theme = document.documentElement.getAttribute("data-theme");
    return theme === "dark" ? "vb6-dark" : "vb6-light";
}

export function updateEditorTheme() {
    if (editor) {
        monaco.editor.setTheme(getCurrentTheme());
    }
}

function startThemeObserver() {
    if (themeObserver) {
        return;
    }

    themeObserver = new MutationObserver(() => {
        updateEditorTheme();
    });

    themeObserver.observe(document.documentElement, {
        attributes: true,
        attributeFilter: ["data-theme"],
    });
}

function updateEditorStats() {
    if (!editor) {
        return;
    }

    const model = editor.getModel();
    const lineCount = model.getLineCount();
    const charCount = model.getValueLength();
    const statsElement = document.getElementById("editor-stats");

    if (statsElement) {
        statsElement.textContent = `${lineCount} lines | ${charCount} chars`;
    }
}

function updateCursorPosition(position) {
    const statusElement = document.getElementById("editor-line-col");
    if (statusElement && position) {
        statusElement.textContent = `Ln ${position.lineNumber}, Col ${position.column}`;
    }
}

export function getEditorContent() {
    return editor ? editor.getValue() : "";
}

export function setEditorContent(code) {
    if (editor) {
        editor.setValue(code);
    }
}

export function clearEditor() {
    if (editor) {
        editor.setValue("");
    }

    clearExecutionHighlight();
}

export function focusEditor() {
    editor?.focus();
}

export function getEditor() {
    return editor;
}

export function highlightExecutionLine(lineNumber) {
    if (!editor || !Number.isInteger(lineNumber) || lineNumber < 1) {
        clearExecutionHighlight();
        return;
    }

    const model = editor.getModel();
    if (!model || lineNumber > model.getLineCount()) {
        clearExecutionHighlight();
        return;
    }

    executionLineDecorations = editor.deltaDecorations(executionLineDecorations, [
        {
            range: new monaco.Range(lineNumber, 1, lineNumber, 1),
            options: {
                isWholeLine: true,
                className: "execution-line-highlight",
                glyphMarginClassName: "execution-line-glyph",
                linesDecorationsClassName: "execution-line-decoration",
                stickiness: monaco.editor.TrackedRangeStickiness.NeverGrowsWhenTypingAtEdges,
            },
        },
    ]);

    editor.revealLineInCenterIfOutsideViewport(lineNumber);
}

export function highlightExecutionRange(cursor) {
    if (!editor || !cursor || cursor.length !== 4) {
        return;
    }

    const [startLine, startColumn, endLine, endColumn] = cursor;
    const model = editor.getModel();
    if (!model || startLine < 1 || startLine > model.getLineCount()) {
        clearExecutionHighlight();
        return;
    }

    const startText = model.getLineContent(startLine);
    const endText = model.getLineContent(endLine);
    const clampedStart = Math.min(Math.max(1, startColumn), startText.length + 1);
    const clampedEnd = Math.min(Math.max(clampedStart, endColumn), endText.length + 1);

    executionLineDecorations = editor.deltaDecorations(executionLineDecorations, [
        {
            range: new monaco.Range(startLine, clampedStart, endLine, clampedEnd),
            options: {
                isWholeLine: false,
                className: "execution-range-highlight",
                glyphMarginClassName: "execution-line-glyph",
                linesDecorationsClassName: "execution-line-decoration",
                stickiness: monaco.editor.TrackedRangeStickiness.NeverGrowsWhenTypingAtEdges,
            },
        },
    ]);

    editor.revealLineInCenterIfOutsideViewport(startLine);
}

export function clearExecutionHighlight() {
    if (!editor) {
        return;
    }

    executionLineDecorations = editor.deltaDecorations(executionLineDecorations, []);
}