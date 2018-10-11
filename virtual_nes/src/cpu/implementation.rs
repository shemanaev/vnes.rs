use super::super::memory::Memory;
use super::cpu::{Info, CPU};
use super::description::AddressingMode;

pub const IMPLEMENTATIONS: &'static [fn(&mut CPU, &Info); 256] = &[
    CPU::brk, CPU::ora, CPU::upl, CPU::slo, CPU::nop, CPU::ora, CPU::asl, CPU::slo, // 0x00
    CPU::php, CPU::ora, CPU::asl, CPU::upl, CPU::nop, CPU::ora, CPU::asl, CPU::slo, // 0x08
    CPU::bpl, CPU::ora, CPU::upl, CPU::slo, CPU::nop, CPU::ora, CPU::asl, CPU::slo, // 0x10
    CPU::clc, CPU::ora, CPU::nop, CPU::slo, CPU::nop, CPU::ora, CPU::asl, CPU::slo, // 0x18
    CPU::jsr, CPU::and, CPU::upl, CPU::rla, CPU::bit, CPU::and, CPU::rol, CPU::rla, // 0x20
    CPU::plp, CPU::and, CPU::rol, CPU::upl, CPU::bit, CPU::and, CPU::rol, CPU::rla, // 0x28
    CPU::bmi, CPU::and, CPU::upl, CPU::rla, CPU::nop, CPU::and, CPU::rol, CPU::rla, // 0x30
    CPU::sec, CPU::and, CPU::nop, CPU::rla, CPU::nop, CPU::and, CPU::rol, CPU::rla, // 0x38
    CPU::rti, CPU::eor, CPU::upl, CPU::sre, CPU::nop, CPU::eor, CPU::lsr, CPU::sre, // 0x40
    CPU::pha, CPU::eor, CPU::lsr, CPU::upl, CPU::jmp, CPU::eor, CPU::lsr, CPU::sre, // 0x48
    CPU::bvc, CPU::eor, CPU::upl, CPU::sre, CPU::nop, CPU::eor, CPU::lsr, CPU::sre, // 0x50
    CPU::cli, CPU::eor, CPU::nop, CPU::sre, CPU::nop, CPU::eor, CPU::lsr, CPU::sre, // 0x58
    CPU::rts, CPU::adc, CPU::upl, CPU::rra, CPU::nop, CPU::adc, CPU::ror, CPU::rra, // 0x60
    CPU::pla, CPU::adc, CPU::ror, CPU::upl, CPU::jmp, CPU::adc, CPU::ror, CPU::rra, // 0x68
    CPU::bvs, CPU::adc, CPU::upl, CPU::rra, CPU::nop, CPU::adc, CPU::ror, CPU::rra, // 0x70
    CPU::sei, CPU::adc, CPU::nop, CPU::rra, CPU::nop, CPU::adc, CPU::ror, CPU::rra, // 0x78
    CPU::nop, CPU::sta, CPU::nop, CPU::sax, CPU::sty, CPU::sta, CPU::stx, CPU::sax, // 0x80
    CPU::dey, CPU::upl, CPU::txa, CPU::upl, CPU::sty, CPU::sta, CPU::stx, CPU::sax, // 0x88
    CPU::bcc, CPU::sta, CPU::upl, CPU::upl, CPU::sty, CPU::sta, CPU::stx, CPU::sax, // 0x90
    CPU::tya, CPU::sta, CPU::txs, CPU::upl, CPU::upl, CPU::sta, CPU::upl, CPU::upl, // 0x98
    CPU::ldy, CPU::lda, CPU::ldx, CPU::lax, CPU::ldy, CPU::lda, CPU::ldx, CPU::lax, // 0xA0
    CPU::tay, CPU::lda, CPU::tax, CPU::upl, CPU::ldy, CPU::lda, CPU::ldx, CPU::lax, // 0xA8
    CPU::bcs, CPU::lda, CPU::upl, CPU::lax, CPU::ldy, CPU::lda, CPU::ldx, CPU::lax, // 0xB0
    CPU::clv, CPU::lda, CPU::tsx, CPU::upl, CPU::ldy, CPU::lda, CPU::ldx, CPU::lax, // 0xB8
    CPU::cpy, CPU::cmp, CPU::upl, CPU::dcp, CPU::cpy, CPU::cmp, CPU::dec, CPU::dcp, // 0xC0
    CPU::iny, CPU::cmp, CPU::dex, CPU::upl, CPU::cpy, CPU::cmp, CPU::dec, CPU::dcp, // 0xC8
    CPU::bne, CPU::cmp, CPU::upl, CPU::dcp, CPU::nop, CPU::cmp, CPU::dec, CPU::dcp, // 0xD0
    CPU::cld, CPU::cmp, CPU::nop, CPU::dcp, CPU::nop, CPU::cmp, CPU::dec, CPU::dcp, // 0xD8
    CPU::cpx, CPU::sbc, CPU::upl, CPU::isc, CPU::cpx, CPU::sbc, CPU::inc, CPU::isc, // 0xE0
    CPU::inx, CPU::sbc, CPU::nop, CPU::sbc, CPU::cpx, CPU::sbc, CPU::inc, CPU::isc, // 0xE8
    CPU::beq, CPU::sbc, CPU::upl, CPU::isc, CPU::nop, CPU::sbc, CPU::inc, CPU::isc, // 0xF0
    CPU::sed, CPU::sbc, CPU::nop, CPU::isc, CPU::nop, CPU::sbc, CPU::inc, CPU::isc, // 0xF8
];

