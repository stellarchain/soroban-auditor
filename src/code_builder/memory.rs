use std::io::{Result as IoResult, Write};

pub fn mem_addr(addr: &str, offset: u32) -> String {
    if offset == 0 {
        format!("{addr} as usize")
    } else {
        format!("{addr} as usize + {offset}")
    }
}

pub fn emit_store_line<W: Write>(
    writer: &mut W,
    indentation: impl std::fmt::Display,
    macro_name: &str,
    addr: &str,
    offset: u32,
    value: &str,
    cast_suffix: &str,
) -> IoResult<()> {
    writeln!(
        writer,
        "{}{}!({}, {}{});",
        indentation,
        macro_name,
        mem_addr(addr, offset),
        value,
        cast_suffix
    )
}
