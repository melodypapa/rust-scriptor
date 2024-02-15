use std::fmt;

pub enum OperationType {
    Condition,
    ForEach,
    Add,
    Remove,
    SetValue,
    SetEnabled,
}

impl fmt::Display for OperationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            OperationType::Add => write!(f, "Add"),
            OperationType::Condition => write!(f, "Condition"),
            OperationType::ForEach => write!(f, "ForEach"),
            OperationType::Remove => write!(f, "Remove"),
            OperationType::SetValue => write!(f, "SetValue"),
            OperationType::SetEnabled => write!(f, "SetEnabled"),
        }
    }
}

pub trait Operable {
    fn new_op(&mut self, op_type: OperationType, expression: &'static str) -> &Operation;

    fn get_op_list(&self) -> &Vec<Operation>;

    fn condition_op(&mut self, expression: &'static str) -> &Operation {
        self.new_op(OperationType::Condition, expression)
    }

    fn for_each_op(&mut self, expression: &'static str) -> &Operation {
        self.new_op(OperationType::ForEach, expression)
    }

    fn add_op(&mut self, expression: &'static str) -> &Operation {
        self.new_op(OperationType::Add, expression)
    }

    fn remove_op(&mut self, expression: &'static str) -> &Operation {
        self.new_op(OperationType::Remove, expression)
    }

    fn set_value_op(&mut self, expression: &'static str) -> &Operation {
        self.new_op(OperationType::SetValue, expression)
    }

    fn set_enable_op(&mut self, enabled: bool) -> &Operation {
        if enabled {}
        self.new_op(
            OperationType::SetEnabled,
            if enabled { "boolean(1)" } else { "boolean(0)" },
        )
    }
}

impl Operable for Operation {
    fn new_op(&mut self, op_type: OperationType, expression: &'static str) -> &Operation {
        let op = Operation::new(op_type, expression);
        self.operations.push(op);
        self.operations.last().unwrap()
    }

    fn get_op_list(&self) -> &Vec<Operation> {
        &self.operations
    }
}

pub struct Operation {
    pub op_type: OperationType,
    pub expression: &'static str,
    pub operations: Vec<Operation>,
}

impl Operation {
    pub fn new(op_type: OperationType, expression: &'static str) -> Self {
        Self {
            op_type,
            expression,
            operations: Vec::new(),
        }
    }
}
