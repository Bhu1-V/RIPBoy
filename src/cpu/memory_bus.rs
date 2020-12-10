use std::fmt;

use super::memory_map::*;
use crate::gpu::*;

pub const MAX_CATRIDGE_SIZE: usize = 0x200000;
pub struct MemoryBus {
    // bios flag
    _inbios: bool,

    // Catridge
    _cartridge: [u8; MAX_CATRIDGE_SIZE],
    _mbc1: bool,
    _mbc2: bool,
    _current_rom_bank: u8,

    _ram_banks: [u8; 0x8000],
    _current_ram_bank: u8,

    _enable_ram: bool,
    _rom_banking: bool,

    // Memory Regions
    _bios: [u8; BIOS_SIZE],

    pub memory: [u8; 0xFFFF],
    pub gpu: GPU,
    pub interupt_master: bool,

    pub mem_timer_counter: u32,
    pub divider_register: u8,

    pub scan_line_counter : u32,
}

impl fmt::Debug for MemoryBus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("memory")
            .field("mem", &self.memory[0])
            .finish()
    }
}

impl MemoryBus {
    pub fn load_catridge(&mut self) {
        // Implement correctly
        // first load catridge

        match self._cartridge[0x147] {
            1 | 2 | 3 => self._mbc1 = true,
            5 | 6 => self._mbc2 = true,

            _ => {
                self._mbc1 = false;
                self._mbc2 = false;
            }
        }

        self._current_rom_bank = 1;
        self._current_ram_bank = 0
    }

    pub fn handle_banking(&mut self, address: usize, value: u8) {
        if address < 0x2000 {
            if self._mbc1 || self._mbc2 {
                self.do_ram_bank_enable(address, value);
            }
        } else if (address >= 0x200) && (address < 0x4000) {
            if self._mbc1 || self._mbc2 {
                self.do_change_lo_rom_bank(value);
            }
        } else if (address >= 0x4000) && (address < 0x6000) {
            if self._mbc1 {
                if self._rom_banking {
                    self.do_change_hi_rom_bank(value);
                } else {
                    self.do_ram_bank_change(value);
                }
            }
        } else if (address >= 0x6000) && (address < 0x8000) {
            if self._mbc1 {
                self.do_change_rom_ram_mode(value);
            }
        }
    }

    pub fn do_ram_bank_enable(&mut self, address: usize, value: u8) {
        if self._mbc2 {
            if (address & 0b_0001_0000) >> 4 == 1 {
                return;
            }
        }

        let test_val = value & 0xF;

        if test_val == 0xA {
            self._enable_ram = true;
        } else if test_val == 0x0 {
            self._enable_ram = false;
        }
    }

    pub fn do_change_lo_rom_bank(&mut self, value: u8) {
        if self._mbc2 {
            self._current_rom_bank = value & 0xF;
            if self._current_rom_bank == 0 {
                self._current_rom_bank += 1;
            }
            return;
        }

        let lower5 = value & 31;
        self._current_rom_bank &= 224; // turning off lower 5 bits
        self._current_rom_bank |= lower5;

        if self._current_rom_bank == 0 {
            self._current_rom_bank += 1;
        }
    }

    pub fn do_change_hi_rom_bank(&mut self, mut value: u8) {
        self._current_rom_bank &= 31;

        value &= 224;
        self._current_rom_bank != value;
        if self._current_rom_bank == 0 {
            self._current_rom_bank += 1;
        }
    }

    pub fn do_ram_bank_change(&mut self, value: u8) {
        self._current_ram_bank = value & 0x3;
    }

    pub fn do_change_rom_ram_mode(&mut self, value: u8) {
        let new_data = value & 0x1;
        self._rom_banking = if new_data == 0 { true } else { false };

        if self._rom_banking {
            self._current_ram_bank = 0;
        }
    }

