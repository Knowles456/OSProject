use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    //initiate handler for the process's standard input
    let stdin = io::stdin();
    // Buffered reader initialized to read standard input. Grabs mutex of shared global buffer
    let reader = io::BufReader::new(stdin.lock()); 

    //for loop uses .lines to iterate over incoming data line by line
    for line in reader.lines() {
        // Read a line from stdin with ? operator to handle any errors
        let line = line?; 
        // Parse sttdin to integer to perform operation
        let number: i32 = line.trim().parse().unwrap(); 
        let squared_number = number * number;
        // Write squared number to stdout
        println!("{}", squared_number); 
    }

    Ok(())
}