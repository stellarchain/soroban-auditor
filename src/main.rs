extern crate parity_wasm;
extern crate structopt;
extern crate unicode_xid;
extern crate unidecode;

use crate::soroban::contract::find_function_specs;
use parity_wasm::deserialize_file;
use parity_wasm::elements::{
    External, FunctionType, ImportCountType, Instruction, Internal, Section, Type, ValueType,
};
use serde_json::Value;
use soroban::common::{env_common_modules_result, take_common_module, take_common_module_by_name};
use soroban::contract::{read_contract_specs, FunctionContractSpec};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use structopt::StructOpt;
use unicode_xid::UnicodeXID;
use unidecode::unidecode_char;

mod code_builder;
mod expr_builder;
mod precedence;
mod reorder_analysis;
mod soroban;

#[derive(StructOpt)]
struct Opt {
    #[structopt(
        short = "n",
        long = "use-name-section",
        help = "Use the names in the name section for the internal function names"
    )]
    use_name_section: bool,
    #[structopt(
        short = "c",
        long = "public-call-indirect",
        help = "Make indirect calling available in the API"
    )]
    public_call_indirect: bool,
    #[structopt(help = "Input file", parse(from_os_str))]
    input: PathBuf,
    #[structopt(
        help = "Output file, stored next to wasm file if not specified",
        parse(from_os_str)
    )]
    output: Option<PathBuf>,
}

pub struct Function<'a> {
    name: String,
    module_name: String,
    ty: &'a FunctionType,
    ty_index: u32,
    real_name: Option<&'a String>,
    make_public: bool,
}

pub struct Global<'a> {
    is_mutable: bool,
    is_pub: bool,
    name: String,
    ty: &'static str,
    const_expr: &'a [Instruction],
}

#[derive(Debug)]
pub enum BlockKind {
    Function {
        evaluates_to_value: bool,
    },
    Block {
        label: usize,
        dst_var: Option<String>,
    },
    If {
        label: usize,
        dst_var: Option<String>,
        is_breakable: bool,
    },
    Loop {
        label: usize,
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

use std::fmt;

struct Indentation(usize);

impl fmt::Display for Indentation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for _ in 0..self.0 {
            write!(f, "    ")?
        }
        Ok(())
    }
}

fn mangle_fn_name(name: &str) -> String {
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
            } else {
                if s.chars().last() != Some('_') {
                    s.push('_');
                }
            }
        }
    }
    s
}

