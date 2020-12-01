use flags_register::FlagsRegister;
use instruction::Instruction;
use memory_bus::MemoryBus;
use registers::Registers;
use target::*;

pub mod flags_register;
pub mod instruction;
pub mod memory_bus;
pub mod registers;
pub mod target;

#[derive(Debug)]
pub struct CPU {
    registers : Registers,
    bus : MemoryBus,
    pc : u16,
    sp : u16,
    is_halted : bool,
}

impl CPU {
    fn execute(&mut self, instruction: Instruction) -> u16 {
        if self.is_halted {
            return self.pc;
        }
        let mut value;
        match instruction {
            Instruction::ADD(target) => {
                match target {
                    ArthemeticTarget::A => {
                        value = self.registers.a;
                    }
                    ArthemeticTarget::B => {
                        value = self.registers.b;
                    }
                    ArthemeticTarget::C => {
                        value = self.registers.c;
                    }
                    ArthemeticTarget::D => {
                        value = self.registers.d;
                    }
                    ArthemeticTarget::E => {
                        value = self.registers.e;
                    }
                    ArthemeticTarget::H => {
                        value = self.registers.h;
                    }
                    ArthemeticTarget::L => {
                        value = self.registers.l;
                    }
                    ArthemeticTarget::HL => {
                        value = self.registers.get_hl() as u8;
                    }
                    ArthemeticTarget::D8 => {
                        // TO-DO : Implement correctly
                        value = self.registers.c;
                    }
                }
                let new_value = self.add(value,false);
                self.registers.a = new_value;
                return self.pc.wrapping_add(1);
            }

            Instruction::SUB(target) => {
                match target {
                    ArthemeticTarget::A => {
                        value = self.registers.a;
                    }
                    ArthemeticTarget::B => {
                        value = self.registers.b;
                    }
                    ArthemeticTarget::C => {
                        value = self.registers.c;
                    }
                    ArthemeticTarget::D => {
                        value = self.registers.d;
                    }
                    ArthemeticTarget::E => {
                        value = self.registers.e;
                    }
                    ArthemeticTarget::H => {
                        value = self.registers.h;
                    }
                    ArthemeticTarget::L => {
                        value = self.registers.l;
                    }
                    ArthemeticTarget::HL => {
                        value = self.registers.get_hl() as u8;
                    }
                    ArthemeticTarget::D8 => {
                        // TO-DO : Implement correctly
                        value = self.registers.c;
                    }
                }
                let new_value = self.sub(value,false);
                self.registers.a = new_value;
                return self.pc.wrapping_add(1);
            }

            Instruction::ADC(target) => {
                match target {
                    ArthemeticTarget::A => {
                        value = self.registers.a;
                    }
                    ArthemeticTarget::B => {
                        value = self.registers.b;
                    }
                    ArthemeticTarget::C => {
                        value = self.registers.c;
                    }
                    ArthemeticTarget::D => {
                        value = self.registers.d;
                    }
                    ArthemeticTarget::E => {
                        value = self.registers.e;
                    }
                    ArthemeticTarget::H => {
                        value = self.registers.h;
                    }
                    ArthemeticTarget::L => {
                        value = self.registers.l;
                    }
                    ArthemeticTarget::HL => {
                        value = self.registers.get_hl() as u8;
                    }
                    ArthemeticTarget::D8 => {
                        // TO-DO : Implement correctly
                        value = self.registers.c;
                    }
                }
                let new_value = self.add(value, true);
                self.registers.a = new_value;
                let new_value = self.add(value,false);
                self.registers.a = new_value;
                return self.pc.wrapping_add(1);
            }

            Instruction::SBC(target) => {
                match target {
                    ArthemeticTarget::A => {
                        value = self.registers.a;
                    }
                    ArthemeticTarget::B => {
                        value = self.registers.b;
                    }
                    ArthemeticTarget::C => {
                        value = self.registers.c;
                    }
                    ArthemeticTarget::D => {
                        value = self.registers.d;
                    }
                    ArthemeticTarget::E => {
                        value = self.registers.e;
                    }
                    ArthemeticTarget::H => {
                        value = self.registers.h;
                    }
                    ArthemeticTarget::L => {
                        value = self.registers.l;
                    }
                    ArthemeticTarget::HL => {
                        value = self.registers.get_hl() as u8;
                    }
                    ArthemeticTarget::D8 => {
                        // TO-DO : Implement correctly
                        value = self.registers.c;
                    }
                }
                let new_value = self.sub(value, true);
                self.registers.a = new_value;
                let new_value = self.sub(value,false);
                self.registers.a = new_value;
                return self.pc.wrapping_add(1);
            }

            Instruction::AND(target) => {
                match target {
                    ArthemeticTarget::A => {
                        value = self.registers.a;
                    }
                    ArthemeticTarget::B => {
                        value = self.registers.b;
                    }
                    ArthemeticTarget::C => {
                        value = self.registers.c;
                    }
                    ArthemeticTarget::D => {
                        value = self.registers.d;
                    }
                    ArthemeticTarget::E => {
                        value = self.registers.e;
                    }
                    ArthemeticTarget::H => {
                        value = self.registers.h;
                    }
                    ArthemeticTarget::L => {
                        value = self.registers.l;
                    }
                    ArthemeticTarget::HL => {
                        value = self.registers.get_hl() as u8;
                    }
                    ArthemeticTarget::D8 => {
                        // TO-DO : Implement correctly
                        value = self.registers.c;
                    }
                }
                let new_value = self.and(value);
                self.registers.a = new_value;
                return self.pc.wrapping_add(1);
            }

            Instruction::OR(target) => {
                match target {
                    ArthemeticTarget::A => {
                        value = self.registers.a;
                    }
                    ArthemeticTarget::B => {
                        value = self.registers.b;
                    }
                    ArthemeticTarget::C => {
                        value = self.registers.c;
                    }
                    ArthemeticTarget::D => {
                        value = self.registers.d;
                    }
                    ArthemeticTarget::E => {
                        value = self.registers.e;
                    }
                    ArthemeticTarget::H => {
                        value = self.registers.h;
                    }
                    ArthemeticTarget::L => {
                        value = self.registers.l;
                    }
                    ArthemeticTarget::HL => {
                        value = self.registers.get_hl() as u8;
                    }
                    ArthemeticTarget::D8 => {
                        // TO-DO : Implement correctly
                        value = self.registers.c;
                    }
                }
                let new_value = self.or(value,false);
                self.registers.a = new_value;
                return self.pc.wrapping_add(1);
            }

            Instruction::XOR(target) => {
                match target {
                    ArthemeticTarget::A => {
                        value = self.registers.a;
                    }
                    ArthemeticTarget::B => {
                        value = self.registers.b;
                    }
                    ArthemeticTarget::C => {
                        value = self.registers.c;
                    }
                    ArthemeticTarget::D => {
                        value = self.registers.d;
                    }
                    ArthemeticTarget::E => {
                        value = self.registers.e;
                    }
                    ArthemeticTarget::H => {
                        value = self.registers.h;
                    }
                    ArthemeticTarget::L => {
                        value = self.registers.l;
                    }
                    ArthemeticTarget::HL => {
                        value = self.registers.get_hl() as u8;
                    }
                    ArthemeticTarget::D8 => {
                        // TO-DO : Implement correctly
                        value = self.registers.c;
                    }
                }
                let new_value = self.or(value,true);
                self.registers.a = new_value;
                return self.pc.wrapping_add(1);
            }

            Instruction::JP(test) => {
                let jump_condition = match test {
                    JumpTest::NotZero => {!self.registers.f.zero},
                    JumpTest::NotCarry => {!self.registers.f.carry},
                    JumpTest::Zero => {self.registers.f.zero},
                  JumpTest::Carry => {self.registers.f.carry},
                    JumpTest::A16 => {true},
                    JumpTest::HL => {
                        return self.jump(true,true);
                    }
                };
                self.jump(jump_condition, false)
            }

            Instruction::LD(load_type) => {
                match load_type {
                    LoadType::Byte(target,source) => {
                        let source_value = match source{
                            LoadByteSource::A => self.registers.a,
                            LoadByteSource::B => self.registers.b,
                            LoadByteSource::C => self.registers.c,
                            LoadByteSource::D => self.registers.d,
                            LoadByteSource::E => self.registers.e,
                            LoadByteSource::H => self.registers.h,
                            LoadByteSource::L => self.registers.l,
                            LoadByteSource::D8 => self.read_next_byte(),
                            LoadByteSource::HLI => self.bus.read_byte(self.registers.get_hl()),
                            _ => panic!("Load source error"),
                        };
                        match target {
                            LoadByteTarget::A => self.registers.a = source_value,
                            LoadByteTarget::B => self.registers.b = source_value,
                            LoadByteTarget::C => self.registers.c = source_value,
                            LoadByteTarget::D => self.registers.d = source_value,
                            LoadByteTarget::E => self.registers.e = source_value,
                            LoadByteTarget::H => self.registers.h = source_value,
                            LoadByteTarget::L => self.registers.l = source_value,
                            LoadByteTarget::HLI => self.bus.write_bytes(self.registers.get_hl(), source_value),
                        }
                        match source {
                            LoadByteSource::D8 => self.pc.wrapping_add(2),
                            _ => self.pc.wrapping_add(1),
                        }
                    }
                    _ => panic!("Load ERROR"),
                }                                
            }

            Instruction::PUSH(target) => {
                let value = match target {
                    StackTarget::BC => self.registers.get_bc(),
                    StackTarget::DE => self.registers.get_de(),
                    StackTarget::HL => self.registers.get_hl(),
                    StackTarget::AF => self.registers.get_af(),
                };
                self.push(value);
                self.pc.wrapping_add(1)
            }

            Instruction::POP(target) => {
                let result = self.pop();
                match target {
                    StackTarget::BC => self.registers.set_bc(result),
                    StackTarget::DE => self.registers.set_de(result),
                    StackTarget::HL => self.registers.set_hl(result),
                    StackTarget::AF => self.registers.set_af(result),
                }
                self.pc.wrapping_add(1)
            }

            Instruction::CALL(test) => {
                let jump_condition = match test {
                    JumpTest::NotZero => !self.registers.f.zero,
                    _ => { panic!("TODO: support more conditions") }
                };
                self.call(jump_condition)
            }

            Instruction::RET(test) => {
                let jump_condition = match test {
                    JumpTest::NotZero => !self.registers.f.zero,
                    _ => { panic!("TODO: support more conditions") }
                };
                self.return_(jump_condition)
            }

            Instruction::NOP => {
                self.pc.wrapping_add(1)
            }

            Instruction::HALT => {
                self.is_halted = true;
                self.pc
            }

            Instruction::INC(target) => {5}
            _ => {
                return self.pc;
            }
        }
    }

