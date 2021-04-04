mod chip8;
mod cpu;
mod memory;

use chip8::Emulator;
use std::env;
use std::fs;

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let rom_path = env::args().nth(1).expect("No ROM path");
    let rom_data = fs::read(rom_path)?;

    let mut emulator = Emulator::new();

    emulator.run(rom_data)
}
