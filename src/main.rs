mod chip8;
mod display;
mod font;
mod keypad;

use crate::chip8::Chip8;

fn main() {
    let chip8 = Chip8::new();
    println!("Hello, world!");
}
