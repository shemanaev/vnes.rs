use super::super::memory::Memory;
use super::description::{AddressingMode, DESCRIPTIONS};
use super::implementation::IMPLEMENTATIONS;
use super::memory::CpuMemory;
use super::description::Instruction;

#[derive(Debug)]
pub struct CPU {
    pub mem: CpuMemory,
	pub pc: u16, // program counter
    pub sp: u8,  // stack pointer
    pub a: u8,   // accumulator
    pub x: u8,   // x register
    pub y: u8,   // y register
    pub c: u8,   // carry flag
    pub z: u8,   // zero flag
    pub i: u8,   // interrupt flag
    pub d: u8,   // decimal flag
    pub v: u8,   // overflow flag
    pub n: u8,   // negative flag
    b: u8,       // unused flag
    u: u8,       // unused flag
    interrupt: Interrupt,
    cycles: usize,
    stall: usize,
}

#[derive(Debug)]
pub enum Interrupt {
    None,
    NMI,
    IRQ,
}

#[derive(Debug)]
pub struct Info {
    pub address: u16,
    pub pc: u16,
    pub mode: &'static AddressingMode,
    pub i: &'static Instruction,
}

impl Info {
    pub fn new() -> Info {
        Info {
            address: 0,
            pc: 0,
            mode: &AddressingMode::Absolute,
            i: &DESCRIPTIONS[0],
        }
    }
}

impl CPU {
    pub fn new(mem: CpuMemory) -> CPU {
        CPU {
            mem: mem,
            pc: 0,
            sp: 0,
            a: 0,
            x: 0,
            y: 0,
            c: 0,
            z: 0,
            i: 0,
            d: 0,
            v: 0,
            n: 0,
            b: 0,
            u: 0,
            interrupt: Interrupt::None,
            cycles: 0,
            stall: 0,
        }
    }

    pub fn reset(&mut self) {
        self.pc = self.mem.read_word(0xFFFC);
        self.sp = 0xFD;
        self.set_flags(0x24);
    }

    // Push byte onto stack
    pub fn push(&mut self, value: u8) {
        let address = self.sp as u16 | 0x100;
        self.mem.write(address, value);
        self.sp = self.sp.wrapping_sub(1);
    }

