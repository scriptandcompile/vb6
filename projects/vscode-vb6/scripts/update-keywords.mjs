import fs from "node:fs";
import path from "node:path";

const root = path.resolve(path.dirname(new URL(import.meta.url).pathname), "..");
const defaultLexerPath = path.resolve(root, "..", "vb6parse", "src", "lexer", "mod.rs");
const lexerPath = process.env.VB6PARSE_LEXER_PATH || defaultLexerPath;
const outputPath = path.resolve(root, "src", "keyword-manifest.json");

if (!fs.existsSync(lexerPath)) {
  console.error(`Could not find lexer file at: ${lexerPath}`);
  console.error("Set VB6PARSE_LEXER_PATH if your workspace layout differs.");
  process.exit(1);
}

const content = fs.readFileSync(lexerPath, "utf8");
const mapRegex = /^\s*"([A-Z0-9_]+)"\s*=>\s*Token::[A-Za-z0-9_]+,/gm;

const keywords = [];
let match;
while ((match = mapRegex.exec(content)) !== null) {
  keywords.push(match[1]);
}

const uniqueSorted = [...new Set(keywords)].sort((a, b) => a.localeCompare(b));
const manifest = {
  source: path.relative(root, lexerPath).replace(/\\/g, "/"),
  generatedAt: new Date().toISOString().slice(0, 10),
  keywords: uniqueSorted
};

fs.writeFileSync(outputPath, `${JSON.stringify(manifest, null, 2)}\n`, "utf8");
console.log(`Wrote ${uniqueSorted.length} keywords to ${outputPath}`);
