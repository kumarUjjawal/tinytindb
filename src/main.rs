mod input_buffer;
use crate::input_buffer::InputBuffer;

fn main() {
    let mut input_buffer = InputBuffer::new();

    loop {
        InputBuffer::print_prompt();

        if let Err(err) = input_buffer.read_input() {
            eprintln!("Error reading input: {}", err);
            continue;
        }

        if input_buffer.buffer.is_empty() {
            continue;
        }

        match input_buffer.buffer.as_str() {
            "exit()" => {
                input_buffer.close();
                println!("Bye!");
                break;
            }

            cmd => {
                println!("Unrecognized command: {}", cmd);
            }
        }
    }
}
