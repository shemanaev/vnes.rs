use std::fmt::Debug;

pub trait Memory: Debug {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8);

    fn read_word(&self, address: u16) -> u16 {
        ((self.read(address + 1) as u16) << 8) | (self.read(address) as u16)
    }

    fn write_word(&mut self, address: u16, value: u16) {
        self.write(address + 1, (value >> 8) as u8);
        self.write(address, (value & 0x00ff) as u8);
    }
}
