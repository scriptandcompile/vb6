#!/usr/bin/env node

import { createHash } from "node:crypto";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const SCRIPT_DIR = dirname(fileURLToPath(import.meta.url));
const REPO_ROOT = resolve(SCRIPT_DIR, "../../..");
const DEFAULT_INPUT = resolve(
  REPO_ROOT,
  "reference/vb_symbols/vb6_ide/MSVBVM60.DLL",
);
const DEFAULT_OUTPUT = resolve(REPO_ROOT, "research/pcode");
const BINARY_ID =
  "sha256:e043c59c66ba2dc549bf5ac104c70279ec05931d531de2971458eab91871f7b6";
const TABLE_RVA = 0x106d14;
const TABLE_COUNT = 0x100;
const SECONDARY_INDEX_LIMITS = new Map([
  [0xfb, 0xff],
  [0xfc, 0xff],
  [0xfd, 0xff],
  [0xfe, 0xff],
  [0xff, 0x46],
]);
const POINTER_EVIDENCE_OPCODES = [0x00, 0x05, 0xfb];

function fail(message) {
  throw new Error(message);
}

function checkedRange(buffer, offset, length, label) {
  if (
    !Number.isSafeInteger(offset) ||
    !Number.isSafeInteger(length) ||
    offset < 0 ||
    length < 0 ||
    offset + length > buffer.length
  ) {
    fail(`${label} is outside the input image`);
  }
}

function readU16(buffer, offset, label) {
  checkedRange(buffer, offset, 2, label);
  return buffer.readUInt16LE(offset);
}

function readU32(buffer, offset, label) {
  checkedRange(buffer, offset, 4, label);
  return buffer.readUInt32LE(offset);
}

export function parsePe32(buffer) {
  checkedRange(buffer, 0, 0x40, "DOS header");
  if (buffer.toString("ascii", 0, 2) !== "MZ") fail("not an MZ image");

  const peOffset = readU32(buffer, 0x3c, "e_lfanew");
  checkedRange(buffer, peOffset, 24, "PE header");
  if (buffer.toString("ascii", peOffset, peOffset + 4) !== "PE\u0000\u0000") {
    fail("missing PE signature");
  }

  const machine = readU16(buffer, peOffset + 4, "COFF machine");
  const sectionCount = readU16(buffer, peOffset + 6, "section count");
  const optionalHeaderSize = readU16(buffer, peOffset + 20, "optional header size");
  const optionalHeader = peOffset + 24;
  checkedRange(buffer, optionalHeader, optionalHeaderSize, "optional header");
  if (readU16(buffer, optionalHeader, "optional header magic") !== 0x10b) {
    fail("input is not PE32");
  }

  const imageBase = readU32(buffer, optionalHeader + 28, "image base");
  const sectionTable = optionalHeader + optionalHeaderSize;
  checkedRange(buffer, sectionTable, sectionCount * 40, "section table");

  const sections = [];
  for (let index = 0; index < sectionCount; index += 1) {
    const offset = sectionTable + index * 40;
    sections.push({
      name: buffer
        .toString("ascii", offset, offset + 8)
        .replace(/\u0000.*$/, ""),
      virtualSize: readU32(buffer, offset + 8, "section virtual size"),
      rva: readU32(buffer, offset + 12, "section RVA"),
      rawSize: readU32(buffer, offset + 16, "section raw size"),
      rawOffset: readU32(buffer, offset + 20, "section raw offset"),
      characteristics: readU32(buffer, offset + 36, "section characteristics"),
    });
  }

  return { machine, imageBase, sections };
}

function sectionForRva(sections, rva, length = 1) {
  return (
    sections.find(
      (section) =>
        rva >= section.rva &&
        rva + length <= section.rva + section.rawSize,
    ) ?? null
  );
}

