use super::cpu::CPU;
use super::memory::Memory;
use minifb::{Key, Window, WindowOptions};
use rand::prelude::*;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

#[derive(Debug)]
pub struct Emulator {
    cpu: CPU,
    memory: Memory,
    screen: [u32; WIDTH * HEIGHT],
    rng: ThreadRng,
}

impl Emulator {
    pub fn new() -> Self {
        let memory = Memory::new();
        let cpu = CPU::new();
        let rng = rand::thread_rng();
        Self {
            cpu,
            memory,
            screen: [0; WIDTH * HEIGHT],
            rng,
        }
    }

    pub fn run(&mut self, rom: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        let mut window = Window::new(
            "CHIP-8 EMULATOR",
            WIDTH,
            HEIGHT,
            WindowOptions {
                scale: minifb::Scale::X16,
                ..WindowOptions::default()
            },
        )?;

        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        while window.is_open() && !window.is_key_down(Key::Escape) {
            for i in self.screen.iter_mut() {
                let random: u32 = self.rng.gen();
                *i = random;
            }

            window.update_with_buffer(&self.screen, WIDTH, HEIGHT)?;
        }
        Ok(())
    }
}
