#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(dead_code)]

use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::rc::Rc;

pub use crate::configuration::Configuration;
pub use crate::globalstate::GlobalState;
pub use crate::instructions::Instruction;
pub use crate::instructionobject::InstructionObject;
pub use crate::lookuptable::LookupTable;

mod configuration;
mod globalstate;
mod instructions;
mod instructionobject;
mod lookuptable;

//macro to build ref counted boxed instruction objects
macro_rules! function_object {
    ($($x: expr),*) => {{
        $(
        Rc::new(instructions::Instruction::new(Box::new($x)))
        )*
    }}
}

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

fn build_lookup_table(lookup: &mut LookupTable, config: &Configuration) {
    let mut nop = ||{};
    let mut increment = function_object!(|state: &mut GlobalState| {state.data[state.current_index] = std::char::from_u32(state.data[state.current_index] as u32 + 1).unwrap_or(0 as char)});
    let mut decrement = function_object!(|state: &mut GlobalState| {state.data[state.current_index] = std::char::from_u32(state.data[state.current_index] as u32 - 1).unwrap_or(0 as char)});

    let mut register_forward = function_object!(|state: &mut GlobalState| {if state.current_index < 8 { state.current_index+= 1}});
    let mut register_back = function_object!(|state: &mut GlobalState| { if state.current_index > 0 {state.current_index-= 1}});

    let mut print_register = function_object!(|state: &mut GlobalState| {println!("{}", state.data[state.current_index])});

    let mut load_x_register = function_object!(|state: &mut GlobalState| {state.x_register = state.data[state.current_index]});
    let mut load_y_register = function_object!(|state: &mut GlobalState| {state.y_register = state.data[state.current_index]});

    let mut register_check = function_object!(|state: &mut GlobalState| {state.register_check_passed = state.y_register == state.x_register});
    let mut inverted_register_check = function_object!(|state: &mut GlobalState| {state.register_check_passed = state.y_register != state.x_register});

    //TODO: Implement probability gate instructions/logic

    //TODO: Implement function parser instruction/logic

    let min_prob = config.min_prob;
    let high_end_probability = config.max_prob - (min_prob + config.nop_prob);
    
    //TODO: Add instruction to the lookup table
    lookup.add_instruction(('i', InstructionObject::new(config.clone(), (high_end_probability, Rc::clone(&increment)), (min_prob, Rc::clone(&decrement)))));
    lookup.add_instruction(('d', InstructionObject::new(config.clone(), (high_end_probability, Rc::clone(&decrement)), (min_prob, Rc::clone(&increment)))));
}

fn main() {
    // let scriptdata = parse_file(env::args().nth(1).expect("No script given"));
    let config = configuration::Configuration::new();
    let mut global_state = globalstate::GlobalState::new();
    let mut lookup_table = lookuptable::LookupTable::new();

    let mut cli1 = |state: &mut GlobalState| {state.data[state.current_index] = std::char::from_u32(state.data[state.current_index] as u32 + 1).unwrap_or(0 as char)};
    let mut decrement = function_object!(|state: &mut GlobalState| {state.data[state.current_index] = std::char::from_u32(state.data[state.current_index] as u32 - 1).unwrap_or(0 as char)});

    let mut i1 = instructions::Instruction::new(Box::new(cli1));

    let inst_ibj = instructionobject::InstructionObject::new(config, (100, Rc::new(i1)), (10, decrement));


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
