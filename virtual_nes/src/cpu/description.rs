#[derive(Debug)]
pub enum AddressingMode {
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Accumulator,
    Immediate,
    Implied,
    IndexedIndirect,
    Indirect,
    IndirectIndexed,
    Relative,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
}

#[derive(Debug)]
pub struct Instruction {
    pub opcode: u8,
    pub name: &'static str,
    pub size: u16,
    pub cycles: usize,
    pub page_cycles: usize,
    pub addressing: AddressingMode,
}

pub const DESCRIPTIONS: &'static [Instruction; 256] = &[
Instruction {opcode: 0x00, name: "BRK", size: 1, cycles: 7, page_cycles: 0, addressing: AddressingMode::Implied,},
Instruction {opcode: 0x01, name: "ORA", size: 2, cycles: 6, page_cycles: 0, addressing: AddressingMode::IndexedIndirect,},
Instruction {opcode: 0x02, name: "KIL", size: 0, cycles: 2, page_cycles: 0, addressing: AddressingMode::Implied,},
Instruction {opcode: 0x03, name: "SLO", size: 2, cycles: 8, page_cycles: 0, addressing: AddressingMode::IndexedIndirect,},
Instruction {opcode: 0x04, name: "NOP", size: 2, cycles: 3, page_cycles: 0, addressing: AddressingMode::ZeroPage,},
Instruction {opcode: 0x05, name: "ORA", size: 2, cycles: 3, page_cycles: 0, addressing: AddressingMode::ZeroPage,},
Instruction {opcode: 0x06, name: "ASL", size: 2, cycles: 5, page_cycles: 0, addressing: AddressingMode::ZeroPage,},
Instruction {opcode: 0x07, name: "SLO", size: 2, cycles: 5, page_cycles: 0, addressing: AddressingMode::ZeroPage,},
Instruction {opcode: 0x08, name: "PHP", size: 1, cycles: 3, page_cycles: 0, addressing: AddressingMode::Implied,},
Instruction {opcode: 0x09, name: "ORA", size: 2, cycles: 2, page_cycles: 0, addressing: AddressingMode::Immediate,},
Instruction {opcode: 0x0A, name: "ASL", size: 1, cycles: 2, page_cycles: 0, addressing: AddressingMode::Accumulator,},
Instruction {opcode: 0x0B, name: "ANC", size: 0, cycles: 2, page_cycles: 0, addressing: AddressingMode::Immediate,},
Instruction {opcode: 0x0C, name: "NOP", size: 3, cycles: 4, page_cycles: 0, addressing: AddressingMode::Absolute,},
Instruction {opcode: 0x0D, name: "ORA", size: 3, cycles: 4, page_cycles: 0, addressing: AddressingMode::Absolute,},
Instruction {opcode: 0x0E, name: "ASL", size: 3, cycles: 6, page_cycles: 0, addressing: AddressingMode::Absolute,},
Instruction {opcode: 0x0F, name: "SLO", size: 3, cycles: 6, page_cycles: 0, addressing: AddressingMode::Absolute,},
Instruction {opcode: 0x10, name: "BPL", size: 2, cycles: 2, page_cycles: 1, addressing: AddressingMode::Relative,},
Instruction {opcode: 0x11, name: "ORA", size: 2, cycles: 5, page_cycles: 1, addressing: AddressingMode::IndirectIndexed,},
Instruction {opcode: 0x12, name: "KIL", size: 0, cycles: 2, page_cycles: 0, addressing: AddressingMode::Implied,},
Instruction {opcode: 0x13, name: "SLO", size: 2, cycles: 8, page_cycles: 0, addressing: AddressingMode::IndirectIndexed,},
Instruction {opcode: 0x14, name: "NOP", size: 2, cycles: 4, page_cycles: 0, addressing: AddressingMode::ZeroPageX,},
Instruction {opcode: 0x15, name: "ORA", size: 2, cycles: 4, page_cycles: 0, addressing: AddressingMode::ZeroPageX,},
Instruction {opcode: 0x16, name: "ASL", size: 2, cycles: 6, page_cycles: 0, addressing: AddressingMode::ZeroPageX,},
Instruction {opcode: 0x17, name: "SLO", size: 2, cycles: 6, page_cycles: 0, addressing: AddressingMode::ZeroPageX,},
Instruction {opcode: 0x18, name: "CLC", size: 1, cycles: 2, page_cycles: 0, addressing: AddressingMode::Implied,},
Instruction {opcode: 0x19, name: "ORA", size: 3, cycles: 4, page_cycles: 1, addressing: AddressingMode::AbsoluteY,},
Instruction {opcode: 0x1A, name: "NOP", size: 1, cycles: 2, page_cycles: 0, addressing: AddressingMode::Implied,},
Instruction {opcode: 0x1B, name: "SLO", size: 3, cycles: 7, page_cycles: 0, addressing: AddressingMode::AbsoluteY,},
Instruction {opcode: 0x1C, name: "NOP", size: 3, cycles: 4, page_cycles: 1, addressing: AddressingMode::AbsoluteX,},
Instruction {opcode: 0x1D, name: "ORA", size: 3, cycles: 4, page_cycles: 1, addressing: AddressingMode::AbsoluteX,},
Instruction {opcode: 0x1E, name: "ASL", size: 3, cycles: 7, page_cycles: 0, addressing: AddressingMode::AbsoluteX,},
Instruction {opcode: 0x1F, name: "SLO", size: 3, cycles: 7, page_cycles: 0, addressing: AddressingMode::AbsoluteX,},
Instruction {opcode: 0x20, name: "JSR", size: 3, cycles: 6, page_cycles: 0, addressing: AddressingMode::Absolute,},
Instruction {opcode: 0x21, name: "AND", size: 2, cycles: 6, page_cycles: 0, addressing: AddressingMode::IndexedIndirect,},
Instruction {opcode: 0x22, name: "KIL", size: 0, cycles: 2, page_cycles: 0, addressing: AddressingMode::Implied,},
Instruction {opcode: 0x23, name: "RLA", size: 2, cycles: 8, page_cycles: 0, addressing: AddressingMode::IndexedIndirect,},
Instruction {opcode: 0x24, name: "BIT", size: 2, cycles: 3, page_cycles: 0, addressing: AddressingMode::ZeroPage,},
Instruction {opcode: 0x25, name: "AND", size: 2, cycles: 3, page_cycles: 0, addressing: AddressingMode::ZeroPage,},
Instruction {opcode: 0x26, name: "ROL", size: 2, cycles: 5, page_cycles: 0, addressing: AddressingMode::ZeroPage,},
Instruction {opcode: 0x27, name: "RLA", size: 2, cycles: 5, page_cycles: 0, addressing: AddressingMode::ZeroPage,},
Instruction {opcode: 0x28, name: "PLP", size: 1, cycles: 4, page_cycles: 0, addressing: AddressingMode::Implied,},
Instruction {opcode: 0x29, name: "AND", size: 2, cycles: 2, page_cycles: 0, addressing: AddressingMode::Immediate,},
Instruction {opcode: 0x2A, name: "ROL", size: 1, cycles: 2, page_cycles: 0, addressing: AddressingMode::Accumulator,},
Instruction {opcode: 0x2B, name: "ANC", size: 0, cycles: 2, page_cycles: 0, addressing: AddressingMode::Immediate,},
Instruction {opcode: 0x2C, name: "BIT", size: 3, cycles: 4, page_cycles: 0, addressing: AddressingMode::Absolute,},
Instruction {opcode: 0x2D, name: "AND", size: 3, cycles: 4, page_cycles: 0, addressing: AddressingMode::Absolute,},
Instruction {opcode: 0x2E, name: "ROL", size: 3, cycles: 6, page_cycles: 0, addressing: AddressingMode::Absolute,},
Instruction {opcode: 0x2F, name: "RLA", size: 3, cycles: 6, page_cycles: 0, addressing: AddressingMode::Absolute,},
Instruction {opcode: 0x30, name: "BMI", size: 2, cycles: 2, page_cycles: 1, addressing: AddressingMode::Relative,},
Instruction {opcode: 0x31, name: "AND", size: 2, cycles: 5, page_cycles: 1, addressing: AddressingMode::IndirectIndexed,},
Instruction {opcode: 0x32, name: "KIL", size: 0, cycles: 2, page_cycles: 0, addressing: AddressingMode::Implied,},
Instruction {opcode: 0x33, name: "RLA", size: 2, cycles: 8, page_cycles: 0, addressing: AddressingMode::IndirectIndexed,},
Instruction {opcode: 0x34, name: "NOP", size: 2, cycles: 4, page_cycles: 0, addressing: AddressingMode::ZeroPageX,},
Instruction {opcode: 0x35, name: "AND", size: 2, cycles: 4, page_cycles: 0, addressing: AddressingMode::ZeroPageX,},
Instruction {opcode: 0x36, name: "ROL", size: 2, cycles: 6, page_cycles: 0, addressing: AddressingMode::ZeroPageX,},
Instruction {opcode: 0x37, name: "RLA", size: 2, cycles: 6, page_cycles: 0, addressing: AddressingMode::ZeroPageX,},
Instruction {opcode: 0x38, name: "SEC", size: 1, cycles: 2, page_cycles: 0, addressing: AddressingMode::Implied,},
Instruction {opcode: 0x39, name: "AND", size: 3, cycles: 4, page_cycles: 1, addressing: AddressingMode::AbsoluteY,},
Instruction {opcode: 0x3A, name: "NOP", size: 1, cycles: 2, page_cycles: 0, addressing: AddressingMode::Implied,},
Instruction {opcode: 0x3B, name: "RLA", size: 3, cycles: 7, page_cycles: 0, addressing: AddressingMode::AbsoluteY,},
Instruction {opcode: 0x3C, name: "NOP", size: 3, cycles: 4, page_cycles: 1, addressing: AddressingMode::AbsoluteX,},
Instruction {opcode: 0x3D, name: "AND", size: 3, cycles: 4, page_cycles: 1, addressing: AddressingMode::AbsoluteX,},
Instruction {opcode: 0x3E, name: "ROL", size: 3, cycles: 7, page_cycles: 0, addressing: AddressingMode::AbsoluteX,},
Instruction {opcode: 0x3F, name: "RLA", size: 3, cycles: 7, page_cycles: 0, addressing: AddressingMode::AbsoluteX,},
Instruction {opcode: 0x40, name: "RTI", size: 1, cycles: 6, page_cycles: 0, addressing: AddressingMode::Implied,},
Instruction {opcode: 0x41, name: "EOR", size: 2, cycles: 6, page_cycles: 0, addressing: AddressingMode::IndexedIndirect,},
Instruction {opcode: 0x42, name: "KIL", size: 0, cycles: 2, page_cycles: 0, addressing: AddressingMode::Implied,},
Instruction {opcode: 0x43, name: "SRE", size: 2, cycles: 8, page_cycles: 0, addressing: AddressingMode::IndexedIndirect,},
Instruction {opcode: 0x44, name: "NOP", size: 2, cycles: 3, page_cycles: 0, addressing: AddressingMode::ZeroPage,},
Instruction {opcode: 0x45, name: "EOR", size: 2, cycles: 3, page_cycles: 0, addressing: AddressingMode::ZeroPage,},
Instruction {opcode: 0x46, name: "LSR", size: 2, cycles: 5, page_cycles: 0, addressing: AddressingMode::ZeroPage,},
Instruction {opcode: 0x47, name: "SRE", size: 2, cycles: 5, page_cycles: 0, addressing: AddressingMode::ZeroPage,},
Instruction {opcode: 0x48, name: "PHA", size: 1, cycles: 3, page_cycles: 0, addressing: AddressingMode::Implied,},
Instruction {opcode: 0x49, name: "EOR", size: 2, cycles: 2, page_cycles: 0, addressing: AddressingMode::Immediate,},
Instruction {opcode: 0x4A, name: "LSR", size: 1, cycles: 2, page_cycles: 0, addressing: AddressingMode::Accumulator,},
Instruction {opcode: 0x4B, name: "ALR", size: 0, cycles: 2, page_cycles: 0, addressing: AddressingMode::Immediate,},
Instruction {opcode: 0x4C, name: "JMP", size: 3, cycles: 3, page_cycles: 0, addressing: AddressingMode::Absolute,},
Instruction {opcode: 0x4D, name: "EOR", size: 3, cycles: 4, page_cycles: 0, addressing: AddressingMode::Absolute,},
Instruction {opcode: 0x4E, name: "LSR", size: 3, cycles: 6, page_cycles: 0, addressing: AddressingMode::Absolute,},
Instruction {opcode: 0x4F, name: "SRE", size: 3, cycles: 6, page_cycles: 0, addressing: AddressingMode::Absolute,},
Instruction {opcode: 0x50, name: "BVC", size: 2, cycles: 2, page_cycles: 1, addressing: AddressingMode::Relative,},
Instruction {opcode: 0x51, name: "EOR", size: 2, cycles: 5, page_cycles: 1, addressing: AddressingMode::IndirectIndexed,},
Instruction {opcode: 0x52, name: "KIL", size: 0, cycles: 2, page_cycles: 0, addressing: AddressingMode::Implied,},
Instruction {opcode: 0x53, name: "SRE", size: 2, cycles: 8, page_cycles: 0, addressing: AddressingMode::IndirectIndexed,},
Instruction {opcode: 0x54, name: "NOP", size: 2, cycles: 4, page_cycles: 0, addressing: AddressingMode::ZeroPageX,},
Instruction {opcode: 0x55, name: "EOR", size: 2, cycles: 4, page_cycles: 0, addressing: AddressingMode::ZeroPageX,},
Instruction {opcode: 0x56, name: "LSR", size: 2, cycles: 6, page_cycles: 0, addressing: AddressingMode::ZeroPageX,},
Instruction {opcode: 0x57, name: "SRE", size: 2, cycles: 6, page_cycles: 0, addressing: AddressingMode::ZeroPageX,},
Instruction {opcode: 0x58, name: "CLI", size: 1, cycles: 2, page_cycles: 0, addressing: AddressingMode::Implied,},
Instruction {opcode: 0x59, name: "EOR", size: 3, cycles: 4, page_cycles: 1, addressing: AddressingMode::AbsoluteY,},
Instruction {opcode: 0x5A, name: "NOP", size: 1, cycles: 2, page_cycles: 0, addressing: AddressingMode::Implied,},
Instruction {opcode: 0x5B, name: "SRE", size: 3, cycles: 7, page_cycles: 0, addressing: AddressingMode::AbsoluteY,},
Instruction {opcode: 0x5C, name: "NOP", size: 3, cycles: 4, page_cycles: 1, addressing: AddressingMode::AbsoluteX,},
Instruction {opcode: 0x5D, name: "EOR", size: 3, cycles: 4, page_cycles: 1, addressing: AddressingMode::AbsoluteX,},
Instruction {opcode: 0x5E, name: "LSR", size: 3, cycles: 7, page_cycles: 0, addressing: AddressingMode::AbsoluteX,},
Instruction {opcode: 0x5F, name: "SRE", size: 3, cycles: 7, page_cycles: 0, addressing: AddressingMode::AbsoluteX,},
Instruction {opcode: 0x60, name: "RTS", size: 1, cycles: 6, page_cycles: 0, addressing: AddressingMode::Implied,},
Instruction {opcode: 0x61, name: "ADC", size: 2, cycles: 6, page_cycles: 0, addressing: AddressingMode::IndexedIndirect,},
Instruction {opcode: 0x62, name: "KIL", size: 0, cycles: 2, page_cycles: 0, addressing: AddressingMode::Implied,},
Instruction {opcode: 0x63, name: "RRA", size: 2, cycles: 8, page_cycles: 0, addressing: AddressingMode::IndexedIndirect,},
Instruction {opcode: 0x64, name: "NOP", size: 2, cycles: 3, page_cycles: 0, addressing: AddressingMode::ZeroPage,},
Instruction {opcode: 0x65, name: "ADC", size: 2, cycles: 3, page_cycles: 0, addressing: AddressingMode::ZeroPage,},
Instruction {opcode: 0x66, name: "ROR", size: 2, cycles: 5, page_cycles: 0, addressing: AddressingMode::ZeroPage,},
Instruction {opcode: 0x67, name: "RRA", size: 2, cycles: 5, page_cycles: 0, addressing: AddressingMode::ZeroPage,},
Instruction {opcode: 0x68, name: "PLA", size: 1, cycles: 4, page_cycles: 0, addressing: AddressingMode::Implied,},
Instruction {opcode: 0x69, name: "ADC", size: 2, cycles: 2, page_cycles: 0, addressing: AddressingMode::Immediate,},
Instruction {opcode: 0x6A, name: "ROR", size: 1, cycles: 2, page_cycles: 0, addressing: AddressingMode::Accumulator,},
Instruction {opcode: 0x6B, name: "ARR", size: 0, cycles: 2, page_cycles: 0, addressing: AddressingMode::Immediate,},
Instruction {opcode: 0x6C, name: "JMP", size: 3, cycles: 5, page_cycles: 0, addressing: AddressingMode::Indirect,},
Instruction {opcode: 0x6D, name: "ADC", size: 3, cycles: 4, page_cycles: 0, addressing: AddressingMode::Absolute,},
Instruction {opcode: 0x6E, name: "ROR", size: 3, cycles: 6, page_cycles: 0, addressing: AddressingMode::Absolute,},
Instruction {opcode: 0x6F, name: "RRA", size: 3, cycles: 6, page_cycles: 0, addressing: AddressingMode::Absolute,},
Instruction {opcode: 0x70, name: "BVS", size: 2, cycles: 2, page_cycles: 1, addressing: AddressingMode::Relative,},
Instruction {opcode: 0x71, name: "ADC", size: 2, cycles: 5, page_cycles: 1, addressing: AddressingMode::IndirectIndexed,},
Instruction {opcode: 0x72, name: "KIL", size: 0, cycles: 2, page_cycles: 0, addressing: AddressingMode::Implied,},
Instruction {opcode: 0x73, name: "RRA", size: 2, cycles: 8, page_cycles: 0, addressing: AddressingMode::IndirectIndexed,},
Instruction {opcode: 0x74, name: "NOP", size: 2, cycles: 4, page_cycles: 0, addressing: AddressingMode::ZeroPageX,},
Instruction {opcode: 0x75, name: "ADC", size: 2, cycles: 4, page_cycles: 0, addressing: AddressingMode::ZeroPageX,},
Instruction {opcode: 0x76, name: "ROR", size: 2, cycles: 6, page_cycles: 0, addressing: AddressingMode::ZeroPageX,},
Instruction {opcode: 0x77, name: "RRA", size: 2, cycles: 6, page_cycles: 0, addressing: AddressingMode::ZeroPageX,},
Instruction {opcode: 0x78, name: "SEI", size: 1, cycles: 2, page_cycles: 0, addressing: AddressingMode::Implied,},
Instruction {opcode: 0x79, name: "ADC", size: 3, cycles: 4, page_cycles: 1, addressing: AddressingMode::AbsoluteY,},
Instruction {opcode: 0x7A, name: "NOP", size: 1, cycles: 2, page_cycles: 0, addressing: AddressingMode::Implied,},
Instruction {opcode: 0x7B, name: "RRA", size: 3, cycles: 7, page_cycles: 0, addressing: AddressingMode::AbsoluteY,},
Instruction {opcode: 0x7C, name: "NOP", size: 3, cycles: 4, page_cycles: 1, addressing: AddressingMode::AbsoluteX,},
Instruction {opcode: 0x7D, name: "ADC", size: 3, cycles: 4, page_cycles: 1, addressing: AddressingMode::AbsoluteX,},
Instruction {opcode: 0x7E, name: "ROR", size: 3, cycles: 7, page_cycles: 0, addressing: AddressingMode::AbsoluteX,},
Instruction {opcode: 0x7F, name: "RRA", size: 3, cycles: 7, page_cycles: 0, addressing: AddressingMode::AbsoluteX,},
Instruction {opcode: 0x80, name: "NOP", size: 2, cycles: 2, page_cycles: 0, addressing: AddressingMode::Immediate,},
Instruction {opcode: 0x81, name: "STA", size: 2, cycles: 6, page_cycles: 0, addressing: AddressingMode::IndexedIndirect,},
Instruction {opcode: 0x82, name: "NOP", size: 0, cycles: 2, page_cycles: 0, addressing: AddressingMode::Immediate,},
Instruction {opcode: 0x83, name: "SAX", size: 2, cycles: 6, page_cycles: 0, addressing: AddressingMode::IndexedIndirect,},
Instruction {opcode: 0x84, name: "STY", size: 2, cycles: 3, page_cycles: 0, addressing: AddressingMode::ZeroPage,},
Instruction {opcode: 0x85, name: "STA", size: 2, cycles: 3, page_cycles: 0, addressing: AddressingMode::ZeroPage,},
Instruction {opcode: 0x86, name: "STX", size: 2, cycles: 3, page_cycles: 0, addressing: AddressingMode::ZeroPage,},
Instruction {opcode: 0x87, name: "SAX", size: 2, cycles: 3, page_cycles: 0, addressing: AddressingMode::ZeroPage,},
Instruction {opcode: 0x88, name: "DEY", size: 1, cycles: 2, page_cycles: 0, addressing: AddressingMode::Implied,},
Instruction {opcode: 0x89, name: "NOP", size: 0, cycles: 2, page_cycles: 0, addressing: AddressingMode::Immediate,},
Instruction {opcode: 0x8A, name: "TXA", size: 1, cycles: 2, page_cycles: 0, addressing: AddressingMode::Implied,},
Instruction {opcode: 0x8B, name: "XAA", size: 0, cycles: 2, page_cycles: 0, addressing: AddressingMode::Immediate,},
Instruction {opcode: 0x8C, name: "STY", size: 3, cycles: 4, page_cycles: 0, addressing: AddressingMode::Absolute,},
Instruction {opcode: 0x8D, name: "STA", size: 3, cycles: 4, page_cycles: 0, addressing: AddressingMode::Absolute,},
Instruction {opcode: 0x8E, name: "STX", size: 3, cycles: 4, page_cycles: 0, addressing: AddressingMode::Absolute,},
Instruction {opcode: 0x8F, name: "SAX", size: 3, cycles: 4, page_cycles: 0, addressing: AddressingMode::Absolute,},
Instruction {opcode: 0x90, name: "BCC", size: 2, cycles: 2, page_cycles: 1, addressing: AddressingMode::Relative,},
Instruction {opcode: 0x91, name: "STA", size: 2, cycles: 6, page_cycles: 0, addressing: AddressingMode::IndirectIndexed,},
Instruction {opcode: 0x92, name: "KIL", size: 0, cycles: 2, page_cycles: 0, addressing: AddressingMode::Implied,},
Instruction {opcode: 0x93, name: "AHX", size: 0, cycles: 6, page_cycles: 0, addressing: AddressingMode::IndirectIndexed,},
Instruction {opcode: 0x94, name: "STY", size: 2, cycles: 4, page_cycles: 0, addressing: AddressingMode::ZeroPageX,},
Instruction {opcode: 0x95, name: "STA", size: 2, cycles: 4, page_cycles: 0, addressing: AddressingMode::ZeroPageX,},
Instruction {opcode: 0x96, name: "STX", size: 2, cycles: 4, page_cycles: 0, addressing: AddressingMode::ZeroPageY,},
Instruction {opcode: 0x97, name: "SAX", size: 2, cycles: 4, page_cycles: 0, addressing: AddressingMode::ZeroPageY,},
Instruction {opcode: 0x98, name: "TYA", size: 1, cycles: 2, page_cycles: 0, addressing: AddressingMode::Implied,},
Instruction {opcode: 0x99, name: "STA", size: 3, cycles: 5, page_cycles: 0, addressing: AddressingMode::AbsoluteY,},
Instruction {opcode: 0x9A, name: "TXS", size: 1, cycles: 2, page_cycles: 0, addressing: AddressingMode::Implied,},
Instruction {opcode: 0x9B, name: "TAS", size: 0, cycles: 5, page_cycles: 0, addressing: AddressingMode::AbsoluteY,},
Instruction {opcode: 0x9C, name: "SHY", size: 0, cycles: 5, page_cycles: 0, addressing: AddressingMode::AbsoluteX,},
Instruction {opcode: 0x9D, name: "STA", size: 3, cycles: 5, page_cycles: 0, addressing: AddressingMode::AbsoluteX,},
Instruction {opcode: 0x9E, name: "SHX", size: 0, cycles: 5, page_cycles: 0, addressing: AddressingMode::AbsoluteY,},
Instruction {opcode: 0x9F, name: "AHX", size: 0, cycles: 5, page_cycles: 0, addressing: AddressingMode::AbsoluteY,},
Instruction {opcode: 0xA0, name: "LDY", size: 2, cycles: 2, page_cycles: 0, addressing: AddressingMode::Immediate,},
Instruction {opcode: 0xA1, name: "LDA", size: 2, cycles: 6, page_cycles: 0, addressing: AddressingMode::IndexedIndirect,},
Instruction {opcode: 0xA2, name: "LDX", size: 2, cycles: 2, page_cycles: 0, addressing: AddressingMode::Immediate,},
Instruction {opcode: 0xA3, name: "LAX", size: 2, cycles: 6, page_cycles: 0, addressing: AddressingMode::IndexedIndirect,},
Instruction {opcode: 0xA4, name: "LDY", size: 2, cycles: 3, page_cycles: 0, addressing: AddressingMode::ZeroPage,},
Instruction {opcode: 0xA5, name: "LDA", size: 2, cycles: 3, page_cycles: 0, addressing: AddressingMode::ZeroPage,},
Instruction {opcode: 0xA6, name: "LDX", size: 2, cycles: 3, page_cycles: 0, addressing: AddressingMode::ZeroPage,},
Instruction {opcode: 0xA7, name: "LAX", size: 2, cycles: 3, page_cycles: 0, addressing: AddressingMode::ZeroPage,},
Instruction {opcode: 0xA8, name: "TAY", size: 1, cycles: 2, page_cycles: 0, addressing: AddressingMode::Implied,},
Instruction {opcode: 0xA9, name: "LDA", size: 2, cycles: 2, page_cycles: 0, addressing: AddressingMode::Immediate,},
Instruction {opcode: 0xAA, name: "TAX", size: 1, cycles: 2, page_cycles: 0, addressing: AddressingMode::Implied,},
Instruction {opcode: 0xAB, name: "LAX", size: 0, cycles: 2, page_cycles: 0, addressing: AddressingMode::Immediate,},
Instruction {opcode: 0xAC, name: "LDY", size: 3, cycles: 4, page_cycles: 0, addressing: AddressingMode::Absolute,},
Instruction {opcode: 0xAD, name: "LDA", size: 3, cycles: 4, page_cycles: 0, addressing: AddressingMode::Absolute,},
Instruction {opcode: 0xAE, name: "LDX", size: 3, cycles: 4, page_cycles: 0, addressing: AddressingMode::Absolute,},
Instruction {opcode: 0xAF, name: "LAX", size: 3, cycles: 4, page_cycles: 0, addressing: AddressingMode::Absolute,},
Instruction {opcode: 0xB0, name: "BCS", size: 2, cycles: 2, page_cycles: 1, addressing: AddressingMode::Relative,},
Instruction {opcode: 0xB1, name: "LDA", size: 2, cycles: 5, page_cycles: 1, addressing: AddressingMode::IndirectIndexed,},
Instruction {opcode: 0xB2, name: "KIL", size: 0, cycles: 2, page_cycles: 0, addressing: AddressingMode::Implied,},
Instruction {opcode: 0xB3, name: "LAX", size: 2, cycles: 5, page_cycles: 1, addressing: AddressingMode::IndirectIndexed,},
Instruction {opcode: 0xB4, name: "LDY", size: 2, cycles: 4, page_cycles: 0, addressing: AddressingMode::ZeroPageX,},
Instruction {opcode: 0xB5, name: "LDA", size: 2, cycles: 4, page_cycles: 0, addressing: AddressingMode::ZeroPageX,},
Instruction {opcode: 0xB6, name: "LDX", size: 2, cycles: 4, page_cycles: 0, addressing: AddressingMode::ZeroPageY,},
Instruction {opcode: 0xB7, name: "LAX", size: 2, cycles: 4, page_cycles: 0, addressing: AddressingMode::ZeroPageY,},
Instruction {opcode: 0xB8, name: "CLV", size: 1, cycles: 2, page_cycles: 0, addressing: AddressingMode::Implied,},
Instruction {opcode: 0xB9, name: "LDA", size: 3, cycles: 4, page_cycles: 1, addressing: AddressingMode::AbsoluteY,},
Instruction {opcode: 0xBA, name: "TSX", size: 1, cycles: 2, page_cycles: 0, addressing: AddressingMode::Implied,},
Instruction {opcode: 0xBB, name: "LAS", size: 0, cycles: 4, page_cycles: 1, addressing: AddressingMode::AbsoluteY,},
Instruction {opcode: 0xBC, name: "LDY", size: 3, cycles: 4, page_cycles: 1, addressing: AddressingMode::AbsoluteX,},
Instruction {opcode: 0xBD, name: "LDA", size: 3, cycles: 4, page_cycles: 1, addressing: AddressingMode::AbsoluteX,},
Instruction {opcode: 0xBE, name: "LDX", size: 3, cycles: 4, page_cycles: 1, addressing: AddressingMode::AbsoluteY,},
Instruction {opcode: 0xBF, name: "LAX", size: 3, cycles: 4, page_cycles: 1, addressing: AddressingMode::AbsoluteY,},
Instruction {opcode: 0xC0, name: "CPY", size: 2, cycles: 2, page_cycles: 0, addressing: AddressingMode::Immediate,},
Instruction {opcode: 0xC1, name: "CMP", size: 2, cycles: 6, page_cycles: 0, addressing: AddressingMode::IndexedIndirect,},
Instruction {opcode: 0xC2, name: "NOP", size: 0, cycles: 2, page_cycles: 0, addressing: AddressingMode::Immediate,},
Instruction {opcode: 0xC3, name: "DCP", size: 2, cycles: 8, page_cycles: 0, addressing: AddressingMode::IndexedIndirect,},
Instruction {opcode: 0xC4, name: "CPY", size: 2, cycles: 3, page_cycles: 0, addressing: AddressingMode::ZeroPage,},
Instruction {opcode: 0xC5, name: "CMP", size: 2, cycles: 3, page_cycles: 0, addressing: AddressingMode::ZeroPage,},
Instruction {opcode: 0xC6, name: "DEC", size: 2, cycles: 5, page_cycles: 0, addressing: AddressingMode::ZeroPage,},
Instruction {opcode: 0xC7, name: "DCP", size: 2, cycles: 5, page_cycles: 0, addressing: AddressingMode::ZeroPage,},
Instruction {opcode: 0xC8, name: "INY", size: 1, cycles: 2, page_cycles: 0, addressing: AddressingMode::Implied,},
Instruction {opcode: 0xC9, name: "CMP", size: 2, cycles: 2, page_cycles: 0, addressing: AddressingMode::Immediate,},
Instruction {opcode: 0xCA, name: "DEX", size: 1, cycles: 2, page_cycles: 0, addressing: AddressingMode::Implied,},
Instruction {opcode: 0xCB, name: "AXS", size: 0, cycles: 2, page_cycles: 0, addressing: AddressingMode::Immediate,},
Instruction {opcode: 0xCC, name: "CPY", size: 3, cycles: 4, page_cycles: 0, addressing: AddressingMode::Absolute,},
Instruction {opcode: 0xCD, name: "CMP", size: 3, cycles: 4, page_cycles: 0, addressing: AddressingMode::Absolute,},
Instruction {opcode: 0xCE, name: "DEC", size: 3, cycles: 6, page_cycles: 0, addressing: AddressingMode::Absolute,},
Instruction {opcode: 0xCF, name: "DCP", size: 3, cycles: 6, page_cycles: 0, addressing: AddressingMode::Absolute,},
Instruction {opcode: 0xD0, name: "BNE", size: 2, cycles: 2, page_cycles: 1, addressing: AddressingMode::Relative,},
Instruction {opcode: 0xD1, name: "CMP", size: 2, cycles: 5, page_cycles: 1, addressing: AddressingMode::IndirectIndexed,},
Instruction {opcode: 0xD2, name: "KIL", size: 0, cycles: 2, page_cycles: 0, addressing: AddressingMode::Implied,},
Instruction {opcode: 0xD3, name: "DCP", size: 2, cycles: 8, page_cycles: 0, addressing: AddressingMode::IndirectIndexed,},
Instruction {opcode: 0xD4, name: "NOP", size: 2, cycles: 4, page_cycles: 0, addressing: AddressingMode::ZeroPageX,},
Instruction {opcode: 0xD5, name: "CMP", size: 2, cycles: 4, page_cycles: 0, addressing: AddressingMode::ZeroPageX,},
Instruction {opcode: 0xD6, name: "DEC", size: 2, cycles: 6, page_cycles: 0, addressing: AddressingMode::ZeroPageX,},
Instruction {opcode: 0xD7, name: "DCP", size: 2, cycles: 6, page_cycles: 0, addressing: AddressingMode::ZeroPageX,},
Instruction {opcode: 0xD8, name: "CLD", size: 1, cycles: 2, page_cycles: 0, addressing: AddressingMode::Implied,},
Instruction {opcode: 0xD9, name: "CMP", size: 3, cycles: 4, page_cycles: 1, addressing: AddressingMode::AbsoluteY,},
Instruction {opcode: 0xDA, name: "NOP", size: 1, cycles: 2, page_cycles: 0, addressing: AddressingMode::Implied,},
Instruction {opcode: 0xDB, name: "DCP", size: 3, cycles: 7, page_cycles: 0, addressing: AddressingMode::AbsoluteY,},
Instruction {opcode: 0xDC, name: "NOP", size: 3, cycles: 4, page_cycles: 1, addressing: AddressingMode::AbsoluteX,},
Instruction {opcode: 0xDD, name: "CMP", size: 3, cycles: 4, page_cycles: 1, addressing: AddressingMode::AbsoluteX,},
Instruction {opcode: 0xDE, name: "DEC", size: 3, cycles: 7, page_cycles: 0, addressing: AddressingMode::AbsoluteX,},
Instruction {opcode: 0xDF, name: "DCP", size: 3, cycles: 7, page_cycles: 0, addressing: AddressingMode::AbsoluteX,},
Instruction {opcode: 0xE0, name: "CPX", size: 2, cycles: 2, page_cycles: 0, addressing: AddressingMode::Immediate,},
Instruction {opcode: 0xE1, name: "SBC", size: 2, cycles: 6, page_cycles: 0, addressing: AddressingMode::IndexedIndirect,},
Instruction {opcode: 0xE2, name: "NOP", size: 0, cycles: 2, page_cycles: 0, addressing: AddressingMode::Immediate,},
Instruction {opcode: 0xE3, name: "ISC", size: 2, cycles: 8, page_cycles: 0, addressing: AddressingMode::IndexedIndirect,},
Instruction {opcode: 0xE4, name: "CPX", size: 2, cycles: 3, page_cycles: 0, addressing: AddressingMode::ZeroPage,},
Instruction {opcode: 0xE5, name: "SBC", size: 2, cycles: 3, page_cycles: 0, addressing: AddressingMode::ZeroPage,},
Instruction {opcode: 0xE6, name: "INC", size: 2, cycles: 5, page_cycles: 0, addressing: AddressingMode::ZeroPage,},
Instruction {opcode: 0xE7, name: "ISC", size: 2, cycles: 5, page_cycles: 0, addressing: AddressingMode::ZeroPage,},
Instruction {opcode: 0xE8, name: "INX", size: 1, cycles: 2, page_cycles: 0, addressing: AddressingMode::Implied,},
Instruction {opcode: 0xE9, name: "SBC", size: 2, cycles: 2, page_cycles: 0, addressing: AddressingMode::Immediate,},
Instruction {opcode: 0xEA, name: "NOP", size: 1, cycles: 2, page_cycles: 0, addressing: AddressingMode::Implied,},
Instruction {opcode: 0xEB, name: "SBC", size: 2, cycles: 2, page_cycles: 0, addressing: AddressingMode::Immediate,},
Instruction {opcode: 0xEC, name: "CPX", size: 3, cycles: 4, page_cycles: 0, addressing: AddressingMode::Absolute,},
Instruction {opcode: 0xED, name: "SBC", size: 3, cycles: 4, page_cycles: 0, addressing: AddressingMode::Absolute,},
Instruction {opcode: 0xEE, name: "INC", size: 3, cycles: 6, page_cycles: 0, addressing: AddressingMode::Absolute,},
Instruction {opcode: 0xEF, name: "ISC", size: 3, cycles: 6, page_cycles: 0, addressing: AddressingMode::Absolute,},
Instruction {opcode: 0xF0, name: "BEQ", size: 2, cycles: 2, page_cycles: 1, addressing: AddressingMode::Relative,},
Instruction {opcode: 0xF1, name: "SBC", size: 2, cycles: 5, page_cycles: 1, addressing: AddressingMode::IndirectIndexed,},
Instruction {opcode: 0xF2, name: "KIL", size: 0, cycles: 2, page_cycles: 0, addressing: AddressingMode::Implied,},
Instruction {opcode: 0xF3, name: "ISC", size: 2, cycles: 8, page_cycles: 0, addressing: AddressingMode::IndirectIndexed,},
Instruction {opcode: 0xF4, name: "NOP", size: 2, cycles: 4, page_cycles: 0, addressing: AddressingMode::ZeroPageX,},
Instruction {opcode: 0xF5, name: "SBC", size: 2, cycles: 4, page_cycles: 0, addressing: AddressingMode::ZeroPageX,},
Instruction {opcode: 0xF6, name: "INC", size: 2, cycles: 6, page_cycles: 0, addressing: AddressingMode::ZeroPageX,},
Instruction {opcode: 0xF7, name: "ISC", size: 2, cycles: 6, page_cycles: 0, addressing: AddressingMode::ZeroPageX,},
Instruction {opcode: 0xF8, name: "SED", size: 1, cycles: 2, page_cycles: 0, addressing: AddressingMode::Implied,},
Instruction {opcode: 0xF9, name: "SBC", size: 3, cycles: 4, page_cycles: 1, addressing: AddressingMode::AbsoluteY,},
Instruction {opcode: 0xFA, name: "NOP", size: 1, cycles: 2, page_cycles: 0, addressing: AddressingMode::Implied,},
Instruction {opcode: 0xFB, name: "ISC", size: 3, cycles: 7, page_cycles: 0, addressing: AddressingMode::AbsoluteY,},
Instruction {opcode: 0xFC, name: "NOP", size: 3, cycles: 4, page_cycles: 1, addressing: AddressingMode::AbsoluteX,},
Instruction {opcode: 0xFD, name: "SBC", size: 3, cycles: 4, page_cycles: 1, addressing: AddressingMode::AbsoluteX,},
Instruction {opcode: 0xFE, name: "INC", size: 3, cycles: 7, page_cycles: 0, addressing: AddressingMode::AbsoluteX,},
Instruction {opcode: 0xFF, name: "ISC", size: 3, cycles: 7, page_cycles: 0, addressing: AddressingMode::AbsoluteX,},
];
