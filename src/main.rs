mod executor;
mod input_buffer;
mod parser;
mod row;
mod statement;
mod table;

use executor::execute_statement;
use input_buffer::InputBuffer;
use parser::{do_meta_command, prepare_statement};
use statement::{MetaCommandResult, PrepareResult, Statement, StatementType};

use crate::table::Table;

fn main() {
    let mut input = InputBuffer::new();
    let mut table = Table::new();

    loop {
        InputBuffer::print_prompt();

        if let Err(err) = input.read_input() {
            eprintln!("Error reading input: {}", err);
            continue;
        }

        if input.buffer.is_empty() {
            continue;
        }

        if input.buffer.starts_with('.') {
            match do_meta_command(&input) {
                MetaCommandResult::Success => continue,
                MetaCommandResult::UnrecognizedCommand => {
                    println!("Unrecognized command {}", input.buffer);
                    continue;
                }
            }
        }

        let mut statement = Statement::new();

        match prepare_statement(&input, &mut statement) {
            PrepareResult::Success => {
                execute_statement(&statement, &mut table);
                println!("Executed.");
            }
            PrepareResult::UnrecognizedStatement => {
                println!("Unrecognized keyword at start of '{}'.", input.buffer);
            }
            PrepareResult::SyntaxError => {
                println!("Syntax error. Could not parse statement");
            }
        }
    }
}
