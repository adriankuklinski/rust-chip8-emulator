pub struct Cpu {
    pub stack: [u16; 16],     // Stack
    pub mem: [u8; 4096],      // Memory
    pub i: u16,               // Index register 
    pub v: [u8; 16],          // Registers
    pub pc: u16,              // Program counter
    pub fb: [u8; 64 * 32],    // Frame buffer
    pub sp: u8,               // Stack pointer
    pub dt: u8,               // Delay timer
    pub st: u8,               // Sound timer
    pub keypad: u16,          // Keypad 
}
