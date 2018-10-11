use super::apu::APU;
use super::controller::Controller;
use super::cpu::CpuMemory;
use super::cpu::CPU;
use super::mapper::create_mapper;
use super::ppu::PPU;
use super::rom::Rom;
use failure::Error;
use std::cell::RefCell;
use std::io::{Read, Seek};
use std::rc::Rc;

#[derive(Debug)]
pub struct VirtualConsole {
    cpu: CPU,
}

const CPU_FREQUENCY: i64 = 1_789_773;

impl VirtualConsole {
    pub fn new<T: Read + Seek>(reader: &mut T) -> Result<VirtualConsole, Error> {
        let rom = Rom::load(reader)?;
        let mapper = Rc::new(RefCell::new(create_mapper(rom)?));
        let ppu = PPU::new(Rc::clone(&mapper));
        let apu = APU::new();
        let player_1 = Controller::new();
        let player_2 = Controller::new();
        let cpu_mem = CpuMemory::new(mapper, ppu, apu, player_1, player_2);
        let cpu = CPU::new(cpu_mem);

        Ok(VirtualConsole {
            cpu: cpu,
        })
    }

    pub fn reset(&mut self) {
        self.cpu.mem.ppu.reset();
        self.cpu.reset();
    }

    pub fn step(&mut self) -> usize {
        let cpu_cycles = self.cpu.step();
        let ppu_cycles = cpu_cycles * 3;
        for _ in 0..ppu_cycles {
            let trigger_nmi = self.cpu.mem.ppu.step();
            if trigger_nmi {
                self.cpu.trigger_nmi();
            }
        }
        cpu_cycles
    }

    pub fn step_seconds(&mut self, seconds: i64) {
        let mut cycles = CPU_FREQUENCY * seconds / 1000;
        while cycles > 0i64 {
            cycles -= self.step() as i64;
        }
    }
}
