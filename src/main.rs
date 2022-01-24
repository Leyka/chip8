mod chip8;
mod display;
mod keypad;

use crate::keypad::Keypad;
use crate::chip8::Chip8;
use crate::display::Display;

fn main() {
    let display = Display::new();
    let keypad = Keypad::new();
    let chip8 = Chip8::new();
    println!("Hello, world!");
}
