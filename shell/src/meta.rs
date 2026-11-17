pub fn execute(cmd: &str) -> bool {
    /*
     * Execute meta command
     * These are non-SQL commands
     */
    
    match cmd {
        "\\q"     => return false,
        "\\h"     => println!("\\q to quit the program"),
        _           => println!("Unrecognized command {}.", cmd)
    }

    true
}
