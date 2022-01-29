use crate::{display::Display, font::*, keypad::Keypad, speaker::Speaker};
use rand::Rng;
use sdl2::Sdl;
use std::{fs::File, io::Read};

// Chip8 has 4KB of RAM
const MEMORY_SIZE: usize = 4096;
// Chip8's memory from 0x000 to 0x1FF is reserved, so the ROM instructions must start at 0x200
const START_ALLOWED_ADDRESS: usize = 0x200;

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
    pub display: Display,
    pub keypad: Keypad,
    speaker: Speaker,
}

impl Chip8 {
    pub fn new(sdl: &Sdl, window_title: &str) -> Self {
        Chip8 {
            pc: START_ALLOWED_ADDRESS,
            v: [0; 16],
            i: 0,
            stack: [0; 16],
            sp: 0,
            memory: Self::init_memory(),
            delay_timer: 0,
            sound_timer: 0,
            display: Display::new(sdl, window_title),
            keypad: Keypad::new(),
            speaker: Speaker::new(sdl),
        }
    }

    /// Returns a fresh memory with loaded font set
    fn init_memory() -> [u8; MEMORY_SIZE] {
        let mut memory = [0; MEMORY_SIZE];
        for i in 0..FONT_SET.len() {
            memory[i] = FONT_SET[i];
        }
        memory
    }

    pub fn load_rom(&mut self, rom_path: &str) {
        let mut f = File::open(rom_path).expect("Rom file not found");
        let mut buffer = Vec::<u8>::new();
        f.read_to_end(&mut buffer).unwrap();

        // Inject rom into memory
        for i in 0..buffer.len() {
            self.memory[START_ALLOWED_ADDRESS + i] = buffer[i];
        }
    }

    /// Cycle = Fetch -> decode -> execute
    pub fn cycle(&mut self) {
        // Fetch
        let opcode = self.fetch_opcode();
        // Increment the PC before we execute anything
        self.pc += 2;
        // Decode and execute
        self.execute_opcode(opcode);
    }

