use super::cpu::CPU;
use super::memory::Memory;
use super::screen::Screen;

#[derive(Debug)]
pub struct Emulator {
    cpu: CPU,
    memory: Memory,
    screen: Screen,
}

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

    pub fn run(&mut self, rom: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        self.memory.load(&rom);
        while self.screen.is_running() {
            let opcode = self.cpu.get_op(&self.memory);
            self.exec_code(opcode);
            self.screen.update();
        }
        Ok(())
    }

    fn exec_code(&mut self, opcode: u16) {
        let codes = self.parse_opcode(opcode);
        let addr = opcode & 0xFFF;
        let kk = (opcode & 0xFF) as u8;
        match codes {
            (0, 0, 0xE, 0) => self.screen.clear(),
            (0, 0, 0xE, 0xE) => self.cpu.return_from_subroutine(),
            (1, _, _, _) => self.cpu.jump(addr),
            (2, _, _, _) => self.cpu.call(addr),
            (3, x, _, _) => self.cpu.skip_equal(x, kk, false),
            (4, x, _, _) => self.cpu.skip_not_equal(x, kk, false),
            (5, x, y, 0) => self.cpu.skip_equal(x, y, true),
            (6, x, _, _) => self.cpu.load(x, kk, false),
            (7, x, _, _) => self.cpu.add(x, kk, false),
            (8, x, y, 0) => self.cpu.add(x, y, true),
            (8, x, y, 1) => self.cpu.or(x, y),
            (8, x, y, 2) => self.cpu.and(x, y),
            (8, x, y, 3) => self.cpu.xor(x, y),
            (8, x, y, 4) => self.cpu.add_with_carrier(x, y),
            (8, x, y, 5) => self.cpu.sub_with_carrier(x, y),
            (8, x, _, 6) => self.cpu.shift_right(x),
            (8, x, y, 7) => self.cpu.sub_not_borrow(x, y),
            (8, x, _, 0xE) => self.cpu.shift_left(x),
            (9, x, y, _) => self.cpu.skip_not_equal(x, y, true),
            (_, _, _, _) => panic!("Unexpected code"),
        };
    }

    fn parse_opcode(&self, opcode: u16) -> (u8, u8, u8, u8) {
        (
            (opcode << 12) as u8,
            ((opcode & 0xF00) >> 8) as u8,
            ((opcode & 0xF0) >> 4) as u8,
            (opcode & 0xF) as u8,
        )
    }
}
