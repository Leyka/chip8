extern crate sdl2;

use crate::chip8::Chip8;
use sdl2::event::Event;

mod chip8;
mod display;
mod font;
mod keypad;
mod speaker;

fn main() {
    let sdl_context = sdl2::init().unwrap();

    let mut chip8 = Chip8::new(&sdl_context);
    chip8.load_rom(&"roms/TEST");

    // Listen to events in the main loop
    let mut event_pump = sdl_context.event_pump().unwrap();
    'main: loop {
        for evt in event_pump.poll_iter() {
            match evt {
                Event::Quit { .. } => {
                    break 'main;
                }
                _ => {}
            }
        }

        chip8.cycle();
        chip8.display.draw_screen();
    }
}