impl CPU {
	fn upl(&mut self, info: &Info) {
		panic!(
			"Unimplemented instruction op: 0x{:02X?}, {:?}",
			info.i.opcode, info
		);
	}

	// NOP - No Operation
	fn nop(&mut self, _info: &Info) {}

	// SEI - Set Interrupt Disable
	fn sei(&mut self, _info: &Info) {
		self.i = 1;
	}

	// CLD - Clear Decimal Mode
	fn cld(&mut self, _info: &Info) {
		self.d = 0;
	}

	// LDX - Load X Register
	fn ldx(&mut self, info: &Info) {
		self.x = self.read(info.address);
		self.set_zn(self.x);
	}

	// TXS - Transfer X to Stack Pointer
	fn txs(&mut self, _info: &Info) {
		self.sp = self.x;
	}

	// LDA - Load Accumulator
	fn lda(&mut self, info: &Info) {
		self.a = self.read(info.address);
		self.set_zn(self.a);
	}

	// BPL - Branch if Positive
	fn bpl(&mut self, info: &Info) {
		if self.n == 0 {
			self.pc = info.address;
			self.add_branch_cycles(info);
		}
	}

	// STA - Store Accumulator
	fn sta(&mut self, info: &Info) {
		self.write(info.address, self.a);
	}

	// PHA - Push Accumulator
	fn pha(&mut self, _info: &Info) {
		self.push(self.a);
	}

	// INX - Increment X Register
	fn inx(&mut self, _info: &Info) {
		self.x = self.x.wrapping_add(1);
		self.set_zn(self.x);
	}

	// BNE - Branch if Not Equal
	fn bne(&mut self, info: &Info) {
		if self.z == 0 {
			self.pc = info.address;
			self.add_branch_cycles(info);
		}
	}

	// JMP - Jump
	fn jmp(&mut self, info: &Info) {
		self.pc = info.address;
	}

	// LSR - Logical Shift Right
	fn lsr(&mut self, info: &Info) {
		match info.mode {
			AddressingMode::Accumulator => {
				self.c = self.a & 1;
				self.a >>= 1;
				self.set_zn(self.a);
			}
			_ => {
				let mut value = self.read(info.address);
				self.c = value & 1;
				value >>= 1;
				self.write(info.address, value);
				self.set_zn(value);
			}
		}
	}

	// CMP - Compare
	fn cmp(&mut self, info: &Info) {
		let value = self.read(info.address);
		self.compare(self.a, value);
	}

	// PLA - Pull Accumulator
	fn pla(&mut self, _info: &Info) {
		self.a = self.pop();
		self.set_zn(self.a);
	}

	// BEQ - Branch if Equal
	fn beq(&mut self, info: &Info) {
		if self.z != 0 {
			self.pc = info.address;
			self.add_branch_cycles(info);
		}
	}

	// JSR - Jump to Subroutine
	fn jsr(&mut self, info: &Info) {
		self.push_word(self.pc.wrapping_sub(1));
		self.pc = info.address;
	}

	// LDY - Load Y Register
	fn ldy(&mut self, info: &Info) {
		self.y = self.read(info.address);
		self.set_zn(self.y);
	}

	// STY - Store Y Register
	fn sty(&mut self, info: &Info) {
		self.write(info.address, self.y);
	}

	// STX - Store X Register
	fn stx(&mut self, info: &Info) {
		self.write(info.address, self.x);
	}

	// INY - Increment Y Register
	fn iny(&mut self, _info: &Info) {
		self.y = self.y.wrapping_add(1);
		self.set_zn(self.y);
	}

