use crate::useful_func::*;
use clock::Clock;
use flags_register::FlagsRegister;
use instruction::Instruction;
use memory_bus::MemoryBus;
use registers::Registers;
use target::*;
use timer::*;

pub mod clock;
pub mod flags_register;
pub mod instruction;
pub mod memory_bus;
pub mod memory_map;
pub mod registers;
pub mod target;
pub mod timer;

#[derive(Debug)]
pub struct CPU {
    clock: Clock,
    pub registers: Registers,
    _rsv: Registers,
    pub bus: MemoryBus,
    pub pc: u16,
    pub sp: u16,
    pub is_halted: bool,
    pub m: u8, // Internal Clock of Last Instruction
    pending_inrerupt_disabled: bool,
    pending_interupt_enabled: bool,
    once: bool,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            clock: Clock::default(),
            registers: Registers::default(),
            _rsv: Registers::default(),
            bus: MemoryBus::new(),
            pc: 0,
            sp: 0,
            is_halted: false,
            m: 0,
            pending_inrerupt_disabled: false,
            pending_interupt_enabled: false,
            once: false,
        }
    }

    pub fn init_game(&mut self) {
        self.reset_cpu();
    }

    pub fn reset_cpu(&mut self) {
        self.m = 0;
        self.pc = 0x0;
        self.registers.set_bc(0x0013);
        self.registers.set_de(0x00D8);
        self.registers.set_hl(0x014D);
        self.is_halted = false;
        self.sp = 0x0;
        self.bus.reset();
    }

    fn _execute(&mut self, instruction: Instruction) -> u16 {
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
                        self.m += 4;
                        value = self.bus.read_byte(self.registers.get_hl());
                    }
                    ArthemeticTarget::D8 => {
                        // TO-DO : Implement correctly
                        self.m += 4;
                        value = self._read_next_byte();
                    }
                    ArthemeticTarget::HLBC => {
                        let new_val = self._add_16bit(self.registers.get_bc(), false);
                        self.registers.set_hl(new_val);
                        self.m += 4;
                        return self.pc.wrapping_add(1);
                    }
                    ArthemeticTarget::HLDE => {
                        let new_val = self._add_16bit(self.registers.get_de(), false);
                        self.registers.set_hl(new_val);
                        self.m += 4;
                        return self.pc.wrapping_add(1);
                    }
                    ArthemeticTarget::HLHL => {
                        let new_val = self._add_16bit(self.registers.get_hl(), false);
                        self.registers.set_hl(new_val);
                        self.m += 4;
                        return self.pc.wrapping_add(1);
                    }
                    ArthemeticTarget::HLSP => {
                        let new_val = self._add_16bit(self.sp, false);
                        self.registers.set_hl(new_val);
                        self.m += 4;
                        return self.pc.wrapping_add(1);
                    }
                    ArthemeticTarget::SP => {
                        // If ERROR CHECK THIS.
                        let b = ((self._read_next_byte() as i8) as i16) as u16;
                        let new_val = self._add_16bit(b, true);
                        self.sp = new_val;
                        self.m += 4;
                        return self.pc.wrapping_add(1);
                    }
                }
                self.registers.a = self._add_8bit(value);
                self.m += 4;
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
                        self.m += 4;
                        value = self.bus.read_byte(self.registers.get_hl());
                    }
                    ArthemeticTarget::D8 => {
                        self.m += 4;
                        value = self._read_next_byte();
                    }

                    _ => panic!("Reached Unreachable code"),
                }
                self.registers.a = self._sub_8bit(value);

                self.m += 4;

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
                        self.m += 4;
                        value = self.bus.read_byte(self.registers.get_hl());
                    }
                    ArthemeticTarget::D8 => {
                        self.m += 4;
                        value = self._read_next_byte();
                    }
                    _ => panic!("Reached Unreachable code"),
                }
                self.registers.a = self._adc(value);
                self.m += 4;

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
                        self.m += 4;
                        value = self.bus.read_byte(self.registers.get_hl());
                    }
                    ArthemeticTarget::D8 => {
                        // TO-DO : Implement correctly
                        self.m += 4;
                        value = self._read_next_byte();
                    }
                    _ => panic!("Reached Unreachable code"),
                }
                self.registers.a = self._sbc(value);
                self.m += 4;

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
                        self.m += 4;
                        value = self.bus.read_byte(self.registers.get_hl());
                    }
                    ArthemeticTarget::D8 => {
                        // TO-DO : Implement correctly
                        self.m += 4;
                        value = self._read_next_byte();
                    }
                    _ => panic!("Reached Unreachable code"),
                }
                self._cp(value);
                self.m += 4;

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
                        self.m += 4;
                        value = self.bus.read_byte(self.registers.get_hl());
                    }
                    ArthemeticTarget::D8 => {
                        // TO-DO : Implement correctly
                        // print!("\n\n pc before reading next byte pc = {:X}\n",self.pc);
                        self.m += 4;
                        value = self._read_next_byte();
                        // print!("\n pc after reading next byte pc = {:X} \n",self.pc);
                    }
                    _ => panic!("Reached Unreachable code at AND Inst."),
                }
                self.registers.a = self._and(value);
                self.m += 4;
                // print!("\n pc after and OP pc = {:X} \n",self.pc);
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
                        self.m += 4;

                        value = self.bus.read_byte(self.registers.get_hl());
                    }
                    ArthemeticTarget::D8 => {
                        // TO-DO : Implement correctly
                        self.m += 4;

                        value = self._read_next_byte();
                    }
                    _ => panic!("Reached Unreachable code"),
                }
                let new_value = self._or(value, false);
                self.m += 4;

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
                        self.m += 4;

                        value = self.bus.read_byte(self.registers.get_hl());
                    }
                    ArthemeticTarget::D8 => {
                        // TO-DO : Implement correctly
                        self.m += 4;

                        value = self._read_next_byte();
                    }
                    _ => panic!("Reached Unreachable code"),
                }
                let new_value = self._or(value, true);
                self.m += 4;

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
                        return self._jump(true, true);
                    }
                    _ => panic!("implement adiitional jumptest"),
                };
                self._jump(jump_condition, false)
            }

            Instruction::INC(target) => {
                match target {
                    IncDecTarget::A => {
                        self.registers.a = self._inc_dec_8bit(self.registers.a, 1);
                    }
                    IncDecTarget::B => {
                        self.registers.b = self._inc_dec_8bit(self.registers.b, 1);
                    }
                    IncDecTarget::C => {
                        self.registers.c = self._inc_dec_8bit(self.registers.c, 1);
                    }
                    IncDecTarget::D => {
                        self.registers.d = self._inc_dec_8bit(self.registers.d, 1);
                    }
                    IncDecTarget::E => {
                        self.registers.e = self._inc_dec_8bit(self.registers.e, 1);
                    }
                    IncDecTarget::H => {
                        self.registers.h = self._inc_dec_8bit(self.registers.h, 1);
                    }
                    IncDecTarget::L => {
                        self.registers.l = self._inc_dec_8bit(self.registers.l, 1);
                    }
                    IncDecTarget::HL2 => {
                        self.m += 8;

                        value = self.bus.read_byte(self.registers.get_hl());
                        value = self._inc_dec_8bit(value, 1);
                        self.bus.write_bytes(self.registers.get_hl(), value)
                    }
                    IncDecTarget::BC => {
                        self.m += 4;
                        self.registers
                            .set_bc(self.registers.get_bc().wrapping_add(1));
                    }

                    IncDecTarget::DE => {
                        self.m += 4;
                        self.registers
                            .set_de(self.registers.get_de().wrapping_add(1));
                    }

                    IncDecTarget::HL => {
                        self.m += 4;
                        self.registers
                            .set_hl(self.registers.get_hl().wrapping_add(1));
                    }

                    IncDecTarget::SP => {
                        self.m += 4;
                        self.sp = self.sp.wrapping_add(1);
                    }
                }
                self.m += 4;

                return self.pc.wrapping_add(1);
            }

            Instruction::DEC(target) => {
                match target {
                    IncDecTarget::A => {
                        self.registers.a = self._inc_dec_8bit(self.registers.a, -1);
                    }
                    IncDecTarget::B => {
                        self.registers.b = self._inc_dec_8bit(self.registers.b, -1);
                    }
                    IncDecTarget::C => {
                        self.registers.c = self._inc_dec_8bit(self.registers.c, -1);
                    }
                    IncDecTarget::D => {
                        self.registers.d = self._inc_dec_8bit(self.registers.d, -1);
                    }
                    IncDecTarget::E => {
                        self.registers.e = self._inc_dec_8bit(self.registers.e, -1);
                    }
                    IncDecTarget::H => {
                        self.registers.h = self._inc_dec_8bit(self.registers.h, -1);
                    }
                    IncDecTarget::L => {
                        self.registers.l = self._inc_dec_8bit(self.registers.l, -1);
                    }
                    IncDecTarget::HL2 => {
                        self.m += 8;

                        value = self.bus.read_byte(self.registers.get_hl());
                        value = self._inc_dec_8bit(value, -1);
                        self.bus.write_bytes(self.registers.get_hl(), value)
                    }
                    IncDecTarget::BC => {
                        self.m += 4;
                        self.registers
                            .set_bc(self.registers.get_bc().wrapping_sub(1));
                    }

                    IncDecTarget::DE => {
                        self.m += 4;
                        self.registers
                            .set_de(self.registers.get_de().wrapping_sub(1));
                    }

                    IncDecTarget::HL => {
                        self.m += 4;
                        self.registers
                            .set_hl(self.registers.get_hl().wrapping_sub(1));
                    }

                    IncDecTarget::SP => {
                        self.m += 4;
                        self.sp = self.sp.wrapping_sub(1);
                    }
                }
                self.m += 4;

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
                        LoadByteSource::HL => {
                            self.m += 4;
                            self.bus.read_byte(self.registers.get_hl())
                        }
                        LoadByteSource::D8 => {
                            self.m += 4;
                            self._read_next_byte()
                        }
                        LoadByteSource::BCV => {
                            self.m += 4;
                            self.bus.read_byte(self.registers.get_bc())
                        }
                        LoadByteSource::DEV => {
                            self.m += 4;
                            self.bus.read_byte(self.registers.get_de())
                        }
                        LoadByteSource::HLI => {
                            self.m += 4;
                            let r = self.bus.read_byte(self.registers.get_hl());
                            let n: u16 = self.registers.get_hl().wrapping_add(1);
                            self.registers.set_hl(n);
                            r
                        }
                        LoadByteSource::HLD => {
                            self.m += 4;
                            let r = self.bus.read_byte(self.registers.get_hl());
                            let n: u16 = self.registers.get_hl().wrapping_sub(1);
                            self.registers.set_hl(n);
                            r
                        }
                        LoadByteSource::OC => {
                            self.m += 4;
                            let u: u16 = 0xFF00;
                            let r = self
                                .bus
                                .read_byte(u.overflowing_add(self.registers.c as u16).0);
                            r
                        }
                        LoadByteSource::OByte => {
                            self.m += 8;
                            let u: u16 = 0xFF00;
                            let b = self._read_next_byte();
                            let r = self.bus.read_byte(u.wrapping_add(b as u16));
                            r
                        }

                        LoadByteSource::OWord => {
                            self.m += 12;
                            let w = self._read_next_word();
                            let r = self.bus.read_byte(w);
                            r
                        }
                    };

                    match target {
                        LoadByteTarget::A => {
                            self.registers.a = source_value;
                            self.m += 4
                        }
                        LoadByteTarget::B => {
                            self.registers.b = source_value;
                            self.m += 4
                        }
                        LoadByteTarget::C => {
                            self.registers.c = source_value;
                            self.m += 4
                        }
                        LoadByteTarget::D => {
                            self.registers.d = source_value;
                            self.m += 4
                        }
                        LoadByteTarget::E => {
                            self.registers.e = source_value;
                            self.m += 4
                        }
                        LoadByteTarget::H => {
                            self.registers.h = source_value;
                            self.m += 4
                        }
                        LoadByteTarget::L => {
                            self.registers.l = source_value;
                            self.m += 4
                        }
                        LoadByteTarget::HL => {
                            self.bus.write_bytes(self.registers.get_hl(), source_value);
                            self.m += 8;
                        }
                        LoadByteTarget::HLI => {
                            self.m += 8;
                            self.bus.write_bytes(self.registers.get_hl(), source_value);
                            let mut b = self.registers.get_hl();
                            b = b.wrapping_add(1);
                            self.registers.set_hl(b);
                        }
                        LoadByteTarget::HLD => {
                            self.m += 8;
                            self.bus.write_bytes(self.registers.get_hl(), source_value);
                            let mut b = self.registers.get_hl();
                            b = b.wrapping_sub(1);
                            self.registers.set_hl(b);
                        }
                        LoadByteTarget::BCV => {
                            self.m += 8;
                            self.bus.write_bytes(self.registers.get_bc(), source_value)
                        }
                        LoadByteTarget::DEV => {
                            self.m += 8;
                            self.bus.write_bytes(self.registers.get_de(), source_value)
                        }
                        LoadByteTarget::OC => {
                            self.m += 8;
                            self.bus
                                .write_bytes(0xFF00 + self.registers.c as u16, source_value);
                        }
                        LoadByteTarget::OByte => {
                            self.m += 12;
                            let b = self._read_next_byte() as u16;
                            self.bus.write_bytes(0xFF00 + b, source_value)
                        }
                        // todo : test this
                        LoadByteTarget::OWord => {
                            self.m += 20;
                            let w = self._read_next_word();
                            self.bus.write_bytes(w, source_value)
                        }
                    }
                    self.pc.wrapping_add(1)
                }
                _ => panic!("Load ERROR"),
            },

            Instruction::LD2(load_type) => match load_type {
                LoadType::Word(target, source) => {
                    let source_value = match source {
                        LoadWordSource::D16 => self._read_next_word(),
                        LoadWordSource::SP => {
                            self.m += 8;
                            self.sp
                        }
                        // DoubtFull
                        LoadWordSource::SPr8 => {
                            let b: i8 = self._read_next_byte() as i8;
                            self.registers.f.half_carry =
                                (self.sp & 0xF).wrapping_add((b as u16) & 0xF) & 0x10 != 0;
                            self.registers.f.carry =
                                ((self.sp & 0xFF).wrapping_add((b as u16) & 0xFF)) & 0x100 != 0;
                            self.registers.f.subtract = false;
                            self.registers.f.zero = false;
                            if b >= 0 {
                                self.sp.wrapping_add(b as u16)
                            } else {
                                self.sp.wrapping_sub((b as i16).abs() as u16)
                            }
                        }
                        LoadWordSource::HL => {
                            self.m = self.m.wrapping_sub(4);
                            self.registers.get_hl()
                        } // _ => panic!("Load source error"),
                    };
                    // todo - add timing
                    match target {
                        LoadWordTarget::BC => self.registers.set_bc(source_value),
                        LoadWordTarget::DE => self.registers.set_de(source_value),
                        LoadWordTarget::HL => self.registers.set_hl(source_value),
                        LoadWordTarget::SP => {
                            self.sp = source_value;
                        }
                        LoadWordTarget::A16 => {
                            let add = self._read_next_word();
                            self.bus.write_bytes(add, (source_value & 0xFF) as u8);
                            self.bus.write_bytes(add + 1, (source_value >> 8) as u8);
                        }
                    }
                    self.m = self.m.wrapping_add(12);
                    self.pc.wrapping_add(1)
                }
                _ => panic!("Load ERROR"),
            },

            Instruction::JR(test) => {
                //tprintln!("Jump Test here. = {}",!self.registers.f.zero);
                let jump_condition = match test {
                    JumpTest::NotZero => !self.registers.f.zero,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::Always => true,
                    _ => panic!("implement adiitional jumptest"),
                };
                self._jump_8bit(jump_condition)
            }

            Instruction::RST(target) => {
                self.m += 16;
                self._push(self.pc.wrapping_add(1));
                match target {
                    RSTTarget::H00 => 0x00 as u16,
                    RSTTarget::H08 => 0x08 as u16,
                    RSTTarget::H10 => 0x10 as u16,
                    RSTTarget::H18 => 0x18 as u16,
                    RSTTarget::H20 => 0x20 as u16,
                    RSTTarget::H28 => 0x28 as u16,
                    RSTTarget::H30 => 0x30 as u16,
                    RSTTarget::H38 => 0x38 as u16,
                }
            }

            Instruction::PUSH(target) => {
                let value = match target {
                    StackTarget::BC => self.registers.get_bc(),
                    StackTarget::DE => self.registers.get_de(),
                    StackTarget::HL => self.registers.get_hl(),
                    StackTarget::AF => self.registers.get_af(),
                };
                self.m += 16;
                self._push(value);
                self.pc.wrapping_add(1)
            }

            Instruction::POP(target) => {
                let result = self._pop();
                match target {
                    StackTarget::BC => self.registers.set_bc(result),
                    StackTarget::DE => self.registers.set_de(result),
                    StackTarget::HL => self.registers.set_hl(result),
                    StackTarget::AF => self.registers.set_af(result),
                }
                self.m += 12;
                self.pc.wrapping_add(1)
            }

            Instruction::CALL(test) => {
                let jump_condition = match test {
                    JumpTest::NotZero => !self.registers.f.zero,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Always => true,
                    _ => panic!("TODO: support more conditions"),
                };
                self._call(jump_condition)
            }

            Instruction::RET(test) => {
                let jump_condition = match test {
                    JumpTest::NotZero => !self.registers.f.zero,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::Always => true,
                    JumpTest::I => {
                        self.bus.interupt_master = true;
                        true
                    }
                    _ => panic!("TODO: support more conditions"),
                };
                self._return_(jump_condition)
            }

            Instruction::NOP => {
                self.m += 4;
                self.pc.wrapping_add(1)
            }
            Instruction::HALT => {
                self.m += 4;
                self.is_halted = true;
                self.pc.wrapping_add(1)
            }

            Instruction::CCF => {
                self.registers.f.carry = !self.registers.f.carry;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                self.m += 4;
                self.pc.wrapping_add(1)
            }
            Instruction::SCF => {
                self.registers.f.carry = true;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                self.m += 4;
                self.pc.wrapping_add(1)
            }
            Instruction::RRA => {
                let mut reg = self.registers.a;
                let bit0 = reg & 1;
                let cy = if self.registers.f.carry { 1 } else { 0 };
                reg >>= 1;
                reg += cy << 7;

                self.registers.f.carry = if bit0 == 1 { true } else { false };
                self.registers.f.half_carry = false;
                self.registers.f.subtract = false;
                self.registers.f.zero = false;

                self.registers.a = reg;
                self.m += 4;
                self.pc.wrapping_add(1)
            }

            Instruction::RRCA => {
                let mut reg = self.registers.a;
                let bit0 = reg & 1;
                reg >>= 1;
                reg += bit0 << 7;

                self.registers.f.carry = if bit0 == 1 { true } else { false };

                self.registers.f.zero = false;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;

                self.registers.a = reg;

                self.m += 4;
                self.pc.wrapping_add(1)
            }

            Instruction::RLA => {
                let mut reg = self.registers.a;
                let bit7 = reg >> 7;
                let cy = if self.registers.f.carry { 1 } else { 0 };
                reg <<= 1;
                reg += cy;

                self.registers.f.carry = if bit7 == 1 { true } else { false };
                self.registers.f.half_carry = false;
                self.registers.f.subtract = false;
                self.registers.f.zero = false;

                self.registers.a = reg;

                self.m += 4;
                self.pc.wrapping_add(1)
            }

            Instruction::RLCA => {
                let bit7 = self.registers.a >> 7;
                self.registers.a = (self.registers.a << 1).wrapping_add(bit7);
                self.registers.f.carry = if bit7 != 0 { true } else { false };

                self.registers.f.half_carry = false;
                self.registers.f.subtract = false;
                self.registers.f.zero = false;

                self.m += 4;
                self.pc.wrapping_add(1)
            }

            Instruction::CPL => {
                self.registers.a = !(self.registers.a);

                self.registers.f.half_carry = true;
                self.registers.f.subtract = true;

                self.m += 4;
                self.pc.wrapping_add(1)
            }

            // DAA = it will convert binary to decimal basically uses carry and half-carry and and six respectively
            Instruction::DAA => {
                if self.registers.f.subtract {
                    if self.registers.f.carry {
                        self.registers.a = self.registers.a.wrapping_sub(0x60);
                    }
                    if self.registers.f.half_carry {
                        self.registers.a = self.registers.a.wrapping_sub(0x06);
                    }
                } else {
                    if self.registers.a > 0x99 || self.registers.f.carry {
                        self.registers.a = self.registers.a.wrapping_add(0x60);
                        self.registers.f.carry = true;
                    }
                    if (self.registers.a & 0x0F) > 0x09 || self.registers.f.half_carry {
                        self.registers.a = self.registers.a.wrapping_add(0x06);
                    }
                }

                self.registers.f.zero = self.registers.a == 0;
                self.registers.f.half_carry = false;

                self.m += 4;
                self.pc.wrapping_add(1)
            }

            Instruction::STOP => {
                println!("Executing Stop Inst.");
                self.m += 4;
                self._stop();
                self.pc.wrapping_add(1)
            }

            Instruction::DI => {
                // println!("called DI at pc = {}",self.pc);
                self.m += 4;
                // self.bus.interupt_master = false;
                self.pending_inrerupt_disabled = true;
                self.pc.wrapping_add(1)
            }

            Instruction::EI => {
                // println!("called EI at pc = {}",self.pc);
                self.pending_interupt_enabled = true;
                // self.bus.interupt_master = true;
                self.m += 4;
                self.pc.wrapping_add(1)
            }

            Instruction::RLC(target) => {
                match target {
                    PrefixTarget::B => {
                        let mut reg = self.registers.b;
                        let bit7 = reg >> 7;
                        reg <<= 1;
                        reg = reg.wrapping_add(bit7);

                        self.registers.f.carry = if bit7 != 0 { true } else { false };
                        self.registers.f.half_carry = false;
                        self.registers.f.subtract = false;
                        self.registers.f.zero = reg == 0;

                        self.registers.b = reg;
                    }
                    PrefixTarget::C => {
                        let mut reg = self.registers.c;
                        let bit7 = reg >> 7;
                        reg <<= 1;
                        reg = reg.wrapping_add(bit7);

                        self.registers.f.carry = if bit7 != 0 { true } else { false };
                        self.registers.f.half_carry = false;
                        self.registers.f.subtract = false;
                        self.registers.f.zero = reg == 0;

                        self.registers.c = reg;
                    }
                    PrefixTarget::D => {
                        let mut reg = self.registers.d;
                        let bit7 = reg >> 7;
                        reg <<= 1;
                        reg = reg.wrapping_add(bit7);

                        self.registers.f.carry = if bit7 != 0 { true } else { false };
                        self.registers.f.half_carry = false;
                        self.registers.f.subtract = false;
                        self.registers.f.zero = reg == 0;

                        self.registers.d = reg;
                    }
                    PrefixTarget::E => {
                        let mut reg = self.registers.e;
                        let bit7 = reg >> 7;
                        reg <<= 1;
                        reg = reg.wrapping_add(bit7);

                        self.registers.f.carry = if bit7 != 0 { true } else { false };
                        self.registers.f.half_carry = false;
                        self.registers.f.subtract = false;
                        self.registers.f.zero = reg == 0;

                        self.registers.e = reg;
                    }
                    PrefixTarget::H => {
                        let mut reg = self.registers.h;
                        let bit7 = reg >> 7;
                        reg <<= 1;
                        reg = reg.wrapping_add(bit7);

                        self.registers.f.carry = if bit7 != 0 { true } else { false };
                        self.registers.f.half_carry = false;
                        self.registers.f.subtract = false;
                        self.registers.f.zero = reg == 0;

                        self.registers.h = reg;
                    }
                    PrefixTarget::L => {
                        let mut reg = self.registers.l;
                        let bit7 = reg >> 7;
                        reg <<= 1;
                        reg = reg.wrapping_add(bit7);

                        self.registers.f.carry = if bit7 != 0 { true } else { false };
                        self.registers.f.half_carry = false;
                        self.registers.f.subtract = false;
                        self.registers.f.zero = reg == 0;

                        self.registers.l = reg;
                    }
                    PrefixTarget::A => {
                        let mut reg = self.registers.a;
                        let bit7 = reg >> 7;
                        reg <<= 1;
                        reg = reg.wrapping_add(bit7);

                        self.registers.f.carry = if bit7 != 0 { true } else { false };
                        self.registers.f.half_carry = false;
                        self.registers.f.subtract = false;
                        self.registers.f.zero = reg == 0;

                        self.registers.a = reg;
                    }
                    PrefixTarget::HLV => {
                        let mut hlv = self.bus.read_byte(self.registers.get_hl());
                        let bit7 = hlv >> 7;
                        hlv = (hlv << 1).wrapping_add(bit7);
                        self.registers.f.carry = if bit7 != 0 { true } else { false };
                        self.registers.f.zero = hlv == 0;
                        self.registers.f.half_carry = false;
                        self.registers.f.subtract = false;
                        self.bus.write_bytes(self.registers.get_hl(), hlv);
                        self.m += 8;
                    }
                }
                self.m += 8;
                self.pc.wrapping_add(1)
            }

            Instruction::RL(target) => {
                match target {
                    PrefixTarget::B => {
                        let mut reg = self.registers.b;
                        let bit7 = reg >> 7;
                        let cy = if self.registers.f.carry { 1 } else { 0 };
                        reg <<= 1;
                        reg += cy;

                        self.registers.f.carry = if bit7 == 1 { true } else { false };
                        self.registers.f.half_carry = false;
                        self.registers.f.subtract = false;
                        self.registers.f.zero = reg == 0;

                        self.registers.b = reg;
                    }
                    PrefixTarget::C => {
                        let mut reg = self.registers.c;
                        let bit7 = reg >> 7;
                        let cy = if self.registers.f.carry { 1 } else { 0 };
                        reg <<= 1;
                        reg += cy;

                        self.registers.f.carry = if bit7 == 1 { true } else { false };
                        self.registers.f.half_carry = false;
                        self.registers.f.subtract = false;
                        self.registers.f.zero = reg == 0;

                        self.registers.c = reg;
                    }
                    PrefixTarget::D => {
                        let mut reg = self.registers.d;
                        let bit7 = reg >> 7;
                        let cy = if self.registers.f.carry { 1 } else { 0 };
                        reg <<= 1;
                        reg += cy;

                        self.registers.f.carry = if bit7 == 1 { true } else { false };
                        self.registers.f.half_carry = false;
                        self.registers.f.subtract = false;
                        self.registers.f.zero = reg == 0;

                        self.registers.d = reg;
                    }

                    PrefixTarget::E => {
                        let mut reg = self.registers.e;
                        let bit7 = reg >> 7;
                        let cy = if self.registers.f.carry { 1 } else { 0 };
                        reg <<= 1;
                        reg += cy;

                        self.registers.f.carry = if bit7 == 1 { true } else { false };
                        self.registers.f.half_carry = false;
                        self.registers.f.subtract = false;
                        self.registers.f.zero = reg == 0;

                        self.registers.e = reg;
                    }

                    PrefixTarget::H => {
                        let mut reg = self.registers.h;
                        let bit7 = reg >> 7;
                        let cy = if self.registers.f.carry { 1 } else { 0 };
                        reg <<= 1;
                        reg += cy;

                        self.registers.f.carry = if bit7 == 1 { true } else { false };
                        self.registers.f.half_carry = false;
                        self.registers.f.subtract = false;
                        self.registers.f.zero = reg == 0;

                        self.registers.h = reg;
                    }

                    PrefixTarget::L => {
                        let mut reg = self.registers.l;
                        let bit7 = reg >> 7;
                        let cy = if self.registers.f.carry { 1 } else { 0 };
                        reg <<= 1;
                        reg += cy;

                        self.registers.f.carry = if bit7 == 1 { true } else { false };
                        self.registers.f.half_carry = false;
                        self.registers.f.subtract = false;
                        self.registers.f.zero = reg == 0;

                        self.registers.l = reg;
                    }

                    PrefixTarget::A => {
                        let mut reg = self.registers.a;
                        let bit7 = reg >> 7;
                        let cy = if self.registers.f.carry { 1 } else { 0 };
                        reg <<= 1;
                        reg += cy;

                        self.registers.f.carry = if bit7 == 1 { true } else { false };
                        self.registers.f.half_carry = false;
                        self.registers.f.subtract = false;
                        self.registers.f.zero = reg == 0;

                        self.registers.a = reg;
                    }

                    PrefixTarget::HLV => {
                        let mut reg = self.bus.read_byte(self.registers.get_hl());
                        let bit7 = reg >> 7;
                        let cy = if self.registers.f.carry { 1 } else { 0 };
                        reg <<= 1;
                        reg += cy;

                        self.registers.f.carry = if bit7 == 1 { true } else { false };
                        self.registers.f.half_carry = false;
                        self.registers.f.subtract = false;
                        self.registers.f.zero = reg == 0;
                        self.m += 8;

                        self.bus.write_bytes(self.registers.get_hl(), reg);
                    }
                }
                self.m += 8;
                self.pc.wrapping_add(1)
            }

            Instruction::RRC(target) => {
                match target {
                    PrefixTarget::B => {
                        let mut reg = self.registers.b;
                        let bit0 = reg & 1;
                        reg >>= 1;
                        reg += bit0 << 7;

                        self.registers.f.carry = if bit0 == 1 { true } else { false };

                        self.registers.f.zero = reg == 0;
                        self.registers.f.subtract = false;
                        self.registers.f.half_carry = false;

                        self.registers.b = reg;
                    }
                    PrefixTarget::C => {
                        let mut reg = self.registers.c;
                        let bit0 = reg & 1;
                        reg >>= 1;
                        reg += bit0 << 7;

                        self.registers.f.carry = if bit0 == 1 { true } else { false };

                        self.registers.f.zero = reg == 0;
                        self.registers.f.subtract = false;
                        self.registers.f.half_carry = false;

                        self.registers.c = reg;
                    }
                    PrefixTarget::D => {
                        let mut reg = self.registers.d;
                        let bit0 = reg & 1;
                        reg >>= 1;
                        reg += bit0 << 7;

                        self.registers.f.carry = if bit0 == 1 { true } else { false };

                        self.registers.f.zero = reg == 0;
                        self.registers.f.subtract = false;
                        self.registers.f.half_carry = false;

                        self.registers.d = reg;
                    }
                    PrefixTarget::E => {
                        let mut reg = self.registers.e;
                        let bit0 = reg & 1;
                        reg >>= 1;
                        reg += bit0 << 7;

                        self.registers.f.carry = if bit0 == 1 { true } else { false };

                        self.registers.f.zero = reg == 0;
                        self.registers.f.subtract = false;
                        self.registers.f.half_carry = false;

                        self.registers.e = reg;
                    }
                    PrefixTarget::H => {
                        let mut reg = self.registers.h;
                        let bit0 = reg & 1;
                        reg >>= 1;
                        reg += bit0 << 7;

                        self.registers.f.carry = if bit0 == 1 { true } else { false };

                        self.registers.f.zero = reg == 0;
                        self.registers.f.subtract = false;
                        self.registers.f.half_carry = false;

                        self.registers.h = reg;
                    }
                    PrefixTarget::L => {
                        let mut reg = self.registers.l;
                        let bit0 = reg & 1;
                        reg >>= 1;
                        reg += bit0 << 7;

                        self.registers.f.carry = if bit0 == 1 { true } else { false };

                        self.registers.f.zero = reg == 0;
                        self.registers.f.subtract = false;
                        self.registers.f.half_carry = false;

                        self.registers.l = reg;
                    }
                    PrefixTarget::A => {
                        let mut reg = self.registers.a;
                        let bit0 = reg & 1;
                        reg >>= 1;
                        reg += bit0 << 7;

                        self.registers.f.carry = if bit0 == 1 { true } else { false };

                        self.registers.f.zero = reg == 0;
                        self.registers.f.subtract = false;
                        self.registers.f.half_carry = false;

                        self.registers.a = reg;
                    }
                    PrefixTarget::HLV => {
                        let mut reg = self.bus.read_byte(self.registers.get_hl());
                        let bit0 = reg & 1;
                        reg >>= 1;
                        reg += bit0 << 7;

                        self.registers.f.carry = if bit0 == 1 { true } else { false };

                        self.registers.f.zero = reg == 0;
                        self.registers.f.subtract = false;
                        self.registers.f.half_carry = false;

                        self.m += 8;

                        self.bus.write_bytes(self.registers.get_hl(), reg);
                    }
                }
                self.m += 8;
                self.pc.wrapping_add(1)
            }

            Instruction::RR(target) => {
                match target {
                    PrefixTarget::B => {
                        let mut reg = self.registers.b;
                        let bit0 = reg & 1;
                        let cy = if self.registers.f.carry { 1 } else { 0 };
                        reg >>= 1;
                        reg += cy << 7;

                        self.registers.f.carry = if bit0 == 1 { true } else { false };
                        self.registers.f.half_carry = false;
                        self.registers.f.subtract = false;
                        self.registers.f.zero = reg == 0;

                        self.registers.b = reg;
                    }
                    PrefixTarget::C => {
                        let mut reg = self.registers.c;
                        let bit0 = reg & 1;
                        let cy = if self.registers.f.carry { 1 } else { 0 };
                        reg >>= 1;
                        reg += cy << 7;

                        self.registers.f.carry = if bit0 == 1 { true } else { false };
                        self.registers.f.half_carry = false;
                        self.registers.f.subtract = false;
                        self.registers.f.zero = reg == 0;

                        self.registers.c = reg;
                    }
                    PrefixTarget::D => {
                        let mut reg = self.registers.d;
                        let bit0 = reg & 1;
                        let cy = if self.registers.f.carry { 1 } else { 0 };
                        reg >>= 1;
                        reg += cy << 7;

                        self.registers.f.carry = if bit0 == 1 { true } else { false };
                        self.registers.f.half_carry = false;
                        self.registers.f.subtract = false;
                        self.registers.f.zero = reg == 0;

                        self.registers.d = reg;
                    }

                    PrefixTarget::E => {
                        let mut reg = self.registers.e;
                        let bit0 = reg & 1;
                        let cy = if self.registers.f.carry { 1 } else { 0 };
                        reg >>= 1;
                        reg += cy << 7;

                        self.registers.f.carry = if bit0 == 1 { true } else { false };
                        self.registers.f.half_carry = false;
                        self.registers.f.subtract = false;
                        self.registers.f.zero = reg == 0;

                        self.registers.e = reg;
                    }

                    PrefixTarget::H => {
                        let mut reg = self.registers.h;
                        let bit0 = reg & 1;
                        let cy = if self.registers.f.carry { 1 } else { 0 };
                        reg >>= 1;
                        reg += cy << 7;

                        self.registers.f.carry = if bit0 == 1 { true } else { false };
                        self.registers.f.half_carry = false;
                        self.registers.f.subtract = false;
                        self.registers.f.zero = reg == 0;

                        self.registers.h = reg;
                    }

                    PrefixTarget::L => {
                        let mut reg = self.registers.l;
                        let bit0 = reg & 1;
                        let cy = if self.registers.f.carry { 1 } else { 0 };
                        reg >>= 1;
                        reg += cy << 7;

                        self.registers.f.carry = if bit0 == 1 { true } else { false };
                        self.registers.f.half_carry = false;
                        self.registers.f.subtract = false;
                        self.registers.f.zero = reg == 0;

                        self.registers.l = reg;
                    }

                    PrefixTarget::A => {
                        let mut reg = self.registers.a;
                        let bit0 = reg & 1;
                        let cy = if self.registers.f.carry { 1 } else { 0 };
                        reg >>= 1;
                        reg += cy << 7;

                        self.registers.f.carry = if bit0 == 1 { true } else { false };
                        self.registers.f.half_carry = false;
                        self.registers.f.subtract = false;
                        self.registers.f.zero = reg == 0;

                        self.registers.a = reg;
                    }

                    PrefixTarget::HLV => {
                        let mut reg = self.bus.read_byte(self.registers.get_hl());
                        let bit0 = reg & 1;
                        let cy = if self.registers.f.carry { 1 } else { 0 };
                        reg >>= 1;
                        reg += cy << 7;

                        self.registers.f.carry = if bit0 == 1 { true } else { false };
                        self.registers.f.half_carry = false;
                        self.registers.f.subtract = false;
                        self.registers.f.zero = reg == 0;

                        self.bus.write_bytes(self.registers.get_hl(), reg);
                        self.m += 8;
                    }
                }
                self.m += 8;
                self.pc.wrapping_add(1)
            }

            Instruction::SLA(target) => {
                match target {
                    PrefixTarget::B => {
                        let mut reg = self.registers.b;
                        let bit7 = reg >> 7;
                        reg <<= 1;

                        self.registers.f.carry = if bit7 == 1 { true } else { false };
                        self.registers.f.half_carry = false;
                        self.registers.f.subtract = false;
                        self.registers.f.zero = reg == 0;

                        self.registers.b = reg;
                    }
                    PrefixTarget::C => {
                        let mut reg = self.registers.c;
                        let bit7 = reg >> 7;
                        reg <<= 1;

                        self.registers.f.carry = if bit7 == 1 { true } else { false };
                        self.registers.f.half_carry = false;
                        self.registers.f.subtract = false;
                        self.registers.f.zero = reg == 0;

                        self.registers.c = reg;
                    }
                    PrefixTarget::D => {
                        let mut reg = self.registers.d;
                        let bit7 = reg >> 7;
                        reg <<= 1;

                        self.registers.f.carry = if bit7 == 1 { true } else { false };
                        self.registers.f.half_carry = false;
                        self.registers.f.subtract = false;
                        self.registers.f.zero = reg == 0;

                        self.registers.d = reg;
                    }
                    PrefixTarget::E => {
                        let mut reg = self.registers.e;
                        let bit7 = reg >> 7;
                        reg <<= 1;

                        self.registers.f.carry = if bit7 == 1 { true } else { false };
                        self.registers.f.half_carry = false;
                        self.registers.f.subtract = false;
                        self.registers.f.zero = reg == 0;

                        self.registers.e = reg;
                    }
                    PrefixTarget::H => {
                        let mut reg = self.registers.h;
                        let bit7 = reg >> 7;
                        reg <<= 1;

                        self.registers.f.carry = if bit7 == 1 { true } else { false };
                        self.registers.f.half_carry = false;
                        self.registers.f.subtract = false;
                        self.registers.f.zero = reg == 0;

                        self.registers.h = reg;
                    }
                    PrefixTarget::L => {
                        let mut reg = self.registers.l;
                        let bit7 = reg >> 7;
                        reg <<= 1;

                        self.registers.f.carry = if bit7 == 1 { true } else { false };
                        self.registers.f.half_carry = false;
                        self.registers.f.subtract = false;
                        self.registers.f.zero = reg == 0;

                        self.registers.l = reg;
                    }
                    PrefixTarget::A => {
                        let mut reg = self.registers.a;
                        let bit7 = reg >> 7;
                        reg <<= 1;

                        self.registers.f.carry = if bit7 == 1 { true } else { false };
                        self.registers.f.half_carry = false;
                        self.registers.f.subtract = false;
                        self.registers.f.zero = reg == 0;

                        self.registers.a = reg;
                    }
                    PrefixTarget::HLV => {
                        let mut reg = self.bus.read_byte(self.registers.get_hl());
                        let bit7 = reg >> 7;
                        reg <<= 1;

                        self.registers.f.carry = if bit7 == 1 { true } else { false };
                        self.registers.f.half_carry = false;
                        self.registers.f.subtract = false;
                        self.registers.f.zero = reg == 0;

                        self.bus.write_bytes(self.registers.get_hl(), reg);
                        self.m += 8;
                    }
                }
                self.m += 8;
                self.pc.wrapping_add(1)
            }

            Instruction::SRL(target) => {
                match target {
                    PrefixTarget::B => {
                        let mut reg = self.registers.b;
                        let bit0 = reg & 1;
                        reg >>= 1;

                        self.registers.f.carry = if bit0 == 1 { true } else { false };
                        self.registers.f.half_carry = false;
                        self.registers.f.subtract = false;
                        self.registers.f.zero = reg == 0;

                        self.registers.b = reg;
                    }
                    PrefixTarget::C => {
                        let mut reg = self.registers.c;
                        let bit0 = reg & 1;
                        reg >>= 1;

                        self.registers.f.carry = if bit0 == 1 { true } else { false };
                        self.registers.f.half_carry = false;
                        self.registers.f.subtract = false;
                        self.registers.f.zero = reg == 0;

                        self.registers.c = reg;
                    }
                    PrefixTarget::D => {
                        let mut reg = self.registers.d;
                        let bit0 = reg & 1;
                        reg >>= 1;

                        self.registers.f.carry = if bit0 == 1 { true } else { false };
                        self.registers.f.half_carry = false;
                        self.registers.f.subtract = false;
                        self.registers.f.zero = reg == 0;

                        self.registers.d = reg;
                    }
                    PrefixTarget::E => {
                        let mut reg = self.registers.e;
                        let bit0 = reg & 1;
                        reg >>= 1;

                        self.registers.f.carry = if bit0 == 1 { true } else { false };
                        self.registers.f.half_carry = false;
                        self.registers.f.subtract = false;
                        self.registers.f.zero = reg == 0;

                        self.registers.e = reg;
                    }
                    PrefixTarget::H => {
                        let mut reg = self.registers.h;
                        let bit0 = reg & 1;
                        reg >>= 1;

                        self.registers.f.carry = if bit0 == 1 { true } else { false };
                        self.registers.f.half_carry = false;
                        self.registers.f.subtract = false;
                        self.registers.f.zero = reg == 0;

                        self.registers.h = reg;
                    }
                    PrefixTarget::L => {
                        let mut reg = self.registers.l;
                        let bit0 = reg & 1;
                        reg >>= 1;

                        self.registers.f.carry = if bit0 == 1 { true } else { false };
                        self.registers.f.half_carry = false;
                        self.registers.f.subtract = false;
                        self.registers.f.zero = reg == 0;

                        self.registers.l = reg;
                    }
                    PrefixTarget::A => {
                        let mut reg = self.registers.a;
                        let bit0 = reg & 1;
                        reg >>= 1;

                        self.registers.f.carry = if bit0 == 1 { true } else { false };
                        self.registers.f.half_carry = false;
                        self.registers.f.subtract = false;
                        self.registers.f.zero = reg == 0;

                        self.registers.a = reg;
                    }
                    PrefixTarget::HLV => {
                        let mut reg = self.bus.read_byte(self.registers.get_hl());
                        let bit0 = reg & 1;
                        reg >>= 1;

                        self.registers.f.carry = if bit0 == 1 { true } else { false };
                        self.registers.f.half_carry = false;
                        self.registers.f.subtract = false;
                        self.registers.f.zero = reg == 0;

                        self.bus.write_bytes(self.registers.get_hl(), reg);
                        self.m += 8;
                    }
                }
                self.m += 8;
                self.pc.wrapping_add(1)
            }

            Instruction::SWAP(target) => {
                match target {
                    PrefixTarget::A => {
                        let upper = (self.registers.a & 0xF0) >> 4;
                        let down = (self.registers.a & 0xF) << 4;
                        self.registers.a = down | upper;
                        self.registers.f.zero = if self.registers.a == 0 { true } else { false };
                    }
                    PrefixTarget::B => {
                        let upper = (self.registers.b & 0xF0) >> 4;
                        let down = (self.registers.b & 0xF) << 4;
                        self.registers.b = down | upper;
                        self.registers.f.zero = if self.registers.b == 0 { true } else { false };
                    }
                    PrefixTarget::C => {
                        let upper = (self.registers.c & 0xF0) >> 4;
                        let down = (self.registers.c & 0xF) << 4;
                        self.registers.c = down | upper;
                        self.registers.f.zero = if self.registers.c == 0 { true } else { false };
                    }
                    PrefixTarget::D => {
                        let upper = (self.registers.d & 0xF0) >> 4;
                        let down = (self.registers.d & 0xF) << 4;
                        self.registers.d = down | upper;
                        self.registers.f.zero = if self.registers.d == 0 { true } else { false };
                    }
                    PrefixTarget::E => {
                        let upper = (self.registers.e & 0xF0) >> 4;
                        let down = (self.registers.e & 0xF) << 4;
                        self.registers.e = down | upper;
                        self.registers.f.zero = if self.registers.e == 0 { true } else { false };
                    }
                    PrefixTarget::H => {
                        let upper = (self.registers.h & 0xF0) >> 4;
                        let down = (self.registers.h & 0xF) << 4;
                        self.registers.h = down | upper;
                        self.registers.f.zero = if self.registers.h == 0 { true } else { false };
                    }
                    PrefixTarget::L => {
                        let upper = (self.registers.l & 0xF0) >> 4;
                        let down = (self.registers.l & 0xF) << 4;
                        self.registers.l = down | upper;
                        self.registers.f.zero = if self.registers.l == 0 { true } else { false };
                    }
                    PrefixTarget::HLV => {
                        let mut value = self.bus.read_byte(self.registers.get_hl());
                        let upper = (value & 0xF0) >> 4;
                        let down = (value & 0xF) << 4;
                        value = down | upper;
                        self.bus.write_bytes(self.registers.get_hl(), value);
                        self.registers.f.zero = if value == 0 { true } else { false };
                        self.m += 8;
                    }
                }
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = false;
                self.m += 8;
                self.pc.wrapping_add(1)
            }

            Instruction::SRA(target) => {
                match target {
                    PrefixTarget::A => {
                        let mut reg = self.registers.a;
                        let bit0 = reg & 1;
                        let bit7 = reg >> 7;
                        reg >>= 1;
                        reg += bit7 << 7;

                        self.registers.f.carry = if bit0 == 1 { true } else { false };
                        self.registers.f.half_carry = false;
                        self.registers.f.subtract = false;
                        self.registers.f.zero = reg == 0;

                        self.registers.a = reg;
                    }
                    PrefixTarget::B => {
                        let mut reg = self.registers.b;
                        let bit0 = reg & 1;
                        let bit7 = reg >> 7;
                        reg >>= 1;
                        reg += bit7 << 7;

                        self.registers.f.carry = if bit0 == 1 { true } else { false };
                        self.registers.f.half_carry = false;
                        self.registers.f.subtract = false;
                        self.registers.f.zero = reg == 0;

                        self.registers.b = reg;
                    }
                    PrefixTarget::C => {
                        let mut reg = self.registers.c;
                        let bit0 = reg & 1;
                        let bit7 = reg >> 7;
                        reg >>= 1;
                        reg += bit7 << 7;

                        self.registers.f.carry = if bit0 == 1 { true } else { false };
                        self.registers.f.half_carry = false;
                        self.registers.f.subtract = false;
                        self.registers.f.zero = reg == 0;

                        self.registers.c = reg;
                    }
                    PrefixTarget::D => {
                        let mut reg = self.registers.d;
                        let bit0 = reg & 1;
                        let bit7 = reg >> 7;
                        reg >>= 1;
                        reg += bit7 << 7;

                        self.registers.f.carry = if bit0 == 1 { true } else { false };
                        self.registers.f.half_carry = false;
                        self.registers.f.subtract = false;
                        self.registers.f.zero = reg == 0;

                        self.registers.d = reg;
                    }
                    PrefixTarget::E => {
                        let mut reg = self.registers.e;
                        let bit0 = reg & 1;
                        let bit7 = reg >> 7;
                        reg >>= 1;
                        reg += bit7 << 7;

                        self.registers.f.carry = if bit0 == 1 { true } else { false };
                        self.registers.f.half_carry = false;
                        self.registers.f.subtract = false;
                        self.registers.f.zero = reg == 0;

                        self.registers.e = reg;
                    }
                    PrefixTarget::H => {
                        let mut reg = self.registers.h;
                        let bit0 = reg & 1;
                        let bit7 = reg >> 7;
                        reg >>= 1;
                        reg += bit7 << 7;

                        self.registers.f.carry = if bit0 == 1 { true } else { false };
                        self.registers.f.half_carry = false;
                        self.registers.f.subtract = false;
                        self.registers.f.zero = reg == 0;

                        self.registers.h = reg;
                    }
                    PrefixTarget::L => {
                        let mut reg = self.registers.l;
                        let bit0 = reg & 1;
                        let bit7 = reg >> 7;
                        reg >>= 1;
                        reg += bit7 << 7;

                        self.registers.f.carry = if bit0 == 1 { true } else { false };
                        self.registers.f.half_carry = false;
                        self.registers.f.subtract = false;
                        self.registers.f.zero = reg == 0;

                        self.registers.l = reg;
                    }
                    PrefixTarget::HLV => {
                        let mut reg = self.bus.read_byte(self.registers.get_hl());
                        let bit0 = reg & 1;
                        let bit7 = reg >> 7;
                        reg >>= 1;
                        reg += bit7 << 7;

                        self.registers.f.carry = if bit0 == 1 { true } else { false };
                        self.registers.f.half_carry = false;
                        self.registers.f.subtract = false;
                        self.registers.f.zero = reg == 0;

                        self.m += 8;
                        self.bus.write_bytes(self.registers.get_hl(), reg);
                    }
                }
                self.m += 8;
                self.pc.wrapping_add(1)
            }

            Instruction::BIT(BitManipulationType::Bit(target_bit, source_register)) => {
                let source = match source_register {
                    SourceRegister::A => self.registers.a,
                    SourceRegister::B => self.registers.b,
                    SourceRegister::C => self.registers.c,
                    SourceRegister::D => self.registers.d,
                    SourceRegister::E => self.registers.e,
                    SourceRegister::H => self.registers.h,
                    SourceRegister::L => self.registers.l,
                    SourceRegister::HLV => {
                        self.m += 4;
                        self.bus.read_byte(self.registers.get_hl())
                    }
                };
                let mask = match target_bit {
                    TargetBit::B0 => source & (1 << 0),
                    TargetBit::B1 => source & (1 << 1),
                    TargetBit::B2 => source & (1 << 2),
                    TargetBit::B3 => source & (1 << 3),
                    TargetBit::B4 => source & (1 << 4),
                    TargetBit::B5 => source & (1 << 5),
                    TargetBit::B6 => source & (1 << 6),
                    TargetBit::B7 => source & (1 << 7),
                };
                self.registers.f.half_carry = true;
                self.registers.f.subtract = false;
                self.registers.f.zero = mask == 0;
                self.m += 8;
                self.pc.wrapping_add(1)
            }

            Instruction::SET(BitManipulationType::Bit(target_bit, source_register)) => {
                let mut source = match source_register {
                    SourceRegister::A => self.registers.a,
                    SourceRegister::B => self.registers.b,
                    SourceRegister::C => self.registers.c,
                    SourceRegister::D => self.registers.d,
                    SourceRegister::E => self.registers.e,
                    SourceRegister::H => self.registers.h,
                    SourceRegister::L => self.registers.l,
                    SourceRegister::HLV => {
                        self.m += 8;
                        self.bus.read_byte(self.registers.get_hl())
                    }
                };
                source = match target_bit {
                    TargetBit::B0 => source | 0b_0000_0001,
                    TargetBit::B1 => source | 0b_0000_0010,
                    TargetBit::B2 => source | 0b_0000_0100,
                    TargetBit::B3 => source | 0b_0000_1000,
                    TargetBit::B4 => source | 0b_0001_0000,
                    TargetBit::B5 => source | 0b_0010_0000,
                    TargetBit::B6 => source | 0b_0100_0000,
                    TargetBit::B7 => source | 0b_1000_0000,
                };
                match source_register {
                    SourceRegister::A => self.registers.a = source,
                    SourceRegister::B => self.registers.b = source,
                    SourceRegister::C => self.registers.c = source,
                    SourceRegister::D => self.registers.d = source,
                    SourceRegister::E => self.registers.e = source,
                    SourceRegister::H => self.registers.h = source,
                    SourceRegister::L => self.registers.l = source,
                    SourceRegister::HLV => self.bus.write_bytes(self.registers.get_hl(), source),
                };
                self.m += 8;
                self.pc.wrapping_add(1)
            }

            Instruction::RES(BitManipulationType::Bit(target_bit, source_register)) => {
                let mut source = match source_register {
                    SourceRegister::A => self.registers.a,
                    SourceRegister::B => self.registers.b,
                    SourceRegister::C => self.registers.c,
                    SourceRegister::D => self.registers.d,
                    SourceRegister::E => self.registers.e,
                    SourceRegister::H => self.registers.h,
                    SourceRegister::L => self.registers.l,
                    SourceRegister::HLV => {
                        self.m += 8;
                        self.bus.read_byte(self.registers.get_hl())
                    }
                };
                source = match target_bit {
                    TargetBit::B0 => source & 0b_1111_1110,
                    TargetBit::B1 => source & 0b_1111_1101,
                    TargetBit::B2 => source & 0b_1111_1011,
                    TargetBit::B3 => source & 0b_1111_0111,
                    TargetBit::B4 => source & 0b_1110_1111,
                    TargetBit::B5 => source & 0b_1101_1111,
                    TargetBit::B6 => source & 0b_1011_1111,
                    TargetBit::B7 => source & 0b_0111_1111,
                };
                match source_register {
                    SourceRegister::A => self.registers.a = source,
                    SourceRegister::B => self.registers.b = source,
                    SourceRegister::C => self.registers.c = source,
                    SourceRegister::D => self.registers.d = source,
                    SourceRegister::E => self.registers.e = source,
                    SourceRegister::H => self.registers.h = source,
                    SourceRegister::L => self.registers.l = source,
                    SourceRegister::HLV => self.bus.write_bytes(self.registers.get_hl(), source),
                };
                self.m += 8;
                self.pc.wrapping_add(1)
            }

            _ => panic!(
                "Add instruction {:?}Support More Instructions.",
                instruction
            ),
        }
    }
    // TO- DO : if error check whether all sp adds are correctly done.
    fn _call(&mut self, should_jump: bool) -> u16 {
        let next_pc = self.pc.wrapping_add(3);
        if should_jump {
            self.m += 24;
            self._push(next_pc);
            self._read_next_word()
        } else {
            self.m += 12;
            next_pc
        }
    }

    fn _return_(&mut self, should_jump: bool) -> u16 {
        if should_jump {
            self.m += 20;
            self._pop()
        } else {
            self.m += 8;
            self.pc.wrapping_add(1)
        }
    }

    fn _read_next_byte(&mut self) -> u8 {
        self.pc = self.pc.wrapping_add(1);
        self.bus.read_byte(self.pc)
    }

    // LITTLE ENDIAN
    fn _read_next_word(&mut self) -> u16 {
        self.pc = self.pc.wrapping_add(2);
        (self.bus.read_byte(self.pc) as u16) << 8 | self.bus.read_byte(self.pc - 1) as u16
    }

    fn _pop(&mut self) -> u16 {
        let lsb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        let msb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        // println!("poping SP = {}",(msb << 8) | lsb);
        (msb << 8) | lsb
    }

    fn _push(&mut self, value: u16) {
        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_bytes(self.sp, ((value & 0xFF00) >> 8) as u8);

        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_bytes(self.sp, (value & 0x00FF) as u8);
    }
    /// fetch - exectue the Opcode
    pub fn step(&mut self) {
        //tprintln!("doing fetch - exectue the Opcode");
        self.m = 0;
        let mut instruction_byte = self.bus.read_byte(self.pc);

        let prefixed = instruction_byte == 0xCB;
        if prefixed {
            self.m += 4;
            self.pc = self.pc.wrapping_add(1);
            instruction_byte = self.bus.read_byte(self.pc);
        }

        if !(self.is_halted) {
            //tprintln!("cpu check 1");
            let next_pc =
                if let Some(instruction) = Instruction::from_byte(instruction_byte, prefixed) {
                    self._execute(instruction)
                } else {
                    self.pc
                };
            self.pc = next_pc;
        } else {
            self.m += 4;
        }

        if self.pending_inrerupt_disabled {
            if self.bus.read_byte(self.pc - 1) != 0xF3 {
                self.pending_inrerupt_disabled = false;
                self.bus.interupt_master = false;
            }
        }

        if self.pending_interupt_enabled {
            if self.bus.read_byte(self.pc - 1) != 0xFB {
                self.pending_interupt_enabled = false;
                self.bus.interupt_master = true;
            }
        }
    }

    fn _add_8bit(&mut self, val: u8) -> u8 {
        let (new_val, over_flowed) = self.registers.a.overflowing_add(val);
        self.registers.f = FlagsRegister {
            zero: new_val == 0,
            subtract: false,
            half_carry: (self.registers.a & 0x0F) + (val & 0x0F) & 0x10 == 0x10,
            carry: over_flowed,
        };
        new_val
    }

    fn _adc(&mut self, val: u8) -> u8 {
        let cy = if self.registers.f.carry { 1 } else { 0 };
        let half_cal = (self.registers.a & 0xF).wrapping_add(val & 0xF);

        self.registers.f.half_carry = half_cal.wrapping_add(cy) & 0x10 == 0x10;

        let (n_v, first_overflow) = val.overflowing_add(cy);
        let (new_val, overflowed) = self.registers.a.overflowing_add(n_v);

        self.registers.f.zero = new_val == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = overflowed | first_overflow;

        new_val
    }

    fn _add_16bit(&mut self, val: u16, sp_add: bool) -> u16 {
        let new_val = if !sp_add {
            let (v, over_flowed) = self.registers.get_hl().overflowing_add(val);
            self.registers.f = FlagsRegister {
                zero: self.registers.f.zero,
                subtract: false,
                carry: over_flowed,
                half_carry: (self.registers.get_hl() & 0xFFF).wrapping_add(val & 0xFFF) & 0x1000
                    == 0x1000,
            };
            v
        } else {
            let offset = val as i16;
            self.registers.f.half_carry =
                (((self.sp & 0x0F) as u8).wrapping_add((offset & 0x0F) as u8) & 0x10) == 0x10;
            self.registers.f.carry = ((self.sp & 0xFF).wrapping_add(val & 0xFF) & 0x100) == 0x100;
            self.registers.f.subtract = false;
            self.registers.f.zero = false;

            if offset > 0 {
                self.sp.wrapping_add(val)
            } else {
                self.sp.wrapping_sub(offset.abs() as u16)
            }
        };
        new_val
    }

    fn _sub_8bit(&mut self, val: u8) -> u8 {
        let (new_val, overflowed) = self.registers.a.overflowing_sub(val);

        self.registers.f = FlagsRegister {
            zero: new_val == 0,
            subtract: true,
            half_carry: (self.registers.a & 0xF).wrapping_sub(val & 0xF) & 0x10 == 0x10,
            carry: overflowed,
        };

        new_val
    }

    fn _sbc(&mut self, val: u8) -> u8 {
        let cy = if self.registers.f.carry { 1 } else { 0 };
        let half_cal = (self.registers.a & 0xF).wrapping_sub(val & 0xF);

        self.registers.f.half_carry = half_cal.wrapping_sub(cy) & 0x10 == 0x10;

        let (n_v, first_overflow) = val.overflowing_add(cy);
        let (new_val, overflowed) = self.registers.a.overflowing_sub(n_v);

        self.registers.f.zero = new_val == 0;
        self.registers.f.subtract = true;
        self.registers.f.carry = overflowed | first_overflow;

        new_val
    }

    fn _cp(&mut self, value: u8) {
        let (new_val, overflowed) = self.registers.a.overflowing_sub(value);

        self.registers.f = FlagsRegister {
            zero: new_val == 0,
            subtract: true,
            half_carry: (self.registers.a & 0xF).wrapping_sub(value & 0xF) & 0x10 == 0x10,
            carry: overflowed,
        };
    }

    fn _and(&mut self, value: u8) -> u8 {
        let new_val = self.registers.a & value;
        self.registers.f = FlagsRegister {
            zero: new_val == 0,
            carry: false,
            half_carry: true,
            subtract: false,
        };
        new_val
    }

    fn _or(&mut self, value: u8, exculsive: bool) -> u8 {
        let new_val = if exculsive {
            self.registers.a ^ value
        } else {
            self.registers.a | value
        };

        self.registers.f = FlagsRegister {
            zero: new_val == 0,
            carry: false,
            half_carry: false,
            subtract: false,
        };

        new_val
    }

    fn _inc_dec_8bit(&mut self, val: u8, offset: i8) -> u8 {
        let new_val = val.wrapping_add(offset as u8);

        self.registers.f.zero = new_val == 0;
        self.registers.f.subtract = offset == -1;
        self.registers.f.half_carry = if offset > 0 {
            (val & 0x0F).wrapping_add(1) & 0x10 == 0x10
        } else {
            (val & 0x0F).wrapping_sub(1) & 0x10 == 0x10
        };
        new_val
    }

    fn _jump_8bit(&mut self, should_jump: bool) -> u16 {
        if should_jump {
            let mut b = self._read_next_byte() as i8;
            b += 1;
            self.m += 12;
            return self.pc.wrapping_add(b as u16);
        } else {
            self.m += 8;
            return self.pc.wrapping_add(2);
        }
    }

    fn _stop(&self) {}

    fn _jump(&mut self, should_jump: bool, exception: bool) -> u16 {
        if should_jump && !(exception) {
            self.m += 16;

            let least_significant_byte = self.bus.read_byte(self.pc + 1) as u16;
            let most_significant_byte = self.bus.read_byte(self.pc + 2) as u16;
            (most_significant_byte << 8) | least_significant_byte
        } else if should_jump && exception {
            self.m += 4;
            self.registers.get_hl()
        } else {
            self.m += 12;

            self.pc.wrapping_add(3)
        }
    }

    pub fn _request_interupt(&mut self, i: u8) {
        // todo implement Inturupt.
        let mut req = self.bus.memory[0xFF0F];
        req = bit_set(req, i);
        self.bus.memory[0xFF0F] = req;
    }

    pub fn get_key_pressed(&mut self, key: u8) {
        if self.bus.key_pressed(key) {
            self._request_interupt(4);
        }
        self.update_graphics(self.m as i16);
    }

    pub fn set_key_relased(&mut self, key: u8) {
        self.bus.key_released(key);
    }

    pub fn update_timers(&mut self, cycles: u32) {
        self.bus.do_divider_register(cycles);

        if self.bus.clock_enabled() {
            self.bus.mem_timer_counter = self.bus.mem_timer_counter.wrapping_sub(cycles as i32);

            if self.bus.mem_timer_counter < 0 {
                let remaining_cycles = self.bus.mem_timer_counter;
                self.bus.set_clock_freq();
                self.bus.mem_timer_counter += remaining_cycles;

                let tma_val = self.bus.read_byte(TIMA as u16);
                self.bus.write_bytes(TIMA as u16, tma_val + 1);
                // overflow
                if self.bus.read_byte(TIMA as u16) == 255 {
                    let tma_val = self.bus.read_byte(TMA as u16);
                    self.bus.write_bytes(TIMA as u16, tma_val);
                    self._request_interupt(2);
                    self.is_halted = false;
                }
            }
        }
    }

    pub fn do_interupts(&mut self) {
        if self.bus.interupt_master {
            let req = self.bus.read_byte(0xFF0F);
            let enabled = self.bus.read_byte(0xFFFF);

            if req > 0 {
                for i in 0..5 {
                    if test_bit(req, i) {
                        if test_bit(enabled, i) {
                            self._service_interupt(i);
                        }
                    }
                }
            }
        }
    }

    pub fn _service_interupt(&mut self, i: u8) {
        self.is_halted = false;

        self.bus.interupt_master = false;
        let mut req = self.bus.read_byte(0xFF0F);
        req = bit_reset(req, i);
        self.bus.write_bytes(0xFF0F, req);

        self._push(self.pc);

        self.pc = match i {
            0 => 0x40,
            1 => 0x48,
            2 => 0x50,
            4 => 0x60,
            _ => panic!("unhandled.. interupt bit"),
        };
        self.m += 20;
    }

    pub fn _set_lcd_status(&mut self) {
        let mut status = self.bus.memory[0xFF41];

        if !self._is_lcd_enabled() {
            self.bus.scan_line_counter = 456;
            self.bus.memory[0xFF44] = 0;
            status &= 252;
            status = bit_set(status, 0);
            self.bus.write_bytes(0xFF41, status);
            return;
        }

        let current_line = self.bus.memory[0xFF44];
        let current_mode = status & 0x3;

        let mut _mode = 0u8;
        let mut req_int = false;

        if current_line >= 144 {
            _mode = 1;
            status = bit_set(status, 0);
            status = bit_set(status, 1);
            req_int = test_bit(status, 4);
        } else {
            let mode_2_bounds = 456 - 80;
            let mode_3_bounds = mode_2_bounds - 172;

            // mode 2
            if self.bus.scan_line_counter >= mode_2_bounds {
                _mode = 2;
                status = bit_set(status, 1);
                status = bit_reset(status, 0);
                req_int = test_bit(status, 5);
            }
            // mode 3
            else if self.bus.scan_line_counter >= mode_3_bounds {
                _mode = 3;
                status = bit_set(status, 1);
                status = bit_set(status, 0);
            } else {
                _mode = 0;
                status = bit_reset(status, 1);
                status = bit_reset(status, 0);
                req_int = test_bit(status, 3);
            }
        }

        if req_int && (_mode != current_mode) {
            if _mode == 0 {
                self._request_interupt(1);
            } else if _mode == 1 {
                self._request_interupt(1);

                if self.bus.memory[0xFFFF] & 1 != 0 {
                    self._request_interupt(0);
                }
            } else if _mode == 2 {
                self._request_interupt(1);
            }
        }

        if self.bus.read_byte(0xFF44) == self.bus.read_byte(0xFF45) {
            status = bit_set(status, 2);
            if test_bit(status, 6) {
                self._request_interupt(1);
            }
        } else {
            status = bit_reset(status, 2);
        }
        self.bus.write_bytes(0xFF41, status);
    }

    pub fn _is_lcd_enabled(&mut self) -> bool {
        let lcd_status = self.bus.read_byte(0xFF40);
        //tprintln!("check lcd_staus = {:08b} ", lcd_status);
        test_bit(lcd_status, 7)
    }

    pub fn _draw_scan_line(&mut self) {
        let control = self.bus.read_byte(0xFF40);

        if test_bit(control, 0) {
            self.bus.render_tiles();
        }

        if test_bit(control, 1) {
            self.bus.render_sprites(control);
        }
    }

    pub fn update_graphics(&mut self, cycles: i16) -> bool {
        self._set_lcd_status();

        if self._is_lcd_enabled() {
            self.bus.scan_line_counter = self.bus.scan_line_counter.wrapping_sub(cycles as i16);
        } else {
        }

        if self.bus.scan_line_counter <= 0 {
            self.bus.memory[0xFF44] = self.bus.memory[0xFF44].wrapping_add(1);
            let current_line = self.bus.read_byte(0xFF44);

            self.bus.scan_line_counter = 456;

            if current_line == 144 {
                self._request_interupt(0);
                return true;
            } else if current_line > 153 {
                self.bus.memory[0xFF44] = 0;
            } else if current_line < 144 {
                self._draw_scan_line()
            }
        }
        false
    }
}
