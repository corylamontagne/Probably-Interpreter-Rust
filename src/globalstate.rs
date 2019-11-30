
//this will be the global state; including what the current data, index, registers, etc are at any time
//this will get trick I think as in C++ we use ptrs to manipulate data dn indices, which we wont be doing here
pub struct GlobalState {
    pub current_index: usize,
    pub data: [char; 8],
    pub x_register: char,
    pub y_register: char,
    pub register_check_passed: bool,
}

impl GlobalState {
    pub fn new() -> GlobalState {
        GlobalState {
            current_index: 0,
            data: [65 as char; 8],
            x_register: 0 as char,
            y_register: 0 as char,
            register_check_passed: true,
        }
    }
}