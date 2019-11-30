
use std::collections::HashMap;
pub use crate::globalstate::GlobalState;
pub use crate::instructionobject::InstructionObject;

//The lookup table for instruction code to Instruction object
pub struct LookupTable {
    pub operation_lookup: HashMap<char, InstructionObject>,
}

impl LookupTable {
    pub fn new() -> LookupTable {
        LookupTable {
            operation_lookup: HashMap::new(),
        }
    }

    pub fn add_instruction(&mut self, instruction: (char, InstructionObject)){
        (self.operation_lookup).entry(instruction.0).or_insert(instruction.1);
    }

    pub fn fetch_instruction(&mut self, instruction: char) -> Option<&mut InstructionObject> {
        match (self.operation_lookup).get_mut(&instruction) {
            Some(x) => return Some(x),
            None => return None
        }
    }
}

