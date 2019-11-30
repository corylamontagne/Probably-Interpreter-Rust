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
pub use crate::instructionobject::InstructionObject;

mod configuration;
mod globalstate;
mod instructions;
mod instructionobject;

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

fn int_cur_index(state: &mut globalstate::GlobalState) {     
    state.data[state.current_index] = std::char::from_u32(state.data[state.current_index] as u32 + 1).unwrap_or(0 as char);
}
fn dec_cur_index(state: &mut globalstate::GlobalState) {
    state.data[state.current_index] = std::char::from_u32(state.data[state.current_index] as u32 - 1).unwrap_or(0 as char);
}

fn main() {
    // let scriptdata = parse_file(env::args().nth(1).expect("No script given"));
    let config = configuration::Configuration::new();
    let mut global_state = globalstate::GlobalState::new();
    let mut i1 = instructions::Instruction::new(Box::new(int_cur_index));
    let mut i2 = instructions::Instruction::new(Box::new(dec_cur_index));

    let inst_ibj = instructionobject::InstructionObject::new(config, (100, i1), (10, i2));


    //TODO: Why does the global state not change here? I think I hosed the probability checks in the instructino object.
    println!("{}", global_state.data[0]); //A
    inst_ibj.call_fn(&mut global_state, 125, 0, 1.0); //inc -> B
    println!("{}", global_state.data[0]); //B
    inst_ibj.call_fn(&mut global_state, 9, 0, 1.0); // NOP -> B
    println!("{}", global_state.data[0]);//B
    inst_ibj.call_fn(&mut global_state, 12, 0, 1.0); //dec -> A
    println!("{}", global_state.data[0]);//A
    inst_ibj.call_fn(&mut global_state, 125, 0, 1.0); //inc -> B
    inst_ibj.call_fn(&mut global_state, 125, 0, 1.0); //inc -> C
    println!("{}", global_state.data[0]);//C

    // match scriptdata {
    //     Some(x) => println!("Contents: {}", x),
    //     None    => println!("No contents or script failed"),
    // }
}
