use crate::input_buffer::InputBuffer;
use crate::row::{COLUMN_EMAIL_SIZE, COLUMN_USERNAME_SIZE, Row};
use crate::row::{Row, RowError};
use crate::statement::{MetaCommandResult, PrepareResult, Statement, StatementType};

pub fn do_meta_command(input: &InputBuffer) -> MetaCommandResult {
    match input.buffer.as_str() {
        ".exit" => {
            println!("Bye!");
            std::process::exit(0);
        }
        _ => MetaCommandResult::UnrecognizedCommand,
    }
}

pub fn prepare_statement(input: &InputBuffer, statement: &mut Statement) -> PrepareResult {
    let mut parts = input.buffer.split_whitespace();

    let cmd = match parts.next() {
        Some(c) => c,
        None => return PrepareResult::SyntaxError,
    };

    if cmd == "insert" {
        let id_str = match parts.next() {
            Some(s) => s,
            None => return PrepareResult::SyntaxError,
        };

        let username_str = match parts.next() {
            Some(s) => s,
            None => return PrepareResult::SyntaxError,
        };

        let email_str = match parts.next() {
            Some(s) => s,
            None => return PrepareResult::SyntaxError,
        };

        let id: u32 = match id_str.parse() {
            Ok(v) => v,
            Err(_) => return PrepareResult::SyntaxError,
        };

        let row = match Row::from_values(id, username_str, email_str) {
            Ok(row) => row,
            Err(RowError::UserNameTooLong) | Err(RowError::EmailTooLong) => {
                return PrepareResult::SyntaxError;
            }
        };

        statement.stype = StatementType::Insert;
        statement.row_to_insert = Some(row);
        return PrepareResult::Success;
    }

    if cmd == "select" {
        statement.stype = StatementType::Select;
        statement.row_to_insert = None;
        return PrepareResult::Success;
    }

    PrepareResult::UnrecognizedStatement
}