export function inspectDispatchTable(buffer, tableRva = TABLE_RVA, count = TABLE_COUNT) {
  const pe = parsePe32(buffer);
  const tableSection = sectionForRva(pe.sections, tableRva, count * 4);
  if (!tableSection) fail("dispatch table is not fully file-backed by one section");

  const tableFileOffset = tableSection.rawOffset + (tableRva - tableSection.rva);
  checkedRange(buffer, tableFileOffset, count * 4, "dispatch table");
  const entries = [];

  for (let opcode = 0; opcode < count; opcode += 1) {
    const entryRva = tableRva + opcode * 4;
    const entryFileOffset = tableFileOffset + opcode * 4;
    const handlerVa = readU32(buffer, entryFileOffset, "dispatch pointer");
    const handlerRva = handlerVa - pe.imageBase;
    const targetSection = sectionForRva(pe.sections, handlerRva);
    const targetFileOffset = targetSection
      ? targetSection.rawOffset + (handlerRva - targetSection.rva)
      : null;

    entries.push({
      opcode,
      entryRva,
      entryFileOffset,
      rawBytes: buffer.subarray(entryFileOffset, entryFileOffset + 4).toString("hex"),
      handlerVa,
      handlerRva,
      targetSection: targetSection?.name ?? null,
      targetFileOffset,
      targetExecutable: Boolean(
        targetSection && (targetSection.characteristics & 0x20000000) !== 0,
      ),
    });
  }

  return {
    machine: pe.machine,
    imageBase: pe.imageBase,
    sections: pe.sections,
    tableRva,
    tableEndRvaExclusive: tableRva + count * 4,
    tableCount: count,
    tableSection: tableSection.name,
    tableFileOffset,
    tableSha256: createHash("sha256")
      .update(buffer.subarray(tableFileOffset, tableFileOffset + count * 4))
      .digest("hex"),
    entries,
  };
}

function rvaFileOffset(sections, rva, length, label) {
  const section = sectionForRva(sections, rva, length);
  if (!section) fail(`${label} is not fully file-backed by one section`);
  return section.rawOffset + (rva - section.rva);
}

export function inspectSecondaryDispatches(buffer, primaryTable) {
  const pe = parsePe32(buffer);
  const dispatchers = [];

  for (const [leadByte, maximumIndex] of SECONDARY_INDEX_LIMITS) {
    const primaryEntry = primaryTable.entries[leadByte];
    if (!primaryEntry || !primaryEntry.targetExecutable) {
      fail(`primary slot 0x${leadByte.toString(16)} is not an executable dispatcher`);
    }

    const handlerRva = primaryEntry.handlerRva;
    const bounded = leadByte === 0xff;
    const handlerLength = bounded ? 21 : 12;
    const handlerFileOffset = rvaFileOffset(
      pe.sections,
      handlerRva,
      handlerLength,
      `lead-byte handler 0x${leadByte.toString(16)}`,
    );
    const handlerBytes = buffer
      .subarray(handlerFileOffset, handlerFileOffset + handlerLength);

    if (handlerBytes.subarray(0, 5).toString("hex") !== "33c08a0646") {
      fail(`lead-byte handler 0x${leadByte.toString(16)} does not read and consume one byte`);
    }

    let tableOperandOffset;
    let fallbackHandlerRva = null;
    if (bounded) {
      if (
        handlerBytes.subarray(5, 8).toString("hex") !== "83f846" ||
        handlerBytes.subarray(8, 10).toString("hex") !== "0f87" ||
        handlerBytes.subarray(14, 17).toString("hex") !== "ff2485"
      ) {
        fail("lead-byte handler 0xff does not match the bounded secondary-dispatch path");
      }
      const branchEndRva = handlerRva + 14;
      fallbackHandlerRva =
        branchEndRva + handlerBytes.readInt32LE(10);
      tableOperandOffset = 17;
    } else {
      if (handlerBytes.subarray(5, 8).toString("hex") !== "ff2485") {
        fail(`lead-byte handler 0x${leadByte.toString(16)} does not index a pointer table`);
      }
      tableOperandOffset = 8;
    }

    const tableVa = handlerBytes.readUInt32LE(tableOperandOffset);
    const tableRva = tableVa - pe.imageBase;
    if (!Number.isSafeInteger(tableRva) || tableRva < 0) {
      fail(`lead-byte handler 0x${leadByte.toString(16)} has an invalid table address`);
    }
    const table = inspectDispatchTable(buffer, tableRva, maximumIndex + 1);
    if (table.entries.some((entry) => !entry.targetExecutable)) {
      fail(`secondary table for lead byte 0x${leadByte.toString(16)} has an unmapped target`);
    }

    if (bounded) {
      const fallbackSection = sectionForRva(pe.sections, fallbackHandlerRva);
      if (!fallbackSection || !(fallbackSection.characteristics & 0x20000000)) {
        fail("lead-byte handler 0xff fallback is not executable");
      }
    }

    dispatchers.push({
      leadByte,
      primaryEntry,
      handlerRva,
      handlerFileOffset,
      handlerBytes: handlerBytes.toString("hex"),
      indexWidthBits: 8,
      maximumIndex,
      fallbackHandlerRva,
      table,
    });
  }

  return dispatchers;
}

