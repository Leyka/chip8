extern crate sdl2;

use std::{thread, time::Duration};

use crate::chip8::Chip8;
use sdl2::{event::Event, keyboard::Keycode};

mod chip8;
mod display;
mod font;
mod keypad;
mod speaker;

const DELAY: Duration = Duration::from_millis(3);

fn main() {
    let sdl_context = sdl2::init().unwrap();

    let mut chip8 = Chip8::new(&sdl_context);
    chip8.load_rom(&"roms/TETRIS");

    // Listen to events in the main loop
    let mut event_pump = sdl_context.event_pump().unwrap();
    'main: loop {
        for evt in event_pump.poll_iter() {
            match evt {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'main;
                }
                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    chip8.keypad.handle_key(key, true);
                }
                Event::KeyUp {
                    keycode: Some(key), ..
                } => {
                    chip8.keypad.handle_key(key, false);
                }
                _ => (),
            }
        }

        chip8.cycle();

        chip8.display.draw_screen();

        thread::sleep(DELAY);
    }
}
