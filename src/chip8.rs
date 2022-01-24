use std::{fs::File, io::Read};

use crate::{display::Display, font::*, keypad::Keypad};

// Chip8 has 4KB of RAM
const MEMORY_SIZE: usize = 4096;
// Chip8's memory from 0x000 to 0x1FF is reserved, so the ROM instructions must start at 0x200
const START_ADDRESS: usize = 0x200;

pub struct Chip8 {
    // Program counter
    pc: usize,
    // Registers & index register
    registers: [u8; 16],
    i: u16,
    // Stack & stack pointer
    stack: [u16; 16],
    sp: u8,
    // Memory
    memory: [u8; MEMORY_SIZE],
    // Timers
    delay_timer: u8,
    sound_timer: u8,
    // Peripherals
    display: Display,
    keypad: Keypad,
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            pc: START_ADDRESS,
            registers: [0; 16],
            i: 0,
            stack: [0; 16],
            sp: 0,
            memory: Self::init_memory(),
            delay_timer: 0,
            sound_timer: 0,
            display: Display::new(),
            keypad: Keypad::new(),
        }
    }

    /// Returns a fresh memory with loaded font set
    fn init_memory() -> [u8; MEMORY_SIZE] {
        let mut memory = [0; MEMORY_SIZE];
        for i in 0..FONT_SET.len() {
            memory[FONT_SET_START_ADDRESS + i] = FONT_SET[i];
        }
        memory
    }

    pub fn load_rom(&mut self, rom_path: &str) {
        let mut f = File::open(rom_path).expect("Rom file not found");
        let mut buffer = Vec::<u8>::new();
        f.read_to_end(&mut buffer).unwrap();

        // Inject rom into memory
        for i in 0..buffer.len() {
            self.memory[START_ADDRESS + i] = buffer[i];
        }
    }

    pub fn execute_cycle(&mut self) {}
}
