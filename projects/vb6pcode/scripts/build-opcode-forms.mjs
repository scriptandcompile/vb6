#!/usr/bin/env node

import { spawnSync } from "node:child_process";
import { readFileSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const SCRIPT_DIR = dirname(fileURLToPath(import.meta.url));
const REPO_ROOT = resolve(SCRIPT_DIR, "../../..");
const P_CODE_DIR = resolve(REPO_ROOT, "research/pcode");
const BINARY_ID =
  "sha256:e043c59c66ba2dc549bf5ac104c70279ec05931d531de2971458eab91871f7b6";
const DEFAULT_INPUT = resolve(
  REPO_ROOT,
  "reference/vb_symbols/vb6_ide/MSVBVM60.DLL",
);
const SELECTED_OPCODES = [
  0x00, 0x02, 0x05, 0x06, 0x1b, 0x75, 0x77, 0xf5, 0xf6, 0xf7,
];
const UNKNOWN_LENGTH_OPCODES = [
  0x01, 0x03, 0x04, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
];
const IMAGE_BASE = 0x66000000;
const FORM_KEYS = new Set([
  "schema_version",
  "form_id",
  "binary_id",
  "prefix",
  "opcode",
  "mnemonic",
  "length_rule",
  "operand_schema",
  "effect_status",
  "evidence_refs",
  "implemented",
]);

function fail(message) {
  throw new Error(message);
}

export function opcodeFormIdentity(form) {
  return `${form.prefix ?? "none"}:${form.opcode}`;
}

export function validateOpcodeForm(form) {
  const required = [
    "schema_version",
    "form_id",
    "binary_id",
    "prefix",
    "opcode",
    "mnemonic",
    "length_rule",
    "operand_schema",
    "effect_status",
    "evidence_refs",
    "implemented",
  ];
  if (
    !form ||
    typeof form !== "object" ||
    Object.keys(form).some((key) => !FORM_KEYS.has(key)) ||
    required.some((key) => !Object.hasOwn(form, key))
  ) {
    return false;
  }
  return (
    form.schema_version === "vb6r.opcode-form.v1" &&
    /^OF-M1-\d{4}$/.test(form.form_id) &&
    /^sha256:[0-9a-f]{64}$/.test(form.binary_id) &&
    (form.prefix === null || /^0x[0-9a-fA-F]{2}$/.test(form.prefix)) &&
    /^0x[0-9a-fA-F]{2}$/.test(form.opcode) &&
    (form.mnemonic === null || typeof form.mnemonic === "string") &&
    ["known", "variable", "unknown"].includes(form.length_rule) &&
    Array.isArray(form.operand_schema) &&
    form.operand_schema.every(
      (operand) => operand && typeof operand === "object" && !Array.isArray(operand),
    ) &&
    ["UNKNOWN", "DOCUMENTED", "STATIC_CONFIRMED"].includes(form.effect_status) &&
    Array.isArray(form.evidence_refs) &&
    form.evidence_refs.length > 0 &&
    form.evidence_refs.every((reference) => typeof reference === "string" && reference.length > 0) &&
    typeof form.implemented === "boolean"
  );
}

export function buildUnknownExtendedForms(dispatchMap, firstFormNumber = 21) {
  if (
    dispatchMap?.schema_version !== "vb6r.dispatch-map.v2" ||
    !Array.isArray(dispatchMap.tables)
  ) {
    fail("extended forms require a vb6r.dispatch-map.v2 export");
  }
  const extendedTables = dispatchMap.tables.filter(
    (table) => table.table_kind === "extended",
  );
  const forms = [];
  for (const table of extendedTables) {
    if (
      !/^0x[0-9a-f]{2}$/.test(table.lead_byte) ||
      !Array.isArray(table.entries) ||
      table.entries.length !== table.entry_count ||
      table.entries.length !== table.maximum_index + 1
    ) {
      fail(`invalid extended dispatch table for lead byte ${table.lead_byte}`);
    }
    for (const entry of table.entries) {
      forms.push({
        schema_version: "vb6r.opcode-form.v1",
        form_id: `OF-M1-${String(firstFormNumber + forms.length).padStart(4, "0")}`,
        binary_id: dispatchMap.binary_id,
        prefix: table.lead_byte,
        opcode: entry.opcode_byte,
        mnemonic: null,
        length_rule: "unknown",
        operand_schema: [],
        effect_status: "UNKNOWN",
        evidence_refs: entry.evidence_refs,
        implemented: false,
      });
    }
  }
  if (forms.some((form) => !validateOpcodeForm(form))) {
    fail("generated extended C4 record does not satisfy the opcode-form schema");
  }
  if (new Set(forms.map(opcodeFormIdentity)).size !== forms.length) {
    fail("generated extended C4 records contain duplicate prefix/opcode identities");
  }
  return forms;
}

function hex(value) {
  return `0x${value.toString(16)}`;
}

function main(inputPath) {
  const cardsPath = resolve(P_CODE_DIR, "evidence-cards.json");
  const scan = spawnSync(
    process.execPath,
    [resolve(SCRIPT_DIR, "scan-linear-lengths.mjs"), inputPath],
    { encoding: "utf8", maxBuffer: 64 * 1024 * 1024 },
  );
  if (scan.status !== 0) fail(scan.stderr || "linear handler scan failed");

  const scanResult = JSON.parse(scan.stdout);
  if (scanResult.binary_id !== BINARY_ID) fail("scanner returned an unexpected binary_id");
  const candidateByOpcode = new Map(
    scanResult.candidates.map((candidate) => [
      Number.parseInt(candidate.opcode, 16),
      candidate,
    ]),
  );
  const selected = SELECTED_OPCODES.map((opcode) => {
    const candidate = candidateByOpcode.get(opcode);
    if (!candidate || candidate.lengthBytes < 2) {
      fail(`no defensible multi-byte linear form found for opcode 0x${opcode.toString(16)}`);
    }
    return candidate;
  });

  const cards = JSON.parse(readFileSync(cardsPath, "utf8"));
  if (
    !Array.isArray(cards) ||
    ![10, 20].includes(cards.length) ||
    cards.slice(0, 10).some((card, index) =>
      card.evidence_id !== `EV-M1-B-${String(index + 1).padStart(4, "0")}`,
    ) ||
    (cards.length === 20 &&
      cards.slice(10).some((card, index) =>
        card.evidence_id !== `EV-M1-B-${String(index + 11).padStart(4, "0")}`,
      ))
  ) {
    fail("run dispatch-table.mjs first; expected its 10 base evidence cards or the 20-card combined export");
  }
  const baseCards = cards.slice(0, 10);
  const dispatchMap = JSON.parse(
    readFileSync(resolve(P_CODE_DIR, "dispatch-map.json"), "utf8"),
  );
  if (dispatchMap.binary_id !== BINARY_ID || dispatchMap.entries.length !== 256) {
    fail("dispatch-map.json does not contain the locked 256-entry table");
  }
  const dispatchByOpcode = new Map(
    dispatchMap.entries.map((entry) => [
      Number.parseInt(entry.opcode_byte, 16),
      entry,
    ]),
  );
  const unknownDispatchEntries = UNKNOWN_LENGTH_OPCODES.map((opcode) => {
    const entry = dispatchByOpcode.get(opcode);
    if (!entry || entry.binary_id !== BINARY_ID) {
      fail(`no locked dispatch entry found for opcode 0x${opcode.toString(16)}`);
    }
    return entry;
  });
  const extendedDispatchMap = JSON.parse(
    readFileSync(resolve(P_CODE_DIR, "dispatch-map-v2.json"), "utf8"),
  );
  if (
    extendedDispatchMap.binary_id !== BINARY_ID ||
    extendedDispatchMap.tables.length !== 6 ||
    extendedDispatchMap.tables[0].table_kind !== "primary"
  ) {
    fail("dispatch-map-v2.json does not contain the locked primary and five secondary tables");
  }
  const newCards = selected.map((candidate, index) => {
    const cardId = `EV-M1-B-${String(baseCards.length + index + 1).padStart(4, "0")}`;
    const trace = candidate.trace.map((instruction) => ({
      rva: hex(Number.parseInt(instruction.address, 16) - IMAGE_BASE),
      bytes: instruction.bytes,
      assembly: instruction.assembly,
    }));
    const last = candidate.trace.at(-1);
    const endRva =
      Number.parseInt(last.address, 16) - IMAGE_BASE + last.bytes.length / 2;
    return {
      evidence_id: cardId,
      kind: "static-handler-stream-advance",
      binary_id: BINARY_ID,
      opcode_byte: candidate.opcode,
      table_entry_rva: candidate.entryRva,
      handler_rva: candidate.handlerRva,
      location: {
        section: "ENGINE",
        start_rva: candidate.handlerRva,
        end_rva_exclusive: hex(endRva),
      },
      initial_esi_role:
        "Address immediately after the current opcode byte, as established by the primary dispatch sequence.",
      next_opcode_offset_from_initial_esi:
        candidate.nextOpcodeOffsetFromInitialEsi,
      handler_esi_advance_bytes: candidate.lengthBytes,
      form_length_bytes: candidate.lengthBytes,
      trace,
      observation:
        `A straight-line path reads the next dispatch byte at ESI+${candidate.nextOpcodeOffsetFromInitialEsi}, ` +
        `advances ESI by ${candidate.lengthBytes}, then transfers through the primary table. ` +
        `Given the primary dispatch's one-byte opcode consumption, this bounds the current form to ` +
        `${candidate.lengthBytes} byte(s), including its opcode.`,
      confidence: "STATIC_CONFIRMED",
      limits:
        "Static local path only; does not establish stream validity, mnemonic, operand value interpretation, stack effect, dynamic execution, or other paths through this handler.",
    };
  });

  const forms = selected.map((candidate, index) => ({
    schema_version: "vb6r.opcode-form.v1",
    form_id: `OF-M1-${String(index + 1).padStart(4, "0")}`,
    binary_id: BINARY_ID,
    prefix: null,
    opcode: candidate.opcode,
    mnemonic: null,
    length_rule: "known",
    operand_schema: [
      {
        kind: "opaque-bytes",
        offset_from_instruction_start: 1,
        length_bytes: candidate.lengthBytes - 1,
        endianness: "not_asserted",
      },
    ],
    effect_status: "UNKNOWN",
    evidence_refs: [
      "EV-M1-B-0001",
      "EV-M1-B-0002",
      newCards[index].evidence_id,
    ],
    implemented: true,
  }));
  for (const [index, entry] of unknownDispatchEntries.entries()) {
    forms.push({
      schema_version: "vb6r.opcode-form.v1",
      form_id: `OF-M1-${String(forms.length + 1).padStart(4, "0")}`,
      binary_id: BINARY_ID,
      prefix: null,
      opcode: entry.opcode_byte,
      mnemonic: null,
      length_rule: "unknown",
      operand_schema: [],
      effect_status: "UNKNOWN",
      evidence_refs: ["EV-M1-B-0001", "EV-M1-B-0002"],
      implemented: false,
    });
  }
  const extendedForms = buildUnknownExtendedForms(extendedDispatchMap, forms.length + 1);
  forms.push(...extendedForms);

  if (forms.some((form) => !validateOpcodeForm(form))) {
    fail("generated C4 record does not satisfy the opcode-form schema");
  }
  if (new Set(forms.map(opcodeFormIdentity)).size !== forms.length) {
    fail("generated C4 records contain duplicate prefix/opcode identities");
  }

  writeFileSync(cardsPath, `${JSON.stringify([...baseCards, ...newCards], null, 2)}\n`);
  writeFileSync(
    resolve(P_CODE_DIR, "opcode-forms.json"),
    `${JSON.stringify(forms, null, 2)}\n`,
  );
  process.stdout.write(
    `${JSON.stringify(
      {
        binary_id: BINARY_ID,
        selected_forms: forms.length,
        known_length_forms: forms.filter((form) => form.length_rule === "known").length,
        unknown_length_forms: forms.filter((form) => form.length_rule === "unknown").length,
        extended_dispatch_forms: extendedForms.length,
        lengths: forms.map((form) => ({
          opcode: form.opcode,
          length_bytes:
            form.length_rule === "known"
              ? selected.find((item) => item.opcode === form.opcode).lengthBytes
              : null,
          evidence_id: form.evidence_refs.at(-1),
        })),
        evidence_cards: baseCards.length + newCards.length,
        outputs: [
          resolve(P_CODE_DIR, "opcode-forms.json"),
          cardsPath,
        ],
      },
      null,
      2,
    )}\n`,
  );
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  try {
    main(resolve(process.argv[2] ?? DEFAULT_INPUT));
  } catch (error) {
    process.stderr.write(`${error.message}\n`);
    process.exitCode = 1;
  }
}
