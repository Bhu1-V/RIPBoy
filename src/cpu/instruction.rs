use super::target::*;
pub enum Instruction {
    ADD(ArthemeticTarget),
    INC(IncDecTarget),
    RLC(RLCTarget),
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
    PUSH(StackTarget),
    POP(StackTarget),
    CALL(JumpTest),
    RET(JumpTest),
    NOP,
    HALT,
    STOP,
    DI,
    CB,
    EI,
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
            0x00 => Some(Instruction::RLC(RLCTarget::B)),
            _ => None,
        }
    }

    fn from_byte_not_prefixed(byte: u8) -> Option<Instruction> {
        match byte {

            // - DATA PROCESSING INST. //
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

            0x90 => Some(Instruction::SUB(ArthemeticTarget::B)),
            0x91 => Some(Instruction::SUB(ArthemeticTarget::C)),
            0x92 => Some(Instruction::SUB(ArthemeticTarget::D)),
            0x93 => Some(Instruction::SUB(ArthemeticTarget::E)),
            0x94 => Some(Instruction::SUB(ArthemeticTarget::H)),
            0x95 => Some(Instruction::SUB(ArthemeticTarget::L)),
            0x96 => Some(Instruction::SUB(ArthemeticTarget::HL)),
            0x97 => Some(Instruction::SUB(ArthemeticTarget::A)),
            0xD6 => Some(Instruction::SUB(ArthemeticTarget::D8)),

            0xA0 => Some(Instruction::AND(ArthemeticTarget::B)),
            0xA1 => Some(Instruction::AND(ArthemeticTarget::C)),
            0xA2 => Some(Instruction::AND(ArthemeticTarget::D)),
            0xA3 => Some(Instruction::AND(ArthemeticTarget::E)),
            0xA4 => Some(Instruction::AND(ArthemeticTarget::H)),
            0xA5 => Some(Instruction::AND(ArthemeticTarget::L)),
            0xA6 => Some(Instruction::AND(ArthemeticTarget::HL)),
            0xA7 => Some(Instruction::AND(ArthemeticTarget::A)),
            0xE6 => Some(Instruction::AND(ArthemeticTarget::D8)),

            0xB0 => Some(Instruction::OR(ArthemeticTarget::B)),
            0xB1 => Some(Instruction::OR(ArthemeticTarget::C)),
            0xB2 => Some(Instruction::OR(ArthemeticTarget::D)),
            0xB3 => Some(Instruction::OR(ArthemeticTarget::E)),
            0xB4 => Some(Instruction::OR(ArthemeticTarget::H)),
            0xB5 => Some(Instruction::OR(ArthemeticTarget::L)),
            0xB6 => Some(Instruction::OR(ArthemeticTarget::HL)),
            0xB7 => Some(Instruction::OR(ArthemeticTarget::A)),
            0xF6 => Some(Instruction::OR(ArthemeticTarget::D8)),
            
            0x88 => Some(Instruction::ADC(ArthemeticTarget::B)),
            0x89 => Some(Instruction::ADC(ArthemeticTarget::C)),
            0x8A => Some(Instruction::ADC(ArthemeticTarget::D)),
            0x8B => Some(Instruction::ADC(ArthemeticTarget::E)),
            0x8C => Some(Instruction::ADC(ArthemeticTarget::H)),
            0x8D => Some(Instruction::ADC(ArthemeticTarget::L)),
            0x8E => Some(Instruction::ADC(ArthemeticTarget::HL)),
            0x8F => Some(Instruction::ADC(ArthemeticTarget::A)),
            0xCE => Some(Instruction::ADC(ArthemeticTarget::D8)),
            
            0x98 => Some(Instruction::SBC(ArthemeticTarget::B)),
            0x99 => Some(Instruction::SBC(ArthemeticTarget::C)),
            0x9A => Some(Instruction::SBC(ArthemeticTarget::D)),
            0x9B => Some(Instruction::SBC(ArthemeticTarget::E)),
            0x9C => Some(Instruction::SBC(ArthemeticTarget::H)),
            0x9D => Some(Instruction::SBC(ArthemeticTarget::L)),
            0x9E => Some(Instruction::SBC(ArthemeticTarget::HL)),
            0x9F => Some(Instruction::SBC(ArthemeticTarget::A)),
            0xDE => Some(Instruction::SBC(ArthemeticTarget::D8)),

            0xA8 => Some(Instruction::XOR(ArthemeticTarget::B)),
            0xA9 => Some(Instruction::XOR(ArthemeticTarget::C)),
            0xAA => Some(Instruction::XOR(ArthemeticTarget::D)),
            0xAB => Some(Instruction::XOR(ArthemeticTarget::E)),
            0xAC => Some(Instruction::XOR(ArthemeticTarget::H)),
            0xAD => Some(Instruction::XOR(ArthemeticTarget::L)),
            0xAE => Some(Instruction::XOR(ArthemeticTarget::HL)),
            0xAF => Some(Instruction::XOR(ArthemeticTarget::A)),
            0xEE => Some(Instruction::XOR(ArthemeticTarget::D8)),

            0xB8 => Some(Instruction::CP(ArthemeticTarget::B)),
            0xB9 => Some(Instruction::CP(ArthemeticTarget::C)),
            0xBA => Some(Instruction::CP(ArthemeticTarget::D)),
            0xBB => Some(Instruction::CP(ArthemeticTarget::E)),
            0xBC => Some(Instruction::CP(ArthemeticTarget::H)),
            0xBD => Some(Instruction::CP(ArthemeticTarget::L)),
            0xBE => Some(Instruction::CP(ArthemeticTarget::HL)),
            0xBF => Some(Instruction::CP(ArthemeticTarget::A)),
            0xFE => Some(Instruction::CP(ArthemeticTarget::D8)),
            
            // Jump Instr.
            0xC2 => Some(Instruction::JP(JumpTest::NotZero)),
            0xD2 => Some(Instruction::JP(JumpTest::NotCarry)),
            0xC3 => Some(Instruction::JP(JumpTest::A16)),
            0xCA => Some(Instruction::JP(JumpTest::Zero)),
            0xDA => Some(Instruction::JP(JumpTest::Carry)),
            0xE9 => Some(Instruction::JP(JumpTest::HL)),

            0xC1 => Some(Instruction::RET(JumpTest::NotZero)),
            0xD1 => Some(Instruction::RET(JumpTest::NotCarry)),
            0xC8 => Some(Instruction::RET(JumpTest::Zero)),
            0xD8 => Some(Instruction::RET(JumpTest::Carry)),
            0xC9 => Some(Instruction::RET(JumpTest::Always)),
            0xD9 => Some(Instruction::RET(JumpTest::I)),

            // Loading Instructions.
            0x40 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B,LoadByteSource::B))),
            0x41 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B,LoadByteSource::C))),
            0x42 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B,LoadByteSource::D))),
            0x43 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B,LoadByteSource::E))),
            0x44 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B,LoadByteSource::H))),
            0x45 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B,LoadByteSource::L))),
            0x46 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B,LoadByteSource::HL))),
            0x47 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B,LoadByteSource::A))),

            0x48 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C,LoadByteSource::B))),
            0x49 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C,LoadByteSource::C))),
            0x4A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C,LoadByteSource::D))),
            0x4B => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C,LoadByteSource::E))),
            0x4C => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C,LoadByteSource::H))),
            0x4D => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C,LoadByteSource::L))),
            0x4E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C,LoadByteSource::HL))),
            0x4F => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C,LoadByteSource::A))),

            0x50 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D,LoadByteSource::B))),
            0x51 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D,LoadByteSource::C))),
            0x52 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D,LoadByteSource::D))),
            0x53 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D,LoadByteSource::E))),
            0x54 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D,LoadByteSource::H))),
            0x55 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D,LoadByteSource::L))),
            0x56 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D,LoadByteSource::HL))),
            0x57 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D,LoadByteSource::A))),

            0x58 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E,LoadByteSource::B))),
            0x59 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E,LoadByteSource::C))),
            0x5A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E,LoadByteSource::D))),
            0x5B => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E,LoadByteSource::E))),
            0x5C => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E,LoadByteSource::H))),
            0x5D => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E,LoadByteSource::L))),
            0x5E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E,LoadByteSource::HL))),
            0x5F => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E,LoadByteSource::A))),

            0x60 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H,LoadByteSource::B))),
            0x61 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H,LoadByteSource::C))),
            0x62 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H,LoadByteSource::D))),
            0x63 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H,LoadByteSource::E))),
            0x64 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H,LoadByteSource::H))),
            0x65 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H,LoadByteSource::L))),
            0x66 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H,LoadByteSource::HL))),
            0x67 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H,LoadByteSource::A))),

            0x68 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L,LoadByteSource::B))),
            0x69 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L,LoadByteSource::C))),
            0x6A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L,LoadByteSource::D))),
            0x6B => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L,LoadByteSource::E))),
            0x6C => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L,LoadByteSource::H))),
            0x6D => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L,LoadByteSource::L))),
            0x6E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L,LoadByteSource::HL))),
            0x6F => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L,LoadByteSource::A))),

            0x70 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HL,LoadByteSource::B))),
            0x71 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HL,LoadByteSource::C))),
            0x72 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HL,LoadByteSource::D))),
            0x73 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HL,LoadByteSource::E))),
            0x74 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HL,LoadByteSource::H))),
            0x75 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HL,LoadByteSource::L))),
            // HALT
            0x77 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HL,LoadByteSource::A))),

            0x78 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A,LoadByteSource::B))),
            0x79 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A,LoadByteSource::C))),
            0x7A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A,LoadByteSource::D))),
            0x7B => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A,LoadByteSource::E))),
            0x7C => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A,LoadByteSource::H))),
            0x7D => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A,LoadByteSource::L))),
            0x7E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A,LoadByteSource::HL))),
            0x7F => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A,LoadByteSource::A))),

            //SPECIAL LOAD INST.

            0x02 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::BCV,LoadByteSource::A))),
            0x12 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::DEV,LoadByteSource::A))),
            0x22 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HLI,LoadByteSource::A))),
            0x32 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HLD,LoadByteSource::A))),

            0x06 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B,LoadByteSource::D8))),
            0x16 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D,LoadByteSource::D8))),
            0x26 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H,LoadByteSource::D8))),
            0x36 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HL,LoadByteSource::D8))),

            0x0A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A,LoadByteSource::BCV))),
            0x1A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A,LoadByteSource::DEV))),
            0x2A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A,LoadByteSource::HLI))),
            0x3A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A,LoadByteSource::HLD))),

            0x0E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C,LoadByteSource::D8))),
            0x1E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E,LoadByteSource::D8))),
            0x2E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L,LoadByteSource::D8))),
            0x3E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A,LoadByteSource::D8))),

            0xE2 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::OC,LoadByteSource::A))),
            0xF2 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A,LoadByteSource::OC))),

            0xE0 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::OByte,LoadByteSource::A))),
            0xF0 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A,LoadByteSource::OByte))),

            0xEA => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::OWord,LoadByteSource::A))),
            0xFA => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A,LoadByteSource::OWord))),

            
            // MISC INST.
            0x00 => Some(Instruction::NOP),
            0x10 => Some(Instruction::STOP),
            0x76 => Some(Instruction::HALT),
            0xF3 => Some(Instruction::DI),
            0xCB => Some(Instruction::CB),
            0xFB => Some(Instruction::EI),

          _ => None,
        }
    }
}
