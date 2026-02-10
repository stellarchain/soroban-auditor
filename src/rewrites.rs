use crate::decompile::DataSegment;
use crate::fingerprint::Fingerprint;
use crate::wasm_ir::Function;
use parity_wasm::elements::{FuncBody, FunctionType};
use std::io::Write;

pub fn try_emit_raw_rewrite<W: Write>(
    writer: &mut W,
    function: &Function,
    fn_type: &FunctionType,
    fingerprint: &Fingerprint,
    body: &FuncBody,
    import_count: usize,
    functions: &[Function],
    data_segments: &[DataSegment],
) -> Result<bool, String> {
    let _ = (
        writer,
        function,
        fn_type,
        fingerprint,
        body,
        import_count,
        functions,
        data_segments,
    );
    Ok(false)
}
