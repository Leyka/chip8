use sdl2::Sdl;

// The original implementation of the Chip-8 language used a 64x32 monochrome pixels
const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const SCALE: usize = 20;

pub struct Display {
    memory: [[u8; WIDTH]; HEIGHT],
}

impl Display {
    pub fn new(sdl: &Sdl) -> Self {
        Display {
            memory: [[0; WIDTH]; HEIGHT],
        }
    }

    pub fn clear(&mut self) {
        self.memory = [[0; WIDTH]; HEIGHT];
    }

    /// Draws all pixels from sprite into memory buffer and returns true if collision
    /// Collision means we already have a pixel ON (1) in the memory and the sprite pixel is trying to override it with a 1 value.
    pub fn draw(&mut self, x: usize, y: usize, sprite: &[u8]) -> bool {
        let mut collision = false;

        let width = 8; // Sprite always takes 8 pixels
        let height = sprite.len();
        for row in 0..height {
            // Loop through each pixel from that row and check if pixel is ON, one by one
            for col in 0..width {
                let pixel = sprite[row] & (0x80 >> col);
                // Do we have pixel on?
                if pixel != 0 {
                    let x_norm = (x + col) % WIDTH;
                    let y_norm = (y + row) % HEIGHT;
                    // And is memory pixel also on? => collision!
                    if self.memory[x_norm][y_norm] == 1 {
                        collision = true;
                    }
                    // XOR memory pixel and sprite pixel
                    self.memory[x_norm][y_norm] ^= 1;
                }
            }
        }

        collision
    }
}
