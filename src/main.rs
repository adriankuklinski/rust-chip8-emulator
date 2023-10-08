use std::fs::File;
use std::io::Read;
use rust_chip8_emulator::chip8::Chip8;

fn main() {
    let mut file = File::open("data/INVADERS").unwrap();
    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data);

    let mut chip8 = Chip8::new();
    chip8.load_rom(&data);
}
