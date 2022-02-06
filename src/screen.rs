const WIDTH: usize = 500;
const HEIGHT: usize = 300;
const FPS: u64 = 60;

#[derive(Debug)]
pub struct Screen {
    pub buffer: [u8; WIDTH * HEIGHT],
}

impl Screen {
    pub fn new() -> Self {
        Self {
            buffer: [0; WIDTH * HEIGHT],
        }
    }

    pub fn clear(&mut self) {
        self.buffer = [0; WIDTH * HEIGHT];
    }
}
