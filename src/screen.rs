use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const FPS: u64 = 120;

#[derive(Debug)]
pub struct Screen {
    window: Window,
    pub buffer: [u32; WIDTH * HEIGHT],
}

fn hex_to_key(hex: u8) -> Key {
    match hex {
        0x0 => Key::X,
        0x1 => Key::Key1,
        0x2 => Key::Key2,
        0x3 => Key::Key3,
        0x4 => Key::Q,
        0x5 => Key::W,
        0x6 => Key::E,
        0x7 => Key::A,
        0x8 => Key::S,
        0x9 => Key::D,
        0xA => Key::Z,
        0xB => Key::K,
        0xC => Key::Key4,
        0xD => Key::R,
        0xE => Key::F,
        0xF => Key::V,
        _ => panic!("Unknown key!"),
    }
}

pub fn key_to_hex(key: Key) -> u8 {
    match key {
        Key::X => 0x0,
        Key::Key1 => 0x1,
        Key::Key2 => 0x2,
        Key::Key3 => 0x3,
        Key::Q => 0x4,
        Key::W => 0x5,
        Key::E => 0x6,
        Key::A => 0x7,
        Key::S => 0x8,
        Key::D => 0x9,
        Key::Z => 0xA,
        Key::K => 0xB,
        Key::Key4 => 0xC,
        Key::R => 0xD,
        Key::F => 0xE,
        Key::V => 0xF,
        _ => panic!("Unknown key!"),
    }
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

        let update_rate = 1_000_000 / FPS;

        window.limit_update_rate(Some(std::time::Duration::from_micros(update_rate)));
        Self {
            buffer: [0; WIDTH * HEIGHT],
            window,
        }
    }

    pub fn clear(&mut self) {
        self.buffer = [0; WIDTH * HEIGHT];
    }

    pub fn update(&mut self) {
        self.window.set_background_color(255, 255, 255);
        self.window
            .update_with_buffer(&self.buffer, WIDTH, HEIGHT)
            .expect("Unable to update window");
    }

    pub fn is_running(&self) -> bool {
        self.window.is_open() && !self.window.is_key_down(Key::Escape)
    }

    pub fn is_key_down(&self, value: u8) -> bool {
        let key = hex_to_key(value);
        self.window.is_key_down(key)
    }

    pub fn is_any_key_down(&self) -> Option<Vec<Key>> {
        self.window.get_keys()
    }
}
