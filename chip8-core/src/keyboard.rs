#[derive(Debug)]
pub(crate) struct Keyboard {
    keys: [bool; 16],
}

impl Keyboard {
    pub fn new() -> Self {
        Self { keys: [false; 16] }
    }

    pub fn set_key_state(&mut self, key: usize, state: bool) {
        self.keys[key] = state;
    }

    /// checks if a given key is pressed
    pub fn is_pressed(&self, key: u8) -> bool {
        self.keys[key as usize]
    }

    /// get first key that is pressed, otherwise return None
    pub fn get_pressed(&self) -> Option<u8> {
        for (i, key) in self.keys.iter().enumerate() {
            if *key == true {
                return Some(i as u8);
            }
        }
        None
    }
}
