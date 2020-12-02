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
