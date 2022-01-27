extern crate sdl2;

use crate::chip8::Chip8;

mod chip8;
mod display;
mod font;
mod keypad;
mod speaker;

fn main() {
    let sdl = sdl2::init().unwrap();

    let mut chip8 = Chip8::new(&sdl);
    chip8.load_rom(&"roms/TEST");
    chip8.cycle();
}
