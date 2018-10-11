// For sanity
#![feature(nll)]
// Easier to create safe synchronization primitives
#![feature(optin_builtin_traits)]
// Result <-> Option
#![feature(transpose_result)]
// Exclusive ranges
#![feature(exclusive_range_pattern)]

// FIXME: remove this
#![allow(dead_code)]

#[macro_use]
extern crate failure;

mod cpu;
mod apu;
mod controller;
mod mapper;
mod memory;
mod ppu;
mod rom;
mod virtual_console;

pub use self::virtual_console::VirtualConsole;

#[cfg(test)]
mod tests {
    use super::rom::Rom;
    use failure::Error;
    use std::fs::File;

    /// Helper function for loading test rom file.
    ///
    /// ROMs can be found on:
    /// https://github.com/christopherpow/nes-test-roms
    /// https://wiki.nesdev.com/w/index.php/Emulator_tests#Validation_ROMs
    pub fn create_rom(name: &str) -> Result<Rom, Error> {
        let mut f = File::open(format!("../roms/{}.nes", name))?;
        Ok(Rom::load(&mut f)?)
    }
}
