use std::fs::File;
use std::io::prelude::*;
use std::env;

fn parse_file(file: String) -> String {
    let mut content = String::new();
    match File::open(file.clone()) {
        // The file is open (no error).
        Ok(mut file) => {
            // Read contents to string
            file.read_to_string(&mut content).unwrap();
        },
        // Error handling.
        Err(error) => {
            println!("Error opening file {}: {}", file, error);
        },
    }
    content
}

fn main() {
    let scriptdata = parse_file(env::args().nth(1).expect("No script given"));
    
    println!("Contents: {}", scriptdata);
}
