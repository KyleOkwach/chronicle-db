mod repl;

use crate::repl::Repl;

fn main() {
    let mut repl = Repl::new();
    let _ = repl.run();
}
