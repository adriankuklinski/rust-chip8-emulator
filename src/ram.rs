struct Ram {
    mem: [u8; 4096],
}

impl Ram {
    fn new() -> Ram {
        Ram {
            mem: [0; 4096]
        }
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        
    }

    fn read_byte(&mut self, address: u16, value: u8) {
        
    }
}
