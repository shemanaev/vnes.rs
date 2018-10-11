#[derive(Debug)]
pub struct APU {}

impl APU {
    pub fn new() -> APU {
        APU{}
    }

    pub fn read_register(&self, _address: u16) -> u8 {
        0
    }

    pub fn write_register(&mut self, _address: u16, _value: u8) {
        
    }
}
