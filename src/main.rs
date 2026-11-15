mod shell;

use crate::shell::repl::Repl;

fn main() {
   let mut repl = Repl::new();

   let _ =repl.run();
}
