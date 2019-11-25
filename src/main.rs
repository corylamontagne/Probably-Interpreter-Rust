#![allow(unused_variables)]
#![allow(unused_mut)]

use std::fs::File;
use std::io::prelude::*;
use std::env;

pub use crate::configuration::Configuration;
pub use crate::globalstate::GlobalState;

mod configuration;
mod globalstate;

fn parse_file(file: String) -> Option<String> {
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
            return None;
        },
    }
    Some(content)
}

fn main() {
    let scriptdata = parse_file(env::args().nth(1).expect("No script given"));
    let config = configuration::Configuration::new();
    let mut global_state = globalstate::GlobalState::new();
    
    match scriptdata {
        Some(x) => println!("Contents: {}", x),
        None    => println!("No contents or script failed"),
    }
    // let f = 'z';
    // println!("{}", std::char::from_u32(f as u32 + 1).unwrap_or(f));
}
