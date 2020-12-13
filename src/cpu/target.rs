#[derive(Debug)]
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
#[derive(Debug)]
pub enum ArthemeticTarget16Bit {
    BC,
    DE,
    HL,
    SP,
}
#[derive(Debug)]
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
#[derive(Debug)]
pub enum StackTarget {
    BC,
    HL,
    DE,
    AF,
}
#[derive(Debug)]
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
#[derive(Debug)]
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
#[derive(Debug)]
pub enum LoadType {
    Byte(LoadByteTarget, LoadByteSource),
    Word(LoadWordTarget, LoadWordSource),
}
#[derive(Debug)]
pub enum LoadWordTarget {
    BC,
    DE,
    HL,
    SP,
    A16,
}
#[derive(Debug)]
pub enum LoadWordSource {
    D16,
    SPr8,
    HL,
    SP,
}
#[derive(Debug)]
pub enum RSTTarget {
    H00,
    H10,
    H20,
    H30,
    H08,
    H18,
    H28,
    H38,
}
#[derive(Debug)]
pub enum PrefixTarget {
    B,
    C,
    D,
    E,
    H,
    L,
    HLV,
    A,
}
#[derive(Debug)]
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
#[derive(Debug)]
pub enum RLCTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    HL,
}
#[derive(Debug)]
pub enum BitManipulationType {
    Bit(TargetBit, SourceRegister),
}
#[derive(Debug)]
pub enum TargetBit {
    B0,
    B1,
    B2,
    B3,
    B4,
    B5,
    B6,
    B7,
}
#[derive(Debug)]
pub enum SourceRegister {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HLV,
}
