import * as vscode from "vscode";
import * as wasm from "./vendor/vb6parse/vb6parse";
import { charOffsetFromByteOffset, ErrorInfo, fileTypeFor } from "./vb6Wasm";

const COLLECTION_NAME = "vb6-parser";
const SEVERITY_PREFIXES = new Map<string, string>([
  ["Note", "information"],
  ["Warning", "warning"],
  ["Parse Error", "error"],
]);

function severityFor(type: string): vscode.DiagnosticSeverity {
  const prefix = SEVERITY_PREFIXES.get(type);
  if (prefix === "error") {
    return vscode.DiagnosticSeverity.Error;
  }
  if (prefix === "warning") {
    return vscode.DiagnosticSeverity.Warning;
  }
  if (type.startsWith("Recovery")) {
    return vscode.DiagnosticSeverity.Warning;
  }
  return vscode.DiagnosticSeverity.Information;
}

function toRange(document: vscode.TextDocument, text: string, range: [number, number]): vscode.Range {
  let start = charOffsetFromByteOffset(text, range[0]);
  let end = charOffsetFromByteOffset(text, range[1]);
  if (end <= start) {
    end = Math.min(start + 1, text.length);
  }
  const startPos = document.positionAt(start);
  const endPos = document.positionAt(end);
  return new vscode.Range(startPos, endPos);
}

function toDiagnostic(document: vscode.TextDocument, text: string, error: ErrorInfo): vscode.Diagnostic {
  const range = toRange(document, text, error.range);
  const diagnostic = new vscode.Diagnostic(range, error.message, severityFor(error.type));
  diagnostic.source = "vb6";
  diagnostic.code = error.type;
  return diagnostic;
}

export class ParserDiagnosticsProvider implements vscode.Disposable {
  private readonly collection: vscode.DiagnosticCollection;
  private readonly timers = new Map<string, NodeJS.Timeout>();
  private readonly changeDisposable: vscode.Disposable;

  constructor() {
    this.collection = vscode.languages.createDiagnosticCollection(COLLECTION_NAME);

    const subscriptions: vscode.Disposable[] = [];
    subscriptions.push(
      vscode.workspace.onDidOpenTextDocument((document) => this.schedule(document)),
      vscode.workspace.onDidChangeTextDocument((event) => this.schedule(event.document)),
      vscode.window.onDidChangeActiveTextEditor((editor) => {
        if (editor) {
          this.schedule(editor.document);
        }
      }),
      vscode.workspace.onDidCloseTextDocument((document) => {
        this.clear(document);
        this.timers.delete(document.uri.toString());
      })
    );
    this.changeDisposable = vscode.Disposable.from(...subscriptions);

    for (const document of vscode.workspace.textDocuments) {
      this.schedule(document);
    }
  }

  /** Parse immediately, bypassing the debounce timer. Used by the on-demand command. */
  public parse(document: vscode.TextDocument): void {
    this.timers.delete(document.uri.toString());
    this.diagnose(document);
  }

  private schedule(document: vscode.TextDocument): void {
    if (document.languageId !== "vb6") {
      return;
    }
    const key = document.uri.toString();
    const existing = this.timers.get(key);
    if (existing) {
      clearTimeout(existing);
    }
    const debounceMs = Math.max(
      0,
      vscode.workspace.getConfiguration("vb6").get<number>("diagnostics.debounceMs", 300)
    );
    const timer = setTimeout(() => {
      this.timers.delete(key);
      this.diagnose(document);
    }, debounceMs);
    this.timers.set(key, timer);
  }

  private diagnose(document: vscode.TextDocument): void {
    const config = vscode.workspace.getConfiguration("vb6");
    if (!config.get<boolean>("diagnostics.enabled", true)) {
      this.collection.delete(document.uri);
      return;
    }

    const text = document.getText();
    if (text.length === 0) {
      this.collection.set(document.uri, []);
      return;
    }

    let output;
    try {
      output = wasm.parse_vb6_code(text, fileTypeFor(document.fileName));
    } catch (error) {
      this.collection.delete(document.uri);
      return;
    }

    const diagnostics: vscode.Diagnostic[] = [];
    const seen = new Set<string>();
    const push = (info: ErrorInfo) => {
      const key = `${info.type}:${info.line}:${info.column}:${info.range[0]}`;
      if (seen.has(key)) {
        return;
      }
      seen.add(key);
      diagnostics.push(toDiagnostic(document, text, info));
    };

    for (const info of output.errors ?? []) {
      push(info);
    }
    for (const info of output.recovery_diagnostics ?? []) {
      push(info);
    }

    this.collection.set(document.uri, diagnostics);
  }

  private clear(document: vscode.TextDocument): void {
    this.collection.delete(document.uri);
  }

  dispose(): void {
    this.changeDisposable.dispose();
    for (const timer of this.timers.values()) {
      clearTimeout(timer);
    }
    this.timers.clear();
    this.collection.dispose();
  }
}
