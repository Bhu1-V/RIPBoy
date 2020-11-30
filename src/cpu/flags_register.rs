pub struct FlagsRegister {
    pub zero : bool,
    pub subtract : bool,
    pub half_carry : bool,
    pub carry : bool,
}

const ZERO_FLAG_BIT_POSITION : u8 = 7;
const SUBTRACT_FLAG_BIT_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BIT_POSITION: u8 = 5;
const CARRY_FLAG_BIT_POSITION: u8 = 4;

impl std::convert::From<FlagsRegister> for u8 {
    fn from(flag : FlagsRegister) -> u8 {
        ( if flag.zero {1} else {0} ) << ZERO_FLAG_BIT_POSITION |
        ( if flag.subtract {1} else {0} ) << SUBTRACT_FLAG_BIT_POSITION |
        ( if flag.half_carry {1} else {0} ) << HALF_CARRY_FLAG_BIT_POSITION |
        ( if flag.carry {1} else {0} ) << CARRY_FLAG_BIT_POSITION 
    }
}

impl std::convert::From<u8> for FlagsRegister {
    fn from(value: u8) -> FlagsRegister {
        let zero: bool = (value >> ZERO_FLAG_BIT_POSITION & 1) != 0;
        let subtract: bool = (value >> SUBTRACT_FLAG_BIT_POSITION & 1) != 0;
        let half_carry: bool = (value >> HALF_CARRY_FLAG_BIT_POSITION & 1) != 0;
        let carry : bool = (value >> CARRY_FLAG_BIT_POSITION & 1) != 0;
        FlagsRegister {
            zero,
            subtract,
            half_carry,
            carry
        }
    }
}
