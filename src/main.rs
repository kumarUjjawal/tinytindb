mod executor;
mod input_buffer;
mod parser;
mod statement;

use executor::execute_statement;
use input_buffer::InputBuffer;
use parser::{do_meta_command, prepare_statement};
use statement::{MetaCommandResult, PrepareResult, Statement, StatementType};

fn main() {
    let mut input = InputBuffer::new();

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

        let mut statement = Statement {
            stype: StatementType::Select,
        };

        match prepare_statement(&input, &mut statement) {
            PrepareResult::Success => { /* continue */ }
            PrepareResult::UnrecognizedStatement => {
                println!("Unrecognized keyword at start of '{}'.", input.buffer);
                continue;
            }
        }

        execute_statement(&statement);
        println!("Executed");
    }
}
