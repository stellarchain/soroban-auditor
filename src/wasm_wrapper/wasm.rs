use crate::ssa::Expr;
use super::wasm_adapter::{InitExpr, LoadError, Module};
use crate::cfg::CfgBuildError;
use crate::fmt::CodeWriter;
use crate::soroban::FunctionInfo;
use std::rc::Rc;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TableElement {
    Null,
    Func(u32),
}

#[derive(Clone, PartialEq, Debug)]
pub struct Table {
    pub elements: Vec<TableElement>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Instance {
    module: Module,
    tables: Vec<Table>,
}

impl Instance {
    pub fn from_file<P: AsRef<::std::path::Path>>(path: P) -> Result<Self, LoadError> {
        let module = Module::from_file(&path)?;
        Ok(Self {
            tables: init_tables(&module),
            module,
        })
    }

    pub fn load_file<P: AsRef<::std::path::Path>>(path: P) -> Rc<Self> {
        match self::Instance::from_file(path) {
            Ok(instance) => Rc::new(instance),
            Err(error) => {
                panic!("Wasm not loaded.");
            }
        }
    }
    pub const fn module(&self) -> &Module {
        &self.module
    }
    pub fn tables(&self) -> &[Table] {
        &self.tables
    }

    pub fn spec_fns(&self) -> &Vec<FunctionInfo> {
        &self.module().spec_fns()
    }

    pub fn decompile_function(&self, func_index: u32) -> Result<(), CfgBuildError> {
        let mut printer = CodeWriter::printer(Rc::new(self.clone()), func_index);
        let empty_args: &[Expr] = &[];
        let code = printer.decompile_func(func_index as u32, false, empty_args).unwrap();
        printer.write_func(&code, false);
        Ok(())
    }
}

fn init_tables(module: &Module) -> Vec<Table> {
    let mut tables: Vec<_> = module
        .tables()
        .iter()
        .map(|table_type| Table {
            elements: vec![TableElement::Null; table_type.limits().initial() as usize],
        })
        .collect();
    for init in module.table_inits() {
        let table = &mut tables[init.index() as usize];
        if let InitExpr::I32Const(offset) = init.offset() {
            for (i, ele) in init.entries().iter().enumerate() {
                let ele = TableElement::Func(*ele);
                let index = i + *offset as usize;
                if index >= table.elements.len() {
                    table.elements.push(ele);
                } else {
                    table.elements[index] = ele;
                }
            }
        }
    }
    tables
}
