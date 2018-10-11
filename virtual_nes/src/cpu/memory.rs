use super::super::apu::APU;
use super::super::controller::Controller;
use super::super::mapper::Mapper;
use super::super::memory::Memory;
use super::super::ppu::PPU;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct CpuMemory {
    pub ram: Vec<u8>,
    pub mapper: Rc<RefCell<Box<dyn Mapper>>>,
    pub ppu: PPU,
    pub apu: APU,
    pub controller_1: Controller,
    pub controller_2: Controller,
}

impl CpuMemory {
    pub fn new(
        mapper: Rc<RefCell<Box<dyn Mapper>>>,
        ppu: PPU,
        apu: APU,
        controller_1: Controller,
        controller_2: Controller,
    ) -> CpuMemory {
        CpuMemory {
            ram: vec![0; 2048],
            mapper: mapper,
            ppu: ppu,
            apu: apu,
            controller_1: controller_1,
            controller_2: controller_2,
        }
    }

    // Bug in 6502: low byte wrapping without high increment
    pub fn read_word_bug(&self, address: u16) -> u16 {
        let addr_hi = (address & 0xFF00) | (address + 1 & 0xFF);
        let lo = self.read(address) as u16;
        let hi = self.read(addr_hi) as u16;
        (hi << 8) | lo
    }
}

impl Memory for CpuMemory {
    fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..0x2000 => self.ram[address as usize % 0x0800],
            // 0x2000..0x4000 => self.ppu.read_register(0x2000 + address % 8),
            // 0x4014 => self.ppu.read_register(address),
            0x4015 => self.apu.read_register(address),
            // Controllers needs mutable reference
            // 0x4016 => self.controller_1.read(),
            // 0x4017 => self.controller_2.read(),
            0x6000...0xFFFF => self.mapper.borrow().read(address),
            _ => panic!("unhandled cpu memory read at address: 0x{:04X?}", address),
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {
            0x0000..0x2000 => self.ram[address as usize % 0x0800] = value,
            0x2000..0x4000 => self.ppu.write_register(0x2000 + address % 8, value),
            0x4000..0x4014 => self.apu.write_register(address, value),
            // 0x4014 => self.ppu.write_register(address, value),
            0x4015 => self.apu.write_register(address, value),
            0x4016 => {
                self.controller_1.write(value);
                self.controller_2.write(value)
            }
            0x4017 => self.apu.write_register(address, value),
            0x6000...0xFFFF => self.mapper.borrow_mut().write(address, value),
            _ => panic!("unhandled cpu memory write at address: 0x{:04X?}", address),
        }
    }
}
