use sdl2::keyboard::Keycode;

// 16 keys from 0 to F
const SIZE: usize = 16;

pub struct Keypad {
    keys: [bool; SIZE],
}

impl Keypad {
    pub fn new() -> Self {
        Keypad {
            keys: [false; SIZE],
        }
    }

    pub fn is_key_pressed(&mut self, index: usize) -> bool {
        self.keys[index]
    }

    pub fn handle_key(&mut self, key: Keycode, state: bool) {
        /*
        Keypad       Keyboard
        +-+-+-+-+    +-+-+-+-+
        |1|2|3|C|    |1|2|3|4|
        +-+-+-+-+    +-+-+-+-+
        |4|5|6|D|    |Q|W|E|R|
        +-+-+-+-+ => +-+-+-+-+
        |7|8|9|E|    |A|S|D|F|
        +-+-+-+-+    +-+-+-+-+
        |A|0|B|F|    |Z|X|C|V|
        +-+-+-+-+    +-+-+-+-+
        */

        let index = match key {
            Keycode::Num1 => Some(0x1),
            Keycode::Num2 => Some(0x2),
            Keycode::Num3 => Some(0x3),
            Keycode::Num4 => Some(0xc),
            Keycode::Q => Some(0x4),
            Keycode::W => Some(0x5),
            Keycode::E => Some(0x6),
            Keycode::R => Some(0xd),
            Keycode::A => Some(0x7),
            Keycode::S => Some(0x8),
            Keycode::D => Some(0x9),
            Keycode::F => Some(0xe),
            Keycode::Z => Some(0xa),
            Keycode::X => Some(0x0),
            Keycode::C => Some(0xb),
            Keycode::V => Some(0xf),
            _ => None,
        };

        if index.is_some() {
            self.keys[index.unwrap()] = state;
        }
    }

    pub fn size(&mut self) -> usize {
        SIZE
    }
}
