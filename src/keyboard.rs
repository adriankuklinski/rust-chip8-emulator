pub struct Keyboard {
    keys: [bool; 16],
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            keys: [false; 16],
        }
    }

    pub fn key_down(&mut self, key: usize) {
        if key < self.keys.len() {
            self.keys[key] = true;
        }
    }

    pub fn key_up(&mut self, key: usize) {
        if key < self.keys.len() {
            self.keys[key] = false;
        }
    }

    pub fn is_key_pressed(&self, key: usize) -> bool {
        key < self.keys.len() && self.keys[key]
    }
}
