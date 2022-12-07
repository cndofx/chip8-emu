#[derive(Debug)]
pub(crate) struct Keyboard {

}

impl Keyboard {
    pub fn new() -> Self { Self {} }

    pub fn is_pressed(&self, keycode: u8) -> bool {
        false // unimplemented
    }
}