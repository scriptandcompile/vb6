import fs from "node:fs";
import path from "node:path";

const root = path.resolve(path.dirname(new URL(import.meta.url).pathname), "..");
const fixturesDir = path.join(root, "test-fixtures");
const wasmPath = path.join(root, "src", "vendor", "vb6parse", "vb6parse.js");

if (!fs.existsSync(wasmPath)) {
  console.error("Vendored vb6parse WASM not found. Run `npm run build-wasm` first.");
  process.exit(1);
}

const wasm = await import(wasmPath);

const expectations = {
  "valid_module.bas": { minErrors: 0, maxErrors: 0, minRecovery: 0, maxRecovery: 0 },
  "mismatched_keywords.bas": { minErrors: 1 },
  "missing_next.bas": { minErrors: 1 },
  "sub_with_end_function.bas": { minErrors: 1 },
  "non_ascii_module.bas": { minErrors: 1 },
};

let failed = false;
const rows = [];

for (const [file, exp] of Object.entries(expectations)) {
  const source = fs.readFileSync(path.join(fixturesDir, file), "utf8");
  let errors = [];
  let recovery = [];
  let note = "";
  try {
    const output = wasm.parse_vb6_code(source, "module");
    errors = output.errors ?? [];
    recovery = output.recovery_diagnostics ?? [];
  } catch (err) {
    note = `parse threw: ${err}`;
  }

  const minErrors = exp.minErrors ?? 0;
  const maxErrors = exp.maxErrors ?? Number.MAX_SAFE_INTEGER;
  const minRecovery = exp.minRecovery ?? 0;
  const maxRecovery = exp.maxRecovery ?? Number.MAX_SAFE_INTEGER;
  const ok =
    !note &&
    errors.length >= minErrors &&
    errors.length <= maxErrors &&
    recovery.length >= minRecovery &&
    recovery.length <= maxRecovery;

  if (!ok) {
    failed = true;
  }
  rows.push({ file, ok, errors: errors.length, recovery: recovery.length, note });
}

for (const row of rows) {
  const status = row.ok ? "PASS" : "FAIL";
  const detail = row.note
    ? ` (${row.note})`
    : ` (errors=${row.errors}, recovery=${row.recovery})`;
  console.log(`${status} ${row.file}${detail}`);
}

if (failed) {
  console.error("Diagnostic fixture checks failed.");
  process.exit(1);
}
console.log("All diagnostic fixture checks passed.");
