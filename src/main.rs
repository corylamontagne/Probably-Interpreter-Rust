#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(dead_code)]

use std::fs::File;
use std::io::prelude::*;
use std::env;

pub use crate::configuration::Configuration;
pub use crate::globalstate::GlobalState;
pub use crate::instructions::Instruction;

mod configuration;
mod globalstate;
mod instructions;

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


struct State {
    regs: Vec<char>
}

struct Instr(Box<dyn Fn(&mut State) -> ()>);

fn gen_run_inc(state: &mut globalstate::GlobalState) {
    state.data[state.current_index] = std::char::from_u32(state.data[state.current_index] as u32 + 1).unwrap_or(state.data[state.current_index]);
}

fn main() {
    // let scriptdata = parse_file(env::args().nth(1).expect("No script given"));
    // let config = configuration::Configuration::new();
    let mut global_state = globalstate::GlobalState::new();
    let mut i1 = instructions::Instruction::new(Box::new(gen_run_inc));

    // let mut state = State {
    //     regs: vec![65 as char]
    // };
    // let instructions = vec![Instr(gen_run_inc(0)),];

    println!("{}", global_state.data[0]);
    i1.call_fn(&mut global_state);
    println!("{}", global_state.data[0]);

    //  for instr in &instructions {
    //      instr.0(&mut state);
    // }
    // println!("{}", state.regs[0]);

    // match scriptdata {
    //     Some(x) => println!("Contents: {}", x),
    //     None    => println!("No contents or script failed"),
    // }
    // let f = 'z';
    // println!("{}", std::char::from_u32(f as u32 + 1).unwrap_or(f));
}
