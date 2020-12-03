pub enum ArthemeticTarget {
    A, B, C, D, E, H, L, D8, HL,HLBC,HLDE,HLHL,HLSP,SP
}

pub enum ArthemeticTarget16Bit{
    BC, DE, HL, SP,
}
pub enum JumpTest {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    A16,
    HL,
    Always,
    I,
}
pub enum StackTarget{
    BC,HL,DE,AF
}
pub enum LoadByteTarget {
    A, B, C, D, E, H, L, HL,HLI,HLD,OC,OWord,OByte,BCV,DEV,
}
pub enum LoadByteSource {
    A, B, C, D, E, H, L, D8,HL, HLI,OC,OWord,OByte,BCV,DEV,HLD,
}
pub enum LoadType{
    Byte(LoadByteTarget,LoadByteSource),
}
pub enum IncDecTarget {
    BC,
    DE,
    HL,
    SP,
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HL2,
}

pub enum RLCTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    HL,
}
