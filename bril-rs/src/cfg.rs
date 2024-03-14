use std::collections::HashMap;

use itertools::Itertools;

use crate::types::{Function, Instruction, InstructionType};

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

impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}({})", self.name, self.instructions.len())?;
        Ok(())
    }
}

// Control Flow Graph

pub struct CFG {
    name: String,
    pub graph: HashMap<String, Vec<String>>,
    pub blocks: HashMap<String, Block>,
}

impl std::fmt::Display for CFG {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let graph_string = self
            .blocks
            .iter()
            .flat_map(|(name, block)| {
                if self.graph[name].is_empty() {
                    vec![format!("\t{}", block)]
                } else {
                    self.graph[name]
                        .iter()
                        .map(|out_node| format!("\t{} -> {}", block, out_node))
                        .collect_vec()
                }
            })
            .join("\n");
        write!(f, "CFG({}) {{\n{}\n}}", self.name, graph_string)?;
        Ok(())
    }
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
                graph.insert(new_block_name, vec![]);
            }
            let unwrapped_block = cur_block.as_deref_mut().unwrap();
            if !matches!(instr.op, InstructionType::Label { .. }) {
                unwrapped_block.push(instr.clone());
            }

            if let InstructionType::Br | InstructionType::Jmp | InstructionType::Ret = instr.op {
                let neighbors = graph.entry(unwrapped_block.name.clone()).or_insert(vec![]);
                instr.labels.as_ref().map(|v| {
                    v.into_iter().for_each(|lbl| {
                        neighbors.push(lbl.clone());
                    })
                });
                cur_block = None;
            };
        }
        CFG {
            name: function.name.clone(),
            graph: graph,
            blocks: blocks,
        }
    }
}
