use std::{env, fs, path::PathBuf};

fn csv_rows(input: &str) -> Vec<Vec<String>> {
    let mut rows = Vec::new();
    let mut row = Vec::new();
    let mut field = String::new();
    let mut quoted = false;
    let mut chars = input.chars().peekable();
    while let Some(ch) = chars.next() {
        match ch {
            '"' if quoted && chars.peek() == Some(&'"') => {
                field.push('"');
                chars.next();
            }
            '"' => quoted = !quoted,
            ',' if !quoted => row.push(std::mem::take(&mut field)),
            '\n' if !quoted => {
                row.push(std::mem::take(&mut field));
                if row.iter().any(|value| !value.is_empty()) {
                    rows.push(std::mem::take(&mut row));
                } else {
                    row.clear();
                }
            }
            '\r' if !quoted => {}
            _ => field.push(ch),
        }
    }
    if !field.is_empty() || !row.is_empty() {
        row.push(field);
        rows.push(row);
    }
    rows
}

fn value(row: &[String], index: usize) -> &str {
    row.get(index).map(String::as_str).unwrap_or("")
}

fn main() {
    println!("cargo:rerun-if-changed=data/opcodes.csv");
    println!("cargo:rerun-if-changed=data/OPCODE-MATRIX.csv");
    let opcodes = fs::read_to_string("data/opcodes.csv").expect("read opcode catalog");
    let matrix = fs::read_to_string("data/OPCODE-MATRIX.csv").expect("read M1 matrix");
    let opcode_rows = csv_rows(&opcodes);
    let matrix_rows = csv_rows(&matrix);
    let header = opcode_rows
        .iter()
        .position(|row| value(row, 0) == "table")
        .expect("opcode CSV header");
    let fields: Vec<_> = opcode_rows[header]
        .iter()
        .enumerate()
        .map(|(index, name)| (name.as_str(), index))
        .collect();
    let col = |name: &str| fields.iter().find(|(key, _)| *key == name).unwrap().1;
    let mut entries = Vec::new();
    for row in opcode_rows.iter().skip(header + 1) {
        let table: usize = value(row, col("table")).parse().expect("table index");
        let opcode = u8::from_str_radix(value(row, col("opcode")).trim_start_matches("0x"), 16)
            .expect("opcode byte");
        entries.push((table, opcode, row));
    }
    assert_eq!(entries.len(), 1536, "catalog must have six complete tables");
    let mut matrix_by_slot = vec![None; 1536];
    let mheader = matrix_rows
        .iter()
        .position(|row| value(row, 0) == "dispatch_table")
        .expect("M1 matrix header");
    for row in matrix_rows.iter().skip(mheader + 1) {
        let table: usize = value(row, 0).parse().expect("M1 table index");
        let opcode =
            u8::from_str_radix(value(row, 2).trim_start_matches("0x"), 16).expect("M1 opcode byte");
        let slot = table * 256 + usize::from(opcode);
        matrix_by_slot[slot] = Some((value(row, 4).to_owned(), value(row, 21).to_owned()));
    }
    assert!(matrix_by_slot.iter().all(Option::is_some));

    let mut generated = String::from("pub static TABLES: [[OpcodeInfo; 256]; 6] = [\n");
    for table in 0..6 {
        generated.push_str("[\n");
        for opcode in 0..256 {
            let row = entries
                .iter()
                .find(|(t, op, _)| *t == table && usize::from(*op) == opcode)
                .expect("all table slots present")
                .2;
            let slot = matrix_by_slot[table * 256 + opcode].as_ref().unwrap();
            generated.push_str("OpcodeInfo { ");
            for (field_name, csv_name) in [
                ("size", "size"),
                ("mnemonic", "mnemonic"),
                ("operand_format", "operand_format"),
                ("pops", "pops"),
                ("pushes", "pushes"),
                ("fpu_pops", "fpu_pops"),
                ("fpu_push", "fpu_push"),
                ("stack_effect", "stack"),
                ("popped", "popped"),
                ("memory_effect", "object"),
                ("movement", "movement"),
                ("category", "category"),
                ("handler_8176", "handler_8176"),
                ("handler_9848", "handler_9848"),
                ("notes", "notes"),
            ] {
                let idx = col(csv_name);
                if field_name == "size" || ["pops", "pushes", "fpu_pops"].contains(&field_name) {
                    let n: i16 = value(row, idx).parse().expect("numeric opcode metadata");
                    generated.push_str(&format!("{field_name}: {n}, "));
                } else {
                    generated.push_str(&format!("{field_name}: {:?}, ", value(row, idx)));
                }
            }
            generated.push_str(&format!(
                "table: {table}, opcode: {opcode}, slot_class: {:?}, m1_classification: {:?} }},\n",
                slot.0, slot.1
            ));
        }
        generated.push_str("],\n");
    }
    generated.push_str("];\n");
    let out = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR"));
    fs::write(out.join("catalog_generated.rs"), generated).expect("write generated catalog");
}
