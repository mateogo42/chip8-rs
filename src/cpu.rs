use super::memory::Memory;
use super::screen::{key_to_hex, Screen};
use rand::prelude::*;

const FG_COLOR: u32 = 0x61AFEF;

#[derive(Debug)]
pub struct CPU {
    v: [u8; 16],
    i: u16,
    delay: u8,
    sound: u8,
    pc: u16,
    sp: u8,
    stack: [u16; 16],
    rng: ThreadRng,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            v: [0; 16],
            i: 0,
            delay: 0,
            sound: 0,
            pc: 0x200,
            sp: 0,
            stack: [0; 16],
            rng: rand::thread_rng(),
        }
    }

    pub fn get_op(&self, memory: &Memory) -> u16 {
        ((memory.get(self.pc as usize) as u16) << 8) | memory.get((self.pc + 1) as usize) as u16
    }

    pub fn update(&mut self) {
        if self.delay > 0 {
            self.delay -= 1;
        }

        if self.sound > 0 {
            self.sound -= 1;
        }
    }

    pub fn clear_screen(&mut self, screen: &mut Screen) {
        screen.clear();
        self.pc += 2;
    }

    pub fn return_from_subroutine(&mut self) {
        self.sp -= 1;
        self.pc = self.stack[self.sp as usize];
        self.pc += 2
    }

    pub fn jump(&mut self, addr: u16) {
        self.pc = addr;
    }

    pub fn call(&mut self, addr: u16) {
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = addr;
    }

    pub fn skip_if_condition(&mut self, condition: bool) {
        if condition {
            self.pc += 2;
        }
    }

    pub fn skip_equal(&mut self, x: u8, y: u8, use_vy: bool) {
        let vx = self.v[x as usize];
        let vy = if use_vy { self.v[y as usize] } else { y };
        self.skip_if_condition(vx == vy);
        self.pc += 2;
    }

    pub fn skip_not_equal(&mut self, x: u8, y: u8, use_vy: bool) {
        let vx = self.v[x as usize];
        let vy = if use_vy { self.v[y as usize] } else { y };
        self.skip_if_condition(vx != vy);
        self.pc += 2;
    }

    pub fn load(&mut self, x: u8, y: u8, use_vy: bool) {
        let vy = if use_vy { self.v[y as usize] } else { y };
        self.v[x as usize] = vy;
        self.pc += 2;
    }

    pub fn add(&mut self, x: u8, y: u8, use_vy: bool) {
        let vy = if use_vy { self.v[y as usize] } else { y };
        self.v[x as usize] = self.v[x as usize].wrapping_add(vy);
        self.pc += 2;
    }

    pub fn or(&mut self, x: u8, y: u8) {
        self.v[x as usize] |= self.v[y as usize];
        self.pc += 2;
    }

    pub fn and(&mut self, x: u8, y: u8) {
        self.v[x as usize] &= self.v[y as usize];
        self.pc += 2;
    }

    pub fn xor(&mut self, x: u8, y: u8) {
        self.v[x as usize] ^= self.v[y as usize];
        self.pc += 2;
    }

    pub fn add_with_carrier(&mut self, x: u8, y: u8) {
        let vx = self.v[x as usize];
        let vy = self.v[y as usize];

        let (res, overflow) = vx.overflowing_add(vy);

        self.v[x as usize] = res;

        self.v[0xF] = if overflow { 1 } else { 0 };

        self.pc += 2;
    }

    pub fn sub_with_carrier(&mut self, x: u8, y: u8) {
        let vx = self.v[x as usize];
        let vy = self.v[y as usize];

        let (res, overflow) = vx.overflowing_sub(vy);

        self.v[x as usize] = res;

        self.v[0xF] = if !overflow { 1 } else { 0 };

        self.pc += 2;
    }

    pub fn sub_not_borrow(&mut self, x: u8, y: u8) {
        let vx = self.v[x as usize];
        let vy = self.v[y as usize];

        let (res, overflow) = vy.overflowing_sub(vx);

        self.v[0xF] = if !overflow { 1 } else { 0 };
        self.v[x as usize] = res;
        self.pc += 2;
    }

    pub fn shift_right(&mut self, x: u8) {
        self.v[0xF] = self.v[x as usize] & 0b1;
        self.v[x as usize] >>= 1;
        self.pc += 2;
    }
    pub fn shift_left(&mut self, x: u8) {
        self.v[0xF] = self.v[x as usize] >> 7;
        self.v[x as usize] <<= 1;
        self.pc += 2;
    }

    pub fn load_i(&mut self, addr: u16) {
        self.i = addr;
        self.pc += 2;
    }

    pub fn jump_v0(&mut self, addr: u16) {
        self.pc = self.v[0] as u16 + addr;
    }

    pub fn set_random_byte(&mut self, x: u8, kk: u8) {
        let random_byte: u8 = self.rng.gen();
        self.v[x as usize] = random_byte & kk;
        self.pc += 2;
    }

    pub fn update_sprite(&mut self, memory: &Memory, buffer: &mut [u32], x: u8, y: u8, n: u8) {
        self.v[0xF] = 0;
        let vx = self.v[x as usize];
        let vy = self.v[y as usize];
        for offset in 0..n {
            let value = memory.get((self.i + offset as u16) as usize);
            let i = (vy as usize + offset as usize) % 32;
            for bit in 0..8 {
                let j = (vx as usize + bit) % 64;
                let index = j + 64 * i;
                let old_pixel = buffer[index];
                let new_pixel = ((value >> (7 - bit)) & 0b1) as u32 * FG_COLOR;

                buffer[index] ^= new_pixel;

                if old_pixel == FG_COLOR && new_pixel == FG_COLOR {
                    self.v[0xF] = 1;
                }
            }
        }
        self.pc += 2;
    }

    pub fn skip_if_key_is_pressed(&mut self, x: u8, screen: &Screen) {
        self.skip_if_condition(screen.is_key_down(self.v[x as usize]));
        self.pc += 2;
    }

    pub fn skip_if_key_is_not_pressed(&mut self, x: u8, screen: &Screen) {
        self.skip_if_condition(!screen.is_key_down(self.v[x as usize]));
        self.pc += 2;
    }

    pub fn load_delay(&mut self, x: u8) {
        self.v[x as usize] = self.delay;
        self.pc += 2;
    }

    pub fn wait_for_key_press(&mut self, x: u8, screen: &Screen) {
        if let Some(keys) = screen.is_any_key_down() {
            if keys.len() > 0 {
                let key = keys[0];
                self.v[x as usize] = key_to_hex(key);
                self.pc += 2;
            }
        }
    }

    pub fn set_delay_timer(&mut self, x: u8) {
        self.delay = self.v[x as usize];
        self.pc += 2;
    }

    pub fn set_sound_timer(&mut self, x: u8) {
        self.sound = self.v[x as usize];
        self.pc += 2;
    }

    pub fn set_i(&mut self, x: u8) {
        self.i += self.v[x as usize] as u16;
        self.pc += 2;
    }

    pub fn load_sprite(&mut self, x: u8) {
        self.i = self.v[x as usize] as u16 * 5;
        self.pc += 2;
    }

    pub fn store_bcd(&mut self, x: u8, memory: &mut Memory) {
        let vx = self.v[x as usize];
        memory.set(self.i as usize, (vx / 100) % 10);
        memory.set(self.i as usize + 1, (vx / 10) % 10);
        memory.set(self.i as usize + 2, vx % 10);
        self.pc += 2;
    }

    pub fn store_registers(&mut self, x: u8, memory: &mut Memory) {
        for offset in 0..(x + 1) {
            memory.set((self.i + offset as u16) as usize, self.v[offset as usize])
        }
        self.pc += 2;
    }

    pub fn load_registers(&mut self, x: u8, memory: &mut Memory) {
        for offset in 0..(x + 1) {
            self.v[offset as usize] = memory.get((self.i + offset as u16) as usize);
        }
        self.pc += 2;
    }
}
