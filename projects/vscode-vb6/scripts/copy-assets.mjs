import fs from "node:fs";
import path from "node:path";

const root = path.resolve(path.dirname(new URL(import.meta.url).pathname), "..");
const srcVendor = path.join(root, "src", "vendor");
const outVendor = path.join(root, "out", "vendor");

if (!fs.existsSync(srcVendor)) {
  console.log("No src/vendor directory; nothing to copy.");
  process.exit(0);
}

fs.mkdirSync(outVendor, { recursive: true });
fs.cpSync(srcVendor, outVendor, { recursive: true });
console.log(`Copied ${path.relative(root, srcVendor)} -> ${path.relative(root, outVendor)}`);
