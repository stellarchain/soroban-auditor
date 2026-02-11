use crate::engine::function::FunctionBlock;

pub trait Pattern {
    #[allow(dead_code)]
    fn name(&self) -> &'static str;
    fn apply(&self, block: &FunctionBlock) -> Option<FunctionBlock>;
}
