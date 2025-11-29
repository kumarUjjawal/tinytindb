use crate::row::Row;

pub enum MetaCommandResult {
    Success,
    UnrecognizedCommand,
}

#[derive(Debug)]
pub enum PrepareResult {
    Success,
    UnrecognizedStatement,
    SyntaxError,
}

#[derive(Debug)]
pub enum StatementType {
    Insert,
    Select,
}

#[derive(Debug)]
pub struct Statement {
    pub stype: StatementType,
    pub row_to_insert: Option<Row>,
}

impl Statement {
    pub fn new() -> Self {
        Self {
            stype: StatementType::Select,
            row_to_insert: None,
        }
    }
}
