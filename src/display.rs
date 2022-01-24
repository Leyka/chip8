pub struct Display {}

impl Display {
    pub fn new() -> Self {
        Display {}
    }

    pub fn clear(&mut self) {
        println!("Display cleared");
    }
}
