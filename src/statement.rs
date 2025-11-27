pub enum MetaCommandResult {
    Success,
    UnrecognizedCommand,
}

pub enum PrepareResult {
    Success,
    UnrecognizedStatement,
}

#[derive(Debug)]
pub enum StatementType {
    Insert,
    Select,
}

#[derive(Debug)]
pub struct Statement {
    pub stype: StatementType,
}
