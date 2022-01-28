extern crate sdl2;

use std::{env, thread, time::Duration};

use crate::chip8::Chip8;
use sdl2::{event::Event, keyboard::Keycode};

mod chip8;
mod display;
mod font;
mod keypad;
mod speaker;

const DELAY: Duration = Duration::from_millis(1);

fn main() {
    // 2nd arg is the rom name to load, default to TEST rom
    let args: Vec<_> = env::args().collect();
    let mut rom = "TEST";
    if args.len() == 2 {
        rom = &args[1];
    }

    let rom_path = format!("roms/{}", rom);
    let sdl_context = sdl2::init().unwrap();
    let window_title = format!("{} - CHIP8", rom);
    let mut chip8 = Chip8::new(&sdl_context, &window_title);

    chip8.load_rom(&rom_path);

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
