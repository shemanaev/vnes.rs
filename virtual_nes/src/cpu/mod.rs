mod cpu;
mod description;
mod implementation;
mod memory;

pub use self::cpu::CPU;
pub use self::memory::CpuMemory;

#[cfg(test)]
mod tests {
    use super::super::apu::APU;
    use super::super::controller::Controller;
    use super::super::mapper::create_mapper;
    use super::super::ppu::PPU;
    use super::super::tests::create_rom;
    use super::*;
    use std::cell::RefCell;
    use std::fs;
    // use std::io::Write;
    use std::rc::Rc;

    #[test]
    fn nestest_log() {
        // http://www.qmtpro.com/~nes/misc/
        let rom = create_rom("nestest/nestest").unwrap();
        let mapper = Rc::new(RefCell::new(create_mapper(rom).unwrap()));
        let mut ppu = PPU::new(Rc::clone(&mapper));
        ppu.reset();
        let apu = APU::new();
        let player_1 = Controller::new();
        let player_2 = Controller::new();
        let cpu_mem = CpuMemory::new(mapper, ppu, apu, player_1, player_2);
        let mut cpu = CPU::new(cpu_mem);

        cpu.reset();
        cpu.pc = 0xC000; // automated test starts at $C000

        // let mut logg = std::fs::File::create("./target/nes.log").unwrap();
        let log = fs::read_to_string("../roms/nestest/nestest.log").unwrap();
        let mut cycles: usize = 0;

        for s in log.lines() {
            let addr = u16::from_str_radix(&s[..4], 16).unwrap();
            let a = u8::from_str_radix(&s[50..52], 16).unwrap();
            let x = u8::from_str_radix(&s[55..57], 16).unwrap();
            let y = u8::from_str_radix(&s[60..62], 16).unwrap();
            let p = u8::from_str_radix(&s[65..67], 16).unwrap();
            let sp = u8::from_str_radix(&s[71..73], 16).unwrap();
            let cy = usize::from_str_radix(&s[78..81].trim(), 10).unwrap();
            // let sl = i32::from_str_radix(&s[85..].trim(), 10).unwrap();

            assert_eq!(addr, cpu.pc);
            assert_eq!(a, cpu.a);
            assert_eq!(x, cpu.x);
            assert_eq!(y, cpu.y);
            assert_eq!(p, cpu.get_flags());
            assert_eq!(sp, cpu.sp);
            assert_eq!(cy as usize, cycles);
            // assert_eq!(cy, cpu.mem.ppu.get_cycle());
            // assert_eq!(sl, cpu.mem.ppu.get_sl());

            // logg.write_all(cpu.print_instruction().as_bytes()).unwrap();
            // let cycles_p = format!(" CYC:{:>3}\n", cycles);
            // logg.write_all(cycles_p.as_bytes()).unwrap();

            cycles += cpu.step() * 3;
            cycles %= 341;
            // let cpu_cycles = cpu.step();
            // let ppu_cycles = cpu_cycles * 3;
            // for _ in 0..ppu_cycles {
            //     let trigger_nmi = cpu.mem.ppu.tick();
            //     if trigger_nmi {
            //         cpu.trigger_nmi();
            //     }
            //     cpu.mem.ppu.step();
            // }
        }

        // ROM reports tests success status at $02 and $03
        assert_eq!(cpu.read(0x02), 0);
        assert_eq!(cpu.read(0x03), 0);
    }

    #[test]
    fn official_only() {
        // https://wiki.nesdev.com/w/index.php/Emulator_tests#CPU_Tests
        let rom = create_rom("instr_test-v5/official_only").unwrap();
        let mapper = Rc::new(RefCell::new(create_mapper(rom).unwrap()));
        let ppu = PPU::new(Rc::clone(&mapper));
        let apu = APU::new();
        let player_1 = Controller::new();
        let player_2 = Controller::new();
        let cpu_mem = CpuMemory::new(mapper, ppu, apu, player_1, player_2);
        let mut cpu = CPU::new(cpu_mem);

        cpu.reset();

        let mut _cycles: usize = 0;
        loop {
            let s = cpu.read(0x6000);
            if s != 0x80 && s != 0 {
                println!("STATUS_CODE: {:02X}", s);
                break;
            }

            _cycles += cpu.step() * 3;
            _cycles %= 341;
        }

        let mut status = String::new();
        let mut i = 0x6004;
        loop {
            let c = cpu.read(i);
            if c == 0 {
                break;
            } else {
                status.push(c as char);
            }
            i += 1;
        }
        println!("STATUS: {}", status);

        // Check valid signature
        assert_eq!(cpu.read(0x6001), 0xDE);
        assert_eq!(cpu.read(0x6002), 0xB0);
        assert_eq!(cpu.read(0x6003), 0x61);
    }

    // #[test]
    // fn all_instrs() {
    //     // https://wiki.nesdev.com/w/index.php/Emulator_tests#CPU_Tests
    //     let rom = create_rom("instr_test-v5/all_instrs").unwrap();
    //     let mapper = Rc::new(RefCell::new(create_mapper(rom).unwrap()));
    //     let ppu = PPU::new(Rc::clone(&mapper));
    //     let apu = APU::new();
    //     let player_1 = Controller::new();
    //     let player_2 = Controller::new();
    //     let cpu_mem = CpuMemory::new(mapper, ppu, apu, player_1, player_2);
    //     let mut cpu = CPU::new(cpu_mem);

    //     cpu.reset();
    //     let mut logg = std::fs::File::create("./target/nes.log").unwrap();

    //     let mut _cycles: usize = 0;
    //     loop {
    //         let s = cpu.read(0x6000);
    //         if s != 0x80 && s != 0 {
    //             println!("STATUS_CODE: {:02X}", s);
    //             break;
    //         }

    //         logg.write_all(cpu.print_instruction().as_bytes()).unwrap();
    //         let cycles_p = format!(" CYC:{:>3}\n", _cycles);
    //         logg.write_all(cycles_p.as_bytes()).unwrap();

    //         _cycles += cpu.step() * 3;
    //         _cycles %= 341;
    //     }

    //     let mut status = String::new();
    //     let mut i = 0x6004;
    //     loop {
    //         let c = cpu.read(i);
    //         if c == 0 {
    //             break;
    //         } else {
    //             status.push(c as char);
    //         }
    //         i += 1;
    //     }
    //     println!("STATUS: {}", status);

    //     // Check valid signature
    //     assert_eq!(cpu.read(0x6001), 0xDE);
    //     assert_eq!(cpu.read(0x6002), 0xB0);
    //     assert_eq!(cpu.read(0x6003), 0x61);
    // }
}
