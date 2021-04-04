#[derive(Debug)]
pub struct Memory {
    pub data: [u8; 4096],
}

impl Memory {
    pub fn new() -> Self {
        Self { data: [0; 4096] }
    }

    pub fn get(&self, index: usize) -> u8 {
        self.data[index]
    }

    pub fn set(&mut self, index: usize, value: u8) {
        self.data[index] = value
    }
}
