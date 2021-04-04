#[derive(Debug)]
pub struct CPU {
    vx: [u8; 16],
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
            vx: [0; 16],
            i: 0,
            delay: 0,
            sound: 0,
            pc: 0x200,
            sp: 0,
            stack: [0; 16],
        }
    }
}
