use super::memory::Memory;

#[derive(Debug)]
pub struct CPU {
    v: [u8; 16],
    i: u16,
    delay: u8,
    sound: u8,
    pc: u16,
    sp: u8,
    stack: [u16; 16],
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
        }
    }

    pub fn get_op(&self, memory: &Memory) -> u16 {
        ((memory.get(self.pc as usize) as u16) << 8) | memory.get((self.pc + 1) as usize) as u16
    }

    pub fn return_from_subroutine(&mut self) {
        self.pc = self.stack[self.sp as usize];
        self.sp -= 1;
    }

    pub fn jump(&mut self, addr: u16) {
        self.pc = addr;
    }

    pub fn call(&mut self, addr: u16) {
        self.sp += 1;
        self.stack[self.sp as usize] = self.pc;
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
    }

    pub fn shift_right(&mut self, x: u8) {
        self.v[0xF] = if (self.v[x as usize]) & 0b00000001 == 1 {
            1
        } else {
            0
        };
        self.v[x as usize] >>= 1;
        self.pc += 2;
    }
    pub fn shift_left(&mut self, x: u8) {
        self.v[0xF] = if ((self.v[x as usize] & 0b10000000) >> 7) == 1 {
            1
        } else {
            0
        };
        self.v[x as usize] <<= 1;
        self.pc += 2;
    }
}