function hex(value) {
  return `0x${value.toString(16)}`;
}

function makeEvidenceCards(image, table, secondaryDispatches) {
  const cards = [
    {
      evidence_id: "EV-M1-B-0001",
      kind: "static-dispatch-instruction-sequence",
      binary_id: BINARY_ID,
      location: { section: "ENGINE", rva: "0x104f4f", length_bytes: 10 },
      bytes: "8a0646ff2485146d1066",
      observation:
        "Loads one byte from [ESI] into AL, increments ESI by one, and jumps through [4*EAX + 0x66106d14].",
      confidence: "STATIC_CONFIRMED",
      limits:
        "Static dispatch mechanics only; does not identify opcode meanings, stream format outside this path, or runtime execution.",
    },
    {
      evidence_id: "EV-M1-B-0002",
      kind: "dispatch-table-range-and-pointer-check",
      binary_id: BINARY_ID,
      location: {
        section: table.tableSection,
        rva: hex(table.tableRva),
        end_rva_exclusive: hex(table.tableEndRvaExclusive),
        file_offset: hex(table.tableFileOffset),
      },
      table_count: table.tableCount,
      table_sha256: table.tableSha256,
      executable_target_count: table.entries.filter((entry) => entry.targetExecutable).length,
      non_executable_or_unmapped_target_count: table.entries.filter(
        (entry) => !entry.targetExecutable,
      ).length,
      observation:
        "Interpreting AL as an unsigned byte indexes 256 consecutive 4-byte entries; every entry is file-backed and every decoded target is in a section marked executable.",
      confidence: "STATIC_CONFIRMED",
      limits:
        "The count follows the unsigned-byte index and scale-4 instruction plus complete mapped target checks; no source-level opcode validity or semantic classification is asserted.",
    },
  ];

  for (const opcode of POINTER_EVIDENCE_OPCODES) {
    const entry = table.entries[opcode];
    cards.push({
      evidence_id: `EV-M1-B-${String(cards.length + 1).padStart(4, "0")}`,
      kind: "dispatch-table-pointer",
      binary_id: BINARY_ID,
      opcode_byte: `0x${entry.opcode.toString(16).padStart(2, "0")}`,
      entry_rva: hex(entry.entryRva),
      entry_file_offset: hex(entry.entryFileOffset),
      bytes: entry.rawBytes,
      handler_va: hex(entry.handlerVa),
      handler_rva: hex(entry.handlerRva),
      target_section: entry.targetSection,
      target_file_offset: hex(entry.targetFileOffset),
      confidence: "STATIC_CONFIRMED",
      limits:
        "Pointer mapping only; table slot does not prove that the opcode occurs in a valid VB6 stream or establish its meaning.",
    });
  }

  for (const dispatcher of secondaryDispatches) {
    const table = dispatcher.table;
    cards.push({
      evidence_id: `EV-M1-B-${String(cards.length + 1).padStart(4, "0")}`,
      kind: "static-secondary-dispatch-and-table",
      binary_id: BINARY_ID,
      lead_byte: `0x${dispatcher.leadByte.toString(16).padStart(2, "0")}`,
      primary_entry_rva: hex(dispatcher.primaryEntry.entryRva),
      handler_rva: hex(dispatcher.handlerRva),
      handler_bytes: dispatcher.handlerBytes,
      table_rva: hex(table.tableRva),
      table_end_rva_exclusive: hex(table.tableEndRvaExclusive),
      table_file_offset: hex(table.tableFileOffset),
      table_entry_count: table.tableCount,
      table_sha256: table.tableSha256,
      index_width_bits: dispatcher.indexWidthBits,
      maximum_index: dispatcher.maximumIndex,
      fallback_handler_rva:
        dispatcher.fallbackHandlerRva === null
          ? null
          : hex(dispatcher.fallbackHandlerRva),
      executable_target_count: table.entries.filter((entry) => entry.targetExecutable).length,
      observation:
        dispatcher.fallbackHandlerRva === null
          ? `The primary slot selects a handler that reads one byte at ESI, advances ESI by one, and dispatches that byte through this table.`
          : `The primary slot selects a handler that reads one byte at ESI, advances ESI by one, dispatches indices 0..0x46 through this table, and branches larger values to the fallback handler.`,
      confidence: "STATIC_CONFIRMED",
      limits:
        "Confirms a two-stage static dispatch path only; it does not establish that any byte pair occurs in a valid P-Code stream, or prove instruction length, mnemonic, operands, or runtime behavior.",
    });
  }

  return cards;
}