    pub fn read_byte(&mut self, address: u16) -> u8 {
        let address = address as usize;

        match address {
            BIOS_BEGIN..=BIOS_END => {
                if self._inbios {
                    if address < 0x0100 {
                        return self._bios[address - BIOS_BEGIN];
                    } else if address == 0x0100 {
                        self._inbios = false;
                    }
                }

                return self.memory[address];
            }

            ROM_0_BEGIN..=ROM_0_END => {
                return self.memory[address - ROM_0_BEGIN];
            }

            ROM_1_BEGIN..=ROM_1_END => {
                return self._cartridge
                    [address - ROM_1_BEGIN + (self._current_rom_bank as usize * 0x4000)];
            }

            VRAM_BEGIN..=VRAM_END => {
                return self.gpu.read_vram(address - VRAM_BEGIN);
            }

            EXTERNAL_RAM_BEGIN..=EXTERNAL_RAM_END => {
                return self._ram_banks
                    [(address - 0xA000) + (self._current_ram_bank as usize * 0x2000)];
            }

            WORKING_RAM_BEGIN..=WORKING_RAM_END | W_SHADOW_RAM_BEGIN..=W_SHADOW_RAM_END => {
                return self.memory[address];
            }

            SPRITE_RAM_BEGIN..=SPRITE_RAM_END => {
                // HANDLE CORRECTLY.
                return self.gpu.read_vram(address - VRAM_BEGIN);
            }

            ZRAM_BEGIN..=ZRAM_END => {
                return self.memory[address - ZRAM_BEGIN];
            }

            _ => self.memory[address as usize],
        }
    }

    pub fn write_bytes(&mut self, address: u16, value: u8) {
        let address = address as usize;

        match address {
            BIOS_BEGIN..=BIOS_END => {
                // NO WRITING TO BIOS
            }

            ROM_0_BEGIN..=ROM_1_END => {
                // NO WRITING to ROM\
                self.handle_banking(address, value);
            }

            VRAM_BEGIN..=VRAM_END => {
                self.gpu.write_vram(address - VRAM_BEGIN, value);
            }

            EXTERNAL_RAM_BEGIN..=EXTERNAL_RAM_END => {
                if self._enable_ram {
                    self._ram_banks
                        [(address - 0xA000) + self._current_ram_bank as usize * 0x2000] = value;
                }
            }

            WORKING_RAM_BEGIN..=WORKING_RAM_END | W_SHADOW_RAM_BEGIN..=W_SHADOW_RAM_END => {
                self.memory[address] = value;
                self.memory[address-0x2000] = value;
            }

            SPRITE_RAM_BEGIN..=SPRITE_RAM_END => {
                self.memory[address] = value;
            }

            0xFF04 => {
                self.memory[address] = 0;
            }

            0xFF07 => {
                let current_freq = self.get_clock_freq();
                self.memory[address] = value;
                let new_freq = self.get_clock_freq();

                if current_freq != new_freq {
                    self.set_clock_freq();
                }
            }

            0xFF44 => {
                self.memory[address] = 0;
            }

            0xFF46 => {
                self.do_dma_transfer(value);
            }

            ZRAM_BEGIN..=ZRAM_END => {
                self.memory[address - ZRAM_BEGIN] = value;
            }

            _ => {
                // unhandled call
                println!("UNHANDLED MEM_WRITE CALL");
            }
        }
    }

    fn new() {

        // set_clock_freq();
    }

    pub fn do_dma_transfer(&mut self,value : u8) {
        let address = (value as u16) << 8;

        for i in 0..0xA0 {
            let v = self.read_byte(address + i);
            self.write_bytes(0xFE00 + i , v);
        }
    }

    pub fn set_clock_freq(&mut self) {
        let freq = self.get_clock_freq();
        match freq {
            1 => self.mem_timer_counter = 1024, // freq 4096
            2 => self.mem_timer_counter = 16,   // freq 262144
            3 => self.mem_timer_counter = 64,   // freq 65536
            4 => self.mem_timer_counter = 256,  // freq 16382
            _ => panic!("Unhandled set_freq arm"),
        }
    }

    pub fn clock_enabled(&mut self) -> bool {
        if ((self.read_byte(0xff07) >> 2) & 1) == 1 {
            true
        } else {
            false
        }
    }

    pub fn do_divider_register(&mut self, cycles: u32) {
        let (new, overflowed) = self.divider_register.overflowing_add(cycles as u8);

        if overflowed {
            self.divider_register = 0;
            self.memory[0xff04] += 1;
        } else {
            self.divider_register = new;
        }
    }

    pub fn get_clock_freq(&mut self) -> u8 {
        self.read_byte(0xFF07) & 0x3
    }
}
