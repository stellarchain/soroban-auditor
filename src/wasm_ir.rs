use parity_wasm::elements::{FunctionType, Instruction, ValueType};
use std::fmt;
use unicode_xid::UnicodeXID;
use unidecode::unidecode_char;

pub struct Function<'a> {
    pub name: String,
    #[allow(dead_code)]
    pub module_name: String,
    pub ty: &'a FunctionType,
    #[allow(dead_code)]
    pub ty_index: u32,
    pub real_name: Option<&'a String>,
    pub make_public: bool,
}

pub struct Global<'a> {
    pub is_mutable: bool,
    pub is_pub: bool,
    pub name: String,
    #[allow(dead_code)]
    pub ty: &'static str,
    #[allow(dead_code)]
    pub const_expr: &'a [Instruction],
}

#[derive(Debug)]
pub enum BlockKind {
    Function {
        evaluates_to_value: bool,
    },
    Block {
        label: Option<usize>,
        dst_var: Option<String>,
    },
    If {
        label: Option<usize>,
        dst_var: Option<String>,
        is_breakable: bool,
    },
    Loop {
        label: Option<usize>,
        dst_var: Option<String>,
    },
}

pub fn to_rs_type(t: ValueType) -> &'static str {
    match t {
        ValueType::I32 => "i32",
        ValueType::I64 => "i64",
        ValueType::F32 => "f32",
        ValueType::F64 => "f64",
    }
}

pub struct Indentation(pub usize);

impl fmt::Display for Indentation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for _ in 0..self.0 {
            write!(f, "    ")?
        }
        Ok(())
    }
}

pub fn mangle_fn_name(name: &str) -> String {
    let mut s = String::new();
    for (i, c) in name.chars().enumerate() {
        if i == 0 {
            if UnicodeXID::is_xid_start(c) {
                s.push(c);
                continue;
            }
            s.push('_');
        }

        if UnicodeXID::is_xid_continue(c) {
            s.push(c);
            continue;
        }

        let decoded = unidecode_char(c);
        if decoded == "[?]" {
            if s.chars().last() != Some('_') {
                s.push('_');
            }
            continue;
        }

        for c in decoded.chars() {
            if UnicodeXID::is_xid_continue(c) {
                s.push(c);
            } else if s.chars().last() != Some('_') {
                s.push('_');
            }
        }
    }
    s
}

pub fn call_indirect_name(f: &FunctionType) -> String {
    let mut s = String::from("call_indirect_");
    for param in f.params() {
        s.push_str(match *param {
            ValueType::I32 => "i",
            ValueType::I64 => "l",
            ValueType::F32 => "f",
            ValueType::F64 => "d",
        });
    }
    for result in f.results() {
        s.push_str(match result {
            ValueType::I32 => "_i",
            ValueType::I64 => "_l",
            ValueType::F32 => "_f",
            ValueType::F64 => "_d",
        });
    }

    if f.results().is_empty() {
        s.push_str("_v");
    }
    s
}
