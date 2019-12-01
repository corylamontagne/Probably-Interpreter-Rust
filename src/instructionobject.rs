
use std::rc::Rc; 
pub use crate::globalstate::GlobalState;
pub use crate::instructions::Instruction;
pub use crate::configuration::Configuration;


//an instruction object which contains all protential instructions for a given function and their rate of occurance
pub struct InstructionObject {
    pub probable_ops: Vec::<(i64, Rc::<Instruction>)>,
    pub configuration: Configuration,
}

fn nop(state: &mut GlobalState) {
    }

impl InstructionObject {
    pub fn new(config: Configuration, operation: (i64, Rc::<Instruction>), inverse: (i64, Rc::<Instruction>)) -> InstructionObject {
        InstructionObject {
            probable_ops: vec![(operation.0, operation.1), (inverse.0, inverse.1), (0, Rc::new(Instruction::new(Box::new(nop))))],
            configuration: config,
        }
    }
    
    //TODO: Handle the multiplier
    pub fn call_fn(&self, state: &mut GlobalState, probability: i64, probability_modifier: i64, probability_multiplier: f64) -> f64 {
        let final_probability = probability + probability_modifier;
        let mut execution_probability = 1.;

        for (chance, op) in &self.probable_ops {
            if final_probability >= *chance {
                (op).call_fn(state);
                execution_probability = (*chance as f64) / (self.configuration.max_prob as f64);
                return execution_probability;
            }
        }

        execution_probability
    }
}