fn call_indirect_name(f: &FunctionType) -> String {
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

fn main() {
    let opt = Opt::from_args();
    let input = opt.input;
    let output = opt.output.unwrap_or_else(|| input.with_extension("rs"));
    let modules = match env_common_modules_result() {
        Ok(modules) => modules,
        Err(err) => {
            eprintln!("Error: {}", err);
            panic!("Error loading modules soroban.")
        }
    };
    let module = deserialize_file(&input).unwrap();
    let module = module.parse_names().unwrap_or_else(|(_, m)| m);

    let mut writer = BufWriter::new(File::create(output).expect("Couldn't create output file"));

    let import_count = module.import_count(ImportCountType::Function);
    let code = module.code_section().unwrap();
    let fns = module.function_section().unwrap();
    let types = module.type_section().unwrap();
    let exports = module.export_section().unwrap();

    //name sections
    let function_names = module
        .sections()
        .iter()
        .filter_map(|s| match s {
            Section::Name(name_section) => name_section.functions(),
            _ => None,
        })
        .next();

    let mut functions = Vec::new();

    if let Some(imports) = module.import_section() {
        for import in imports.entries() {
            // TODO Handle modules
            if let &External::Function(ty_index) = import.external() {
                let typ: &Type = &types.types()[ty_index as usize];
                let fn_type = match *typ {
                    Type::Function(ref t) => t,
                };
                let module_import = take_common_module(
                    &modules,
                    import.module().to_owned().as_str(),
                    import.field().to_owned().as_str(),
                );
                let (field_name, module_name) = match module_import {
                    Some(module) => (module.function_name, module.module_name),
                    None => (import.field().to_owned(), String::new()),
                };

                functions.push(Function {
                    name: field_name,
                    module_name,
                    ty: fn_type,
                    ty_index,
                    real_name: None,
                    make_public: false,
                });
            }
        }
    }

    for function in fns.entries() {
        let ty_index = function.type_ref();
        let Type::Function(ref fn_type) = types.types()[ty_index as usize];
        let mut real_name = function_names.and_then(|f| f.names().get(functions.len() as _));
        let mut name = format!("func{}", functions.len());
        if opt.use_name_section {
            if let Some(real_name) = real_name.take() {
                name = real_name.to_string();
                while functions.iter().any(|f| f.name == name) {
                    name.push_str("_");
                }
            }
        }
        functions.push(Function {
            name,
            module_name: String::new(),
            ty: fn_type,
            ty_index,
            real_name,
            make_public: false,
        });
    }

    for function in &mut functions {
        function.name = mangle_fn_name(&function.name);
    }

    writeln!(
        writer,
        "#![no_std]
use soroban_sdk::{{contract, contractimpl, contracttype, token, Address, Env, Vec}};

#[contract]
pub struct Contract;"
    )
    .unwrap();

    let verbose = false;
    let mut indirect_fns = BTreeMap::new();
    let contract_specs = match read_contract_specs(&input) {
        Ok(spec_fns_result) => spec_fns_result,
        Err(_err) => Vec::new(),
    };

    if verbose {
        for function in &functions[..import_count] {
            write!(
                writer,
                "    fn {}(&mut self, context: &mut Context<Self::Memory>",
                function.name
            )
            .unwrap();

            let module_import = take_common_module_by_name(
                &modules,
                function.module_name.as_str(),
                function.name.as_str(),
            );

            let (arguments, return_type) = match module_import {
                Some(module) => {
                    let args = module.function.as_object().and_then(|obj| obj.get("args"));
                    let ret_type = module
                        .function
                        .as_object()
                        .and_then(|obj| obj.get("return"));
                    let new_args = match args {
                        Some(args_value) => {
                            if let Some(args_array) = args_value.as_array() {
                                args_array.to_vec()
                            } else {
                                Vec::new()
                            }
                        }
                        None => Vec::new(),
                    };

                    let return_type_str = match ret_type {
                        Some(val) => val.as_str().unwrap_or_default().to_string(),
                        None => String::new(),
                    };

                    (new_args, return_type_str)
                }
                None => (Vec::new(), String::new()),
            };

            for (i, &param) in function.ty.params().iter().enumerate() {
                if let Some(argument) = arguments.get(i).and_then(|arg| arg.as_object()) {
                    if let (Some(name), Some(type_str)) = (
                        argument.get("name").and_then(Value::as_str),
                        argument.get("type").and_then(Value::as_str),
                    ) {
                        write!(writer, ", {}: {}", name, type_str).unwrap();
                        continue;
                    }
                } else {
                    write!(writer, ", var{}: {}", i, to_rs_type(param)).unwrap();
                }
            }
            write!(writer, ")").unwrap();
            if !return_type.is_empty() {
                write!(writer, " -> {}", return_type).unwrap();
            } else {
                for result in function.ty.results() {
                    write!(writer, " -> {}", to_rs_type(*result)).unwrap();
                }
            }
            writeln!(writer, ";").unwrap();
        }
    }

    let mut globals = Vec::new();

    if let Some(imports) = module.import_section() {
        for import in imports.entries() {
            if let &External::Global(ty) = import.external() {
                let name = import.field().to_string();
                globals.push(Global {
                    is_mutable: ty.is_mutable(),
                    is_pub: true, // Doesn't really apply
                    name,
                    ty: to_rs_type(ty.content_type()),
                    const_expr: &[],
                });
            }
        }
    }

    let imported_globals_count = globals.len();

    if let Some(global_section) = module.global_section() {
        for entry in global_section.entries() {
            let ty = entry.global_type();
            let const_expr = entry.init_expr().code();
            let is_mutable =
                ty.is_mutable() || is_const_expr_immutable_instead_of_const(const_expr);
            let name = if is_mutable {
                format!("global{}", globals.len())
            } else {
                format!("GLOBAL{}", globals.len())
            };
            globals.push(Global {
                is_mutable,
                is_pub: false,
                name,
                ty: to_rs_type(ty.content_type()),
                const_expr,
            });
        }
    }

    for export in exports.entries() {
        if let &Internal::Global(global_index) = export.internal() {
            let global = &mut globals[global_index as usize];
            global.name = export.field().to_string();
            global.is_pub = true;
        }
    }

    let mut has_dynamic_element_section_offset = false;
    if let Some(entry) = module.elements_section().and_then(|e| e.entries().get(0)) {
        let const_expr = entry.offset().as_ref().unwrap().code();
        if is_const_expr_immutable_instead_of_const(const_expr) {
            writeln!(writer, "    element_section_offset: i32,").unwrap();
            has_dynamic_element_section_offset = true;
        }
    }

    if verbose {
        for global in &globals[..imported_globals_count] {
            // if global.is_mutable {
            writeln!(
                writer,
                "    fn {}(&mut self, context: &mut Context<Self::Memory>) -> &mut {};",
                global.name, global.ty
            )
            .unwrap();
            // } else {
            //     writeln!(writer, "    const {}: {};", global.name, global.ty);
            // }
        }

        writeln!(
            writer,
            "{}",
            r#"}

    pub trait Memory {
        fn load8(&mut self, addr: usize) -> u8;
        fn load16(&mut self, addr: usize) -> u16;
        fn load32(&mut self, addr: usize) -> u32;
        fn load64(&mut self, addr: usize) -> u64;

        fn store8(&mut self, addr: usize, val: u8);
        fn store16(&mut self, addr: usize, val: u16);
        fn store32(&mut self, addr: usize, val: u32);
        fn store64(&mut self, addr: usize, val: u64);

        fn store_slice(&mut self, addr: usize, val: &'static [u8]);

        fn grow(&mut self, pages: usize) -> i32;
        fn size(&mut self) -> i32;
    }

    #[contract]
    pub struct Contract<I: Imports<Memory = M>, M: Memory> {
        pub imports: I,
        pub context: Context<M>,
    }

    #[contracttype]
    pub struct Context<M: Memory> {
        pub memory: M,"#
        )
        .unwrap();

        for global in &globals[imported_globals_count..] {
            if global.is_mutable {
                write!(writer, "    ").unwrap();
                if global.is_pub {
                    write!(writer, "pub ").unwrap();
                }
                writeln!(writer, "{}: {},", global.name, global.ty).unwrap();
            }
        }

        if globals[imported_globals_count..]
            .iter()
            .any(|g| !g.is_mutable)
        {
            writeln!(
                writer,
                "{}",
                r#"}

    #[contracttype]
    pub mod Consts {"#
            )
            .unwrap();

            for global in &globals[imported_globals_count..] {
                if !global.is_mutable {
                    write!(writer, "    pub").unwrap();
                    if !global.is_pub {
                        write!(writer, "(super)").unwrap();
                    }
                    write!(writer, " const {}: {} = ", global.name, global.ty).unwrap();
                    write_const_expr(
                        &mut writer,
                        global.const_expr,
                        &globals,
                        imported_globals_count,
                        "",
                        "",
                        "",
                    );
                    writeln!(writer, ";").unwrap();
                }
            }
        }

        writeln!(
            writer,
            "{}",
            r#"}

#[contractimpl]
impl Contract {"#
        )
        .unwrap();
        if let Some(memory) = module.memory_section().and_then(|m| m.entries().first()) {
            writeln!(
                writer,
                r#"pub fn new(imports: I, mut memory: M) -> Self {{
            let current_pages = memory.size() as usize;
            if current_pages < {0} {{
                memory.grow({0} - current_pages);
                assert_eq!(memory.size(), {0}, "Not enough memory pages allocated");
            }}"#,
                memory.limits().initial()
            )
            .unwrap();
        }

        writeln!(
            writer,
            "{}",
            r#"        let mut instance = Self {
                imports,
                context: Context {
                    memory,"#
        )
        .unwrap();

        for global in &globals[imported_globals_count..] {
            if global.is_mutable {
                write!(writer, "                {}: ", global.name).unwrap();
                if is_const_expr_immutable_instead_of_const(global.const_expr) {
                    write!(writer, "Default::default()").unwrap();
                } else {
                    write_const_expr(
                        &mut writer,
                        global.const_expr,
                        &globals,
                        imported_globals_count,
                        "consts::",
                        "",
                        "",
                    );
                }
                writeln!(writer, ",").unwrap();
            }
        }

        if has_dynamic_element_section_offset {
            writeln!(writer, "                element_section_offset: 0,").unwrap();
        }

        writeln!(
            writer,
            "{}",
            r#"            },
            };"#
        )
        .unwrap();

        for global in &globals[imported_globals_count..] {
            if global.is_mutable && is_const_expr_immutable_instead_of_const(global.const_expr) {
                write!(writer, "        instance.context.{} = ", global.name).unwrap();
                write_const_expr(
                    &mut writer,
                    global.const_expr,
                    &globals,
                    imported_globals_count,
                    "consts::",
                    "instance.imports",
                    "&mut instance.context",
                );
                writeln!(writer, ";").unwrap();
            }
        }

        if let Some(entry) = module.elements_section().and_then(|e| e.entries().get(0)) {
            let const_expr = entry.offset().as_ref().unwrap().code();
            let offset = if has_dynamic_element_section_offset {
                write!(writer, "        instance.context.element_section_offset = ").unwrap();
                write_const_expr(
                    &mut writer,
                    const_expr,
                    &globals,
                    imported_globals_count,
                    "consts::",
                    "instance.imports",
                    "&mut instance.context",
                );
                writeln!(writer, ";").unwrap();
                0
            } else {
                assert!(const_expr.len() == 2);
                match const_expr[0] {
                    Instruction::I32Const(c) => c,
                    _ => panic!("Unexpected Element Section Offset {:#?}", const_expr),
                }
            };
            for (fn_ptr, &fn_index) in entry.members().iter().enumerate() {
                let type_index = functions[fn_index as usize].ty_index;
                indirect_fns
                    .entry(type_index)
                    .or_insert_with(Vec::new)
                    .push(((fn_ptr as i32 + offset) as u32, fn_index));
            }
        }

        if let Some(data_section) = module.data_section() {
            for entry in data_section.entries() {
                let offset = entry.offset().as_ref().unwrap().code();
                assert!(offset.len() == 2);
                if let Instruction::I32Const(c) = offset[0] {
                    if entry.value().windows(2).all(|a| a[0] == a[1]) {
                        writeln!(
                            writer,
                            r#"        instance.context.memory.store_slice({}, &[{}; {}]);"#,
                            c,
                            entry.value().first().cloned().unwrap_or_default(),
                            entry.value().len()
                        )
                        .unwrap();
                    } else {
                        write!(
                            writer,
                            r#"        instance.context.memory.store_slice({}, b""#,
                            c
                        )
                        .unwrap();
                        for &b in entry.value() {
                            match b {
                                b'\0' => write!(writer, r#"\0"#).unwrap(),
                                b'"' => write!(writer, r#"\""#).unwrap(),
                                b'\\' => write!(writer, r#"\\"#).unwrap(),
                                b'\r' => write!(writer, r#"\r"#).unwrap(),
                                b'\n' => write!(writer, r#"\n"#).unwrap(),
                                b'\t' => write!(writer, r#"\t"#).unwrap(),
                                0x00..=0x7F => {
                                    write!(writer, "{}", std::char::from_u32(b as _).unwrap())
                                        .unwrap()
                                }
                                _ => write!(writer, r#"\x{:X}"#, b).unwrap(),
                            }
                        }
                        writeln!(writer, r#"");"#,).unwrap();
                    }
                } else {
                    panic!("Data Segment with init expression mismatch");
                }
            }
        }

        if let Some(start) = module.start_section() {
            let name = &functions[start as usize].name;
            writeln!(
                writer,
                "        instance.context.{}(&mut instance.imports);",
                name
            )
            .unwrap();
        }

        writeln!(
            writer,
            "{}",
            r#"        instance
        }"#
        )
        .unwrap();

        for export in exports.entries() {
            if let &Internal::Function(fn_index) = export.internal() {
                let function = &functions[fn_index as usize];
                let spec_fn = match find_function_specs(&contract_specs, export.field()) {
                    Some(spec_fn) => spec_fn,
                    None => FunctionContractSpec::default(),
                };

                write!(
                    writer,
                    "    pub fn {}(&mut self",
                    mangle_fn_name(export.field())
                )
                .unwrap();

                let arguments = spec_fn.inputs();
                for (i, &param) in function.ty.params().iter().enumerate() {
                    if let Some(argument) = arguments.get(i) {
                        write!(writer, ", {}: {}", argument.name(), argument.type_ident()).unwrap();
                    } else {
                        write!(writer, ", var{}: {}", i, to_rs_type(param)).unwrap();
                    }
                }

                write!(writer, ")").unwrap();

                let return_type = spec_fn.output();

                if let Some(argument) = return_type {
                    write!(writer, " -> {}", argument.type_ident()).unwrap();
                } else {
                    for result in function.ty.results() {
                        write!(writer, " -> {}", to_rs_type(*result)).unwrap();
                    }
                }
                writeln!(writer, " {{").unwrap();
                write!(
                    writer,
                    "        self.context.{}(&mut self.imports",
                    function.name
                )
                .unwrap();
                for (i, _) in function.ty.params().iter().enumerate() {
                    if let Some(argument) = arguments.get(i) {
                        write!(writer, ", {}", argument.name()).unwrap();
                    } else {
                        write!(writer, ", var{}", i).unwrap();
                    }
                }
                writeln!(writer, ")").unwrap();
                writeln!(writer, "    }}").unwrap();
            }
        }

        if opt.public_call_indirect {
            for (&type_index, _) in &indirect_fns {
                let Type::Function(ref fn_type) = types.types()[type_index as usize];
                let fn_name = call_indirect_name(fn_type);
                write!(writer, "    pub fn {}", fn_name).unwrap();
                write!(writer, "(&mut self, ptr: i32").unwrap();
                for (i, &param) in fn_type.params().iter().enumerate() {
                    write!(writer, ", var{}: {}", i, to_rs_type(param)).unwrap();
                }
                write!(writer, ")").unwrap();
                for result in fn_type.results() {
                    write!(writer, " -> {}", to_rs_type(*result)).unwrap();
                }
                write!(
                    writer,
                    " {{
            self.context.{}(&mut self.imports, ptr",
                    fn_name
                )
                .unwrap();
                for (i, _) in fn_type.params().iter().enumerate() {
                    write!(writer, ", var{}", i).unwrap();
                }
                writeln!(
                    writer,
                    r#")
        }}"#
                )
                .unwrap();
            }
        }
    }

    //Contract function bodies
    writeln!(
        writer,
        "{}",
        r#"

#[contractimpl]
impl Contract {"#
    )
    .unwrap();

    //TODO write the body in same function
    for export in exports.entries() {
        if let &Internal::Function(fn_index) = export.internal() {
            let function = &mut functions[fn_index as usize];
            let spec_fn = match find_function_specs(&contract_specs, export.field()) {
                Some(spec_fn) => spec_fn,
                None => FunctionContractSpec::default(),
            };
            if function.name == export.field() {
                function.make_public = true;
                continue;
            }
            write!(
                writer,
                "    pub fn {}(&mut self",
                mangle_fn_name(export.field())
            )
            .unwrap();
            let arguments = spec_fn.inputs();
            for (i, &param) in function.ty.params().iter().enumerate() {
                if let Some(argument) = arguments.get(i) {
                    write!(writer, ", {}: {}", argument.name(), argument.type_ident()).unwrap();
                } else {
                    write!(writer, ", var{}: {}", i, to_rs_type(param)).unwrap();
                }
            }
            write!(writer, ")").unwrap();
            let return_type = spec_fn.output();
            if let Some(argument) = return_type {
                write!(writer, " -> {}", argument.type_ident()).unwrap();
            } else {
                for result in function.ty.results() {
                    write!(writer, " -> {}", to_rs_type(*result)).unwrap();
                }
            }
            writeln!(writer, " {{").unwrap();
            write!(writer, "        self.{}(&self", function.name).unwrap();
            for (i, _) in function.ty.params().iter().enumerate() {
                if let Some(argument) = arguments.get(i) {
                    write!(writer, ", {}", argument.name()).unwrap();
                } else {
                    write!(writer, ", var{}", i).unwrap();
                }
            }
            writeln!(writer, ")").unwrap();
            writeln!(writer, "    }}").unwrap();
        }
    }

    for (i, (body, func)) in code.bodies().iter().zip(fns.entries()).enumerate() {
        let type_index = func.type_ref();
        let typ = &types.types()[type_index as usize];
        let fn_type = match *typ {
            Type::Function(ref t) => t,
        };
        let fn_index = import_count + i;
        let function = &functions[fn_index];
        // TODO Ensure there's no collisions with the exports
        if let Some(real_name) = function.real_name {
            writeln!(writer, "    // {}", real_name).unwrap();
        }
        write!(writer, "    ").unwrap();
        if function.make_public {
            write!(writer, "pub ").unwrap();
        }
        write!(
            writer,
            "fn {}(&mut self",
            function.name
        )
        .unwrap();
        for (i, &param) in fn_type.params().iter().enumerate() {
            write!(writer, ", mut arg{}: {}", i, to_rs_type(param)).unwrap();
        }
        write!(writer, ")").unwrap();
        for result in fn_type.results() {
            write!(writer, " -> {}", to_rs_type(*result)).unwrap();
        }
        writeln!(writer, " {{").unwrap();

        let mut expr_index = fn_type.params().len();
        for local in body.locals() {
            let ty = to_rs_type(local.value_type());
            let decimals = if ty.starts_with("f") { ".0" } else { "" };
            for _ in 0..local.count() {
                writeln!(
                    writer,
                    "        let mut var{}: {} = 0{};",
                    expr_index, ty, decimals
                )
                .unwrap();
                expr_index += 1;
            }
        }

        code_builder::build(
            &mut writer,
            expr_index,
            fn_type.results().first().is_some(),
            import_count,
            imported_globals_count,
            &functions,
            &mut indirect_fns,
            &globals,
            types,
            body.code().elements(),
            2,
            &contract_specs,
            fn_index,
        );
    }

    if verbose {
        for (type_index, fns) in indirect_fns {
            let Type::Function(ref fn_type) = types.types()[type_index as usize];
            write!(writer, "    ").unwrap();
            if opt.public_call_indirect {
                write!(writer, "pub ").unwrap();
            }
            write!(
                writer,
                "fn {}",
                call_indirect_name(fn_type),
            )
            .unwrap();
            write!(writer, "(&mut self, ptr: i32").unwrap();
            for (i, &param) in fn_type.params().iter().enumerate() {
                write!(writer, ", var{}: {}", i, to_rs_type(param)).unwrap();
            }
            write!(writer, ")").unwrap();
            for result in fn_type.results() {
                write!(writer, " -> {}", to_rs_type(*result)).unwrap();
            }
            write!(
                writer,
                " {{
            match ptr"
            )
            .unwrap();
            if has_dynamic_element_section_offset {
                write!(writer, " - self.element_section_offset").unwrap();
            }
            writeln!(writer, " {{").unwrap();
            for (fn_ptr, fn_index) in fns {
                let function = &functions[fn_index as usize];
                write!(writer, "            {} => ", fn_ptr).unwrap();
                let is_imported = (fn_index as usize) < import_count;
                if is_imported {
                    write!(writer, "imports.").unwrap();
                } else {
                    write!(writer, "self.").unwrap();
                }
                write!(writer, "{}(", function.name).unwrap();
                if is_imported {
                    write!(writer, "self").unwrap();
                } else {
                    write!(writer, "imports").unwrap();
                }
                for i in 0..function.ty.params().len() {
                    write!(writer, ", var{}", i).unwrap();
                }
                if let Some(real_name) = function.real_name {
                    writeln!(writer, "), // {}", real_name).unwrap();
                } else {
                    writeln!(writer, "),").unwrap();
                }
            }
            writeln!(
                writer,
                r#"            _ => panic!("Invalid Function Pointer"),
            }}
        }}"#
            )
            .unwrap();
        }
    }
    writeln!(writer, "}}").unwrap();
}

fn write_const_expr<W: Write>(
    writer: &mut W,
    opcodes: &[Instruction],
    globals: &[Global],
    imported_globals_count: usize,
    consts_path: &str,
    imports_path: &str,
    context_path: &str,
) {
    assert!(
        opcodes.len() == 2,
        "Invalid Constant Expression {:#?}",
        opcodes
    );
    match opcodes[0] {
        Instruction::I32Const(c) => write!(writer, "{}", c).unwrap(),
        Instruction::I64Const(c) => write!(writer, "{}", c).unwrap(),
        Instruction::F32Const(c) => write!(writer, "{}", c).unwrap(),
        Instruction::F64Const(c) => write!(writer, "{}", c).unwrap(),
        Instruction::GetGlobal(i) => {
            let global = &globals[i as usize];
            let name = &global.name;
            if (i as usize) < imported_globals_count {
                write!(writer, "*{}.{}({})", imports_path, name, context_path).unwrap();
            } else if global.is_mutable {
                write_const_expr(
                    writer,
                    global.const_expr,
                    globals,
                    imported_globals_count,
                    consts_path,
                    imports_path,
                    context_path,
                );
            } else {
                write!(writer, "{}{}", consts_path, name).unwrap();
            }
        }
        _ => panic!("Invalid Constant Expression {:#?}", opcodes),
    }
}

fn is_const_expr_immutable_instead_of_const(opcodes: &[Instruction]) -> bool {
    assert!(
        opcodes.len() == 2,
        "Invalid Constant Expression {:#?}",
        opcodes
    );
    match opcodes[0] {
        Instruction::I32Const(_)
        | Instruction::I64Const(_)
        | Instruction::F32Const(_)
        | Instruction::F64Const(_) => false,
        _ => true, // This could actually be fully const, but it's hard to tell this early
    }
}
