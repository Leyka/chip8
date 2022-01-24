mod chip8;
mod display;
mod font;
mod keypad;

use crate::chip8::Chip8;

fn main() {
    let mut chip8 = Chip8::new();
    chip8.load_rom(&"roms/PONG");
    chip8.execute_cycle();
}
