use std::io::{self, Write};

pub struct InputBuffer {
    pub buffer: String,
}

impl InputBuffer {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
        }
    }

    pub fn print_prompt() {
        print!("db > ");
        io::stdout().flush().unwrap();
    }

    pub fn read_input(&mut self) -> io::Result<()> {
        self.buffer.clear();
        io::stdin().read_line(&mut self.buffer)?;

        self.buffer = self.buffer.trim().to_string();

        Ok(())
    }
}
