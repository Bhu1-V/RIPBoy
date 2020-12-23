use super::target::*;
#[derive(Debug)]
pub enum Instruction {
    ADD(ArthemeticTarget),
    INC(IncDecTarget),
    DEC(IncDecTarget),
    SUB(ArthemeticTarget),
    AND(ArthemeticTarget),
    OR(ArthemeticTarget),
    ADC(ArthemeticTarget),
    SBC(ArthemeticTarget),
    XOR(ArthemeticTarget),
    CP(ArthemeticTarget),
    JP(JumpTest),
    LD(LoadType),
    LD2(LoadType),
    PUSH(StackTarget),
    POP(StackTarget),
    CALL(JumpTest),
    RET(JumpTest),
    JR(JumpTest),

    NOP,
    HALT,
    STOP,

    DI,
    CB,
    EI,
    SCF,

    CCF,
    RRA,
    RRCA,
    RLA,
    RLCA,
    CPL,
    DAA,

    RST(RSTTarget),

    // Prefix Instructions
    RLC(PrefixTarget),
    RRC(PrefixTarget),
    RL(PrefixTarget),
    RR(PrefixTarget),
    SLA(PrefixTarget),
    SRA(PrefixTarget),
    SWAP(PrefixTarget),
    SRL(PrefixTarget),

    BIT(BitManipulationType),
    RES(BitManipulationType),
    SET(BitManipulationType),
}

impl Instruction {
    pub fn from_byte(byte: u8, prefixed: bool) -> Option<Instruction> {
        if prefixed {
            return Instruction::from_byte_prefixed(byte);
        } else {
            return Instruction::from_byte_not_prefixed(byte);
        }
    }

    fn from_byte_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            // RLC checked
            0x00 => Some(Instruction::RLC(PrefixTarget::B)),
            0x01 => Some(Instruction::RLC(PrefixTarget::C)),
            0x02 => Some(Instruction::RLC(PrefixTarget::D)),
            0x03 => Some(Instruction::RLC(PrefixTarget::E)),
            0x04 => Some(Instruction::RLC(PrefixTarget::H)),
            0x05 => Some(Instruction::RLC(PrefixTarget::L)),
            0x06 => Some(Instruction::RLC(PrefixTarget::HLV)),
            0x07 => Some(Instruction::RLC(PrefixTarget::A)),

            // RRC CHECKED
            0x08 => Some(Instruction::RRC(PrefixTarget::B)),
            0x09 => Some(Instruction::RRC(PrefixTarget::C)),
            0x0A => Some(Instruction::RRC(PrefixTarget::D)),
            0x0B => Some(Instruction::RRC(PrefixTarget::E)),
            0x0C => Some(Instruction::RRC(PrefixTarget::H)),
            0x0D => Some(Instruction::RRC(PrefixTarget::L)),
            0x0E => Some(Instruction::RRC(PrefixTarget::HLV)),
            0x0F => Some(Instruction::RRC(PrefixTarget::A)),

            // RL checked
            0x10 => Some(Instruction::RL(PrefixTarget::B)),
            0x11 => Some(Instruction::RL(PrefixTarget::C)),
            0x12 => Some(Instruction::RL(PrefixTarget::D)),
            0x13 => Some(Instruction::RL(PrefixTarget::E)),
            0x14 => Some(Instruction::RL(PrefixTarget::H)),
            0x15 => Some(Instruction::RL(PrefixTarget::L)),
            0x16 => Some(Instruction::RL(PrefixTarget::HLV)),
            0x17 => Some(Instruction::RL(PrefixTarget::A)),

            // RR CHECKED
            0x18 => Some(Instruction::RR(PrefixTarget::B)),
            0x19 => Some(Instruction::RR(PrefixTarget::C)),
            0x1A => Some(Instruction::RR(PrefixTarget::D)),
            0x1B => Some(Instruction::RR(PrefixTarget::E)),
            0x1C => Some(Instruction::RR(PrefixTarget::H)),
            0x1D => Some(Instruction::RR(PrefixTarget::L)),
            0x1E => Some(Instruction::RR(PrefixTarget::HLV)),
            0x1F => Some(Instruction::RR(PrefixTarget::A)),

            // SLA checked
            0x20 => Some(Instruction::SLA(PrefixTarget::B)),
            0x21 => Some(Instruction::SLA(PrefixTarget::C)),
            0x22 => Some(Instruction::SLA(PrefixTarget::D)),
            0x23 => Some(Instruction::SLA(PrefixTarget::E)),
            0x24 => Some(Instruction::SLA(PrefixTarget::H)),
            0x25 => Some(Instruction::SLA(PrefixTarget::L)),
            0x26 => Some(Instruction::SLA(PrefixTarget::HLV)),
            0x27 => Some(Instruction::SLA(PrefixTarget::A)),

