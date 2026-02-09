use crate::engine::function::FunctionBlock;

pub trait Pattern {
    fn name(&self) -> &'static str;
    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock>;
}
