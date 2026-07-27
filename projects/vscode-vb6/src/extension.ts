import * as vscode from "vscode";

export function activate(context: vscode.ExtensionContext): void {
  const disposable = vscode.commands.registerCommand(
    "vb6.updateKeywordManifest",
    async () => {
      await vscode.window.showInformationMessage(
        "Run 'npm run update-keywords' in vscode-vb6 to refresh keyword-manifest.json from vb6parse."
      );
    }
  );

  context.subscriptions.push(disposable);
}

export function deactivate(): void {
  // No-op for now.
}
