use flags_register::FlagsRegister;
use memory_bus::MemoryBus;
use registers::Registers;

pub mod registers;
pub mod flags_register;
pub mod target;
pub mod memory_bus;

pub struct CPU{
    registers : Registers,
    bus : MemoryBus,
    pc : u16,
}

enum Instruction{
    ADD(target::ArthemeticTarget),
}

impl CPU {
    fn execute(&mut self , instruction:Instruction){
        match instruction {
            Instruction::ADD(target) => {
                match target {
                    B => {
                        let value = self.registers.b;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                }
            }
        }
    }

    fn add (&mut self,val:u8) -> u8 {
        let (new_val , over_flowed ) = self.registers.a.overflowing_add(val);
        self.registers.f = FlagsRegister{
            zero : new_val == 0,
            subtract : false,
            carry : over_flowed,
            half_carry : (self.registers.a & 0xF) + (val & 0xF) > 0xF,
        };
        new_val
    }
}