            // SRA CHECKED
            0x28 => Some(Instruction::SRA(PrefixTarget::B)),
            0x29 => Some(Instruction::SRA(PrefixTarget::C)),
            0x2A => Some(Instruction::SRA(PrefixTarget::D)),
            0x2B => Some(Instruction::SRA(PrefixTarget::E)),
            0x2C => Some(Instruction::SRA(PrefixTarget::H)),
            0x2D => Some(Instruction::SRA(PrefixTarget::L)),
            0x2E => Some(Instruction::SRA(PrefixTarget::HLV)),
            0x2F => Some(Instruction::SRA(PrefixTarget::A)),

            // SWAP checked
            0x30 => Some(Instruction::SWAP(PrefixTarget::B)),
            0x31 => Some(Instruction::SWAP(PrefixTarget::C)),
            0x32 => Some(Instruction::SWAP(PrefixTarget::D)),
            0x33 => Some(Instruction::SWAP(PrefixTarget::E)),
            0x34 => Some(Instruction::SWAP(PrefixTarget::H)),
            0x35 => Some(Instruction::SWAP(PrefixTarget::L)),
            0x36 => Some(Instruction::SWAP(PrefixTarget::HLV)),
            0x37 => Some(Instruction::SWAP(PrefixTarget::A)),

            // SRL CHECKED
            0x38 => Some(Instruction::SRL(PrefixTarget::B)),
            0x39 => Some(Instruction::SRL(PrefixTarget::C)),
            0x3A => Some(Instruction::SRL(PrefixTarget::D)),
            0x3B => Some(Instruction::SRL(PrefixTarget::E)),
            0x3C => Some(Instruction::SRL(PrefixTarget::H)),
            0x3D => Some(Instruction::SRL(PrefixTarget::L)),
            0x3E => Some(Instruction::SRL(PrefixTarget::HLV)),
            0x3F => Some(Instruction::SRL(PrefixTarget::A)),

