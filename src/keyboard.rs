use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

// CHIP8   = QWERTY
// 1 2 3 C = 1 2 3 4
// 4 5 6 D = Q W E R
// 7 8 9 E = A S D F
// A 0 B F = Z X C V

pub struct Keyboard {
    keys: [bool; 16],
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            keys: [false; 16],
        }
    }

    pub fn update_keys(&mut self, input: &WinitInputHelper) {
        self.keys[0x1] = input.key_held(VirtualKeyCode::Key1); 
        self.keys[0x2] = input.key_held(VirtualKeyCode::Key2); 
        self.keys[0x3] = input.key_held(VirtualKeyCode::Key3); 
        self.keys[0xC] = input.key_held(VirtualKeyCode::Key4);

        self.keys[0x4] = input.key_held(VirtualKeyCode::Q); 
        self.keys[0x5] = input.key_held(VirtualKeyCode::W); 
        self.keys[0x6] = input.key_held(VirtualKeyCode::E); 
        self.keys[0xD] = input.key_held(VirtualKeyCode::R); 

        self.keys[0x7] = input.key_held(VirtualKeyCode::A); 
        self.keys[0x8] = input.key_held(VirtualKeyCode::S); 
        self.keys[0x9] = input.key_held(VirtualKeyCode::D); 
        self.keys[0xE] = input.key_held(VirtualKeyCode::F); 

        self.keys[0xA] = input.key_held(VirtualKeyCode::Z); 
        self.keys[0x0] = input.key_held(VirtualKeyCode::X); 
        self.keys[0xB] = input.key_held(VirtualKeyCode::C); 
        self.keys[0xF] = input.key_held(VirtualKeyCode::V); 
    }

    pub fn is_key_pressed(&self, key: u8) -> bool {
        self.keys[key as usize]
    }
}