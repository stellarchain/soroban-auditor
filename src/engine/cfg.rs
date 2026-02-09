use parity_wasm::elements::{BlockType, Instruction};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockKind {
    Block,
    Loop,
    If,
}

#[derive(Debug, Clone)]
pub struct BlockFrame {
    pub kind: BlockKind,
    pub start: usize,
    pub end: usize,
    pub else_start: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct Cfg {
    pub block_frames: Vec<BlockFrame>,
    pub start_to_frame: HashMap<usize, usize>,
    pub end_to_frame: HashMap<usize, usize>,
}

#[derive(Debug, Clone)]
pub struct BlockStackEntry {
    pub kind: BlockKind,
    pub start: usize,
    pub else_start: Option<usize>,
}

#[derive(Debug, Clone)]
pub enum LabelTarget {
    LoopStart(usize),
    BlockEnd(usize),
}

pub fn build_cfg(instrs: &[Instruction]) -> Cfg {
    let mut stack: Vec<BlockStackEntry> = Vec::new();
    let mut frames: Vec<BlockFrame> = Vec::new();

    for (idx, instr) in instrs.iter().enumerate() {
        match instr {
            Instruction::Block(_) => {
                stack.push(BlockStackEntry {
                    kind: BlockKind::Block,
                    start: idx,
                    else_start: None,
                });
            }
            Instruction::Loop(_) => {
                stack.push(BlockStackEntry {
                    kind: BlockKind::Loop,
                    start: idx,
                    else_start: None,
                });
            }
            Instruction::If(_) => {
                stack.push(BlockStackEntry {
                    kind: BlockKind::If,
                    start: idx,
                    else_start: None,
                });
            }
            Instruction::Else => {
                if let Some(top) = stack.last_mut() {
                    if top.kind == BlockKind::If {
                        top.else_start = Some(idx);
                    }
                }
            }
            Instruction::End => {
                if let Some(top) = stack.pop() {
                    frames.push(BlockFrame {
                        kind: top.kind,
                        start: top.start,
                        end: idx,
                        else_start: top.else_start,
                    });
                }
            }
            _ => {}
        }
    }

    let mut start_to_frame = HashMap::new();
    let mut end_to_frame = HashMap::new();
    for (i, frame) in frames.iter().enumerate() {
        start_to_frame.insert(frame.start, i);
        end_to_frame.insert(frame.end, i);
    }

    Cfg {
        block_frames: frames,
        start_to_frame,
        end_to_frame,
    }
}

pub fn resolve_label_target(instrs: &[Instruction], cfg: &Cfg, depth: u32, at: usize) -> Option<LabelTarget> {
    let mut stack: Vec<usize> = Vec::new();
    for (idx, instr) in instrs.iter().enumerate() {
        match instr {
            Instruction::Block(_) | Instruction::Loop(_) | Instruction::If(_) => {
                if let Some(frame_id) = cfg.start_to_frame.get(&idx) {
                    stack.push(*frame_id);
                }
            }
            Instruction::End => {
                if let Some(frame_id) = cfg.end_to_frame.get(&idx) {
                    if let Some(pos) = stack.iter().position(|id| id == frame_id) {
                        stack.remove(pos);
                    }
                }
            }
            _ => {}
        }
        if idx == at {
            break;
        }
    }

    if stack.is_empty() {
        return None;
    }
    let target_index = stack.len().saturating_sub(1 + depth as usize);
    if target_index >= stack.len() {
        return None;
    }
    let frame = &cfg.block_frames[stack[target_index]];
    match frame.kind {
        BlockKind::Loop => Some(LabelTarget::LoopStart(frame.start)),
        BlockKind::Block | BlockKind::If => Some(LabelTarget::BlockEnd(frame.end)),
    }
}

pub fn block_stack_at(instrs: &[Instruction], cfg: &Cfg, at: usize) -> Vec<LabelTarget> {
    let mut stack: Vec<usize> = Vec::new();
    for (idx, instr) in instrs.iter().enumerate() {
        match instr {
            Instruction::Block(_) | Instruction::Loop(_) | Instruction::If(_) => {
                if let Some(frame_id) = cfg.start_to_frame.get(&idx) {
                    stack.push(*frame_id);
                }
            }
            Instruction::End => {
                if let Some(frame_id) = cfg.end_to_frame.get(&idx) {
                    if let Some(pos) = stack.iter().position(|id| id == frame_id) {
                        stack.remove(pos);
                    }
                }
            }
            _ => {}
        }
        if idx == at {
            break;
        }
    }

    let mut out = Vec::new();
    for frame_id in stack {
        let frame = &cfg.block_frames[frame_id];
        match frame.kind {
            BlockKind::Loop => out.push(LabelTarget::LoopStart(frame.start)),
            BlockKind::Block | BlockKind::If => out.push(LabelTarget::BlockEnd(frame.end)),
        }
    }
    out
}
