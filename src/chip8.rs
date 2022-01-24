use std::{fs::File, io::Read};

use crate::{display::Display, font::*, keypad::Keypad, speaker::Speaker};

// Chip8 has 4KB of RAM
const MEMORY_SIZE: usize = 4096;
// Chip8's memory from 0x000 to 0x1FF is reserved, so the ROM instructions must start at 0x200
const START_ADDRESS: usize = 0x200;

pub struct Chip8 {
    // Program counter
    pc: usize,
    // Registers & index register
    registers: [u8; 16],
    i: usize,
    // Stack & stack pointer
    stack: [u16; 16],
    sp: usize,
    // Memory
    memory: [u8; MEMORY_SIZE],
    // Timers
    delay_timer: u8,
    sound_timer: u8,
    // Peripherals
    display: Display,
    keypad: Keypad,
    speaker: Speaker,
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
            speaker: Speaker::new(),
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

    /// Cycle = Fetch -> decode -> execute
    pub fn execute_cycle(&mut self) {
        let opcode = self.fetch_opcode();
        self.execute_opcode(opcode)
    }

    fn fetch_opcode(&mut self) -> u16 {
        // Since opcode (instruction) is a group of 2 bytes,
        // we need to fetch each byte from memory according to PC and merge them together.
        // Example: 6A and 12 -> 6A00 | 0012 = 6A12
        (self.memory[self.pc] as u16) << 8 | (self.memory[self.pc + 1] as u16)
    }

    fn execute_opcode(&mut self, opcode: u16) {
        // Break each nibble from the 2 bytes instruction (4 nibbles)
        let op_1 = ((opcode & 0xF000) >> 12) as u8;
        let op_2 = ((opcode & 0x0F00) >> 8) as u8;
        let op_3 = ((opcode & 0x00F0) >> 4) as u8;
        let op_4 = (opcode & 0x000F) as u8;

        let nnn = (opcode & 0x0FFF) as usize; // 12-bit value, the lowest 12 bits of the instruction
        let x = op_2; // 4-bit value, the lower 4 bits of the high byte of the instruction
        let y = op_3; // 4-bit value, the upper 4 bits of the low byte of the instruction
        let n = op_4; // 4-bit value, the lowest 4 bits of the instruction
        let kk = (opcode & 0x00FF) as u8; // 8-bit value, the lowest 8 bits of the instruction

        match (op_1, op_2, op_3, op_4) {
            (0, 0, 0xe, 0) => self.op_00e0(),
            (0, 0, 0xe, 0xe) => self.op_00ee(),
            (0x1, _, _, _) => self.op_1nnn(nnn),
            _ => panic!("Unrecognized or unsupported opcode"),
        };
    }

    /// CLS - Clear the display
    fn op_00e0(&mut self) {
        self.display.clear();
    }

    /// RET - Return from a subroutine.
    fn op_00ee(&mut self) {
        self.sp -= 1;
        self.pc = (self.stack[self.sp]) as usize;
    }

    /// JP - Jump to location nnn.
    fn op_1nnn(&mut self, nnn: usize) {
        self.pc = nnn;
    }
}
