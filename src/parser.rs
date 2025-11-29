use crate::input_buffer::InputBuffer;
use crate::row::{COLUMN_EMAIL_SIZE, COLUMN_USERNAME_SIZE, Row};
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
        let id_str = parts.next().ok_or(PrepareResult::SyntaxError).unwrap();
        let username_str = parts.next().ok_or(PrepareResult::SyntaxError).unwrap();
        let email_str = parts.next().ok_or(PrepareResult::SyntaxError).unwrap();

        let id: u32 = match id_str.parse() {
            Ok(num) => num,
            Err(_) => return PrepareResult::SyntaxError,
        };

        let username_bytes = username_str.as_bytes();
        let email_bytes = email_str.as_bytes();

        if username_bytes.len() > COLUMN_USERNAME_SIZE || email_bytes.len() > COLUMN_EMAIL_SIZE {
            return PrepareResult::SyntaxError;
        }

        let mut row = Row::empty();
        row.id = id;
        row.username[..username_bytes.len()].copy_from_slice(username_bytes);
        row.email[..email_bytes.len()].copy_from_slice(email_bytes);

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
