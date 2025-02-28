use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();

    // Write number to standard output, followed by a newline
    for i in 1..=5 {
        writeln!(stdout, "{}", i)?; 
    }

    Ok(())
}