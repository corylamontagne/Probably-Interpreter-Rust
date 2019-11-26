
pub use crate::globalstate::GlobalState;

//a single instruction to be executed with the global state
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

