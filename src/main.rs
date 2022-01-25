mod chip8;
mod display;
mod font;
mod keypad;
mod speaker;

use crate::chip8::Chip8;

fn main() {
    let mut chip8 = Chip8::new();
    chip8.load_rom(&"roms/TEST");
    chip8.cycle();
}
