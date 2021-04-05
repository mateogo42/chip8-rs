use minifb::{Key, Window, WindowOptions};
use rand::prelude::*;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

#[derive(Debug)]
pub struct Screen {
    window: Window,
    pub buffer: [u32; WIDTH * HEIGHT],
}

impl Screen {
    pub fn new() -> Self {
        let mut window = Window::new(
            "CHIP-8 EMULATOR",
            WIDTH,
            HEIGHT,
            WindowOptions {
                scale: minifb::Scale::X16,
                ..WindowOptions::default()
            },
        )
        .expect("Unable to create window");

        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
        Self {
            buffer: [0; WIDTH * HEIGHT],
            window,
        }
    }

    pub fn clear(&mut self) {
        self.buffer = [0; WIDTH * HEIGHT];
    }

    pub fn update(&mut self) {
        self.window
            .update_with_buffer(&self.buffer, WIDTH, HEIGHT)
            .expect("Unable to update window");
    }

    pub fn is_running(&self) -> bool {
        self.window.is_open() && !self.window.is_key_down(Key::Escape)
    }
}
