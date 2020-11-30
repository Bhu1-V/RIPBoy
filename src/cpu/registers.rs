use crate::cpu::flags_register::FlagsRegister;
pub struct Registers{
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: FlagsRegister,
    pub h: u8,
    pub l: u8,
}

impl Registers {
    fn get_bc(&self) -> u16{
        (self.b as u16) << 8 | (self.c as u16)
    }

    fn set_bc(&mut self , value:u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    fn get_de(&self) -> u16{
        (self.d as u16) << 8 | (self.e as u16)
    }

    fn set_de(&mut self , value:u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }
    fn get_hl(&self) -> u16{
        (self.h as u16) << 8 | (self.l as u16)
    }

    fn set_hl(&mut self , value:u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }
}
