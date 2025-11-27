use crate::input_buffer::InputBuffer;
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
    let cmd = input.buffer.as_str();

    if cmd.starts_with("Insert") {
        statement.stype = StatementType::Insert;
        return PrepareResult::Success;
    }

    if cmd.starts_with("Select") {
        statement.stype = StatementType::Select;
        return PrepareResult::Success;
    }

    PrepareResult::UnrecognizedStatement
}
