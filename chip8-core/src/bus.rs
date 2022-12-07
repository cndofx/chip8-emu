use crate::{memory::Memory, display::Display, keyboard::Keyboard};

#[derive(Debug)]
pub(crate) struct Bus {
    pub memory: Memory,
    pub display: Display,
    pub keyboard: Keyboard,
}

impl Bus {
    pub fn new() -> Self {
        Self {
            memory: Memory::default(),
            display: Display::new(),
            keyboard: Keyboard::new(),
        }
    }
}