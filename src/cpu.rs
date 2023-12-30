use rand;
use crate::ram::Ram;

pub struct Cpu {
    pub stack: [u16; 16],     // Stack
    pub fb: [u8; 64 * 32],    // Frame buffer
    pub v: [u8; 16],          // Registers
    pub i: u16,               // Index register
    pub pc: u16,              // Program counter
    pub sp: u8,               // Stack pointer
    pub dt: u8,               // Delay timer
    pub st: u8,               // Sound timer
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            stack: [0; 16],
            fb: [0; 64 * 32],
            v: [0; 16],
            i: 0,
            pc: 0,
            sp: 0,
            dt: 0,
            st: 0,
        }
    }

    pub fn get_stack(&self) -> Vec<u16> {
        self.stack.to_vec()
    }

    pub fn get_v_register(&self) -> Vec<u8> {
        self.v.to_vec()
    }

    pub fn get_fb_register(&self) -> Vec<u8> {
        self.fb.to_vec()
    }

    pub fn set_stack(&mut self, value: &[u16]) {
        for (i, &val) in value.iter().enumerate() {
            if i < self.stack.len() {
                self.stack[i] = val;
            }
        }
    }

    pub fn set_v_register(&mut self, value: &[u8]) {
        for (i, &val) in value.iter().enumerate() {
            if i < self.v.len() {
                self.v[i] = val;
            }
        }
    }

    pub fn set_fb_register(&mut self, value: &[u8]) {
        for (i, &val) in value.iter().enumerate() {
            if i < self.fb.len() {
                self.fb[i] = val;
            }
        }
    }

    pub fn fetch_opcode(&self, ram: &mut Ram) -> u16 {
        let hi = ram.read_byte(self.pc) as u16;
        let lo = ram.read_byte(self.pc) as u16;
        (hi << 8) | lo
    }

    pub fn execute_opcode(&mut self, ram: &mut Ram, opcode: u16) {
        match opcode & 0xF000 {
            0x0000 => match opcode & 0x00FF {
                // 00E0: Clear the screen
                0x00E0 => {
                    self.fb = [0; 64 * 32];
                },
                // 00EE: Return from subroutine
                0x00EE => {
                    self.sp -= 1;
                    self.pc = self.stack[self.sp as usize];
                },
                _ => eprintln!("Unknown opcode 0x{:04X}", opcode),
            },
            // 1nnn: Jump to location nnn
            0x1000 => {
                let address = opcode & 0x0FFF;
                self.pc = address;
                return; // Jump to the address, don't increment PC
            },
            // 2nnn: Call subroutine at nnn
            0x2000 => {
                let address = opcode & 0x0FFF;
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = address;
                return; // Call the address, don't increment PC
            },
            // 3xkk: Skip next instruction if Vx = kk
            0x3000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let kk = (opcode & 0x00FF) as u8;
                if self.v[x] == kk {
                    self.pc += 2; // Skip the next instruction
                }
            },
            // 4xkk: Skip next instruction if Vx != kk
            0x4000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let kk = (opcode & 0x00FF) as u8;
                if self.v[x] != kk {
                    self.pc += 2; // Skip the next instruction
                }
            },
            // 5xy0: Skip next instruction if Vx = Vy
            0x5000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let y = ((opcode & 0x00F0) >> 4) as usize;
                if self.v[x] == self.v[y] {
                    self.pc += 2; // Skip the next instruction
                }
            },
            // 6xkk: Set Vx = kk
            0x6000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let kk = (opcode & 0x00FF) as u8;
                self.v[x] = kk;
            },
            // 7xkk: Set Vx = Vx + kk
            0x7000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let kk = (opcode & 0x00FF) as u8;
                let result = self.v[x] as u16 + kk as u16;
                self.v[x] = result as u8; // Note: No carry flag is changed
            },
            // 8xyN: A variety of arithmetic/logical operations
            0x8000 => {
                self.execute_8xyx_opcodes(ram, opcode);
            },
            // 9xy0: Skip next instruction if Vx != Vy
            0x9000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let y = ((opcode & 0x00F0) >> 4) as usize;
                if self.v[x] != self.v[y] {
                    self.pc += 2; // Skip the next instruction
                }
            },
            // Annn: Set I = nnn
            0xA000 => {
                self.i = opcode & 0x0FFF;
            },
            // Bnnn: Jump to location nnn + V0
            0xB000 => {
                self.pc = (opcode & 0x0FFF) + self.v[0] as u16;
                return; // Jump to the address, don't increment PC
            },
            // Cxkk: Set Vx = random byte AND kk
            0xC000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let kk = (opcode & 0x00FF) as u8;
                self.v[x] = rand::random::<u8>() & kk;
            },
            // Dxyn: Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision
            0xD000 => {
                // The actual implementation for this opcode can be quite lengthy.
                // It involves drawing sprites to the display and checking for collisions.
            },
            // Ex9E and ExA1: Skip instructions based on key state
            0xE000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                match opcode & 0x00FF {
                    0x9E => {
                        // Skip next instruction if key with the value of Vx is pressed
                    },
                    0xA1 => {
                        // Skip next instruction if key with the value of Vx is not pressed
                    },
                    _ => eprintln!("Unknown opcode 0x{:04X}", opcode),
                }
            },
            // Fx07, Fx0A, Fx15, Fx18, Fx1E, Fx29, Fx33, Fx55, Fx65: A range of opcodes for timers, memory operations, etc.
            0xF000 => {
                self.execute_fxnn_opcodes(ram, opcode);
            },
            _ => eprintln!("Unknown opcode 0x{:04X}", opcode),
        }

        // Increment program counter, except for jump/call opcodes where PC is modified directly
        self.pc += 2;
    }

    fn execute_8xyx_opcodes(&mut self, ram: &mut Ram, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;

        match opcode & 0x000F {
            0x0000 => self.v[x] = self.v[y],                     // 8xy0 - LD Vx, Vy
            0x0001 => self.v[x] |= self.v[y],                    // 8xy1 - OR Vx, Vy
            0x0002 => self.v[x] &= self.v[y],                    // 8xy2 - AND Vx, Vy
            0x0003 => self.v[x] ^= self.v[y],                    // 8xy3 - XOR Vx, Vy
            0x0004 => {                                          // 8xy4 - ADD Vx, Vy
                let sum = self.v[x] as u16 + self.v[y] as u16;
                self.v[0xF] = if sum > 0xFF { 1 } else { 0 };
                self.v[x] = sum as u8;
            },
            0x0005 => {                                          // 8xy5 - SUB Vx, Vy
                self.v[0xF] = if self.v[x] > self.v[y] { 1 } else { 0 };
                self.v[x] = self.v[x].wrapping_sub(self.v[y]);
            },
            0x0006 => {                                          // 8xy6 - SHR Vx {, Vy}
                self.v[0xF] = self.v[x] & 1;
                self.v[x] >>= 1;
            },
            0x0007 => {                                          // 8xy7 - SUBN Vx, Vy
                self.v[0xF] = if self.v[y] > self.v[x] { 1 } else { 0 };
                self.v[x] = self.v[y].wrapping_sub(self.v[x]);
            },
            0x000E => {                                          // 8xyE - SHL Vx {, Vy}
                self.v[0xF] = self.v[x] >> 7;
                self.v[x] <<= 1;
            },
            _ => eprintln!("Unknown 8xyx opcode 0x{:04X}", opcode),
        }
    }

    fn execute_fxnn_opcodes(&mut self, ram: &mut Ram, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;

        match opcode & 0x00FF {
            0x0007 => self.v[x] = self.dt,                        // Fx07 - LD Vx, DT
            0x000A => {                                           // Fx0A - LD Vx, K
                                                                  // Wait for a key press, store the value of the key in Vx
                                                                  // Implementation depends on your input handling
            },
            0x0015 => self.dt = self.v[x],                        // Fx15 - LD DT, Vx
            0x0018 => self.st = self.v[x],                        // Fx18 - LD ST, Vx
            0x001E => {                                           // Fx1E - ADD I, Vx
                self.i += self.v[x] as u16;
            },
            0x0029 => {                                           // Fx29 - LD F, Vx
                                                                  // Set I to the location of the sprite for the character in Vx
                                                                  // Characters 0-F (in hexadecimal) are represented by a 4x5 font
            },
            0x0033 => {                                           // Fx33 - LD B, Vx
                ram.write_byte(self.i, self.v[x] / 100);
                ram.write_byte(self.i, (self.v[x] / 10) % 10);
                ram.write_byte(self.i, (self.v[x] % 100) % 10);
            },
            0x0055 => {                                           // Fx55 - LD [I], Vx
                                                                  // Store registers V0 through Vx in memory starting at location I
                for idx in 0..=x {
                    ram.write_byte(self.i, self.v[idx]);
                }
            },
            0x0065 => {                                           // Fx65 - LD Vx, [I]
                                                                  // Read registers V0 through Vx from memory starting at location I
                for idx in 0..=x {
                    self.v[idx] = ram.read_byte(self.i);
                }
            },
            _ => eprintln!("Unknown fxnn opcode 0x{:04X}", opcode),
        }
    }

    pub fn update_timers(&mut self) {
        if self.dt > 0 { self.dt -= 1; }
        if self.st > 0 { self.st -= 1; }
    }
}
