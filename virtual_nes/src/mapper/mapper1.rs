use super::Mapper;
use super::super::memory::Memory;
use super::super::rom::{Rom, Mirroring};

const PRG_ROM_SIZE: i32 = 16 * 1024;
const CHR_ROM_SIZE: i32 = 4 * 1024;

#[derive(Debug)]
pub struct Mapper1 {
    rom: Rom,
    prg_bank: u8,
    shift: u8,
    control: u8,
    prg_mode: u8,
    chr_mode: u8,
    chr_bank_0: u8,
    chr_bank_1: u8,
    prg_offsets: [usize; 2],
    chr_offsets: [usize; 2],
}

impl Mapper1 {
    pub fn from_rom(rom: Rom) -> Self {
        let prg_offset_1 = Self::prg_bank_offset(&rom, -1);

        Mapper1 {
            rom: rom,
            prg_bank: 0,
            shift: 0x10,
            control: 0,
            prg_mode: 0,
            chr_mode: 0,
            chr_bank_0: 0,
            chr_bank_1: 0,
            prg_offsets: [0, prg_offset_1],
            chr_offsets: [0, 0],
        }
    }

    fn load_register(&mut self, address: u16, value: u8) {
        if value & 0x80 == 0x80 {
            self.shift = 0x10;
            self.write_control(self.control | 0x0C);
        } else {
            let complete = self.shift & 1 == 1;
            self.shift >>= 1;
            self.shift |= (value & 1) << 4;
            if complete {
                self.write_register(address, self.shift);
                self.shift = 0x10;
            }
        }
    }

    fn write_control(&mut self, value: u8) {
        self.control = value;
        self.chr_mode = (value >> 4) & 1;
        self.prg_mode = (value >> 2) & 3;
        match value & 3 {
            0 => self.rom.mirroring = Mirroring::Single0,
            1 => self.rom.mirroring = Mirroring::Single1,
            2 => self.rom.mirroring = Mirroring::Vertical,
            3 => self.rom.mirroring = Mirroring::Horizontal,
            _ => {}
        }
        self.update_offsets();
    }

    fn write_register(&mut self, address: u16, value: u8) {
        match address {
            0x0000...0x9FFF => self.write_control(value),
            0xA000...0xBFFF => self.write_chr_bank_0(value),
            0xC000...0xDFFF => self.write_chr_bank_1(value),
            _ => self.write_prg_bank(value),
        }
    }

    fn write_chr_bank_0(&mut self, value: u8) {
        self.chr_bank_0 = value;
        self.update_offsets();
    }

    fn write_chr_bank_1(&mut self, value: u8) {
        self.chr_bank_1 = value;
        self.update_offsets();
    }

    fn write_prg_bank(&mut self, value: u8) {
        self.prg_bank = value;
        self.update_offsets();
    }

    fn update_offsets(&mut self) {
        match self.prg_mode {
            0 | 1 => {
                self.prg_offsets[0] = Self::prg_bank_offset(&self.rom, self.prg_bank as i32 & 0xFE);
                self.prg_offsets[1] = Self::prg_bank_offset(&self.rom, self.prg_bank as i32 | 0x01);
            }
            2 => {
                self.prg_offsets[0] = 0;
                self.prg_offsets[1] = Self::prg_bank_offset(&self.rom, self.prg_bank as i32);
            }
            3 => {
                self.prg_offsets[0] = Self::prg_bank_offset(&self.rom, self.prg_bank as i32);
                self.prg_offsets[1] = Self::prg_bank_offset(&self.rom, -1);
            }
            _ => {}
        }

        match self.chr_mode {
            0 => {
                self.chr_offsets[0] = Self::chr_bank_offset(&self.rom, self.chr_bank_0 as i32 & 0xFE);
                self.chr_offsets[1] = Self::chr_bank_offset(&self.rom, self.chr_bank_0 as i32 | 0x01);
            }
            1 => {
                self.chr_offsets[0] = Self::chr_bank_offset(&self.rom, self.chr_bank_0 as i32);
                self.chr_offsets[1] = Self::chr_bank_offset(&self.rom, self.chr_bank_1 as i32);
            }
            _ => {}
        }
    }

    fn prg_bank_offset(rom: &Rom, index: i32) -> usize {
        let mut index = index;
        if index >= 0x80 {
            index -= 0x100;
        }
        index %= rom.prg.len() as i32 / PRG_ROM_SIZE;
        let mut offset = index * PRG_ROM_SIZE;
        if offset < 0 {
            offset += rom.prg.len() as i32;
        }
        offset as usize
    }

    fn chr_bank_offset(rom: &Rom, index: i32) -> usize {
        let mut index = index;
        if index >= 0x80 {
            index -= 0x100;
        }
        //index %= rom.chr.len() as i32 / CHR_ROM_SIZE;
        index = index.checked_rem(rom.chr.len() as i32 / CHR_ROM_SIZE).unwrap_or(0);
        let mut offset = index * CHR_ROM_SIZE;
        if offset < 0 {
            offset += rom.chr.len() as i32;
        }
        offset as usize
    }
}

impl Mapper for Mapper1 {
    fn get_mirroring(&self) -> &Mirroring {
        &self.rom.mirroring
    }
}

impl Memory for Mapper1 {
    fn read(&self, address: u16) -> u8 {
        let address = address as usize;
        match address {
            0x0000..0x2000 => {
                let bank = address / 0x1000;
                let offset = address % 0x1000;
                self.rom.chr[self.chr_offsets[bank] + offset]
            }
            0x6000..0x8000 => self.rom.ram[address - 0x6000],
            0x8000...0xFFFF => {
                let address = address - 0x8000;
                let bank = address / 0x4000;
                let offset = address % 0x4000;
                self.rom.prg[self.prg_offsets[bank] + offset]
            }
            _ => panic!("Read at address: {:04x}", address),
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        let addr = address as usize;
        match addr {
            0x0000..0x2000 => {
                let bank = addr / 0x1000;
                let offset = addr % 0x1000;
                self.rom.chr[self.chr_offsets[bank] + offset] = value
            }
            0x6000..0x8000 => self.rom.ram[addr - 0x6000] = value,
            0x8000...0xFFFF => self.load_register(address, value),
            _ => panic!("Write at address: {:04x}", address),
        }
    }
}
