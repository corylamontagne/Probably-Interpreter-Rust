
pub use crate::globalstate::GlobalState;

//configuration data used to set immutable prob properties
pub struct Instruction {
    pub call: Box<dyn Fn(&mut GlobalState) -> ()>,
}

impl Instruction {
    pub fn new(func: Box<dyn Fn(&mut GlobalState) -> ()>) -> Instruction {
        Instruction {
            call: func,
        }
    }

    pub fn call_fn(&mut self, state: &mut GlobalState) {
        (self.call)(state);
    }
}

