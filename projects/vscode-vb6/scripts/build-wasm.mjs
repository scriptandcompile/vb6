import { execFileSync } from "node:child_process";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";

const root = path.resolve(path.dirname(new URL(import.meta.url).pathname), "..");
const defaultParserRoot = path.resolve(root, "..", "vb6parse");
const parserRoot = process.env.VB6PARSE_ROOT || defaultParserRoot;
const vendorDir = path.resolve(root, "src", "vendor", "vb6parse");

const wasmPack = process.env.WASMPACK_PATH || "wasm-pack";
const wasmOpt = process.env.WASMOPT_PATH || "wasm-opt";

if (!fs.existsSync(path.join(parserRoot, "Cargo.toml"))) {
  console.error(`Could not find vb6parse crate at: ${parserRoot}`);
  console.error("Set VB6PARSE_ROOT if your workspace layout differs.");
  process.exit(1);
}

const buildDir = fs.mkdtempSync(path.join(os.tmpdir(), "vb6parse-wasm-"));

const run = (cmd, description) => {
  console.log(`Building ${description}...`);
  try {
    execFileSync(cmd[0], cmd.slice(1), { cwd: parserRoot, stdio: "inherit" });
  } catch (err) {
    console.error(`${description} failed (exit code ${err.status})`);
    fs.rmSync(buildDir, { recursive: true, force: true });
    process.exit(1);
  }
};

run(
  [wasmPack, "build", "--target", "nodejs", "--out-dir", buildDir, "--release", "--no-opt"],
  "vb6parse WASM (nodejs target)"
);

fs.mkdirSync(vendorDir, { recursive: true });

for (const file of ["vb6parse.js", "vb6parse_bg.wasm", "vb6parse.d.ts"]) {
  const source = path.join(buildDir, file);
  if (!fs.existsSync(source)) {
    console.error(`Expected ${file} in wasm-pack output, not found.`);
    fs.rmSync(buildDir, { recursive: true, force: true });
    process.exit(1);
  }
  fs.copyFileSync(source, path.join(vendorDir, file));
}

fs.rmSync(buildDir, { recursive: true, force: true });

const wasmFile = path.join(vendorDir, "vb6parse_bg.wasm");
const originalSize = fs.statSync(wasmFile).size;

try {
  execFileSync(
    wasmOpt,
    ["-Oz", "--enable-bulk-memory", "-o", wasmFile, wasmFile],
    { stdio: "inherit" }
  );
  const optimizedSize = fs.statSync(wasmFile).size;
  console.log(
    `Optimized wasm: ${originalSize.toLocaleString()} -> ${optimizedSize.toLocaleString()} bytes`
  );
} catch {
  console.log("Skipping wasm-opt optimization (not installed).");
}

console.log(`Vendored vb6parse WASM into ${path.relative(root, vendorDir)}`);