    pub fn decrement_timers(&mut self) {
        // Decrement delay timer and sound timer if their values is above 0
        // This will be done 60 times per second (60Hz)
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    pub fn handle_sound(&mut self) {
        if self.sound_timer > 0 {
            self.speaker.emit_sound();
        } else {
            self.speaker.stop_emitting();
        }
    }

    fn fetch_opcode(&mut self) -> u16 {
        // Since opcode (instruction) is a group of 2 bytes,
        // we need to fetch each byte from memory according to PC and merge them together.
        // Example: 6A and 12 -> 6A00 | 0012 = 6A12
        (self.memory[self.pc] as u16) << 8 | (self.memory[self.pc + 1] as u16)
    }

    fn execute_opcode(&mut self, opcode: u16) {
        // Break each nibble from the 2 bytes instruction (4 nibbles)
        let op_1 = ((opcode & 0xF000) >> 12) as usize;
        let op_2 = ((opcode & 0x0F00) >> 8) as usize;
        let op_3 = ((opcode & 0x00F0) >> 4) as usize;
        let op_4 = (opcode & 0x000F) as usize;

        let nnn = (opcode & 0x0FFF) as usize; // 12-bit value, the lowest 12 bits of the instruction
        let x = op_2; // 4-bit value, the lower 4 bits of the high byte of the instruction
        let y = op_3; // 4-bit value, the upper 4 bits of the low byte of the instruction
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
            (0x9, _, _, 0) => self.op_9xy0(x, y),
            (0xa, _, _, _) => self.op_annn(nnn),
            (0xb, _, _, _) => self.op_bnnn(nnn),
            (0xc, _, _, _) => self.op_cxkk(x, kk),
            (0xd, _, _, _) => self.op_dxyn(x, y, n),
            (0xe, _, 0x9, 0xe) => self.op_ex9e(x),
            (0xe, _, 0xa, 0x1) => self.op_exa1(x),
            (0xf, _, 0, 0x7) => self.op_fx07(x),
            (0xf, _, 0, 0xa) => self.op_fx0a(x),
            (0xf, _, 0x1, 0x5) => self.op_fx15(x),
            (0xf, _, 0x1, 0x8) => self.op_fx18(x),
            (0xf, _, 0x1, 0xe) => self.op_fx1e(x),
            (0xf, _, 0x2, 0x9) => self.op_fx29(x),
            (0xf, _, 0x3, 0x3) => self.op_fx33(x),
            (0xf, _, 0x5, 0x5) => self.op_fx55(x),
            (0xf, _, 0x6, 0x5) => self.op_fx65(x),
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
        // Since our PC has already been incremented by 2 in Cycle(),
        // we can just increment by 2 again to skip the next instruction
        // This is valid for every function we do pc += 2
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
        self.v[x] = self.v[x].wrapping_add(kk);
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
        // We have a risk of overflow
        let (new_vx, has_overflowed) = self.v[x].overflowing_add(self.v[y]);
        self.v[0xf] = has_overflowed as u8;
        self.v[x] = new_vx;
    }

    /// Set Vx = Vx - Vy, set VF = NOT borrow.
    /// If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx.
    fn op_8xy5(&mut self, x: usize, y: usize) {
        // We also have risk of going under 0
        let (new_vx, has_overflowed) = self.v[x].overflowing_sub(self.v[y]);
        self.v[0xf] = has_overflowed as u8;
        self.v[x] = new_vx;
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
        let (new_vx, has_overflowed) = self.v[y].overflowing_sub(self.v[x]);
        self.v[0xf] = has_overflowed as u8;
        self.v[x] = new_vx;
    }

    /// Set Vx = Vx SHL 1
    /// If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0.
    /// Then Vx is multiplied by 2.
    fn op_8xye(&mut self, x: usize) {
        self.v[0xf] = self.v[x] & 0x80; // 0x80 => 0b10000000
        self.v[x] <<= 1; // multiply by 2
    }

    /// Skip next instruction if Vx != Vy.
    fn op_9xy0(&mut self, x: usize, y: usize) {
        if self.v[x] != self.v[y] {
            self.pc += 2;
        }
    }

    /// Set I = nnn.
    fn op_annn(&mut self, nnn: usize) {
        self.i = nnn;
    }

    /// Jump to location nnn + V0.
    fn op_bnnn(&mut self, nnn: usize) {
        self.pc = nnn + self.v[0] as usize;
    }

    /// Set Vx = random byte AND kk.
    fn op_cxkk(&mut self, x: usize, kk: u8) {
        let random_byte: u8 = rand::thread_rng().gen();
        self.v[x] = random_byte & kk;
    }

    /// Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
    /// The interpreter reads n bytes from memory, starting at the address stored in I.
    /// These bytes are then displayed as sprites on screen at coordinates (Vx, Vy).
    /// Sprites are XORed onto the existing screen. If this causes any pixels to be erased, VF is set to 1, otherwise it is set to 0.
    /// If the sprite is positioned so part of it is outside the coordinates of the display, it wraps around to the opposite side of the screen.
    fn op_dxyn(&mut self, x: usize, y: usize, n: usize) {
        let start = self.i;
        let end = start + n;
        let sprite_bytes = &self.memory[start..end];

        let has_collision = self
            .display
            .draw(self.v[x] as usize, self.v[y] as usize, sprite_bytes);

        self.v[0xf] = has_collision as u8;
    }

    /// Skip next instruction if key with the value of Vx is pressed.
    fn op_ex9e(&mut self, x: usize) {
        let key = self.v[x] as usize;
        if self.keypad.is_key_pressed(key) {
            self.pc += 2;
        }
    }

    /// Skip next instruction if key with the value of Vx is not pressed.
    fn op_exa1(&mut self, x: usize) {
        let key = self.v[x] as usize;
        if !self.keypad.is_key_pressed(key) {
            self.pc += 2;
        }
    }

    /// Set Vx = delay timer value.
    fn op_fx07(&mut self, x: usize) {
        self.v[x] = self.delay_timer;
    }

    /// Wait for a key press, store the value of the key in Vx.
    fn op_fx0a(&mut self, x: usize) {
        // Since PC is pre-incremented, here I will only increment PC if I actually have key press
        self.pc -= 2;

        for i in 0..self.keypad.size() {
            if self.keypad.is_key_pressed(i) {
                self.v[x] = i as u8;
                self.pc += 2;
                break;
            }
        }
    }

    /// Set delay timer = Vx.
    fn op_fx15(&mut self, x: usize) {
        self.delay_timer = self.v[x];
    }

    /// Set sound timer = Vx.
    fn op_fx18(&mut self, x: usize) {
        self.sound_timer = self.v[x];
    }

    /// Set I = I + Vx.
    fn op_fx1e(&mut self, x: usize) {
        self.i += self.v[x] as usize;
    }

    /// Set I = location of sprite for digit Vx.
    /// We know that each digit takes 5 bytes each
    fn op_fx29(&mut self, x: usize) {
        let digit = self.v[x] as usize;
        self.i = digit * 5;
    }

    /// Store BCD representation of Vx in memory locations I, I+1, and I+2.
    // The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at location in I,
    // the tens digit at location I+1, and the ones digit at location I+2.
    fn op_fx33(&mut self, x: usize) {
        let vx = self.v[x];
        self.memory[self.i] = vx / 100;
        self.memory[self.i + 1] = (vx / 10) % 10;
        self.memory[self.i + 2] = vx % 10;
    }

    /// Store registers V0 through Vx in memory starting at location I.
    fn op_fx55(&mut self, x: usize) {
        for idx in 0..x + 1 {
            self.memory[self.i + idx] = self.v[idx];
        }
    }

    /// Read registers V0 through Vx from memory starting at location I.
    fn op_fx65(&mut self, x: usize) {
        for idx in 0..x + 1 {
            self.v[idx] = self.memory[self.i + idx];
        }
    }
}
