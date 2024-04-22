use crate::ssa::Expr;
use crate::analysis;
use crate::cfg::{Cfg, CfgBuildError};
use crate::soroban::sdk_linker::search_for_patterns;
use crate::soroban::FunctionInfo;
use crate::ssa;
use crate::structuring;
use std::rc::Rc;

use crate::ssa::Stmt;
use crate::wasm_wrapper::wasm;
use crate::wasm_wrapper::wasm_adapter::{Function, Module};

pub trait CodeDisplay {
    fn fmt_code(&self, f: &mut CodeWriter);
    fn create_str(&self, wasm: Rc<wasm::Instance>, func_index: u32) -> String {
        let mut fmt = CodeWriter::formatter(wasm, func_index);
        self.fmt_code(&mut fmt);
        fmt.get_output()
    }
}

impl CodeDisplay for &str {
    fn fmt_code(&self, f: &mut CodeWriter) {
        write!(f, "{}", self);
    }
}

impl<T: CodeDisplay> CodeDisplay for &T {
    fn fmt_code(&self, f: &mut CodeWriter) {
        (*self).fmt_code(f);
    }
}

impl<T: CodeDisplay> CodeDisplay for Box<T> {
    fn fmt_code(&self, f: &mut CodeWriter) {
        (**self).fmt_code(f);
    }
}

impl<T: CodeDisplay> CodeDisplay for &[T] {
    fn fmt_code(&self, f: &mut CodeWriter) {
        for e in *self {
            e.fmt_code(f);
        }
    }
}

enum Output {
    Stdout(std::io::Stdout),
    Str(String),
}

impl Output {
    fn stdout() -> Self {
        Output::Stdout(std::io::stdout())
    }

    fn str() -> Self {
        Output::Str(String::new())
    }

    fn write_fmt(&mut self, args: std::fmt::Arguments) {
        match self {
            Output::Stdout(out) => std::io::Write::write_fmt(out, args).unwrap(),
            Output::Str(ref mut s) => std::fmt::Write::write_fmt(s, args).unwrap(),
        }
    }
}

pub struct CodeWriter {
    indent: usize,
    wasm: Rc<wasm::Instance>,
    func_index: u32,
    output: Output,
    suppress_newline: bool,
}

impl CodeWriter {
    pub fn formatter(wasm: Rc<wasm::Instance>, func_index: u32) -> CodeWriter {
        CodeWriter {
            indent: 0,
            wasm,
            func_index,
            output: Output::str(),
            suppress_newline: false,
        }
    }

    pub fn printer(wasm: Rc<wasm::Instance>, func_index: u32) -> CodeWriter {
        CodeWriter {
            indent: 0,
            wasm,
            func_index,
            output: Output::stdout(),
            suppress_newline: false,
        }
    }

    pub fn wasm(&self) -> &wasm::Instance {
        &self.wasm
    }

    pub fn module(&self) -> &Module {
        self.wasm.module()
    }

    pub fn specs_fns(&self) -> &Vec<FunctionInfo> {
        self.wasm.spec_fns()
    }

    pub fn func(&self) -> &Function {
        self.wasm.module().func(self.func_index)
    }

    pub fn indent(&mut self) {
        self.indent += 1;
    }

    pub fn dedent(&mut self) {
        self.indent -= 1;
    }

    pub fn write(&mut self, fmt: impl CodeDisplay) {
        fmt.fmt_code(self)
    }

    pub fn string_func(&mut self, fmt: impl CodeDisplay) -> String {
        fmt.create_str(self.wasm.clone(), self.func_index)
    }

    pub fn write_fmt(&mut self, args: std::fmt::Arguments) {
        self.output.write_fmt(args);
    }

    pub fn decompile_func(&mut self, func_index: u32, is_call: bool, args: &[Expr]) -> Result<Vec<Stmt>, CfgBuildError> {
        let mut cfg = Cfg::build(self.wasm.clone(), func_index)?;
        let mut def_use_map = ssa::transform_to_ssa(&mut cfg);
        analysis::propagate_expressions(&mut cfg, &mut def_use_map);
        analysis::eliminate_dead_code(&mut cfg, &mut def_use_map);
        ssa::transform_out_of_ssa(&mut cfg);

        let (_decls, code) = structuring::structure(cfg);
        Ok(code)
    }

    pub fn write_func(&mut self, code: &[Stmt], is_call: bool) {
        let func = self.func();
        let ret_type = match func.return_type() {
            Some(type_ret) => type_ret.to_string(),
            None => "".to_string(),
        };

        let env_arg = "env";
        if let Some(spec) = func.spec_fn() {
            if spec == &FunctionInfo::default() && is_call == false {
                return; // Exit early
            }

            let mut params_vec = vec![format!("{}: {}", env_arg, "Env")];
            params_vec.extend(
                spec.inputs()
                    .iter()
                    .enumerate()
                    .map(|(_i, param)| format!("{}: {}", param.name(), param.type_ident().type_str())),
            );
            let params = params_vec.join(", ");
            let output = spec.output();
            let return_type = output.map_or(ret_type, |o| o.type_ident().type_str().to_string());

            let func_header = if return_type.len() > 0 {
                format!("pub fn {}({}) -> {} {{", func.name(), params, return_type)
            } else {
                format!("pub fn {}({}) {{", func.name(), params)
            };

            self.indent();
            if !is_call {
                self.write(func_header.as_str());
            }

            let code_str = self.string_func(&code[..]);
            self.write(code_str.as_str());

            self.dedent();
            if !is_call {
                self.newline();
                self.write("}");
            }
            return; // Exit early
        }

        // If no spec, continue with default behavior
        let mut params_vec = vec![format!("{}: {}", env_arg, "Env")];
        params_vec.extend(
            func.params()
                .iter()
                .enumerate()
                .map(|(i, t)| format!("arg_{}: {} ", (i as u8 + b'a') as char, t)),
        );
        let params = params_vec.join(", ");

        let func_header = if ret_type.is_empty() {
            format!("pub fn {}({}) {{", func.name(), params)
        } else {
            format!("pub fn {}({}) -> {} {{", func.name(), params, ret_type)
        };

        self.write(func_header.as_str());
        self.indent();
        self.write(&code[..]);
        self.dedent();
        self.write("}");
        self.newline();
    }

    pub fn suppress_newline(&mut self) {
        self.suppress_newline = true;
    }

    pub fn newline(&mut self) {
        if self.suppress_newline {
            self.suppress_newline = false;
        } else {
            write!(self, "\n{: >1$}", "", self.indent * 4);
        }
    }

    pub fn get_output(self) -> String {
        match self.output {
            Output::Str(s) => s,
            _ => String::from(""),
        }
    }

    pub fn write_body(&mut self, code: &[Stmt]) {
        self.write(&code[..]);
    }
}
