use failure::{format_err, Error};

mod mapper1;
mod mapper2;

use self::mapper1::Mapper1;
use self::mapper2::Mapper2;
use super::memory::Memory;
use super::rom::{Mirroring, Rom};

pub trait Mapper: Memory {
    fn get_mirroring(&self) -> &Mirroring;
}

pub fn create_mapper(rom: Rom) -> Result<Box<dyn Mapper>, Error> {
    match rom.mapper {
        1 => Ok(Box::new(Mapper1::from_rom(rom))),
        0 | 2 => Ok(Box::new(Mapper2::from_rom(rom))),
        _ => Err(format_err!("Mapper {} not supported yet.", rom.mapper)),
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::create_rom;
    use super::create_mapper;

    #[test]
    fn mapper1() {
        let rom = create_rom("instr_test-v5/official_only").unwrap();
        let mut mapper = create_mapper(rom).unwrap();

        // CHR
        // mapper.write(0x1234, 12);
        // assert_eq!(mapper.read(0x1234), 12);

        // RAM
        mapper.write(0x6234, 12);
        assert_eq!(mapper.read(0x6234), 12);

        // PRG
        assert_eq!(mapper.read(0x8000), 255);
    }

    #[test]
    fn mapper2() {
        let rom = create_rom("nestest/nestest").unwrap();
        let mut mapper = create_mapper(rom).unwrap();

        // CHR
        mapper.write(0x1234, 12);
        assert_eq!(mapper.read(0x1234), 12);

        // RAM
        mapper.write(0x6234, 12);
        assert_eq!(mapper.read(0x6234), 12);

        // PRG
        assert_eq!(mapper.read(0x8000), 76);
    }
}
