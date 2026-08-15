import * as vscode from "vscode";
import { ParserDiagnosticsProvider } from "./parserDiagnostics";

export function activate(context: vscode.ExtensionContext): void {
  const provider = new ParserDiagnosticsProvider();
  context.subscriptions.push(provider);

  const disposable = vscode.commands.registerCommand(
    "vb6.updateKeywordManifest",
    async () => {
      await vscode.window.showInformationMessage(
        "Run 'npm run update-keywords' in vscode-vb6 to refresh keyword-manifest.json from vb6parse."
      );
    }
  );
  context.subscriptions.push(disposable);

  context.subscriptions.push(
    vscode.commands.registerCommand("vb6.parseFile", () => {
      const editor = vscode.window.activeTextEditor;
      if (!editor || editor.document.languageId !== "vb6") {
        vscode.window.showInformationMessage("Open a VB6 file to parse.");
        return;
      }
      provider.parse(editor.document);
    })
  );
}

export function deactivate(): void {
  // No-op for now.
}
