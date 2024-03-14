use crate::types::{BrilType, BrilValue, Function, Instruction, InstructionType};
use itertools::Itertools;
use serde_json::Value;

fn as_str(v: &Value) -> String {
    v.as_str().unwrap_or("").to_string()
}

fn as_str_option(v: &Value) -> Option<String> {
    v.as_str().map(|x| x.to_string())
}

fn to_bril_type(str: &str) -> BrilType {
    match str {
        "bool" => BrilType::Bool,
        "int" => BrilType::Int,
        "" => BrilType::Nil,
        _ => panic!("Type not found {}", str),
    }
}

impl Instruction {
    fn from_json(json_value: &Value) -> Instruction {
        let get_str_arr_option = |field| {
            json_value[field]
                .as_array()
                .map(|x| x.into_iter().map(as_str).collect_vec())
        };

        Instruction {
            op: if let Some(op_name) = json_value["op"].as_str() {
                match op_name {
                    "call" => InstructionType::Call,
                    "const" => InstructionType::Const,
                    "print" => InstructionType::Print,
                    "ret" => InstructionType::Ret,
                    _ => InstructionType::Unknown {
                        op: op_name.to_string(),
                    },
                }
            } else if let Some(label_name) = json_value["label"].as_str() {
                InstructionType::Label {
                    name: label_name.to_string(),
                }
            } else {
                panic!("Invalid instruction: {:?}", json_value)
            },
            dest: as_str_option(&json_value["dest"]),
            value: {
                if let Some(v) = json_value.get("value") {
                    if let Some(t) = json_value["type"].as_str() {
                        Some(match t {
                            "int" => BrilValue::Int(v.as_i64().unwrap()),
                            "bool" => BrilValue::Bool(v.as_bool().unwrap()),
                            t => panic!("invalid type: {}", t),
                        });
                    } else {
                        panic!("No type associated with {v}")
                    }
                }
                None
            },
            r#type: as_str_option(&json_value["type"]),
            args: get_str_arr_option("args"),
            funcs: get_str_arr_option("funcs"),
            labels: get_str_arr_option("labels"),
        }
    }
}

pub fn parse_bril_fn(json_value: &Value) -> Function {
    Function {
        name: as_str(&json_value["name"]),
        args: vec![],
        ret_type: to_bril_type(&as_str(&json_value["type"])),
        instructions: json_value["instrs"]
            .as_array()
            .unwrap()
            .into_iter()
            .map(Instruction::from_json)
            .collect_vec(),
    }
}

pub fn parse_bril(json_string: &str) -> serde_json::Result<Vec<Function>> {
    let bril_prg: Value = serde_json::from_str(json_string)?;
    let functions = bril_prg["functions"]
        .as_array()
        .unwrap()
        .into_iter()
        .map(parse_bril_fn)
        .collect_vec();
    Ok(functions)
}
