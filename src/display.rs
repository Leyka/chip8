use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window, Sdl};

// The original implementation of the Chip-8 language used a 64x32 monochrome pixels
const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const SCALE: usize = 15;

// Colors
const BLACK_COLOR: Color = Color::RGB(0, 0, 0);
const WHITE_COLOR: Color = Color::RGB(255, 255, 255);

pub struct Display {
    memory: [[u8; WIDTH]; HEIGHT],
    canvas: Canvas<Window>,
}

impl Display {
    pub fn new(sdl: &Sdl) -> Self {
        let video_subsystem = sdl.video().unwrap();
        let window = video_subsystem
            .window("Chip 8", (WIDTH * SCALE) as u32, (HEIGHT * SCALE) as u32)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        canvas.clear();
        canvas.present();

        Display {
            memory: [[0; WIDTH]; HEIGHT],
            canvas,
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
                    if self.memory[y_norm][x_norm] == 1 {
                        collision = true;
                    }
                    // XOR memory pixel and sprite pixel
                    self.memory[y_norm][x_norm] ^= 1;
                }
            }
        }

        collision
    }

    pub fn draw_screen(&mut self) {
        // Clear previous canvas by setting screen to black
        self.canvas.set_draw_color(BLACK_COLOR);
        self.canvas.clear();
        // Draw white pixel any time we have a pixel at true
        self.canvas.set_draw_color(WHITE_COLOR);
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                if self.memory[y][x] == 1 {
                    let rect = Rect::new(
                        (x * SCALE) as i32,
                        (y * SCALE) as i32,
                        SCALE as u32,
                        SCALE as u32,
                    );

                    self.canvas.fill_rect(rect).unwrap();
                }
            }
        }

        self.canvas.present();
    }
}
