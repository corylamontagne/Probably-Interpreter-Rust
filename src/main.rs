use std::fs::File;
use std::io::prelude::*;

fn main() {
    let filename = "test.prob";
    match File::open(filename) {
            // The file is open (no error).
            Ok(mut file) => {
                let mut content = String::new();

                // Read all the file content into a variable (ignoring the result of the operation).
                file.read_to_string(&mut content).unwrap();

                println!("{}", content);

                // The file is automatically closed when is goes out of scope.
            },
            // Error handling.
            Err(error) => {
                println!("Error opening file {}: {}", filename, error);
            },
        }
}
