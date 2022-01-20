use winit_input_helper::WinitInputHelper;

use crate::cpu::CPU;

pub struct System {
    cpu: CPU,
}

impl System {
    pub fn new() -> System {
        System {
            cpu: CPU::new(),
        }
    }

    pub fn load_rom(&mut self, rom_data: &Vec<u8>) {
        let offset = 0x200;
        for i in 0..rom_data.len() {
            self.cpu.memory.write_byte((i + offset) as u16, rom_data[i]);
        }
    }

    pub fn execute(&mut self) {
        self.cpu.execute();
    }

    pub fn get_screen(&self) -> &[u8] {
        self.cpu.get_screen()
    }

    pub fn update_keys(&mut self, input: &WinitInputHelper) {
        self.cpu.keyboard.update_keys(input);
    }
}