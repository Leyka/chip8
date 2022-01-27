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

    pub fn is_key_pressed(&mut self, key: usize) -> bool {
        self.keys[key]
    }

    pub fn press_key(&mut self, key: usize) {
        self.keys[key] = true;
    }

    pub fn release_key(&mut self, key: usize) {
        self.keys[key] = false;
    }

    pub fn size(&mut self) -> usize {
        SIZE
    }
}