	// INC - Increment Memory
	fn inc(&mut self, info: &Info) {
		let value = self.read(info.address).wrapping_add(1);
		self.write(info.address, value);
		self.set_zn(value);
	}

	// TAX - Transfer Accumulator to X
	fn tax(&mut self, _info: &Info) {
		self.x = self.a;
		self.set_zn(self.x);
	}

	// TSX - Transfer Stack Pointer to X
	fn tsx(&mut self, _info: &Info) {
		self.x = self.sp;
		self.set_zn(self.x);
	}

	// DEX - Decrement X Register
	fn dex(&mut self, _info: &Info) {
		self.x = self.x.wrapping_sub(1);
		self.set_zn(self.x);
	}

	// RTS - Return from Subroutine
	fn rts(&mut self, _info: &Info) {
		self.pc = self.pop_word().wrapping_add(1);
	}

	// SLO - ASL -> ORA
	fn slo(&mut self, info: &Info) {
		self.asl(info);
		self.ora(info);
	}

	// PHP - Push Processor Status
	pub fn php(&mut self, _info: &Info) {
		self.push(self.get_flags() | 0x10);
	}

	// SEC - Set Carry Flag
	fn sec(&mut self, _info: &Info) {
		self.c = 1;
	}

	// BCS - Branch if Carry Set
	fn bcs(&mut self, info: &Info) {
		if self.c != 0 {
			self.pc = info.address;
			self.add_branch_cycles(info);
		}
	}

	// CLC - Clear Carry Flag
	fn clc(&mut self, _info: &Info) {
		self.c = 0;
	}

	// BCC - Branch if Carry Clear
	fn bcc(&mut self, info: &Info) {
		if self.c == 0 {
			self.pc = info.address;
			self.add_branch_cycles(info);
		}
	}

	// BIT - Bit Test
	fn bit(&mut self, info: &Info) {
		let value = self.read(info.address);
		self.v = (value >> 6) & 1;
		self.set_z(value & self.a);
		self.set_n(value);
	}

	// BVS - Branch if Overflow Set
	fn bvs(&mut self, info: &Info) {
		if self.v != 0 {
			self.pc = info.address;
			self.add_branch_cycles(info);
		}
	}

	// BVC - Branch if Overflow Clear
	fn bvc(&mut self, info: &Info) {
		if self.v == 0 {
			self.pc = info.address;
			self.add_branch_cycles(info);
		}
	}

	// BRK - Force Interrupt
	fn brk(&mut self, info: &Info) {
		self.push_word(self.pc);
		// self.php(info);
		self.push(self.get_flags() | 0x30);
		self.sei(info);
		self.pc = self.mem.read_word(0xFFFE);
	}

	// RTI - Return from Interrupt
	fn rti(&mut self, info: &Info) {
		//let value = self.pop();
		//self.set_flags(value & 0xEF | 0x20);
		self.plp(info);
		self.pc = self.pop_word();
	}

	// AND - Logical AND
	fn and(&mut self, info: &Info) {
		self.a = self.a & self.read(info.address);
		self.set_zn(self.a);
	}

	// SED - Set Decimal Flag
	fn sed(&mut self, _info: &Info) {
		self.d = 1
	}

	// PLP - Pull Processor Status
	fn plp(&mut self, _info: &Info) {
		let value = self.pop();
		self.set_flags(value & 0xEF | 0x20);
		// self.set_flags(value | 0x30);
	}

	// BMI - Branch if Minus
	fn bmi(&mut self, info: &Info) {
		if self.n != 0 {
			self.pc = info.address;
			self.add_branch_cycles(info);
		}
	}

	// ROL - Rotate Left
	fn rol(&mut self, info: &Info) {
		match info.mode {
			AddressingMode::Accumulator => {
				let c = self.c;
				self.c = (self.a >> 7) & 1;
				self.a = (self.a << 1) | c;
				self.set_zn(self.a);
			}
			_ => {
				let c = self.c;
				let mut value = self.read(info.address);
				self.c = (value >> 7) & 1;
				value = (value << 1) | c;
				self.write(info.address, value);
				self.set_zn(value);
			}
		}
	}

	// ROR - Rotate Right
	fn ror(&mut self, info: &Info) {
		match info.mode {
			AddressingMode::Accumulator => {
				let c = self.c;
				self.c = self.a & 1;
				self.a = (self.a >> 1) | (c << 7);
				self.set_zn(self.a);
			}
			_ => {
				let c = self.c;
				let mut value = self.read(info.address);
				self.c = value & 1;
				value = (value >> 1) | (c << 7);
				self.write(info.address, value);
				self.set_zn(value);
			}
		}
	}

