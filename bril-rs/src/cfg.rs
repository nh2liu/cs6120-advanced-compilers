use std::collections::HashMap;

use crate::types::{Function, Instruction, InstructionType};

#[derive(Debug)]
pub struct Block {
    name: String,
    instructions: Vec<Instruction>,
}

impl Block {
    pub fn new(name: String) -> Block {
        Block {
            name: name,
            instructions: vec![],
        }
    }
    pub fn push(&mut self, x: Instruction) {
        self.instructions.push(x)
    }
}

// Control Flow Graph
#[derive(Debug)]
pub struct CFG {
    pub graph: HashMap<String, Vec<String>>,
    pub blocks: HashMap<String, Block>,
}

impl CFG {
    pub fn from_function(function: &Function) -> CFG {
        let mut block_idx = 0;
        let mut graph: HashMap<String, Vec<String>> = HashMap::new();
        let mut cur_block: Option<&mut Block> = None;
        let mut blocks = HashMap::new();

        for instr in &function.instructions {
            if matches!(instr.op, InstructionType::Label { .. }) || cur_block.is_none() {
                let new_block_name = match &instr.op {
                    InstructionType::Label { name } => name.clone(),
                    _ => {
                        block_idx += 1;
                        format!("__anon__{block_idx}").to_string()
                    }
                };
                cur_block = Some(
                    blocks
                        .entry(new_block_name.clone())
                        .or_insert(Block::new(new_block_name.clone())),
                );
            }
            let unwrapped_block = cur_block.as_deref_mut().unwrap();
            if !matches!(instr.op, InstructionType::Label { .. }) {
                unwrapped_block.push(instr.clone());
            }

            if let InstructionType::Br | InstructionType::Jmp = instr.op {
                let neighbors = graph.entry(unwrapped_block.name.clone()).or_insert(vec![]);
                instr
                    .labels
                    .as_ref()
                    .expect("Expect at least 1 label.")
                    .into_iter()
                    .for_each(|lbl| {
                        neighbors.push(lbl.clone());
                    });
                cur_block = None;
            };
        }
        CFG {
            graph: graph,
            blocks: blocks,
        }
    }
}
