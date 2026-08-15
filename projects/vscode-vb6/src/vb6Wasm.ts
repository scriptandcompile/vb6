export interface ErrorInfo {
  type: string;
  line: number;
  column: number;
  range: [number, number];
  message: string;
}

export interface ParseStats {
  token_count: number;
  node_count: number;
  tree_depth: number;
}

export interface PlaygroundOutput {
  errors: ErrorInfo[];
  recovery_diagnostics: ErrorInfo[];
  parse_time_ms: number;
  stats: ParseStats;
}

/**
 * Map a UTF-8 byte offset (as produced by the vb6parse wasm range fields)
 * to a character offset in the JavaScript string. The wasm was fed the same
 * text encoded as UTF-8, so walking the string and accumulating UTF-8 byte
 * widths recovers the exact position even for non-ASCII content.
 */
export function charOffsetFromByteOffset(text: string, byteOffset: number): number {
  let bytes = 0;
  for (let i = 0; i < text.length; i++) {
    if (bytes >= byteOffset) {
      return i;
    }
    const code = text.codePointAt(i)!;
    bytes += code > 0xffff ? 4 : code > 0x7ff ? 3 : code > 0x7f ? 2 : 1;
    if (code > 0xffff) {
      i += 1;
    }
  }
  return text.length;
}

export function fileTypeFor(fileName: string): string {
  const ext = fileName.toLowerCase().split(".").pop() ?? "";
  switch (ext) {
    case "cls":
    case "ctl":
      return "class";
    case "frm":
    case "dob":
      return "form";
    default:
      return "module";
  }
}
