use super::mapper::Mapper;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct PPU {
    mapper: Rc<RefCell<Box<dyn Mapper>>>,
}

impl PPU {
    pub fn new(mapper: Rc<RefCell<Box<dyn Mapper>>>) -> PPU {
        PPU {
            mapper: mapper,
        }
    }

    pub fn reset(&mut self) {
    }

    pub fn read_register(&mut self, address: u16) -> u8 {
        match address {
            _ => 0
        }
    }

    pub fn write_register(&mut self, address: u16, value: u8) {
        match address {
            _ => {}
        }
    }

    pub fn step(&mut self) -> bool {
        false
    }
}
