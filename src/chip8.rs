use crate::{display::Display, font::*, keypad::Keypad};

// Chip8 has 4KB of RAM
const MEMORY_SIZE: usize = 4096;
// Chip8’s memory from 0x000 to 0x1FF is reserved, so the ROM instructions must start at 0x200
const START_ADDRESS: u16 = 0x200;

pub struct Chip8 {
    // Program counter
    pc: u16,
    // Registers & index register
    registers: [u8; 16],
    i: u16,
    // Stack & stack pointer
    stack: [u16; 16],
    sp: u8,
    // Memory
    memory: [u8; MEMORY_SIZE],
    // Timers
    delayTimer: u8,
    soundTimer: u8,
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
            delayTimer: 0,
            soundTimer: 0,
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

    pub fn load_rom(&mut self) {}

    pub fn execute_cycle(&mut self) {}
}
