use crate::statement::{Statement, StatementType};

pub fn execute_statement(statement: &Statement) {
    match statement.stype {
        StatementType::Insert => {
            println!("We will insert");
        }
        StatementType::Select => {
            println!("We will select");
        }
    }
}
