mod cpu;
mod memory;
mod screen;

use crate::cpu::CPU;
use crate::memory::Memory;
use crate::screen::Screen;
use wasm_bindgen::prelude::*;

#[derive(Debug)]
#[wasm_bindgen]
pub struct Emulator {
    cpu: CPU,
    memory: Memory,
    screen: Screen,
}

#[wasm_bindgen]
impl Emulator {
    pub fn new() -> Self {
        let memory = Memory::new();
        let cpu = CPU::new();
        let screen = Screen::new();
        Self {
            cpu,
            memory,
            screen,
        }
    }

    pub fn memory(&self) -> Vec<u8> {
        self.memory.data.to_vec()
    }

    pub fn tick(&mut self) -> Vec<u8> {
        let opcode = self.cpu.get_op(&self.memory);
        self.exec_code(opcode);
        self.cpu.update();
        self.screen.buffer.to_vec()
    }

    pub fn load_rom_data(&mut self, data: Vec<u8>) -> Vec<u8> {
        self.memory.load(&data);
        self.memory.data.to_vec()
    }
    fn exec_code(&mut self, opcode: u16) {
        //println!("{:#02x}", opcode);
        let addr = opcode & 0xFFF;
        let kk = (opcode & 0xFF) as u8;
        let x = ((opcode >> 8) & 0xF) as u8;
        let y = ((opcode >> 4) & 0xF) as u8;
        let n = (opcode & 0xF) as u8;
        let code_id = ((opcode >> 12) & 0xF) as u8;
        match code_id {
            0x0 => match n {
                0x0 => self.screen.clear(),
                0xE => self.cpu.return_from_subroutine(),
                _ => panic!("Unexpected code {}", opcode),
            },
            1 => self.cpu.jump(addr),
            2 => self.cpu.call(addr),
            3 => self.cpu.skip_equal(x, kk, false),
            4 => self.cpu.skip_not_equal(x, kk, false),
            5 => self.cpu.skip_equal(x, y, true),
            6 => self.cpu.load(x, kk, false),
            7 => self.cpu.add(x, kk, false),
            8 => match n {
                0 => self.cpu.load(x, y, true),
                1 => self.cpu.or(x, y),
                2 => self.cpu.and(x, y),
                3 => self.cpu.xor(x, y),
                4 => self.cpu.add_with_carrier(x, y),
                5 => self.cpu.sub_with_carrier(x, y),
                6 => self.cpu.shift_right(x),
                7 => self.cpu.sub_not_borrow(x, y),
                0xE => self.cpu.shift_left(x),
                _ => panic!("Unexpected code {}", opcode),
            },
            9 => self.cpu.skip_not_equal(x, y, true),
            0xA => self.cpu.load_i(addr),
            0xB => self.cpu.jump_v0(addr),
            0xC => self.cpu.set_random_byte(x, kk),
            0xD => self
                .cpu
                .update_sprite(&self.memory, &mut self.screen.buffer, x, y, n),
            0xE => match kk {
                // 0x9E => self.cpu.skip_if_key_is_pressed(x, &self.screen),
                // 0xA1 => self.cpu.skip_if_key_is_not_pressed(x, &self.screen),
                _ => panic!("Unexpected code {}", opcode),
            },
            0xF => match kk {
                0x07 => self.cpu.load_delay(x),
                // 0x0A => self.cpu.wait_for_key_press(x, &self.screen),
                0x15 => self.cpu.set_delay_timer(x),
                0x18 => self.cpu.set_sound_timer(x),
                0x1E => self.cpu.set_i(x),
                0x29 => self.cpu.load_sprite(x),
                0x33 => self.cpu.store_bcd(x, &mut self.memory),
                0x55 => self.cpu.store_registers(x, &mut self.memory),
                0x65 => self.cpu.load_registers(x, &mut self.memory),
                _ => panic!("Unexpected code {}", opcode),
            },
            _ => panic!("Unexpected code {}", opcode),
        };
    }
}
