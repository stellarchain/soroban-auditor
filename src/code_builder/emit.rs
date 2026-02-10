use crate::wasm_ir::{BlockKind, Indentation};
use std::io::Write;

pub fn emit_br_table_arm<W: Write>(
    writer: &mut W,
    indentation: &Indentation,
    index: usize,
    target: Option<&BlockKind>,
) {
    match target {
        Some(BlockKind::Block { label, .. }) | Some(BlockKind::If { label, .. }) => {
            if label.is_some() {
                writeln!(
                    writer,
                    "{}{} => break 'label{},",
                    indentation,
                    index,
                    label.unwrap()
                )
                .unwrap();
            } else {
                writeln!(writer, "{}{} => break,", indentation, index).unwrap();
            }
        }
        Some(BlockKind::Loop { label, .. }) => {
            if label.is_some() {
                writeln!(
                    writer,
                    "{}{} => continue 'label{},",
                    indentation,
                    index,
                    label.unwrap()
                )
                .unwrap();
            } else {
                writeln!(writer, "{}{} => continue,", indentation, index).unwrap();
            }
        }
        Some(BlockKind::Function { .. }) | None => {
            writeln!(writer, "{}{} => return,", indentation, index).unwrap();
        }
    }
}

pub fn emit_br_table_default<W: Write>(
    writer: &mut W,
    indentation: &Indentation,
    target: Option<&BlockKind>,
) {
    match target {
        Some(BlockKind::Block { label, .. }) | Some(BlockKind::If { label, .. }) => {
            if label.is_some() {
                writeln!(writer, "{}_ => break 'label{},", indentation, label.unwrap()).unwrap();
            } else {
                writeln!(writer, "{}_ => break,", indentation).unwrap();
            }
        }
        Some(BlockKind::Loop { label, .. }) => {
            if label.is_some() {
                writeln!(
                    writer,
                    "{}_ => continue 'label{},",
                    indentation,
                    label.unwrap()
                )
                .unwrap();
            } else {
                writeln!(writer, "{}_ => continue,", indentation).unwrap();
            }
        }
        Some(BlockKind::Function { .. }) | None => {
            writeln!(writer, "{}_ => return,", indentation).unwrap();
        }
    }
}
