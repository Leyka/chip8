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
    v: [u8; 16],
    i: usize,
    // Stack & stack pointer
    stack: [usize; 16],
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
            v: [0; 16],
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
    pub fn cycle(&mut self) {
        // Fetch
        let opcode = self.fetch_opcode();

        // Increment the PC before we execute anything
        self.pc += 2;

        // Decode and execute
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
        let x = op_2 as usize; // 4-bit value, the lower 4 bits of the high byte of the instruction
        let y = op_3 as usize; // 4-bit value, the upper 4 bits of the low byte of the instruction
        let n = op_4; // 4-bit value, the lowest 4 bits of the instruction
        let kk = (opcode & 0x00FF) as u8; // 8-bit value, the lowest 8 bits of the instruction

        match (op_1, op_2, op_3, op_4) {
            (0, 0, 0xe, 0) => self.op_00e0(),
            (0, 0, 0xe, 0xe) => self.op_00ee(),
            (0x1, _, _, _) => self.op_1nnn(nnn),
            (0x2, _, _, _) => self.op_2nnn(nnn),
            (0x3, _, _, _) => self.op_3xkk(x, kk),
            (0x4, _, _, _) => self.op_4xkk(x, kk),
            (0x5, _, _, 0) => self.op_5xy0(x, y),
            (0x6, _, _, _) => self.op_6xkk(x, kk),
            (0x7, _, _, _) => self.op_7xkk(x, kk),
            (0x8, _, _, 0) => self.op_8xy0(x, y),
            (0x8, _, _, 0x1) => self.op_8xy1(x, y),
            (0x8, _, _, 0x2) => self.op_8xy2(x, y),
            (0x8, _, _, 0x3) => self.op_8xy3(x, y),
            (0x8, _, _, 0x4) => self.op_8xy4(x, y),
            (0x8, _, _, 0x5) => self.op_8xy5(x, y),
            (0x8, _, _, 0x6) => self.op_8xy6(x),
            (0x8, _, _, 0x7) => self.op_8xy7(x, y),
            (0x8, _, _, 0xe) => self.op_8xye(x),
            _ => panic!("Unrecognized or unsupported opcode: {:#02x}", opcode),
        };
    }

    /// CLS - Clear the display
    fn op_00e0(&mut self) {
        self.display.clear();
    }

    /// RET - Return from a subroutine.
    fn op_00ee(&mut self) {
        // "Remove" return address from the stack
        self.sp -= 1;
        self.pc = self.stack[self.sp];
    }

    /// JP - Jump to location nnn.
    fn op_1nnn(&mut self, nnn: usize) {
        self.pc = nnn;
    }

    /// CALL - Call subroutine at nnn.
    fn op_2nnn(&mut self, nnn: usize) {
        // Save the current PC to go back to where it was when it hit the CALL
        self.stack[self.sp] = self.pc;
        self.sp += 1;
        self.pc = nnn;
    }

    /// Skip next instruction if Vx = kk
    fn op_3xkk(&mut self, x: usize, kk: u8) {
        if self.v[x] == kk {
            self.pc += 2;
        }
    }

    /// Skip next instruction if Vx != kk
    fn op_4xkk(&mut self, x: usize, kk: u8) {
        if self.v[x] != kk {
            self.pc += 2;
        }
    }

    /// Skip next instruction if Vx = Vy
    fn op_5xy0(&mut self, x: usize, y: usize) {
        if self.v[x] == self.v[y] {
            self.pc += 2;
        }
    }

    /// Set Vx = kk
    fn op_6xkk(&mut self, x: usize, kk: u8) {
        self.v[x] = kk;
    }

    /// Set Vx = Vx + kk
    fn op_7xkk(&mut self, x: usize, kk: u8) {
        self.v[x] += kk;
    }

    /// Set Vx = Vy
    fn op_8xy0(&mut self, x: usize, y: usize) {
        self.v[x] = self.v[y];
    }

    /// Set Vx = Vx OR Vy
    fn op_8xy1(&mut self, x: usize, y: usize) {
        self.v[x] |= self.v[y];
    }

    /// Set Vx = Vx AND Vy
    fn op_8xy2(&mut self, x: usize, y: usize) {
        self.v[x] &= self.v[y];
    }

    /// Set Vx = Vx XOR Vy
    fn op_8xy3(&mut self, x: usize, y: usize) {
        self.v[x] ^= self.v[y];
    }

    /// Set Vx = Vx + Vy, set VF = carry
    /// This is an ADD with an overflow flag.
    /// If the sum is greater than what can fit into a byte (255), register VF will be set to 1 as a flag
    fn op_8xy4(&mut self, x: usize, y: usize) {
        let sum = (self.v[x] + self.v[y]) as u16;
        self.v[x] = (sum & 0xff) as u8;
        self.v[0xf] = (sum > 255) as u8;
    }

    /// Set Vx = Vx - Vy, set VF = NOT borrow.
    /// If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx.
    fn op_8xy5(&mut self, x: usize, y: usize) {
        self.v[0xf] = (self.v[x] > self.v[y]) as u8;
        self.v[x] -= self.v[y];
    }

    /// Set Vx = Vx SHR 1
    /// If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
    fn op_8xy6(&mut self, x: usize) {
        self.v[0xf] = self.v[x] & 0x1; // we only care about last number if it's 1 then 1, else 0
        self.v[x] >>= 1; // divide by 2
    }

    /// Set Vx = Vy - Vx, set VF = NOT borrow
    /// If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.
    fn op_8xy7(&mut self, x: usize, y: usize) {
        self.v[0xf] = (self.v[y] > self.v[x]) as u8;
        self.v[x] = self.v[y] - self.v[x];
    }

    /// Set Vx = Vx SHL 1
    /// If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0.
    /// Then Vx is multiplied by 2.
    fn op_8xye(&mut self, x: usize) {
        self.v[0xf] = self.v[x] & 0x80; // 0x80 => 0b10000000
        self.v[x] <<= 1; // multiply by 2
    }
}