    // Pop byte from stack
    pub fn pop(&mut self) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        self.read(self.sp as u16 | 0x100)
    }

    // Push word onto stack
    pub fn push_word(&mut self, value: u16) {
        self.push((value >> 8) as u8);
        self.push((value & 0x00ff) as u8);
    }

    // Pop word from stack
    pub fn pop_word(&mut self) -> u16 {
        let lo = self.pop() as u16;
        let hi = self.pop() as u16;
        (hi << 8) | lo
    }

    // returns flags register as byte
    pub fn get_flags(&self) -> u8 {
        let mut value: u8 = 0;
        value |= self.c << 0;
        value |= self.z << 1;
        value |= self.i << 2;
        value |= self.d << 3;
        value |= self.b << 4;
        value |= self.u << 5;
        value |= self.v << 6;
        value |= self.n << 7;
        value
    }

    pub fn set_flags(&mut self, value: u8) {
        self.c = (value >> 0) & 1;
        self.z = (value >> 1) & 1;
        self.i = (value >> 2) & 1;
        self.d = (value >> 3) & 1;
        self.b = (value >> 4) & 1;
        self.u = (value >> 5) & 1;
        self.v = (value >> 6) & 1;
        self.n = (value >> 7) & 1;
    }

	// sets the zero flag if the argument is zero
	pub fn set_z(&mut self, value: u8) {
		self.z = match value {
			0 => 1,
			_ => 0,
		}
	}
	
	// sets the negative flag if the argument is negative (high bit is set)
	pub fn set_n(&mut self, value: u8) {
		self.n = match value & 0x80 {
			0 => 0,
			_ => 1,
		}
	}

	// set both zero and negative flags
	pub fn set_zn(&mut self, value: u8) {
		self.set_z(value);
		self.set_n(value);
	}

	// adds a cycle for taking a branch and adds another cycle
	// if the branch jumps to a new page
	pub fn add_branch_cycles(&mut self, info: &Info) {
		self.cycles += 1;
		if is_different_pages(info.pc, info.address) {
			self.cycles += 1;
		}
	}

    // compares two values and sets zero, negative and carry flags
    pub fn compare(&mut self, a: u8, b: u8) {
        let value = (a as u32).wrapping_sub(b as u32);
        self.set_zn(value as u8);
        //if a >= b {
        if value & 0x100 == 0 {
            self.c = 1;
        } else {
            self.c = 0;
        }
    }

    // NMI - Non-Maskable Interrupt
    pub fn nmi(&mut self) {
        self.push_word(self.pc);
        self.php(&Info::new());
        self.pc = self.mem.read_word(0xFFFA);
        self.i = 1;
        self.cycles += 7;
    }

    // IRQ - IRQ Interrupt
    pub fn irq(&mut self) {
        self.push_word(self.pc);
        self.php(&Info::new());
        self.pc = self.mem.read_word(0xFFFE);
        self.i = 1;
        self.cycles += 7;
    }

    pub fn trigger_nmi(&mut self) {
        self.interrupt = Interrupt::NMI;
    }

    pub fn trigger_irq(&mut self) {
        self.interrupt = Interrupt::IRQ;
    }

    // Controller and PPU needs to be mutable while reading
    pub fn read(&mut self, address: u16) -> u8 {
        match address {
            0x2000..0x4000 => self.mem.ppu.read_register(0x2000 + address % 8),
            0x4014 => self.mem.ppu.read_register(address),
            0x4016 => self.mem.controller_1.read(),
            0x4017 => self.mem.controller_2.read(),
            _ => self.mem.read(address),
        }
    }

    // DMA demands CPU access
    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            _ => self.mem.write(address, value),
        }
    }

    // returns operand address based on addressing mode
    // and extra cycles count when pages is crossed
    fn get_address(&mut self, pc: u16, instruction: &Instruction) -> (u16, usize) {
        let mut page_crossed = false;
        let address = match instruction.addressing {
            AddressingMode::Absolute => self.mem.read_word(pc),
            AddressingMode::AbsoluteX => {
                let addr = self.mem.read_word(pc).wrapping_add(self.x as u16);
                page_crossed = is_different_pages(addr - self.x as u16, addr);
                addr
            }
            AddressingMode::AbsoluteY => {
                let addr = self.mem.read_word(pc).wrapping_add(self.y as u16);
                page_crossed = is_different_pages(addr.wrapping_sub(self.y as u16), addr);
                addr
            }
            AddressingMode::Accumulator => 0,
            AddressingMode::Immediate => pc,
            AddressingMode::Implied => 0,
            AddressingMode::IndexedIndirect => {
                let addr = (self.mem.read(pc) as u16).wrapping_add(self.x as u16) & 0xFF;
                self.mem.read_word_bug(addr)
            }
            AddressingMode::Indirect => {
                let addr = self.mem.read_word(pc);
                self.mem.read_word_bug(addr)
            }
            AddressingMode::IndirectIndexed => {
                let addr = self.mem.read_word_bug(self.mem.read(pc) as u16).wrapping_add(self.y as u16);
                page_crossed = is_different_pages(addr.wrapping_sub(self.y as u16), addr);
                addr
            }
            AddressingMode::Relative => {
                match self.mem.read(pc) as u16 {
                    offset if offset < 0x80 => self.pc.wrapping_add(2).wrapping_add(offset),
                    offset => self.pc.wrapping_add(2).wrapping_add(offset).wrapping_sub(0x100)
                }
            }
            AddressingMode::ZeroPage => self.mem.read(pc) as u16,
            AddressingMode::ZeroPageX => (self.mem.read(pc).wrapping_add(self.x)) as u16 & 0xFF,
            AddressingMode::ZeroPageY => (self.mem.read(pc).wrapping_add(self.y)) as u16 & 0xFF,
        };

        let cycles = match page_crossed {
            true => instruction.page_cycles,
            false => 0, 
        };

        (address, cycles)
    }

    pub fn step(&mut self) -> usize {
        if self.stall > 0 {
            self.stall -= 1;
            return 1
        }

        let cycles = self.cycles;

        match self.interrupt {
            Interrupt::NMI => self.nmi(),
            Interrupt::IRQ => self.irq(),
            _ => {}
        }
        self.interrupt = Interrupt::None;

        let opcode = self.mem.read(self.pc) as usize;
        let instruction = &DESCRIPTIONS[opcode];
        let fun = &IMPLEMENTATIONS[opcode];

        let pc = self.pc.wrapping_add(1);
        let (address, page_cycles) = self.get_address(pc, instruction);
        self.cycles += instruction.cycles;
        self.cycles += page_cycles;
        self.pc += instruction.size;
        
        let info = Info {
            address: address,
            pc: self.pc,
            mode: &instruction.addressing,
            i: &instruction,
        };
        fun(self, &info);

        //#[cfg(test)]
        // self.dump_regs(&info);
        // println!("{}", self.print_instruction());

        self.cycles - cycles
    }

    #[cfg(test)]
    pub fn print_instruction(&mut self) -> String {
        let opcode = self.read(self.pc);
        let instruction = &DESCRIPTIONS[opcode as usize];
        let o1 = self.read(self.pc + 1);
        let o2 = self.read(self.pc + 2);
        let w0 = format!("{:02X}", opcode);
        let mut w1 = format!("{:02X}", o1);
        let mut w2 = format!("{:02X}", o2);
        if instruction.size < 2 {
            w1 = "  ".to_string();
        }
        if instruction.size < 3 {
            w2 = "  ".to_string();
        }

        let operands = if instruction.size == 2 {
            format!("{:02X}", self.read(self.pc + 1) as u16)
        } else if instruction.size == 3 {
            let a = (self.read(self.pc + 2) as u16) << 8 | self.read(self.pc + 1) as u16;
            format!("{:04X}", a)
        } else {
            "".to_string()
        };

        let operand = match instruction.addressing {
            AddressingMode::Absolute => format!("${}", operands),
            AddressingMode::Immediate => format!("#${}", operands),
            AddressingMode::ZeroPage => format!("${} = {:02X}", operands, self.read(o1 as u16)),
            AddressingMode::Relative =>  {
                let address = match o1 as u16 {
                    offset if offset < 0x80 => self.pc + 2 + offset,
                    offset => self.pc + 2 + offset - 0x100
                };
                format!("${:04X}", address)
            }
            AddressingMode::Accumulator => "A".to_string(),
            AddressingMode::IndirectIndexed => {
                let addr = self.mem.read_word_bug(o1 as u16).wrapping_add(self.y as u16);
                let addr2 = self.mem.read_word(o1 as u16).wrapping_add(self.y as u16);
                let value = self.read(addr);
                format!("(${:02X}),Y = {:04X} @ {:04X} = {:02X}", o1, addr2, addr, value)
            }
            _ => "".to_string(),
        };

        format!("{:04X}  {} {} {}  {} {:27} A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X}", 
            self.pc,
            w0,
            w1,
            w2,
            instruction.name,
            operand,
            self.a, 
            self.x, 
            self.y, 
            self.get_flags(), 
            self.sp)
    }

    // #[cfg(test)]
    // fn dump_regs(&mut self, info: &Info) {
    //     println!("op: 0x{:02X}, name: {}, pc: 0x{:04X}, addr: 0x{:04X}, sp: 0x{:02X}, a: 0x{:02X}, x: 0x{:02X}, y: 0x{:02X}, c: {}, z: {}, i: {}, d: {}, v: {}, n: {}", 
    //         info.i.opcode,
    //         info.i.name,
    //         self.pc, 
    //         info.address,
    //         self.sp, 
    //         self.a, 
    //         self.x, 
    //         self.y, 
    //         self.c, 
    //         self.z, 
    //         self.i, 
    //         self.d, 
    //         self.v, 
    //         self.n);
    // }
}

// Is the two addresses reference different pages
#[inline(always)]
fn is_different_pages(a: u16, b: u16) -> bool {
	a & 0xFF00 != b & 0xFF00
}
