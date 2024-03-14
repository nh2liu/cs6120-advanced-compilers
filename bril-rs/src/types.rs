use std::fmt::Formatter;

#[derive(Debug)]
pub enum BrilType {
    Bool,
    Int,
}

impl BrilType {
    pub fn to_str(&self) -> &str {
        match self {
            BrilType::Bool => "bool",
            BrilType::Int => "int",
        }
    }
}

#[derive(Debug, Clone)]
pub enum BrilValue {
    Bool(bool),
    Int(i64),
}

#[derive(Debug, Clone)]
pub enum InstructionType {
    Label { name: String },
    Const,
    Call,
    Ret,
    Jmp,
    Br,
    Print,
    Unknown { op: String },
}

#[derive(Clone)]
pub struct Instruction {
    pub op: InstructionType,
    pub dest: Option<String>,
    pub value: Option<BrilValue>,
    pub r#type: Option<String>,
    pub args: Option<Vec<String>>,
    pub funcs: Option<Vec<String>>,
    pub labels: Option<Vec<String>>,
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match &self.op {
            InstructionType::Label { name } => {
                write!(f, "Label({name})")
            }
            _ => {
                let mut fields = Vec::new();

                if let Some(dest) = &self.dest {
                    fields.push(format!("dest: {}", dest));
                }

                if let Some(value) = &self.value {
                    fields.push(format!("value: {:?}", value));
                }

                if let Some(r#type) = &self.r#type {
                    fields.push(format!("type: {}", r#type));
                }

                if let Some(args) = &self.args {
                    fields.push(format!("args: [{}]", args.join(", ")));
                }

                if let Some(funcs) = &self.funcs {
                    fields.push(format!("funcs: [{}]", funcs.join(", ")));
                }

                if let Some(labels) = &self.labels {
                    fields.push(format!("labels: [{}]", labels.join(", ")));
                }

                let op_string = fields.join(", ");

                write!(f, "{:?}({op_string})", self.op)
            }
        }
    }
}

impl std::fmt::Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.to_string(), f)
    }
}
#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub args: Vec<(String, BrilType)>,
    pub ret_type: Option<BrilType>,
    pub instructions: Vec<Instruction>,
}
