
//configuration data used to set immutable prob properties
pub struct Configuration {
    pub max_prob: i64,
    pub min_prob: i64,
    pub nop_prob: i64,
    pub max_instruction: i64,
    pub function_probability_gate: i64,
    pub if_block_probability: i64,
    pub function_down_multiplier: f64,
    pub function_up_multiplier: f64,
}

impl Configuration {
    pub fn new() -> Configuration {
        Configuration {
            max_prob: 1_000_000_000,
            min_prob: 500,
            nop_prob: 100,
            max_instruction: 20,
            function_probability_gate: 10_000,
            if_block_probability: 10_000,
            function_down_multiplier: 1_000.0,
            function_up_multiplier: 0.1,
        }
    }
}