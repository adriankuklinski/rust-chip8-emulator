use crate::ram::Ram;
use crate::cpu::Cpu;
use web_sys::CanvasRenderingContext2d;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use js_sys::Uint8Array;

#[wasm_bindgen]
pub struct Chip8 {
    ram: Ram,
    cpu: Cpu,
}

#[wasm_bindgen]
impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            ram: Ram::new(),
            cpu: Cpu::new(),
        }
    }

    pub fn get_frame_buffer_ptr(&self) -> *const u8 {
        self.cpu.fb.as_ptr()
    }

    pub fn copy_frame_buffer_to_js(&self, js_array: Uint8Array) {
        js_array.copy_from(&self.cpu.fb);
    }

    pub fn get_rom(&mut self, game_name: &str) {
        match game_name {
            "15puzzle" => self.load_rom_data(include_bytes!("../games/15PUZZLE")),
            "blinky" => self.load_rom_data(include_bytes!("../games/BLINKY")),
            "blitz" => self.load_rom_data(include_bytes!("../games/BLITZ")),
            "brix" => self.load_rom_data(include_bytes!("../games/BRIX")),
            "connect4" => self.load_rom_data(include_bytes!("../games/CONNECT4")),
            "guess" => self.load_rom_data(include_bytes!("../games/GUESS")),
            "hidden" => self.load_rom_data(include_bytes!("../games/HIDDEN")),
            "invaders" => self.load_rom_data(include_bytes!("../games/INVADERS")),
            "kaleid" => self.load_rom_data(include_bytes!("../games/KALEID")),
            "maze" => self.load_rom_data(include_bytes!("../games/MAZE")),
            "merlin" => self.load_rom_data(include_bytes!("../games/MERLIN")),
            "missile" => self.load_rom_data(include_bytes!("../games/MISSILE")),
            "pong" => self.load_rom_data(include_bytes!("../games/PONG")),
            "pong2" => self.load_rom_data(include_bytes!("../games/PONG2")),
            "puzzle" => self.load_rom_data(include_bytes!("../games/PUZZLE")),
            "syzygy" => self.load_rom_data(include_bytes!("../games/SYZYGY")),
            "tank" => self.load_rom_data(include_bytes!("../games/TANK")),
            "tetris" => self.load_rom_data(include_bytes!("../games/TETRIS")),
            "tictac" => self.load_rom_data(include_bytes!("../games/TICTAC")),
            "ufo" => self.load_rom_data(include_bytes!("../games/UFO")),
            "vbrix" => self.load_rom_data(include_bytes!("../games/VBRIX")),
            "vers" => self.load_rom_data(include_bytes!("../games/VERS")),
            "wipeoff" => self.load_rom_data(include_bytes!("../games/WIPEOFF")),
            _ => panic!("Unknown game"),
        }
    }

    pub fn load_rom_data(&mut self, data: &[u8]) {
        let offset = 0x200;
        for i in 0..data.len() {
            self.ram.write_byte((offset + i) as u16, data[i]);
        }
    }

    pub fn run_cycle(&mut self) {
        let opcode = self.cpu.fetch_opcode(&mut self.ram);
        self.cpu.execute_opcode(&mut self.ram, opcode);
        self.cpu.update_timers();
        self.update_display();
    }

    pub fn update_display(&self) {
        let window = web_sys::window().expect("should have a window in this context");
        let document = window.document().expect("window should have a document");

        let canvas = document.get_element_by_id("chip8-canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        for (index, &pixel) in self.cpu.fb.iter().enumerate() {
            let x = (index % 64) as f64;
            let y = (index / 64) as f64;

            context.set_fill_style(&JsValue::from_str(if pixel == 0 { "black" } else { "white" }));
            context.fill_rect(x, y, 10.0, 10.0);
        }
    }
}
