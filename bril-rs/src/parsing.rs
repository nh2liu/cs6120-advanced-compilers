use crate::types::{BrilType, BrilValue, Function, Instruction};
use itertools::Itertools;
use serde_json::Value;

fn as_str(v: &Value) -> String {
    v.as_str().unwrap_or("").to_string()
}

fn to_bril_type(str: &str) -> BrilType {
    match str {
        "bool" => BrilType::Bool,
        "int" => BrilType::Int,
        "" => BrilType::Nil,
        _ => panic!("Type not found {}", str),
    }
}

pub fn parse_bril_instr<'a>(json_value: &'a Value) -> Instruction {
    let get_str_arr = |field| {
        json_value[field]
            .as_array()
            .map_or(vec![], |x| x.into_iter().map(as_str).collect_vec())
    };
    let get_dest = || as_str(&json_value["dest"]);

    if let Some(op_name) = json_value["op"].as_str() {
        match op_name {
            "call" => Instruction::Call {
                args: get_str_arr("args"),
                dest: get_dest(),
            },
            "const" => Instruction::Const {
                dest: get_dest(),
                value: match as_str(&json_value["type"]).as_str() {
                    "int" => BrilValue::Int(json_value["value"].as_i64().unwrap()),
                    "bool" => BrilValue::Bool(json_value["value"].as_bool().unwrap()),
                    t => panic!("invalid type: {}", t),
                },
            },
            "print" => Instruction::Print {
                args: get_str_arr("args"),
            },
            "ret" => Instruction::Ret {
                args: get_str_arr("args"),
            },
            _ => Instruction::GenericFunction {
                dest: get_dest(),
                btype: as_str(&json_value["type"]),
                args: get_str_arr("args"),
                funcs: get_str_arr("funcs"),
                labels: get_str_arr("labels"),
            },
        }
    } else if let Some(label_name) = json_value["label"].as_str() {
        Instruction::Label {
            name: label_name.to_string(),
        }
    } else {
        panic!("Invalid instruction: {:?}", json_value)
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
            .map(parse_bril_instr)
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