    fn call(&mut self, should_jump: bool) -> u16 {
        let next_pc = self.pc.wrapping_add(3);
        if should_jump {
            self.push(next_pc);
            self.read_next_word()
        }else {
            next_pc
        }
    }

    fn return_(&mut self, should_jump: bool) -> u16 {
        if should_jump {
          self.pop()
        } else {
          self.pc.wrapping_add(1)
        }
    }

    fn read_next_byte(&self) -> u8{
        self.bus.read_byte(self.pc+1)
    }

    fn read_next_word(&self) -> u16 {
        (self.bus.read_byte(self.pc+1) as u16) << 8 |
        self.bus.read_byte(self.pc+2) as u16
    }

    fn pop(&mut self) -> u16{
        let lsb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);
        
        let msb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        (msb << 8) | lsb
    }

    fn push(&mut self , value : u16) {
        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_bytes(self.sp, ((value & 0xFF00) >> 8) as u8);
        
        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_bytes(self.sp, (value & 0x00FF) as u8);
    }

    fn step(&mut self) {
        let instruction_byte = self.bus.read_byte(self.pc);
        let prefixed = instruction_byte == 0xCB;
        if prefixed {
            let instruction_byte = self.bus.read_byte(self.pc + 1);
        }

        let next_pc = if let Some(instruction) = Instruction::from_byte(instruction_byte, prefixed)
        {
            self.execute(instruction)
        } else {
            panic!("Invaild Instruction {:x}", instruction_byte);
        };

        self.pc = next_pc;
    }

    fn add(&mut self, val: u8 , carry : bool) -> u8 {
        let cy = if carry {
            if self.registers.f.carry == true{
                1
            }else{
                0
            }
        } else {0};
        let (new_val, over_flowed) = self.registers.a.overflowing_add(val);
        self.registers.f = FlagsRegister {
            zero: new_val == 0,
            subtract: false,
            carry: over_flowed,
            half_carry: 
            ((self.registers.a & 0xF).overflowing_add(val & 0xF)).0  > 0xF,
        };
        if carry {
            self.add(cy, false)
        }else{
            new_val
        }
    }

    fn sub(&mut self, val: u8 , carry : bool) -> u8 {
        let cy = if carry {
            if self.registers.f.carry == true{
                1
            }else{
                0
            }
        } else {0};
        let (new_val, over_flowed) = self.registers.a.overflowing_sub(val);
        self.registers.f = FlagsRegister {
            zero: new_val == 0,
            subtract: true,
            carry: over_flowed,
            half_carry: ((self.registers.a & 0xF).overflowing_sub(val & 0xF)).0  > 0xF,
        };
        if carry {
            self.sub(cy, false)
        }else{
            new_val
        }
    }

    fn and(&mut self, value:u8) -> u8{
        let new_val = self.registers.a & value;
        self.registers.f = FlagsRegister {
            zero : new_val == 0,
            carry : false ,
            half_carry : true ,
            subtract : false ,
        };
        new_val   
    }

    fn or(&mut self, value:u8 , exculsive:bool) -> u8{
        let new_val = if exculsive {self.registers.a ^ value } else {self.registers.a | value} ;
        println!("from or = new_val = {} , 0x{:x}", new_val,new_val);
        self.registers.f = FlagsRegister {
            zero : new_val == 0,
            carry : false ,
            half_carry : false ,
            subtract : false ,
        };
        new_val   
    }

    fn jump(&self, should_jump: bool,exception:bool) -> u16 {
        if should_jump & !(exception) {
          // Gameboy is little endian so read pc + 2 as most significant bit
          // and pc + 1 as least significant bit
          let least_significant_byte = self.bus.read_byte(self.pc + 1) as u16;
          let most_significant_byte = self.bus.read_byte(self.pc + 2) as u16;
          (most_significant_byte << 8) | least_significant_byte

        } else if should_jump & exception {
            
            let least_significant_byte = self.bus.read_byte(self.pc + 1) as u16;
            least_significant_byte

        }else{
          // If we don't jump we need to still move the program
          // counter forward by 3 since the jump instruction is
          // 3 bytes wide (1 byte for tag and 2 bytes for jump address)
          self.pc.wrapping_add(3)
        }
      }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn add_checker(){
        let r:Registers = Registers{
            a:0xff,
            b:0x0f,
            c:0x38,
            d:0,
            e:0,
            f:FlagsRegister{
                zero : false,
                subtract : false,
                half_carry : false,
                carry : true,
            },
            h:0,
            l:0x8a,
        };
        let mem = [1; 0xFFFF];
        let mut  c:CPU = CPU{
            registers : r,
            bus : MemoryBus {
                memory: mem,
            },
            pc : 0,
        };
        let b: u8 = c.registers.get_hl() as u8;
        // let a: u8 = c.registers.a;
        // let new_value = c.sub(b, true);
        // c.registers.a = new_value;
        println!("1st {:?}",c);
        let new_value = c.or(b,true);
        c.registers.a = new_value;
        println!("a = 0x{:x} , {}",new_value,new_value);
        println!("2nd {:?}",c);
        assert_eq!(1,0);
    }
}
