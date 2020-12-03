use clock::Clock;
use flags_register::FlagsRegister;
use instruction::Instruction;
use memory_bus::MemoryBus;
use registers::Registers;
use target::*;

pub mod clock;
pub mod flags_register;
pub mod instruction;
pub mod memory_bus;
pub mod registers;
pub mod target;

#[derive(Debug)]
pub struct CPU {
    clock: Clock,
    registers: Registers,
    bus: MemoryBus,
    pc: u16,
    sp: u16,
    is_halted: bool,
    m: u8, // Internal Clock of Last Instruction
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
                        self.m = 1;
                        
                        value = self.bus.read_byte(self.registers.get_hl());
                    }
                    ArthemeticTarget::D8 => {
                        // TO-DO : Implement correctly
                        self.m = 1;
                        
                        value = self.read_next_byte();
                    }
                    ArthemeticTarget::HLBC => {
                        let new_val = self.add_16bit(
                            self.registers.get_hl(),
                            self.registers.get_bc()
                        );
                        self.registers.set_hl(new_val);
                        self.m = 3;
                        return self.pc.wrapping_add(1);
                    }
                    ArthemeticTarget::HLDE => {
                        let new_val = self.add_16bit(
                            self.registers.get_hl(),
                            self.registers.get_de()
                        );
                        self.registers.set_hl(new_val);
                        self.m = 3;
                        return self.pc.wrapping_add(1);
                    }
                    ArthemeticTarget::HLHL => {
                        let new_val = self.add_16bit(
                            self.registers.get_hl(),
                            self.registers.get_hl()
                        );
                        self.registers.set_hl(new_val);
                        self.m = 3;
                        return self.pc.wrapping_add(1);
                    }
                    ArthemeticTarget::HLSP => {
                        let new_val = self.add_16bit(
                            self.registers.get_hl(),
                            self.sp,
                        );
                        self.registers.set_hl(new_val);
                        self.m = 3;
                        return self.pc.wrapping_add(1);
                    }
                    ArthemeticTarget::SP => {
                        // ITS ERROR CHECK THIS.
                        let new_val = self.add_16bit(
                            self.sp,
                            self.bus.read_byte(self.pc) as u16
                        );
                        self.sp = new_val;
                        self.pc = self.pc.wrapping_add(1);
                        self.m = 4;
                        return self.pc.wrapping_add(1);
                    }
                }
                let new_value = self.add(self.registers.a, value, false);
                self.registers.a = new_value;
                self.m += 1;
                
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
                        self.m = 1;
                        
                        value = self.bus.read_byte(self.registers.get_hl());
                    }
                    ArthemeticTarget::D8 => {
                        // TO-DO : Implement correctly
                        self.m = 1;
                        
                        value = self.read_next_byte();
                    }

                    _ => panic!("Reached Unreachable code"),
                }
                let new_value = self.sub(self.registers.a,value, false);
                self.registers.a = new_value;
                self.m += 1;
                
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
                        self.m = 1;
                        
                        value = self.bus.read_byte(self.registers.get_hl());
                    }
                    ArthemeticTarget::D8 => {
                        // TO-DO : Implement correctly
                        self.m = 1;
                        
                        value = self.read_next_byte();
                    }
                    _ => panic!("Reached Unreachable code"),
                }
                let new_value = self.add(self.registers.a, value, true);
                self.registers.a = new_value;
                let new_value = self.add(self.registers.a, value, false);
                self.registers.a = new_value;
                self.m += 1;
                
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
                        self.m = 1;
                        
                        value = self.bus.read_byte(self.registers.get_hl());
                    }
                    ArthemeticTarget::D8 => {
                        // TO-DO : Implement correctly
                        self.m = 1;
                        
                        value = self.read_next_byte();
                    }
                    _ => panic!("Reached Unreachable code"),
                }
                let new_value = self.sub(self.registers.a,value, true);
                self.registers.a = new_value;
                let new_value = self.sub(self.registers.a,value, false);
                self.m += 1;
                
                self.registers.a = new_value;
                return self.pc.wrapping_add(1);
            }

            Instruction::CP(target) => {
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
                        self.m = 1;
                        
                        value = self.bus.read_byte(self.registers.get_hl());
                    }
                    ArthemeticTarget::D8 => {
                        // TO-DO : Implement correctly
                        self.m = 1;
                        
                        value = self.read_next_byte();
                    }
                    _ => panic!("Reached Unreachable code"),
                }
                self.sub(self.registers.a,value, false);
                self.m += 1;
                
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
                        self.m = 1;
                        value = self.bus.read_byte(self.registers.get_hl());
                    }
                    ArthemeticTarget::D8 => {
                        // TO-DO : Implement correctly
                        self.m = 1;
                        
                        value = self.read_next_byte();
                    }
                    _ => panic!("Reached Unreachable code"),
                }
                let new_value = self.and(value);
                self.registers.a = new_value;
                self.m += 1;
                
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
                        self.m = 1;
                        
                        value = self.bus.read_byte(self.registers.get_hl());
                    }
                    ArthemeticTarget::D8 => {
                        // TO-DO : Implement correctly
                        self.m = 1;
                        
                        value = self.read_next_byte();
                    }
                    _ => panic!("Reached Unreachable code"),
                }
                let new_value = self.or(value, false);
                self.m += 1;
                
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
                        self.m = 1;
                        
                        value = self.bus.read_byte(self.registers.get_hl());
                    }
                    ArthemeticTarget::D8 => {
                        // TO-DO : Implement correctly
                        self.m = 1;
                        
                        value = self.read_next_byte();
                    }
                    _ => panic!("Reached Unreachable code"),
                }
                let new_value = self.or(value, true);
                self.m += 1;
                
                self.registers.a = new_value;
                return self.pc.wrapping_add(1);
            }

            Instruction::JP(test) => {
                let jump_condition = match test {
                    JumpTest::NotZero => !self.registers.f.zero,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::A16 => true,
                    JumpTest::HL => {
                        return self.jump(true, true);
                    }
                    _ => panic!("implement adiitional jumptest"),
                };
                self.jump(jump_condition, false)
            }

            Instruction::INC(target) => {
                match target {
                    IncDecTarget::A => {
                        self.registers.a = self.add(self.registers.a, 1, false);
                    }
                    IncDecTarget::B => {
                        self.registers.b = self.add(self.registers.b, 1, false);
                    }
                    IncDecTarget::C => {
                        self.registers.c = self.add(self.registers.c, 1, false);
                    }
                    IncDecTarget::D => {
                        self.registers.d = self.add(self.registers.d, 1, false);
                    }
                    IncDecTarget::E => {
                        self.registers.e = self.add(self.registers.e, 1, false);
                    }
                    IncDecTarget::H => {
                        self.registers.h = self.add(self.registers.h, 1, false);
                    }
                    IncDecTarget::L => {
                        self.registers.l = self.add(self.registers.l, 1, false);
                    }
                    IncDecTarget::HL2 => {
                        self.m = 2;
                        
                        value = self.bus.read_byte(self.registers.get_hl());
                        value = self.add(value,1,false);
                        self.bus.write_bytes(self.registers.get_hl(), value)
                    }
                    IncDecTarget::BC => {
                        // TO-DO : Implement correctly
                        let (val , over_flowed) = self.registers.c.overflowing_add(1);
                        self.registers.c = val;
                        if over_flowed {
                            let (val , _over_flowed) = self.registers.b.overflowing_add(1);
                            self.registers.b = val;
                        }
                    }

                    IncDecTarget::DE => {
                        // TO-DO : Implement correctly
                        let (val , over_flowed) = self.registers.e.overflowing_add(1);
                        self.registers.e = val;
                        if over_flowed {
                            let (val , _over_flowed) = self.registers.d.overflowing_add(1);
                            self.registers.d = val;
                        }
                    }

                    IncDecTarget::HL => {
                        // TO-DO : Implement correctly
                        let (val , over_flowed) = self.registers.l.overflowing_add(1);
                        self.registers.l = val;
                        if over_flowed {
                            let (val , _over_flowed) = self.registers.h.overflowing_add(1);
                            self.registers.h = val;
                        }
                    }

                    IncDecTarget::SP => {
                        self.sp = self.sp.wrapping_add(1);
                    }

                }
                self.m += 1;
                
                return self.pc.wrapping_add(1);
            }

            Instruction::DEC(target) => {
                match target {
                    IncDecTarget::A => {
                        self.registers.a = self.sub(self.registers.a, 1, false);
                    }
                    IncDecTarget::B => {
                        self.registers.b = self.sub(self.registers.b, 1, false);
                    }
                    IncDecTarget::C => {
                        self.registers.c = self.sub(self.registers.c, 1, false);
                    }
                    IncDecTarget::D => {
                        self.registers.d = self.sub(self.registers.d, 1, false);
                    }
                    IncDecTarget::E => {
                        self.registers.e = self.sub(self.registers.e, 1, false);
                    }
                    IncDecTarget::H => {
                        self.registers.h = self.sub(self.registers.h, 1, false);
                    }
                    IncDecTarget::L => {
                        self.registers.l = self.sub(self.registers.l, 1, false);
                    }
                    IncDecTarget::HL2 => {
                        self.m = 2;
                        
                        value = self.bus.read_byte(self.registers.get_hl());
                        value = self.sub(value,1,false);
                        self.bus.write_bytes(self.registers.get_hl(), value)
                    }
                    IncDecTarget::BC => {
                        // TO-DO : Implement correctly
                        let (val , over_flowed) = self.registers.c.overflowing_sub(1);
                        self.registers.c = val;
                        if over_flowed {
                            let (val , _over_flowed) = self.registers.b.overflowing_sub(1);
                            self.registers.b = val;
                        }
                    }

                    IncDecTarget::DE => {
                        // TO-DO : Implement correctly
                        let (val , over_flowed) = self.registers.e.overflowing_sub(1);
                        self.registers.e = val;
                        if over_flowed {
                            let (val , _over_flowed) = self.registers.d.overflowing_sub(1);
                            self.registers.d = val;
                        }
                    }

                    IncDecTarget::HL => {
                        // TO-DO : Implement correctly
                        let (val , over_flowed) = self.registers.l.overflowing_sub(1);
                        self.registers.l = val;
                        if over_flowed {
                            let (val , _over_flowed) = self.registers.h.overflowing_sub(1);
                            self.registers.h = val;
                        }
                    }

                    IncDecTarget::SP => {
                        self.sp = self.sp.wrapping_sub(1);
                    }
                }
                self.m += 1;
                
                return self.pc.wrapping_add(1);
            }

            Instruction::LD(load_type) => match load_type {
                LoadType::Byte(target, source) => {
                    let source_value = match source {
                        LoadByteSource::A => self.registers.a,
                        LoadByteSource::B => self.registers.b,
                        LoadByteSource::C => self.registers.c,
                        LoadByteSource::D => self.registers.d,
                        LoadByteSource::E => self.registers.e,
                        LoadByteSource::H => self.registers.h,
                        LoadByteSource::L => self.registers.l,
                        LoadByteSource::D8 => self.read_next_byte(),
                        LoadByteSource::HL => self.bus.read_byte(self.registers.get_hl()),
                        LoadByteSource::BCV => self.bus.read_byte(self.registers.get_bc()),
                        LoadByteSource::DEV => self.bus.read_byte(self.registers.get_de()),
                        LoadByteSource::HLI => {
                            let r = self.bus.read_byte(self.registers.get_hl());
                            let n:u16 = self.registers.get_hl().wrapping_add(1);
                            self.registers.set_hl(n);
                            r
                        }
                        LoadByteSource::HLD => {
                            let r = self.bus.read_byte(self.registers.get_hl());
                            let n:u16 = self.registers.get_hl().wrapping_sub(1);
                            self.registers.set_hl(n);
                            r
                        }
                        LoadByteSource::OC => {
                            let u :u16 = 0xFF00;
                            let r = self.bus.read_byte(u.overflowing_add(self.registers.c as u16).0);
                            r
                        }
                        LoadByteSource::OByte => {
                            let u :u16 = 0xFF00;
                            let r = self.bus.read_byte(u.overflowing_add(self.pc).0);
                            r
                        }
                        // TO - DO test this.
                        LoadByteSource::OWord => {
                            let r = self.bus.read_byte(self.read_next_word());
                            r
                        }
                        _ => panic!("Load source error"),
                        
                    };
                    // todo - add timing
                    match target {
                        LoadByteTarget::A => self.registers.a = source_value,
                        LoadByteTarget::B => self.registers.b = source_value,
                        LoadByteTarget::C => self.registers.c = source_value,
                        LoadByteTarget::D => self.registers.d = source_value,
                        LoadByteTarget::E => self.registers.e = source_value,
                        LoadByteTarget::H => self.registers.h = source_value,
                        LoadByteTarget::L => self.registers.l = source_value,
                        LoadByteTarget::HLI | LoadByteTarget::HL |
                        LoadByteTarget::HLD => {
                            self.bus.write_bytes(self.registers.get_hl(), source_value)
                        }
                        LoadByteTarget::BCV => {
                            self.bus.write_bytes(self.registers.get_bc(), source_value)
                        }
                        LoadByteTarget::DEV => {
                            self.bus.write_bytes(self.registers.get_de(), source_value)
                        }
                        LoadByteTarget::OC => {
                            self.bus.write_bytes(0xFF00 + self.registers.c as u16, source_value)
                        }
                        LoadByteTarget::OByte => {
                            self.bus.write_bytes(0xFF0 + (self.bus.read_byte(self.pc) as u16), source_value)
                        }
                        // todo : test this
                        LoadByteTarget::OWord => {
                            self.bus.write_bytes(self.read_next_word(), source_value)
                        }

                        _ => panic!("add more"),
                    }
                    match source {
                        LoadByteSource::D8 => self.pc.wrapping_add(2),
                        _ => self.pc.wrapping_add(1),
                    }
                }
                _ => panic!("Load ERROR"),
            },

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
                    _ => panic!("TODO: support more conditions"),
                };
                self.call(jump_condition)
            }

            Instruction::RET(test) => {
                let jump_condition = match test {
                    JumpTest::NotZero => !self.registers.f.zero,
                    _ => panic!("TODO: support more conditions"),
                };
                self.return_(jump_condition)
            }

            Instruction::NOP => self.pc.wrapping_add(1),

            Instruction::HALT => {
                self.is_halted = true;
                self.pc
            }

            _ => panic!("Support More Languages."),
        }
    }
    // TO- DO : if error check whether all sp adds are correctly done.

    fn call(&mut self, should_jump: bool) -> u16 {
        let next_pc = self.pc.wrapping_add(3);
        if should_jump {
            self.push(next_pc);
            self.read_next_word()
        } else {
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

    fn read_next_byte(&self) -> u8 {
        self.bus.read_byte(self.pc + 1)
    }

    fn read_next_word(&self) -> u16 {
        (self.bus.read_byte(self.pc + 1) as u16) << 8 | self.bus.read_byte(self.pc + 2) as u16
    }

    fn pop(&mut self) -> u16 {
        let lsb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        let msb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        (msb << 8) | lsb
    }

    fn push(&mut self, value: u16) {
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

    fn add(&mut self, reg: u8, val: u8, carry: bool) -> u8 {
        let cy = if carry {
            if self.registers.f.carry == true {
                1
            } else {
                0
            }
        } else {
            0
        };
        let (new_val, over_flowed) = reg.overflowing_add(val);
        self.registers.f = FlagsRegister {
            zero: new_val == 0,
            subtract: false,
            carry: over_flowed,
            half_carry: ((reg & 0xF).overflowing_add(val & 0xF)).0 > 0xF,
        };
        if carry {
            self.add(reg, cy, false)
        } else {
            new_val
        }
    }

    fn add_16bit(&mut self,reg: u16, val: u16) -> u16{
        let (new_val, over_flowed) = reg.overflowing_add(val);
        self.registers.f = FlagsRegister {
            zero: self.registers.f.zero,
            subtract: false,
            carry: over_flowed,
            half_carry: ((reg & 0xFF).overflowing_add(val & 0xFF)).0 > 0xFF,
        };
        new_val
    }


    fn sub(&mut self,reg:u8, val: u8, carry: bool) -> u8 {
        let cy = if carry {
            if self.registers.f.carry == true {
                1
            } else {
                0
            }
        } else {
            0
        };
        let (new_val, over_flowed) = reg.overflowing_sub(val);
        self.registers.f = FlagsRegister {
            zero: new_val == 0,
            subtract: true,
            carry: over_flowed,
            half_carry: ((reg & 0xF).overflowing_sub(val & 0xF)).0 > 0xF,
        };
        if carry {
            self.sub(reg,cy, false)
        } else {
            new_val
        }
    }

    fn and(&mut self, value: u8) -> u8 {
        let new_val = self.registers.a & value;
        self.registers.f = FlagsRegister {
            zero: new_val == 0,
            carry: false,
            half_carry: true,
            subtract: false,
        };
        new_val
    }

    fn or(&mut self, value: u8, exculsive: bool) -> u8 {
        let new_val = if exculsive {
            self.registers.a ^ value
        } else {
            self.registers.a | value
        };
        println!("from or = new_val = {} , 0x{:x}", new_val, new_val);
        self.registers.f = FlagsRegister {
            zero: new_val == 0,
            carry: false,
            half_carry: false,
            subtract: false,
        };
        new_val
    }

    fn jump(&self, should_jump: bool, exception: bool) -> u16 {
        if should_jump & !(exception) {
            // Gameboy is little endian so read pc + 2 as most significant bit
            // and pc + 1 as least significant bit
            let least_significant_byte = self.bus.read_byte(self.pc + 1) as u16;
            let most_significant_byte = self.bus.read_byte(self.pc + 2) as u16;
            (most_significant_byte << 8) | least_significant_byte
        } else if should_jump & exception {
            let least_significant_byte = self.bus.read_byte(self.pc + 1) as u16;
            least_significant_byte
        } else {
            // If we don't jump we need to still move the program
            // counter forward by 3 since the jump instruction is
            // 3 bytes wide (1 byte for tag and 2 bytes for jump address)
            self.pc.wrapping_add(3)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::gpu::{GPU, VRAM::VRAM_SIZE};

    use super::*;

    #[test]
    fn add_checker() {
        let r: Registers = Registers {
            a: 0xff,
            b: 0x0f,
            c: 0x38,
            d: 0,
            e: 0,
            f: FlagsRegister {
                zero: false,
                subtract: false,
                half_carry: false,
                carry: true,
            },
            h: 0,
            l: 0x8a,
        };
        let mem = [1; 0xFFFF];
        let mut c: CPU = CPU {
            clock: Clock { m: 0},
            registers: r,
            bus: MemoryBus {
                memory: mem,
                gpu: GPU {
                    vram: [0; VRAM_SIZE],
                    tile_set: [crate::gpu::empty_tile(); 384],
                },
            },
            sp: 0,
            is_halted: false,
            pc: 0,
            m: 0,
        };
        let b: u8 = c.registers.get_hl() as u8;
        // let a: u8 = c.registers.a;
        // let new_value = c.sub(b, true);
        // c.registers.a = new_value;
        println!("1st {:?}", c);
        let new_value = c.or(b, true);
        c.registers.a = new_value;
        println!("a = 0x{:x} , {}", new_value, new_value);
        println!("2nd {:?}", c);
        assert_eq!(1, 0);
    }
}
