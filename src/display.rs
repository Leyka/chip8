// The original implementation of the Chip-8 language used a 64x32 monochrome pixels
const WIDTH: usize = 64;
const HEIGHT: usize = 32;

const SCALE: usize = 20;

pub struct Display {
    gfx: [[u8; WIDTH]; HEIGHT],
}

impl Display {
    pub fn new() -> Self {
        Display {
            gfx: [[0; WIDTH]; HEIGHT],
        }
    }

    pub fn clear(&mut self) {
        self.gfx = [[0; WIDTH]; HEIGHT];
    }

    /// Sprites are XORed onto the existing screen. If this causes any pixels to be erased, VF is set to 1, otherwise it is set to 0.
    /// If the sprite is positioned so part of it is outside the coordinates of the display,
    /// it wraps around to the opposite side of the screen.
    pub fn draw(&mut self, x: usize, y: usize, sprite: &[u8]) -> u8 {
        let mut collision = 0u8;

        // Sprite takes 8 pixels of width and dynamic height
        for row in 0..sprite.len() {
            for col in 0..8 {
                // Normalize x & y
                let x_norm = x % WIDTH;
                let y_norm = y % HEIGHT;
                // TODO: Continue
            }
        }

        collision
    }
}
