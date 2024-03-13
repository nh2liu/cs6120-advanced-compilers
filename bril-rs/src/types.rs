#[derive(Debug)]
pub enum BrilType {
    Bool,
    Int,
    Nil,
}

#[derive(Debug)]
pub enum BrilValue {
    Bool(bool),
    Int(i64),
}

#[derive(Debug)]
pub enum Instruction {
    Label {
        name: String,
    },
    Const {
        dest: String,
        value: BrilValue,
    },
    Call {
        dest: String,
        args: Vec<String>,
    },
    Ret {
        args: Vec<String>,
    },
    Print {
        args: Vec<String>,
    },
    GenericFunction {
        dest: String,
        btype: String,
        args: Vec<String>,
        funcs: Vec<String>,
        labels: Vec<String>,
    },
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub args: Vec<String>,
    pub ret_type: BrilType,
    pub instructions: Vec<Instruction>,
}
