pub enum ArthemeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    D8,
    HL,
    HLBC,
    HLDE,
    HLHL,
    HLSP,
    SP,
}

pub enum ArthemeticTarget16Bit {
    BC,
    DE,
    HL,
    SP,
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
pub enum StackTarget {
    BC,
    HL,
    DE,
    AF,
}
pub enum LoadByteTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
    HLI,
    HLD,
    OC,
    OWord,
    OByte,
    BCV,
    DEV,
}
pub enum LoadByteSource {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    D8,
    HL,
    HLI,
    OC,
    OWord,
    OByte,
    BCV,
    DEV,
    HLD,
}
pub enum LoadType {
    Byte(LoadByteTarget, LoadByteSource),
    Word(LoadWordTarget , LoadWordSource),
}
pub enum LoadWordTarget{
    BC, DE ,HL ,SP , A16 ,
}
pub enum LoadWordSource{
    D16 , SPr8 , HL, SP
}
pub enum RSTTarget{
    H00, H10, H20, H30 ,H08 ,H18 ,H28 ,H38,
}
pub enum PrefixTarget{
    B,C,D,E,H,L,HLV,A,
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

pub enum BitManipulationType{
    Bit(TargetBit,SourceRegister),
}
pub enum TargetBit{
    B0 , B1 , B2 , B3 , B4 , B5 , B6 , B7 ,
}
pub enum SourceRegister{
    A,B,C,D,E,H,L,HLV,
}