            // BIT man 0
            0x40 => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B0,
                SourceRegister::B,
            ))),
            0x41 => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B0,
                SourceRegister::C,
            ))),
            0x42 => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B0,
                SourceRegister::D,
            ))),
            0x43 => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B0,
                SourceRegister::E,
            ))),
            0x44 => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B0,
                SourceRegister::H,
            ))),
            0x45 => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B0,
                SourceRegister::L,
            ))),
            0x46 => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B0,
                SourceRegister::HLV,
            ))),
            0x47 => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B0,
                SourceRegister::A,
            ))),

            // bit man 1
            0x48 => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B1,
                SourceRegister::B,
            ))),
            0x49 => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B1,
                SourceRegister::C,
            ))),
            0x4A => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B1,
                SourceRegister::D,
            ))),
            0x4B => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B1,
                SourceRegister::E,
            ))),
            0x4C => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B1,
                SourceRegister::H,
            ))),
            0x4D => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B1,
                SourceRegister::L,
            ))),
            0x4E => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B1,
                SourceRegister::HLV,
            ))),
            0x4F => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B1,
                SourceRegister::A,
            ))),

            // bit man 2
            0x50 => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B2,
                SourceRegister::B,
            ))),
            0x51 => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B2,
                SourceRegister::C,
            ))),
            0x52 => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B2,
                SourceRegister::D,
            ))),
            0x53 => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B2,
                SourceRegister::E,
            ))),
            0x54 => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B2,
                SourceRegister::H,
            ))),
            0x55 => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B2,
                SourceRegister::L,
            ))),
            0x56 => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B2,
                SourceRegister::HLV,
            ))),
            0x57 => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B2,
                SourceRegister::A,
            ))),

            // bit man 3
            0x58 => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B3,
                SourceRegister::B,
            ))),
            0x59 => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B3,
                SourceRegister::C,
            ))),
            0x5A => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B3,
                SourceRegister::D,
            ))),
            0x5B => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B3,
                SourceRegister::E,
            ))),
            0x5C => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B3,
                SourceRegister::H,
            ))),
            0x5D => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B3,
                SourceRegister::L,
            ))),
            0x5E => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B3,
                SourceRegister::HLV,
            ))),
            0x5F => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B3,
                SourceRegister::A,
            ))),

            // bit man 4
            0x60 => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B4,
                SourceRegister::B,
            ))),
            0x61 => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B4,
                SourceRegister::C,
            ))),
            0x62 => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B4,
                SourceRegister::D,
            ))),
            0x63 => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B4,
                SourceRegister::E,
            ))),
            0x64 => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B4,
                SourceRegister::H,
            ))),
            0x65 => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B4,
                SourceRegister::L,
            ))),
            0x66 => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B4,
                SourceRegister::HLV,
            ))),
            0x67 => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B4,
                SourceRegister::A,
            ))),

            // bit man 5
            0x68 => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B5,
                SourceRegister::B,
            ))),
            0x69 => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B5,
                SourceRegister::C,
            ))),
            0x6A => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B5,
                SourceRegister::D,
            ))),
            0x6B => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B5,
                SourceRegister::E,
            ))),
            0x6C => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B5,
                SourceRegister::H,
            ))),
            0x6D => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B5,
                SourceRegister::L,
            ))),
            0x6E => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B5,
                SourceRegister::HLV,
            ))),
            0x6F => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B5,
                SourceRegister::A,
            ))),

            // bit man 6
            0x70 => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B6,
                SourceRegister::B,
            ))),
            0x71 => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B6,
                SourceRegister::C,
            ))),
            0x72 => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B6,
                SourceRegister::D,
            ))),
            0x73 => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B6,
                SourceRegister::E,
            ))),
            0x74 => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B6,
                SourceRegister::H,
            ))),
            0x75 => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B6,
                SourceRegister::L,
            ))),
            0x76 => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B6,
                SourceRegister::HLV,
            ))),
            0x77 => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B6,
                SourceRegister::A,
            ))),

            // bit man 7
            0x78 => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B7,
                SourceRegister::B,
            ))),
            0x79 => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B7,
                SourceRegister::C,
            ))),
            0x7A => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B7,
                SourceRegister::D,
            ))),
            0x7B => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B7,
                SourceRegister::E,
            ))),
            0x7C => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B7,
                SourceRegister::H,
            ))),
            0x7D => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B7,
                SourceRegister::L,
            ))),
            0x7E => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B7,
                SourceRegister::HLV,
            ))),
            0x7F => Some(Instruction::BIT(BitManipulationType::Bit(
                TargetBit::B7,
                SourceRegister::A,
            ))),

            // RES man 0
            0x80 => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B0,
                SourceRegister::B,
            ))),
            0x81 => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B0,
                SourceRegister::C,
            ))),
            0x82 => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B0,
                SourceRegister::D,
            ))),
            0x83 => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B0,
                SourceRegister::E,
            ))),
            0x84 => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B0,
                SourceRegister::H,
            ))),
            0x85 => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B0,
                SourceRegister::L,
            ))),
            0x86 => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B0,
                SourceRegister::HLV,
            ))),
            0x87 => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B0,
                SourceRegister::A,
            ))),

            // res man 1
            0x88 => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B1,
                SourceRegister::B,
            ))),
            0x89 => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B1,
                SourceRegister::C,
            ))),
            0x8A => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B1,
                SourceRegister::D,
            ))),
            0x8B => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B1,
                SourceRegister::E,
            ))),
            0x8C => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B1,
                SourceRegister::H,
            ))),
            0x8D => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B1,
                SourceRegister::L,
            ))),
            0x8E => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B1,
                SourceRegister::HLV,
            ))),
            0x8F => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B1,
                SourceRegister::A,
            ))),

            // res man 2
            0x90 => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B2,
                SourceRegister::B,
            ))),
            0x91 => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B2,
                SourceRegister::C,
            ))),
            0x92 => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B2,
                SourceRegister::D,
            ))),
            0x93 => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B2,
                SourceRegister::E,
            ))),
            0x94 => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B2,
                SourceRegister::H,
            ))),
            0x95 => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B2,
                SourceRegister::L,
            ))),
            0x96 => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B2,
                SourceRegister::HLV,
            ))),
            0x97 => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B2,
                SourceRegister::A,
            ))),

            // res man 3
            0x98 => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B3,
                SourceRegister::B,
            ))),
            0x99 => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B3,
                SourceRegister::C,
            ))),
            0x9A => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B3,
                SourceRegister::D,
            ))),
            0x9B => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B3,
                SourceRegister::E,
            ))),
            0x9C => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B3,
                SourceRegister::H,
            ))),
            0x9D => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B3,
                SourceRegister::L,
            ))),
            0x9E => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B3,
                SourceRegister::HLV,
            ))),
            0x9F => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B3,
                SourceRegister::A,
            ))),

            // res man 4
            0xA0 => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B4,
                SourceRegister::B,
            ))),
            0xA1 => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B4,
                SourceRegister::C,
            ))),
            0xA2 => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B4,
                SourceRegister::D,
            ))),
            0xA3 => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B4,
                SourceRegister::E,
            ))),
            0xA4 => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B4,
                SourceRegister::H,
            ))),
            0xA5 => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B4,
                SourceRegister::L,
            ))),
            0xA6 => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B4,
                SourceRegister::HLV,
            ))),
            0xA7 => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B4,
                SourceRegister::A,
            ))),

            // res man 5
            0xA8 => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B5,
                SourceRegister::B,
            ))),
            0xA9 => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B5,
                SourceRegister::C,
            ))),
            0xAA => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B5,
                SourceRegister::D,
            ))),
            0xAB => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B5,
                SourceRegister::E,
            ))),
            0xAC => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B5,
                SourceRegister::H,
            ))),
            0xAD => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B5,
                SourceRegister::L,
            ))),
            0xAE => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B5,
                SourceRegister::HLV,
            ))),
            0xAF => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B5,
                SourceRegister::A,
            ))),

            // res man 6
            0xB0 => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B6,
                SourceRegister::B,
            ))),
            0xB1 => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B6,
                SourceRegister::C,
            ))),
            0xB2 => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B6,
                SourceRegister::D,
            ))),
            0xB3 => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B6,
                SourceRegister::E,
            ))),
            0xB4 => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B6,
                SourceRegister::H,
            ))),
            0xB5 => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B6,
                SourceRegister::L,
            ))),
            0xB6 => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B6,
                SourceRegister::HLV,
            ))),
            0xB7 => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B6,
                SourceRegister::A,
            ))),

            // res man 7
            0xB8 => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B7,
                SourceRegister::B,
            ))),
            0xB9 => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B7,
                SourceRegister::C,
            ))),
            0xBA => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B7,
                SourceRegister::D,
            ))),
            0xBB => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B7,
                SourceRegister::E,
            ))),
            0xBC => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B7,
                SourceRegister::H,
            ))),
            0xBD => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B7,
                SourceRegister::L,
            ))),
            0xBE => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B7,
                SourceRegister::HLV,
            ))),
            0xBF => Some(Instruction::RES(BitManipulationType::Bit(
                TargetBit::B7,
                SourceRegister::A,
            ))),

            // set man 0
            0xC0 => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B0,
                SourceRegister::B,
            ))),
            0xC1 => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B0,
                SourceRegister::C,
            ))),
            0xC2 => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B0,
                SourceRegister::D,
            ))),
            0xC3 => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B0,
                SourceRegister::E,
            ))),
            0xC4 => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B0,
                SourceRegister::H,
            ))),
            0xC5 => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B0,
                SourceRegister::L,
            ))),
            0xC6 => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B0,
                SourceRegister::HLV,
            ))),
            0xC7 => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B0,
                SourceRegister::A,
            ))),

            // set man 1
            0xC8 => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B1,
                SourceRegister::B,
            ))),
            0xC9 => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B1,
                SourceRegister::C,
            ))),
            0xCA => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B1,
                SourceRegister::D,
            ))),
            0xCB => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B1,
                SourceRegister::E,
            ))),
            0xCC => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B1,
                SourceRegister::H,
            ))),
            0xCD => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B1,
                SourceRegister::L,
            ))),
            0xCE => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B1,
                SourceRegister::HLV,
            ))),
            0xCF => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B1,
                SourceRegister::A,
            ))),

            // set man 2
            0xD0 => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B2,
                SourceRegister::B,
            ))),
            0xD1 => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B2,
                SourceRegister::C,
            ))),
            0xD2 => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B2,
                SourceRegister::D,
            ))),
            0xD3 => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B2,
                SourceRegister::E,
            ))),
            0xD4 => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B2,
                SourceRegister::H,
            ))),
            0xD5 => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B2,
                SourceRegister::L,
            ))),
            0xD6 => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B2,
                SourceRegister::HLV,
            ))),
            0xD7 => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B2,
                SourceRegister::A,
            ))),

            // set man 3
            0xD8 => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B3,
                SourceRegister::B,
            ))),
            0xD9 => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B3,
                SourceRegister::C,
            ))),
            0xDA => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B3,
                SourceRegister::D,
            ))),
            0xDB => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B3,
                SourceRegister::E,
            ))),
            0xDC => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B3,
                SourceRegister::H,
            ))),
            0xDD => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B3,
                SourceRegister::L,
            ))),
            0xDE => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B3,
                SourceRegister::HLV,
            ))),
            0xDF => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B3,
                SourceRegister::A,
            ))),

            // set man 4
            0xE0 => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B4,
                SourceRegister::B,
            ))),
            0xE1 => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B4,
                SourceRegister::C,
            ))),
            0xE2 => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B4,
                SourceRegister::D,
            ))),
            0xE3 => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B4,
                SourceRegister::E,
            ))),
            0xE4 => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B4,
                SourceRegister::H,
            ))),
            0xE5 => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B4,
                SourceRegister::L,
            ))),
            0xE6 => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B4,
                SourceRegister::HLV,
            ))),
            0xE7 => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B4,
                SourceRegister::A,
            ))),

            // set man 5
            0xE8 => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B5,
                SourceRegister::B,
            ))),
            0xE9 => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B5,
                SourceRegister::C,
            ))),
            0xEA => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B5,
                SourceRegister::D,
            ))),
            0xEB => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B5,
                SourceRegister::E,
            ))),
            0xEC => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B5,
                SourceRegister::H,
            ))),
            0xED => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B5,
                SourceRegister::L,
            ))),
            0xEE => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B5,
                SourceRegister::HLV,
            ))),
            0xEF => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B5,
                SourceRegister::A,
            ))),

            // set man 6
            0xF0 => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B6,
                SourceRegister::B,
            ))),
            0xF1 => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B6,
                SourceRegister::C,
            ))),
            0xF2 => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B6,
                SourceRegister::D,
            ))),
            0xF3 => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B6,
                SourceRegister::E,
            ))),
            0xF4 => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B6,
                SourceRegister::H,
            ))),
            0xF5 => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B6,
                SourceRegister::L,
            ))),
            0xF6 => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B6,
                SourceRegister::HLV,
            ))),
            0xF7 => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B6,
                SourceRegister::A,
            ))),

            // set man 7
            0xF8 => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B7,
                SourceRegister::B,
            ))),
            0xF9 => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B7,
                SourceRegister::C,
            ))),
            0xFA => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B7,
                SourceRegister::D,
            ))),
            0xFB => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B7,
                SourceRegister::E,
            ))),
            0xFC => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B7,
                SourceRegister::H,
            ))),
            0xFD => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B7,
                SourceRegister::L,
            ))),
            0xFE => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B7,
                SourceRegister::HLV,
            ))),
            0xFF => Some(Instruction::SET(BitManipulationType::Bit(
                TargetBit::B7,
                SourceRegister::A,
            ))),
        }
    }

    fn from_byte_not_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            // - DATA PROCESSING INST. //
            // 37 + 63 Inst.= 100

            // checked INC
            0x03 => Some(Instruction::INC(IncDecTarget::BC)),
            0x13 => Some(Instruction::INC(IncDecTarget::DE)),
            0x23 => Some(Instruction::INC(IncDecTarget::HL)),
            0x33 => Some(Instruction::INC(IncDecTarget::SP)),
            0x04 => Some(Instruction::INC(IncDecTarget::B)),
            0x14 => Some(Instruction::INC(IncDecTarget::D)),
            0x24 => Some(Instruction::INC(IncDecTarget::H)),
            0x34 => Some(Instruction::INC(IncDecTarget::HL2)),
            0x0C => Some(Instruction::INC(IncDecTarget::C)),
            0x1C => Some(Instruction::INC(IncDecTarget::E)),
            0x2C => Some(Instruction::INC(IncDecTarget::L)),
            0x3C => Some(Instruction::INC(IncDecTarget::A)),

            // checked DEC
            0x05 => Some(Instruction::DEC(IncDecTarget::B)),
            0x15 => Some(Instruction::DEC(IncDecTarget::D)),
            0x25 => Some(Instruction::DEC(IncDecTarget::H)),
            0x35 => Some(Instruction::DEC(IncDecTarget::HL2)),
            0x0B => Some(Instruction::DEC(IncDecTarget::BC)),
            0x1B => Some(Instruction::DEC(IncDecTarget::DE)),
            0x2B => Some(Instruction::DEC(IncDecTarget::HL)),
            0x3B => Some(Instruction::DEC(IncDecTarget::SP)),
            0x0D => Some(Instruction::DEC(IncDecTarget::C)),
            0x1D => Some(Instruction::DEC(IncDecTarget::E)),
            0x2D => Some(Instruction::DEC(IncDecTarget::L)),
            0x3D => Some(Instruction::DEC(IncDecTarget::A)),

            // checked ADD
            0x80 => Some(Instruction::ADD(ArthemeticTarget::B)),
            0x81 => Some(Instruction::ADD(ArthemeticTarget::C)),
            0x82 => Some(Instruction::ADD(ArthemeticTarget::D)),
            0x83 => Some(Instruction::ADD(ArthemeticTarget::E)),
            0x84 => Some(Instruction::ADD(ArthemeticTarget::H)),
            0x85 => Some(Instruction::ADD(ArthemeticTarget::L)),
            0x86 => Some(Instruction::ADD(ArthemeticTarget::HL)),
            0x87 => Some(Instruction::ADD(ArthemeticTarget::A)),

            0xC6 => Some(Instruction::ADD(ArthemeticTarget::D8)),

            0x09 => Some(Instruction::ADD(ArthemeticTarget::HLBC)),
            0x19 => Some(Instruction::ADD(ArthemeticTarget::HLDE)),
            0x29 => Some(Instruction::ADD(ArthemeticTarget::HLHL)),
            0x39 => Some(Instruction::ADD(ArthemeticTarget::HLSP)),

            0xE8 => Some(Instruction::ADD(ArthemeticTarget::SP)),

            // 7 * 9 = 63
            // checked SUB
            0x90 => Some(Instruction::SUB(ArthemeticTarget::B)),
            0x91 => Some(Instruction::SUB(ArthemeticTarget::C)),
            0x92 => Some(Instruction::SUB(ArthemeticTarget::D)),
            0x93 => Some(Instruction::SUB(ArthemeticTarget::E)),
            0x94 => Some(Instruction::SUB(ArthemeticTarget::H)),
            0x95 => Some(Instruction::SUB(ArthemeticTarget::L)),
            0x96 => Some(Instruction::SUB(ArthemeticTarget::HL)),
            0x97 => Some(Instruction::SUB(ArthemeticTarget::A)),
            0xD6 => Some(Instruction::SUB(ArthemeticTarget::D8)),

            // checked AND
            0xA0 => Some(Instruction::AND(ArthemeticTarget::B)),
            0xA1 => Some(Instruction::AND(ArthemeticTarget::C)),
            0xA2 => Some(Instruction::AND(ArthemeticTarget::D)),
            0xA3 => Some(Instruction::AND(ArthemeticTarget::E)),
            0xA4 => Some(Instruction::AND(ArthemeticTarget::H)),
            0xA5 => Some(Instruction::AND(ArthemeticTarget::L)),
            0xA6 => Some(Instruction::AND(ArthemeticTarget::HL)),
            0xA7 => Some(Instruction::AND(ArthemeticTarget::A)),
            0xE6 => Some(Instruction::AND(ArthemeticTarget::D8)),

            // checked OR
            0xB0 => Some(Instruction::OR(ArthemeticTarget::B)),
            0xB1 => Some(Instruction::OR(ArthemeticTarget::C)),
            0xB2 => Some(Instruction::OR(ArthemeticTarget::D)),
            0xB3 => Some(Instruction::OR(ArthemeticTarget::E)),
            0xB4 => Some(Instruction::OR(ArthemeticTarget::H)),
            0xB5 => Some(Instruction::OR(ArthemeticTarget::L)),
            0xB6 => Some(Instruction::OR(ArthemeticTarget::HL)),
            0xB7 => Some(Instruction::OR(ArthemeticTarget::A)),
            0xF6 => Some(Instruction::OR(ArthemeticTarget::D8)),

            //checked ADC
            0x88 => Some(Instruction::ADC(ArthemeticTarget::B)),
            0x89 => Some(Instruction::ADC(ArthemeticTarget::C)),
            0x8A => Some(Instruction::ADC(ArthemeticTarget::D)),
            0x8B => Some(Instruction::ADC(ArthemeticTarget::E)),
            0x8C => Some(Instruction::ADC(ArthemeticTarget::H)),
            0x8D => Some(Instruction::ADC(ArthemeticTarget::L)),
            0x8E => Some(Instruction::ADC(ArthemeticTarget::HL)),
            0x8F => Some(Instruction::ADC(ArthemeticTarget::A)),
            0xCE => Some(Instruction::ADC(ArthemeticTarget::D8)),

            // checked SBC
            0x98 => Some(Instruction::SBC(ArthemeticTarget::B)),
            0x99 => Some(Instruction::SBC(ArthemeticTarget::C)),
            0x9A => Some(Instruction::SBC(ArthemeticTarget::D)),
            0x9B => Some(Instruction::SBC(ArthemeticTarget::E)),
            0x9C => Some(Instruction::SBC(ArthemeticTarget::H)),
            0x9D => Some(Instruction::SBC(ArthemeticTarget::L)),
            0x9E => Some(Instruction::SBC(ArthemeticTarget::HL)),
            0x9F => Some(Instruction::SBC(ArthemeticTarget::A)),
            0xDE => Some(Instruction::SBC(ArthemeticTarget::D8)),

            // Checked XOR
            0xA8 => Some(Instruction::XOR(ArthemeticTarget::B)),
            0xA9 => Some(Instruction::XOR(ArthemeticTarget::C)),
            0xAA => Some(Instruction::XOR(ArthemeticTarget::D)),
            0xAB => Some(Instruction::XOR(ArthemeticTarget::E)),
            0xAC => Some(Instruction::XOR(ArthemeticTarget::H)),
            0xAD => Some(Instruction::XOR(ArthemeticTarget::L)),
            0xAE => Some(Instruction::XOR(ArthemeticTarget::HL)),
            0xAF => Some(Instruction::XOR(ArthemeticTarget::A)),
            0xEE => Some(Instruction::XOR(ArthemeticTarget::D8)),

            //Checked CP
            0xB8 => Some(Instruction::CP(ArthemeticTarget::B)),
            0xB9 => Some(Instruction::CP(ArthemeticTarget::C)),
            0xBA => Some(Instruction::CP(ArthemeticTarget::D)),
            0xBB => Some(Instruction::CP(ArthemeticTarget::E)),
            0xBC => Some(Instruction::CP(ArthemeticTarget::H)),
            0xBD => Some(Instruction::CP(ArthemeticTarget::L)),
            0xBE => Some(Instruction::CP(ArthemeticTarget::HL)),
            0xBF => Some(Instruction::CP(ArthemeticTarget::A)),
            0xFE => Some(Instruction::CP(ArthemeticTarget::D8)),

            // Jump Instr.  12 Inst.
            // checked JP
            0xC2 => Some(Instruction::JP(JumpTest::NotZero)),
            0xD2 => Some(Instruction::JP(JumpTest::NotCarry)),
            0xC3 => Some(Instruction::JP(JumpTest::A16)),
            0xCA => Some(Instruction::JP(JumpTest::Zero)),
            0xDA => Some(Instruction::JP(JumpTest::Carry)),
            0xE9 => Some(Instruction::JP(JumpTest::HL)),

            // Checked RET
            0xC0 => Some(Instruction::RET(JumpTest::NotZero)),
            0xD0 => Some(Instruction::RET(JumpTest::NotCarry)),
            0xC8 => Some(Instruction::RET(JumpTest::Zero)),
            0xD8 => Some(Instruction::RET(JumpTest::Carry)),
            0xC9 => Some(Instruction::RET(JumpTest::Always)),
            0xD9 => Some(Instruction::RET(JumpTest::I)),

            // Loading Instructions. 15 + 8*6 = 63

            // checked LD BASIC TYPE

            // B as Target
            0x40 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::B,
            ))),
            0x41 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::C,
            ))),
            0x42 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::D,
            ))),
            0x43 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::E,
            ))),
            0x44 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::H,
            ))),
            0x45 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::L,
            ))),
            0x46 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::HL,
            ))),
            0x47 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::A,
            ))),

            // C as target
            0x48 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::C,
                LoadByteSource::B,
            ))),
            0x49 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::C,
                LoadByteSource::C,
            ))),
            0x4A => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::C,
                LoadByteSource::D,
            ))),
            0x4B => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::C,
                LoadByteSource::E,
            ))),
            0x4C => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::C,
                LoadByteSource::H,
            ))),
            0x4D => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::C,
                LoadByteSource::L,
            ))),
            0x4E => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::C,
                LoadByteSource::HL,
            ))),
            0x4F => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::C,
                LoadByteSource::A,
            ))),

            // D as target
            0x50 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::D,
                LoadByteSource::B,
            ))),
            0x51 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::D,
                LoadByteSource::C,
            ))),
            0x52 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::D,
                LoadByteSource::D,
            ))),
            0x53 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::D,
                LoadByteSource::E,
            ))),
            0x54 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::D,
                LoadByteSource::H,
            ))),
            0x55 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::D,
                LoadByteSource::L,
            ))),
            0x56 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::D,
                LoadByteSource::HL,
            ))),
            0x57 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::D,
                LoadByteSource::A,
            ))),

            // E as Target
            0x58 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::E,
                LoadByteSource::B,
            ))),
            0x59 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::E,
                LoadByteSource::C,
            ))),
            0x5A => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::E,
                LoadByteSource::D,
            ))),
            0x5B => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::E,
                LoadByteSource::E,
            ))),
            0x5C => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::E,
                LoadByteSource::H,
            ))),
            0x5D => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::E,
                LoadByteSource::L,
            ))),
            0x5E => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::E,
                LoadByteSource::HL,
            ))),
            0x5F => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::E,
                LoadByteSource::A,
            ))),

            // H as Target
            0x60 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::H,
                LoadByteSource::B,
            ))),
            0x61 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::H,
                LoadByteSource::C,
            ))),
            0x62 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::H,
                LoadByteSource::D,
            ))),
            0x63 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::H,
                LoadByteSource::E,
            ))),
            0x64 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::H,
                LoadByteSource::H,
            ))),
            0x65 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::H,
                LoadByteSource::L,
            ))),
            0x66 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::H,
                LoadByteSource::HL,
            ))),
            0x67 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::H,
                LoadByteSource::A,
            ))),

            // L as target
            0x68 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::L,
                LoadByteSource::B,
            ))),
            0x69 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::L,
                LoadByteSource::C,
            ))),
            0x6A => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::L,
                LoadByteSource::D,
            ))),
            0x6B => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::L,
                LoadByteSource::E,
            ))),
            0x6C => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::L,
                LoadByteSource::H,
            ))),
            0x6D => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::L,
                LoadByteSource::L,
            ))),
            0x6E => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::L,
                LoadByteSource::HL,
            ))),
            0x6F => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::L,
                LoadByteSource::A,
            ))),

            // Hl as Target
            0x70 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::HL,
                LoadByteSource::B,
            ))),
            0x71 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::HL,
                LoadByteSource::C,
            ))),
            0x72 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::HL,
                LoadByteSource::D,
            ))),
            0x73 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::HL,
                LoadByteSource::E,
            ))),
            0x74 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::HL,
                LoadByteSource::H,
            ))),
            0x75 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::HL,
                LoadByteSource::L,
            ))),
            // HALT
            0x77 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::HL,
                LoadByteSource::A,
            ))),

            // A as target
            0x78 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::B,
            ))),
            0x79 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::C,
            ))),
            0x7A => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::D,
            ))),
            0x7B => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::E,
            ))),
            0x7C => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::H,
            ))),
            0x7D => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::L,
            ))),
            0x7E => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::HL,
            ))),
            0x7F => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::A,
            ))),

            //SPECIAL LOAD INST.  22 INSTRUCTIONS
            0x02 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::BCV,
                LoadByteSource::A,
            ))),
            0x12 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::DEV,
                LoadByteSource::A,
            ))),
            0x22 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::HLI,
                LoadByteSource::A,
            ))),
            0x32 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::HLD,
                LoadByteSource::A,
            ))),

            // LD reg , n8
            0x06 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::D8,
            ))),
            0x16 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::D,
                LoadByteSource::D8,
            ))),
            0x26 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::H,
                LoadByteSource::D8,
            ))),
            0x36 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::HL,
                LoadByteSource::D8,
            ))),

            0x0E => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::C,
                LoadByteSource::D8,
            ))),
            0x1E => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::E,
                LoadByteSource::D8,
            ))),
            0x2E => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::L,
                LoadByteSource::D8,
            ))),
            0x3E => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::D8,
            ))),

            0x0A => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::BCV,
            ))),
            0x1A => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::DEV,
            ))),
            0x2A => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::HLI,
            ))),
            0x3A => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::HLD,
            ))),

            0xE2 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::OC,
                LoadByteSource::A,
            ))),
            0xF2 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::OC,
            ))),

            0xE0 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::OByte,
                LoadByteSource::A,
            ))),
            0xF0 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::OByte,
            ))),

            0xEA => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::OWord,
                LoadByteSource::A,
            ))),
            0xFA => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::OWord,
            ))),

            // MISC INST. [304 - 291 = 13]
            0x00 => Some(Instruction::NOP),
            0x10 => Some(Instruction::STOP),
            0x76 => Some(Instruction::HALT),
            0xF3 => Some(Instruction::DI),
            0xCB => Some(Instruction::CB),
            0xFB => Some(Instruction::EI),
            0x3F => Some(Instruction::CCF),
            0x37 => Some(Instruction::SCF),
            0x1F => Some(Instruction::RRA),
            0x17 => Some(Instruction::RLA),
            0x0F => Some(Instruction::RRCA),
            0x07 => Some(Instruction::RLCA),
            0x2F => Some(Instruction::CPL),
            0x27 => Some(Instruction::DAA),

            // 16-bit load INSTRUCTIONS
            0xC1 => Some(Instruction::POP(StackTarget::BC)),
            0xD1 => Some(Instruction::POP(StackTarget::DE)),
            0xE1 => Some(Instruction::POP(StackTarget::HL)),
            0xF1 => Some(Instruction::POP(StackTarget::AF)),

            0xC5 => Some(Instruction::PUSH(StackTarget::BC)),
            0xD5 => Some(Instruction::PUSH(StackTarget::DE)),
            0xE5 => Some(Instruction::PUSH(StackTarget::HL)),
            0xF5 => Some(Instruction::PUSH(StackTarget::AF)),

            // more call instructions
            0xC4 => Some(Instruction::CALL(JumpTest::NotZero)),
            0xD4 => Some(Instruction::CALL(JumpTest::NotCarry)),
            0xCC => Some(Instruction::CALL(JumpTest::Zero)),
            0xDC => Some(Instruction::CALL(JumpTest::Carry)),
            0xCD => Some(Instruction::CALL(JumpTest::Always)),

            // LAST 20 Instr.
            0x20 => Some(Instruction::JR(JumpTest::NotZero)),
            0x30 => Some(Instruction::JR(JumpTest::NotCarry)),
            0x28 => Some(Instruction::JR(JumpTest::Zero)),
            0x38 => Some(Instruction::JR(JumpTest::Carry)),
            0x18 => Some(Instruction::JR(JumpTest::Always)),

            // more 16 bit load instructions
            0x01 => Some(Instruction::LD2(LoadType::Word(
                LoadWordTarget::BC,
                LoadWordSource::D16,
            ))),
            0x11 => Some(Instruction::LD2(LoadType::Word(
                LoadWordTarget::DE,
                LoadWordSource::D16,
            ))),
            0x21 => Some(Instruction::LD2(LoadType::Word(
                LoadWordTarget::HL,
                LoadWordSource::D16,
            ))),
            0x31 => Some(Instruction::LD2(LoadType::Word(
                LoadWordTarget::SP,
                LoadWordSource::D16,
            ))),
            0x08 => Some(Instruction::LD2(LoadType::Word(
                LoadWordTarget::A16,
                LoadWordSource::SP,
            ))),

            0xF8 => Some(Instruction::LD2(LoadType::Word(
                LoadWordTarget::HL,
                LoadWordSource::SPr8,
            ))),
            0xF9 => Some(Instruction::LD2(LoadType::Word(
                LoadWordTarget::SP,
                LoadWordSource::HL,
            ))),

            // RST Instructions
            0xC7 => Some(Instruction::RST(RSTTarget::H00)),
            0xD7 => Some(Instruction::RST(RSTTarget::H10)),
            0xE7 => Some(Instruction::RST(RSTTarget::H20)),
            0xF7 => Some(Instruction::RST(RSTTarget::H30)),

            0xCF => Some(Instruction::RST(RSTTarget::H08)),
            0xDF => Some(Instruction::RST(RSTTarget::H18)),
            0xEF => Some(Instruction::RST(RSTTarget::H28)),
            0xFF => Some(Instruction::RST(RSTTarget::H38)),

            _ => None,
        }
    }
}
