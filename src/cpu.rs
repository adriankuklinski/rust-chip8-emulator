pub struct Cpu {
    pub index_register: u16,
    pub program_counter: u16,
    pub memory: [u8; 4096],
    pub registers: [u8; 16],
    pub keypad: Keypad,
    pub display: Display,
    pub stack: [u16; 16],
    pub stack_pointer: u8,
    pub delay_timer: u8,
}
