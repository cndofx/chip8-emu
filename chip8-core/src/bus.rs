use crate::{memory::Memory, display::Display};

#[derive(Debug)]
pub(crate) struct Bus {
    pub memory: Memory,
    pub display: Display,
}

impl Bus {
    pub fn new() -> Self {
        Self {
            memory: Memory::default(),
            display: Display::new(),
        }
    }
}