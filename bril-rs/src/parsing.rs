use crate::types::{BrilType, BrilValue, Function, Instruction, InstructionType};
use itertools::Itertools;
use serde_json::{json, Value};

fn as_str(v: &Value) -> String {
    v.as_str().unwrap_or("").to_string()
}

fn as_str_option(v: &Value) -> Option<String> {
    v.as_str().map(|x| x.to_string())
}

fn to_bril_type(str: &str) -> Option<BrilType> {
    match str {
        "bool" => Some(BrilType::Bool),
        "int" => Some(BrilType::Int),
        "" => None,
        _ => panic!("Type not found {}", str),
    }
}

pub fn parse_bril_fn(json_value: &Value) -> Function {
    Function {
        name: as_str(&json_value["name"]),
        args: json_value["args"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| {
                (
                    as_str(&v["name"]),
                    to_bril_type(&as_str(&v["type"])).unwrap(),
                )
            })
            .collect_vec(),
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

pub fn to_json(functions: &Vec<Function>) -> Value {
    json!({"functions": json!(functions.iter().map(Function::to_json).collect_vec())})
}

impl Instruction {
    pub fn from_json(json_value: &Value) -> Instruction {
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
                    "br" => InstructionType::Br,
                    "jmp" => InstructionType::Jmp,
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
                        })
                    } else {
                        panic!("No type associated with {v}")
                    }
                } else {
                    None
                }
            },
            r#type: as_str_option(&json_value["type"]),
            args: get_str_arr_option("args"),
            funcs: get_str_arr_option("funcs"),
            labels: get_str_arr_option("labels"),
        }
    }

    pub fn to_json(&self) -> Value {
        let mut map = serde_json::Map::new();
        if let InstructionType::Label { name } = &self.op {
            map.insert("label".to_string(), json!(name));
        } else {
            if let Some(dest) = &self.dest {
                map.insert("dest".to_string(), json!(dest));
            }
            if let Some(r#type) = &self.r#type {
                map.insert("type".to_string(), json!(r#type));
            }
            if let Some(args) = &self.args {
                map.insert("args".to_string(), json!(args));
            }
            if let Some(funcs) = &self.funcs {
                map.insert("funcs".to_string(), json!(funcs));
            }
            if let Some(labels) = &self.labels {
                map.insert("labels".to_string(), json!(labels));
            }

            if let Some(value) = &self.value {
                map.insert(
                    "value".to_string(),
                    match value {
                        BrilValue::Bool(b) => json!(*b),
                        BrilValue::Int(i) => json!(i),
                    },
                );
            }
            map.insert(
                "op".to_string(),
                json!(match &self.op {
                    InstructionType::Const => "const",
                    InstructionType::Call => "call",
                    InstructionType::Ret => "ret",
                    InstructionType::Jmp => "jmp",
                    InstructionType::Br => "br",
                    InstructionType::Print => "print",
                    InstructionType::Unknown { op } => op.as_str(),
                    InstructionType::Label { .. } =>
                        unreachable!("Unreachable because of match above."),
                }),
            );
        }
        Value::Object(map)
    }
}

impl Function {
    pub fn to_json(&self) -> Value {
        let mut map = serde_json::Map::new();
        map.insert("name".to_string(), json!(self.name));
        if let Some(t) = &self.ret_type {
            map.insert("type".to_string(), json!(t.to_str()));
        }

        map.insert(
            "args".to_string(),
            json!(self
                .args
                .iter()
                .map(|(name, t)| json!({"name": name, "type": t.to_str()}))
                .collect_vec()),
        );
        map.insert(
            "instrs".to_string(),
            json!(self
                .instructions
                .iter()
                .map(Instruction::to_json)
                .collect_vec()),
        );

        Value::Object(map)
    }
}