function makeC2Map(table) {
  return {
    schema_version: "vb6r.dispatch-map.v1",
    binary_id: BINARY_ID,
    table: {
      table_kind: "primary",
      base_rva: hex(table.tableRva),
      end_rva_exclusive: hex(table.tableEndRvaExclusive),
      entry_width_bytes: 4,
      index_width_bits: 8,
      evidence_refs: ["EV-M1-B-0001", "EV-M1-B-0002"],
    },
    entries: table.entries.map((entry) => ({
      schema_version: "vb6r.handler.v1",
      binary_id: BINARY_ID,
      table_kind: "primary",
      lead_byte: null,
      opcode_byte: `0x${entry.opcode.toString(16).padStart(2, "0")}`,
      entry_rva: hex(entry.entryRva),
      handler_rva: hex(entry.handlerRva),
      mnemonic: null,
      symbol_name: null,
      confidence: "STATIC_CONFIRMED",
      evidence_refs: [
        "EV-M1-B-0001",
        "EV-M1-B-0002",
        ...(POINTER_EVIDENCE_OPCODES.includes(entry.opcode)
          ? [
              `EV-M1-B-${String(
                3 + POINTER_EVIDENCE_OPCODES.indexOf(entry.opcode),
              ).padStart(4, "0")}`,
            ]
          : []),
      ],
      notes:
        `Observed table-slot-to-pointer mapping; target section=${entry.targetSection ?? "UNMAPPED"}, ` +
        `executable=${entry.targetExecutable}. No opcode meaning or stream-validity claim.`,
    })),
  };
}

function makeDispatchMapV2(primaryTable, secondaryDispatches) {
  const pointerEntry = (entry, tableKind, leadByte, evidenceRefs) => ({
    schema_version: "vb6r.handler.v1",
    binary_id: BINARY_ID,
    table_kind: tableKind,
    lead_byte: leadByte === null ? null : `0x${leadByte.toString(16).padStart(2, "0")}`,
    opcode_byte: `0x${entry.opcode.toString(16).padStart(2, "0")}`,
    entry_rva: hex(entry.entryRva),
    handler_rva: hex(entry.handlerRva),
    mnemonic: null,
    symbol_name: null,
    confidence: "STATIC_CONFIRMED",
    evidence_refs: evidenceRefs,
    notes:
      `Observed ${tableKind} table-slot-to-pointer mapping; target section=${entry.targetSection}, ` +
      "executable=true. No opcode meaning or valid-stream claim.",
  });
  const tables = [
    {
      table_kind: "primary",
      lead_byte: null,
      base_rva: hex(primaryTable.tableRva),
      end_rva_exclusive: hex(primaryTable.tableEndRvaExclusive),
      entry_width_bytes: 4,
      index_width_bits: 8,
      maximum_index: 0xff,
      entry_count: primaryTable.tableCount,
      evidence_refs: ["EV-M1-B-0001", "EV-M1-B-0002"],
      entries: primaryTable.entries.map((entry) =>
        pointerEntry(
          entry,
          "primary",
          null,
          [
            "EV-M1-B-0001",
            "EV-M1-B-0002",
            ...(POINTER_EVIDENCE_OPCODES.includes(entry.opcode)
              ? [
                  `EV-M1-B-${String(
                    3 + POINTER_EVIDENCE_OPCODES.indexOf(entry.opcode),
                  ).padStart(4, "0")}`,
                ]
              : []),
          ],
        ),
      ),
    },
    ...secondaryDispatches.map((dispatcher) => {
      const table = dispatcher.table;
      const evidenceId = `EV-M1-B-${String(6 + secondaryDispatches.indexOf(dispatcher)).padStart(4, "0")}`;
      return {
        table_kind: "extended",
        lead_byte: `0x${dispatcher.leadByte.toString(16).padStart(2, "0")}`,
        base_rva: hex(table.tableRva),
        end_rva_exclusive: hex(table.tableEndRvaExclusive),
        entry_width_bytes: 4,
        index_width_bits: dispatcher.indexWidthBits,
        maximum_index: dispatcher.maximumIndex,
        entry_count: table.tableCount,
        fallback_handler_rva:
          dispatcher.fallbackHandlerRva === null
            ? null
            : hex(dispatcher.fallbackHandlerRva),
        evidence_refs: ["EV-M1-B-0001", "EV-M1-B-0002", evidenceId],
        entries: table.entries.map((entry) =>
          pointerEntry(entry, "extended", dispatcher.leadByte, [
            "EV-M1-B-0001",
            evidenceId,
          ]),
        ),
      };
    }),
  ];
  return {
    schema_version: "vb6r.dispatch-map.v2",
    binary_id: BINARY_ID,
    description:
      "Build-specific primary and statically reached secondary dispatch slots; mappings do not establish valid-stream use or opcode semantics.",
    tables,
  };
}

