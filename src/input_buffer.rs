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
        self.normalize();

        Ok(())
    }

    pub fn is_meta_command(&self) -> bool {
        self.buffer.starts_with('.')
    }

    fn normalize(&mut self) {
        self.buffer = self.buffer.trim().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::InputBuffer;

    #[test]
    fn normalize_trims_newline() {
        let mut buf = InputBuffer {
            buffer: "hello\n".to_string(),
        };

        buf.normalize();

        assert_eq!(buf.buffer, "hello");
    }

    #[test]
    fn normalize_trims_crlf() {
        let mut buf = InputBuffer {
            buffer: "hello\n\r".to_string(),
        };

        buf.normalize();

        assert_eq!(buf.buffer, "hello");
    }

    #[test]
    fn normalize_handles_empty_string() {
        let mut buf = InputBuffer {
            buffer: "".to_string(),
        };
        buf.normalize();

        assert_eq!(buf.buffer, "");
    }

    #[test]
    fn is_meta_command_true_for_dot() {
        let buf = InputBuffer {
            buffer: ".exit".to_string(),
        };

        assert!(buf.is_meta_command());
    }

    #[test]
    fn is_meta_command_false_for_normal() {
        let buf = InputBuffer {
            buffer: "select".to_string(),
        };

        assert!(!buf.is_meta_command());
    }
}
