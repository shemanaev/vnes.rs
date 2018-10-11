use super::Mapper;
use super::super::memory::Memory;
use super::super::rom::{Rom, PRG_BANK_SIZE, Mirroring};

#[derive(Debug)]
pub struct Mapper2 {
    rom: Rom,
    banks_count: usize,
    current_bank: usize,
    last_bank: usize,
}

impl Mapper2 {
    pub fn from_rom(rom: Rom) -> Self {
        let count = rom.prg.len() / PRG_BANK_SIZE;
        let last = count - 1;

        Mapper2 {
            rom: rom,
            banks_count: count,
            current_bank: 0,
            last_bank: last,
        }
    }
}

impl Mapper for Mapper2 {
    fn get_mirroring(&self) -> &Mirroring {
        &self.rom.mirroring
    }
}

impl Memory for Mapper2 {
    fn read(&self, address: u16) -> u8 {
        let address = address as usize;
        match address {
            0x0000..0x2000 => self.rom.chr[address],
            0x6000..0x8000 => self.rom.ram[address - 0x6000],
            0x8000..0xC000 => self.rom.prg[self.current_bank * PRG_BANK_SIZE + address - 0x8000],
            0xC000...0xFFFF => self.rom.prg[self.last_bank * PRG_BANK_SIZE + address - 0xC000],
            _ => panic!("Read at address: {:04x}", address),
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        let address = address as usize;
        match address {
            0x0000..0x2000 => self.rom.chr[address] = value,
            0x6000..0x8000 => self.rom.ram[address - 0x6000] = value,
            0x8000...0xFFFF => self.current_bank = value as usize % self.banks_count,
            _ => panic!("Write at address: {:04x}", address),
        }
    }
}
