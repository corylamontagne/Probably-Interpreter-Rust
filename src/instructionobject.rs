
pub use crate::instructions::Instruction;

//an instruction object which contains all protential instructions for a given function and their rate of occurance
pub struct InstructionObject {
}

impl InstructionObject {
    pub fn new(func: Box<dyn Fn(&mut GlobalState) -> ()>) -> InstructionObject {
        InstructionObject {
        }
    }
}

