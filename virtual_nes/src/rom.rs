use byteorder::{LittleEndian, ReadBytesExt};
use failure::Error;
use std::io::{Read, Seek, SeekFrom};

const INES_MAGIC: u32 = 0x1a53454e;
pub const PRG_BANK_SIZE: usize = 16 * 1024;
const CHR_BANK_SIZE: usize = 8 * 1024;
const PRG_RAM_SIZE: usize = 8 * 1024;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mirroring {
    Horizontal,
    Vertical,
    Single0,
    Single1,
    FourScreen,
}

#[derive(Debug)]
pub struct Rom {
    pub mirroring: Mirroring,
    pub mapper: u8,
    pub prg: Vec<u8>,
    pub chr: Vec<u8>,
    pub ram: Vec<u8>,
}

impl Rom {
    /// Load from standard iNES formatted ROM file.
    ///
    /// http://wiki.nesdev.com/w/index.php/INES
    pub fn load<T: Read + Seek>(reader: &mut T) -> Result<Self, Error> {
        let magic = reader.read_u32::<LittleEndian>()?;
        if magic != INES_MAGIC {
            return Err(format_err!(
                "This is not iNES ROM: magic mismatch 0x{:08x}",
                magic
            ));
        }

        let prg_rom_size = reader.read_u8()? as usize;
        let chr_rom_size = reader.read_u8()? as usize;
        let flags6 = reader.read_u8()?;
        let flags7 = reader.read_u8()?;
        let prg_ram_size = match reader.read_u8()? {
            0 => 1,
            v => v as usize,
        };

        let mapper = (flags7 & 0xF0) | ((flags6 & 0xF0) >> 4);
        let mirroring = {
            if flags6 & 0b1000 != 0 {
                Mirroring::FourScreen
            } else if flags6 & 0b1 == 0 {
                Mirroring::Horizontal
            } else {
                Mirroring::Vertical
            }
        };

        let has_trainer = flags6 & 0b100 != 0;
        reader.seek(SeekFrom::Current(7))?;
        if has_trainer {
            reader.seek(SeekFrom::Current(512))?;
        }

        let ram = vec![0; prg_ram_size * PRG_RAM_SIZE];
        let mut prg = vec![0; prg_rom_size * PRG_BANK_SIZE];
        let mut chr = vec![0; chr_rom_size * CHR_BANK_SIZE];

        reader.read_exact(&mut prg)?;
        reader.read_exact(&mut chr)?;

        if chr_rom_size == 0 {
            chr = vec![0; 1 * CHR_BANK_SIZE];
        }

        Ok(Rom {
            mirroring: mirroring,
            mapper: mapper,
            prg: prg,
            chr: chr,
            ram: ram,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::create_rom;
    use super::{Mirroring, PRG_RAM_SIZE};

    #[test]
    fn rom_loading() {
        let rom = create_rom("instr_test-v5/official_only").unwrap();

        //println!("{:?}", rom);
        assert_eq!(rom.mapper, 1);
        assert_eq!(rom.mirroring, Mirroring::Vertical);
        assert_eq!(rom.ram.len(), PRG_RAM_SIZE);
    }
}