function main(args) {
  const inputPath = resolve(args[0] ?? DEFAULT_INPUT);
  const outputDir = resolve(args[1] ?? DEFAULT_OUTPUT);
  const image = readFileSync(inputPath);
  const sha256 = createHash("sha256").update(image).digest("hex");
  if (`sha256:${sha256}` !== BINARY_ID) {
    fail(`input digest ${sha256} does not match locked binary_id ${BINARY_ID}`);
  }

  const table = inspectDispatchTable(image);
  if (table.entries.length !== 256) fail("expected exactly 256 byte-indexed entries");
  const secondaryDispatches = inspectSecondaryDispatches(image, table);

  const c2 = makeC2Map(table);
  const c2V2 = makeDispatchMapV2(table, secondaryDispatches);
  const cards = makeEvidenceCards(image, table, secondaryDispatches);
  mkdirSync(outputDir, { recursive: true });
  writeFileSync(`${outputDir}/dispatch-map.json`, `${JSON.stringify(c2, null, 2)}\n`);
  writeFileSync(`${outputDir}/dispatch-map-v2.json`, `${JSON.stringify(c2V2, null, 2)}\n`);
  writeFileSync(`${outputDir}/evidence-cards.json`, `${JSON.stringify(cards, null, 2)}\n`);
  process.stdout.write(
    `${JSON.stringify({
      binary_id: BINARY_ID,
      input_bytes: image.length,
      table_rva: hex(table.tableRva),
      table_entries: table.tableCount,
      executable_targets: table.entries.filter((entry) => entry.targetExecutable).length,
      non_executable_or_unmapped_targets: table.entries.filter(
        (entry) => !entry.targetExecutable,
      ).length,
      table_sha256: table.tableSha256,
      secondary_tables: secondaryDispatches.map((dispatcher) => ({
        lead_byte: `0x${dispatcher.leadByte.toString(16).padStart(2, "0")}`,
        table_rva: hex(dispatcher.table.tableRva),
        entry_count: dispatcher.table.tableCount,
        maximum_index: dispatcher.maximumIndex,
        table_sha256: dispatcher.table.tableSha256,
      })),
      evidence_cards: cards.length,
      outputs: [
        `${outputDir}/dispatch-map.json`,
        `${outputDir}/dispatch-map-v2.json`,
        `${outputDir}/evidence-cards.json`,
      ],
    }, null, 2)}\n`,
  );
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  try {
    main(process.argv.slice(2));
  } catch (error) {
    process.stderr.write(`${error.message}\n`);
    process.exitCode = 1;
  }
}