	// ORA - Logical Inclusive OR
	fn ora(&mut self, info: &Info) {
		self.a = self.a | self.read(info.address);
		self.set_zn(self.a);
	}

	// ADC - Add with Carry
	fn adc(&mut self, info: &Info) {
		let a = self.a;
		let b = self.read(info.address);
		let c = self.c;
		self.a = a.wrapping_add(b).wrapping_add(c);
		self.set_zn(self.a);

		if a as u16 + b as u16 + c as u16 > 0xFF {
			self.c = 1;
		} else {
			self.c = 0;
		}

		if (a ^ b) & 0x80 == 0 && (a ^ self.a) & 0x80 != 0 {
			self.v = 1;
		} else {
			self.v = 0;
		}
	}

	// RRA - ROR -> ADC
	fn rra(&mut self, info: &Info) {
		self.ror(info);
		self.adc(info);
	}

	// CLV - Clear Overflow Flag
	fn clv(&mut self, _info: &Info) {
		self.v = 0;
	}

	// EOR - Exclusive OR
	fn eor(&mut self, info: &Info) {
		self.a = self.a ^ self.read(info.address);
		self.set_zn(self.a);
	}

	// CPY - Compare Y Register
	fn cpy(&mut self, info: &Info) {
		let value = self.read(info.address);
		self.compare(self.y, value);
	}

	// CPX - Compare X Register
	fn cpx(&mut self, info: &Info) {
		let value = self.read(info.address);
		self.compare(self.x, value);
	}

	// SBC - Subtract with Carry
	fn sbc(&mut self, info: &Info) {
		let a = self.a;
		let b = self.read(info.address);
		let c = self.c;
		self.a = a.wrapping_sub(b).wrapping_sub(1u8.wrapping_sub(c));
		self.set_zn(self.a);

		if a as i16 - b as i16 - (1 - c as i16) >= 0 {
			self.c = 1;
		} else {
			self.c = 0;
		}

		if (a ^ b) & 0x80 != 0 && (a ^ self.a) & 0x80 != 0 {
			self.v = 1;
		} else {
			self.v = 0;
		}
	}

	// DEY - Decrement Y Register
	fn dey(&mut self, _info: &Info) {
		self.y = self.y.wrapping_sub(1);
		self.set_zn(self.y);
	}

	// TAY - Transfer Accumulator to Y
	fn tay(&mut self, _info: &Info) {
		self.y = self.a;
		self.set_zn(self.y);
	}

	// TYA - Transfer Y to Accumulator
	fn tya(&mut self, _info: &Info) {
		self.a = self.y;
		self.set_zn(self.a);
	}

	// TXA - Transfer X to Accumulator
	fn txa(&mut self, _info: &Info) {
		self.a = self.x;
		self.set_zn(self.a);
	}

	// ASL - Arithmetic Shift Left
	fn asl(&mut self, info: &Info) {
		match info.mode {
			AddressingMode::Accumulator => {
				self.c = (self.a >> 7) & 1;
				self.a <<= 1;
				self.set_zn(self.a);
			}
			_ => {
				let mut value = self.read(info.address);
				self.c = (value >> 7) & 1;
				value <<= 1;
				self.write(info.address, value);
				self.set_zn(value);
			}
		}
	}

	// DEC - Decrement Memory
	fn dec(&mut self, info: &Info) {
		let value = self.read(info.address).wrapping_sub(1);
		self.write(info.address, value);
		self.set_zn(value);
	}

	// LAX - LDA -> TAX
	fn lax(&mut self, info: &Info) {
		self.lda(info);
		self.tax(info);
	}

	// SAX - Stores the bitwise AND of A and X
	fn sax(&mut self, info: &Info) {
		self.sta(info);
		self.stx(info);
		self.write(info.address, self.a & self.x);
	}

	// DCP - DEC -> CMP
	fn dcp(&mut self, info: &Info) {
		self.dec(info);
		self.cmp(info);
	}

	// ISC - INC -> SBC
	fn isc(&mut self, info: &Info) {
		self.inc(info);
		self.sbc(info);
	}

	// RLA - ROL -> AND
	fn rla(&mut self, info: &Info) {
		self.rol(info);
		self.and(info);
	}

	// SRE - LSR -> EOR
	fn sre(&mut self, info: &Info) {
		self.lsr(info);
		self.eor(info);
	}

	// CLI - Clear Interrupt Disable
	fn cli(&mut self, _info: &Info) {
		self.i = 0;
	}
}
