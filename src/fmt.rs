use crate::soroban;
use crate::soroban::FunctionInfo;
use std::rc::Rc;

use crate::ssa::{Stmt, Var};
use crate::wasm_wrapper::wasm;
use crate::wasm_wrapper::wasm_adapter::{Function, Module, ValueType};

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

    pub fn specs_fns(&self) -> &Vec<FunctionInfo>{
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
        fmt.fmt_code(self);
    }

    pub fn write_fmt(&mut self, args: std::fmt::Arguments) {
        self.output.write_fmt(args);
    }

    pub fn write_func(&mut self, 
        func_index: u32, 
        decls: &[(Var, ValueType)], 
        code: &[Stmt],     
    ) {
        let func = self.func();
        let spec_fns = soroban::find_function_specs(self.specs_fns(), func.name());
        let params = match  spec_fns {
            Some(spec) => {
                let params = spec 
                .inputs()
                .iter()
                .enumerate()
                .map(|(i, param)| {
                    format!(
                        "{}: {}",
                        param.name(),
                        param.type_ident().type_str()
                    )
                })
                .collect::<Vec<_>>()
                .join(", ");
                params
            }
            None => {
                let params = func
                .params()
                .iter()
                .enumerate()
                .map(|(i, t)| format!("{} arg_{}", t, i))
                .collect::<Vec<_>>()
                .join(", ");
                params
            }, 
        };

        let return_type = match spec_fns {
            Some(spec) => {
                let output = spec.output().unwrap();
                output.type_ident().value_type()
            }
            None => func.return_type()
        };

        let func_header = if let Some(ret_type) = return_type {
            format!("pub fn {}(env: Env, {}) -> {} {{", func.name(), params, ret_type)
        } else {
            format!("pub fn {}({}) {{", func.name(), params)
        };

        self.write(func_header.as_str());
        self.indent();

        self.write(&code[..]);
        self.dedent();
        self.newline();
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
}
