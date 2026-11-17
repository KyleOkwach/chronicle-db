use std::io::{ self, Write };
use crate::meta;

pub struct Repl {
    buffer: String,
}

impl Repl {
    pub fn new() -> Self {
        Self {
            buffer: String::with_capacity(1024),  // Buffer size
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        let stdin = io::stdin;

        loop {
            // Print prompt
            let _ =self.print_prompt()?;

            // Read input and store in buffer
            let bytes_read = stdin().read_line(&mut self.buffer)?;

            if bytes_read == 0 {
                println!("Exiting...");
                break;
            }

            let trimmed  = self.buffer.trim();  // Trim trailing \n
            
            if trimmed.is_empty() {
                self.buffer.clear();
                continue;
            }

            if trimmed.starts_with('\\') && !trimmed.contains('\n') {
                if !meta::execute(trimmed) {
                    break;
                }

                self.buffer.clear();
                continue;
            }

            if trimmed.ends_with(';') {
                self.execute_sql(trimmed);
                self.buffer.clear();
            }
        }

        Ok(())
    }

    fn execute_sql(&self, sql: &str) {
        /*
         * Execute SQL Command
         */

        println!("Running command {}", sql);
    }

    fn print_prompt(&self) -> io::Result<()> {
        let mut stdout = io::stdout();

        if self.buffer.is_empty() {
            print!("db > ");
        } else {
            print!("  -> ");
        }

        stdout.flush()?;
        Ok(())
    }
}
