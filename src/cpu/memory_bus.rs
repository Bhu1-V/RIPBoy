use std::fmt;
use std::fs::File;
use std::io;
use std::io::prelude::*;

use super::memory_map::*;
use crate::gpu::*;

use crate::useful_func::*;

pub const MAX_CATRIDGE_SIZE: usize = 0x200000;
pub const RETRACE_START: u16 = 456;

#[derive(PartialEq)]
pub enum Color {
    White,
    LightGray,
    DarkGray,
    Black,
}

pub struct RGB {
    red: u8,
    green: u8,
    blue: u8,
}
pub struct MemoryBus {
    // bios flag
    _inbios: bool,
    _gameLoaded: bool,

    // Catridge
    _cartridge: Vec<u8>,
    _mbc1: bool,
    _mbc2: bool,
    _current_rom_bank: u8,

    _ram_banks: [u8; 0x8000],
    _current_ram_bank: u8,

    _enable_ram: bool,
    _ram_mode: u8,
    _rom_banking: bool,

    // Memory Regions
    _bios: [u8; 256],
    _first_rom: [u8; 256],

    pub memory: [u8; 0x10000],
    pub gpu: GPU,
    pub interupt_master: bool,

    pub mem_timer_counter: i32,
    pub divider_register: u8,

    pub scan_line_counter: i16,
    pub joypad_state: u8,
}

impl fmt::Debug for MemoryBus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("memory")
            .field("mem", &self.memory[0])
            .finish()
    }
}

impl MemoryBus {
    pub fn print_mem_snap(&self) {
        let mut s: String = String::from("");
        for i in 0..0x1000 {
            if i == 0 {
                for k in 0..16 {
                    if k == 0 {
                        s += "     ";
                    }
                    s.push_str(&format!("{:>4X} ", k));
                }
                s += "\n";
            }
            for j in 0..16 {
                if j == 0 {
                    s.push_str(&format!("{:>4X} ", i));
                }
                s.push_str(&format!("{:>4X} ", &self.memory[(16 * i) + j]));
            }
            s += "\n";
        }

        print!("{}", s);
    }
}

impl MemoryBus {
    pub fn new() -> MemoryBus {
        MemoryBus {
            _inbios: false,
            _gameLoaded: false,

            _cartridge: vec![0; MAX_CATRIDGE_SIZE],
            _mbc1: false,
            _mbc2: false,
            _current_rom_bank: 0,

            _ram_banks: [0; 0x8000],
            _current_ram_bank: 0,
            _ram_mode: 0,
            _enable_ram: false,
            _rom_banking: true,

            // _bios : [
            //     0x31, 0xFE, 0xFF, 0xAF, 0x21, 0xFF, 0x9F, 0x32, 0xCB, 0x7C, 0x20, 0xFB, 0x21, 0x26, 0xFF, 0x0E,
            //     0x11, 0x3E, 0x80, 0x32, 0xE2, 0x0C, 0x3E, 0xF3, 0xE2, 0x32, 0x3E, 0x77, 0x77, 0x3E, 0xFC, 0xE0,
            //     0x47, 0x11, 0x04, 0x01, 0x21, 0x10, 0x80, 0x1A, 0xCD, 0x95, 0x00, 0xCD, 0x96, 0x00, 0x13, 0x7B,
            //     0xFE, 0x34, 0x20, 0xF3, 0x11, 0xD8, 0x00, 0x06, 0x08, 0x1A, 0x13, 0x22, 0x23, 0x05, 0x20, 0xF9,
            //     0x3E, 0x19, 0xEA, 0x10, 0x99, 0x21, 0x2F, 0x99, 0x0E, 0x0C, 0x3D, 0x28, 0x08, 0x32, 0x0D, 0x20,
            //     0xF9, 0x2E, 0x0F, 0x18, 0xF3, 0x67, 0x3E, 0x64, 0x57, 0xE0, 0x42, 0x3E, 0x91, 0xE0, 0x40, 0x04,
            //     0x1E, 0x02, 0x0E, 0x0C, 0xF0, 0x44, 0xFE, 0x90, 0x20, 0xFA, 0x0D, 0x20, 0xF7, 0x1D, 0x20, 0xF2,
            //     0x0E, 0x13, 0x24, 0x7C, 0x1E, 0x83, 0xFE, 0x62, 0x28, 0x06, 0x1E, 0xC1, 0xFE, 0x64, 0x20, 0x06,
            //     0x7B, 0xE2, 0x0C, 0x3E, 0x87, 0xF2, 0xF0, 0x42, 0x90, 0xE0, 0x42, 0x15, 0x20, 0xD2, 0x05, 0x20,
            //     0x4F, 0x16, 0x20, 0x18, 0xCB, 0x4F, 0x06, 0x04, 0xC5, 0xCB, 0x11, 0x17, 0xC1, 0xCB, 0x11, 0x17,
            //     0x05, 0x20, 0xF5, 0x22, 0x23, 0x22, 0x23, 0xC9, 0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B,
            //     0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E,
            //     0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC,
            //     0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E, 0x3c, 0x42, 0xB9, 0xA5, 0xB9, 0xA5, 0x42, 0x4C,
            //     0x21, 0x04, 0x01, 0x11, 0xA8, 0x00, 0x1A, 0x13, 0xBE, 0x20, 0xFE, 0x23, 0x7D, 0xFE, 0x34, 0x20,
            //     0xF5, 0x06, 0x19, 0x78, 0x86, 0x23, 0x05, 0x20, 0xFB, 0x86, 0x20, 0xFE, 0x3E, 0x01, 0xE0, 0x50,
            //   ],
            _bios: [
                49, 254, 255, 175, 33, 255, 159, 50, 203, 124, 32, 251, 33, 38, 255, 14, 17, 62,
                128, 50, 226, 12, 62, 243, 226, 50, 62, 119, 119, 62, 252, 224, 71, 17, 168, 0, 33,
                16, 128, 26, 205, 149, 0, 205, 150, 0, 19, 123, 254, 52, 32, 243, 17, 216, 0, 6, 8,
                26, 19, 34, 35, 5, 32, 249, 62, 25, 234, 16, 153, 33, 47, 153, 14, 12, 61, 40, 8,
                50, 13, 32, 249, 46, 15, 24, 243, 103, 62, 100, 87, 224, 66, 62, 145, 224, 64, 4,
                30, 2, 14, 12, 240, 68, 254, 144, 32, 250, 13, 32, 247, 29, 32, 242, 14, 19, 36,
                124, 30, 131, 254, 98, 40, 6, 30, 193, 254, 100, 32, 6, 123, 226, 12, 62, 135, 226,
                240, 66, 144, 224, 66, 21, 32, 210, 5, 32, 79, 22, 32, 24, 203, 79, 6, 4, 197, 203,
                17, 23, 193, 203, 17, 23, 5, 32, 245, 34, 35, 34, 35, 201, 0, 0, 0, 13, 0, 9, 17,
                9, 137, 57, 8, 201, 0, 11, 0, 3, 0, 12, 204, 204, 0, 15, 0, 0, 0, 0, 236, 204, 236,
                204, 221, 221, 153, 153, 152, 137, 238, 251, 103, 99, 110, 14, 204, 221, 31, 159,
                136, 136, 0, 0, 0, 0, 0, 0, 0, 0, 33, 168, 0, 17, 168, 0, 26, 19, 190, 32, 254, 35,
                125, 254, 52, 32, 245, 6, 25, 120, 134, 35, 5, 32, 251, 134, 32, 254, 62, 1, 224,
                80,
            ],

            _first_rom: [0; 256],

            memory: [0; 0x10000],

            gpu: GPU::new(),
            interupt_master: false,

            mem_timer_counter: 0,
            divider_register: 0,

            scan_line_counter: 456,
            joypad_state: 0,
        }
    }

    pub fn reset(&mut self) -> bool {
        self.gpu.reset();
        self.joypad_state = 0xFF;
        self.divider_register = 0;
        self._current_ram_bank = 0;

        self.memory[0xFF00] = 0xFF;
        self.memory[0xFF05] = 0x00;
        self.memory[0xFF06] = 0x00;
        self.memory[0xFF07] = 0x00;
        self.memory[0xFF10] = 0x80;
        self.memory[0xFF11] = 0xBF;
        self.memory[0xFF12] = 0xF3;
        self.memory[0xFF14] = 0xBF;
        self.memory[0xFF16] = 0x3F;
        self.memory[0xFF17] = 0x00;
        self.memory[0xFF19] = 0xBF;
        self.memory[0xFF1A] = 0x7F;
        self.memory[0xFF1B] = 0xFF;
        self.memory[0xFF1C] = 0x9F;
        self.memory[0xFF1E] = 0xBF;
        self.memory[0xFF20] = 0xFF;
        self.memory[0xFF21] = 0x00;
        self.memory[0xFF22] = 0x00;
        self.memory[0xFF23] = 0xBF;
        self.memory[0xFF24] = 0x77;
        self.memory[0xFF25] = 0xF3;
        self.memory[0xFF26] = 0xF1;
        self.memory[0xFF40] = 0x91;
        self.memory[0xFF42] = 0x00;
        self.memory[0xFF43] = 0x00;
        self.memory[0xFF45] = 0x00;
        self.memory[0xFF47] = 0xFC;
        self.memory[0xFF48] = 0xFF;
        self.memory[0xFF49] = 0xFF;
        self.memory[0xFF4A] = 0x00;
        self.memory[0xFF4B] = 0x00;
        self.memory[0xFFFF] = 0x00;
        self.scan_line_counter = 456;

        self._ram_mode = 0;
        self._enable_ram = false;
        self._mbc2 = false;
        self.interupt_master = false;

        match self._cartridge[0x147] {
            1 | 2 | 3 => self._mbc1 = true,
            5 | 6 => self._mbc2 = true,

            _ => {
                return false;
            }
        }

        true
    }

    pub fn stop_game(&mut self) {
        self._gameLoaded = false;
    }

    pub fn load_catridge(&mut self) -> io::Result<()> {
        if self._gameLoaded {
            self.stop_game();
        }

        self._gameLoaded = true;

        self.memory = [0; 0x10000];
        self._cartridge = vec![0; MAX_CATRIDGE_SIZE];

        let mut file = File::open("game.gb")?;
        println!("read file size = {:?}", file.read(&mut self._cartridge));

        (self.memory[0..0x8001]).copy_from_slice(&self._cartridge[0..0x8001]);
        (self._first_rom[0..256]).copy_from_slice(&self.memory[0..256]);
        (self.memory[0..256]).copy_from_slice(&self._bios);

        self.memory[0xFF41] = 1;
        self.memory[0xFF43] = 0;

        self._current_rom_bank = 1;
        // self._current_ram_bank = 0;

        Ok(())
    }

    pub fn render_tiles(&mut self) {
        let control = self.memory[0xFF40];

        let mut tile_data: u16 = 0;
        let mut background_memory: u16 = 0;
        let mut unsig = true;

        let scroll_Y = self.read_byte(0xFF42);
        let scroll_X = self.read_byte(0xFF43);
        let window_Y = self.read_byte(0xFF4A);
        let window_X = self.read_byte(0xFF4B).wrapping_sub(7);

        let mut using_window = false;

        if test_bit(control, 5) {
            if window_Y <= self.read_byte(0xFF44) {
                using_window = true;
            }
        }

        if test_bit(control, 4) {
            tile_data = 0x8000;
        } else {
            tile_data = 0x8800;
            unsig = false;
        }

        if !using_window {
            if test_bit(control, 3) {
                background_memory = 0x9C00;
            } else {
                background_memory = 0x9800;
            }
        } else {
            if test_bit(control, 6) {
                background_memory = 0x9C00;
            } else {
                background_memory = 0x9800;
            }
        }

        let mut yPos: u8 = 0;

        if !using_window {
            yPos = scroll_Y.wrapping_add(self.read_byte(0xFF44));
        } else {
            yPos = self.read_byte(0xFF44).wrapping_sub(window_Y);
        }

        let tile_row: u16 = (yPos.wrapping_div(8) as u16).wrapping_mul(32);

        for pixel in 0u8..160 {
            let mut x_pos = pixel.wrapping_add(scroll_X);

            if using_window {
                if pixel >= window_X {
                    x_pos = pixel - window_X;
                }
            }

            let tile_col: u16 = x_pos as u16 >> 3;
            let tile_num: i16;

            let tile_address: u16 = background_memory + tile_row + tile_col;

            if unsig {
                tile_num = (self.read_byte(tile_address) as u8) as i16;
            } else {
                tile_num = (self.read_byte(tile_address) as i8) as i16;
            }

            let mut tile_location: u16 = tile_data;

            if unsig {
                tile_location += (tile_num * 16) as u16;
            } else {
                tile_location += ((tile_num + 128) * 16) as u16;
            }

            let mut line: u8 = yPos % 8;
            line *= 2;
            let data_1 = self.read_byte(tile_location + line as u16);
            let data_2 = self.read_byte(tile_location + line as u16 + 1);

            let mut color_bit = x_pos % 8;
            color_bit = color_bit.wrapping_sub(7);
            color_bit = color_bit.wrapping_mul((-1 as i8) as u8);

            let mut color_num = if test_bit(data_2, color_bit) { 1 } else { 0 };
            color_num <<= 1;
            color_num |= if test_bit(data_1, color_bit) { 1 } else { 0 };

            let col = self.get_color(color_num, 0xFF47);

            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            match col {
                Color::White => {
                    red = 255;
                    green = 255;
                    blue = 255;
                }

                Color::LightGray => {
                    red = 0xCC;
                    green = 0xCC;
                    blue = 0xCC;
                }

                Color::DarkGray => {
                    red = 0x77;
                    green = 0x77;
                    blue = 0x77;
                }

                _ => {
                    red = 0;
                    green = 0;
                    blue = 0;
                }
            }

            let finaly: u16 = self.read_byte(0xFF44) as u16;

            if (finaly < 0) || (finaly > 143) || (pixel < 0) || (pixel > 159) {
                continue;
            }

            // ADD RGB INTO SCREEN BUFFER
            let index: u16 = 160 * finaly + pixel as u16;
            self.gpu.buffer[index as usize] = from_u8_rgb(red, blue, green);
        }
    }

    pub fn render_sprites(&mut self, control: u8) {
        let mut use8x16 = false;
        if test_bit(control, 2) {
            use8x16 = true;
        }

        for sprite in 0u8..40 {
            let index = sprite.wrapping_mul(4);
            let y_pos = self.read_byte(0xFE00 + index as u16).wrapping_sub(16);
            let x_pos = self.read_byte(0xFE00 + index as u16 + 1).wrapping_sub(8);
            let tile_location =
                self.read_byte(0xFE00u16.wrapping_add(index as u16).wrapping_add(2));
            let attributes = self.read_byte(0xFE00u16.wrapping_add(index as u16).wrapping_add(3));

            let y_flip = test_bit(attributes, 6);
            let x_flip = test_bit(attributes, 5);

            let scan_line = self.read_byte(0xFF44);

            let ysize: u8 = if use8x16 { 16 } else { 8 };

            if (scan_line >= y_pos) && (scan_line < (y_pos.wrapping_add(ysize))) {
                let mut line = scan_line - y_pos;

                if y_flip {
                    line = line.wrapping_sub(ysize);
                    line = !(line).wrapping_add(1);
                }

                line = line.wrapping_mul(2);

                // check again
                let data_address: u16 = 0x8000
                    + (((tile_location as u16) & 0x00FF).wrapping_mul(16))
                    + (line as u16 & 0x00FF);

                let data_1 = self.read_byte(data_address);
                let data_2 = self.read_byte(data_address + 1);

                for tile_pixel in (0..8).rev() {
                    let mut color_bit: i32 = tile_pixel;

                    if x_flip {
                        color_bit -= 7;
                        color_bit *= -1;
                    }

                    let mut color_num = if test_bit(data_2, color_bit as u8) {
                        1
                    } else {
                        0
                    };
                    color_num <<= 1;
                    let b = if test_bit(data_1, color_bit as u8) {
                        1
                    } else {
                        0
                    };
                    color_num |= b;

                    let color_address = if test_bit(attributes, 4) {
                        0xFF49
                    } else {
                        0xFF48
                    };

                    let col = self.get_color(color_num, color_address);

                    if col == Color::White {
                        continue;
                    }

                    let mut red: u8 = 0;
                    let mut green: u8 = 0;
                    let mut blue: u8 = 0;

                    match col {
                        Color::White => {
                            red = 255;
                            green = 255;
                            blue = 255;
                        }
                        Color::LightGray => {
                            red = 0xCC;
                            green = 0xCC;
                            blue = 0xCC;
                        }
                        Color::DarkGray => {
                            red = 0x77;
                            green = 0x77;
                            blue = 0x77;
                        }
                        _ => {
                            red = 0;
                            green = 0;
                            blue = 0;
                        }
                    }

                    let mut x_pix = 0 - tile_pixel;
                    x_pix += 7;

                    let pixel: i32 = x_pos as i32 + x_pix;

                    if (scan_line < 0) || (scan_line > 143) || (pixel < 0) || (pixel > 159) {
                        continue;
                    }

                    let index = 160 * scan_line as i32 + pixel;

                    self.gpu.buffer[index as usize] = from_u8_rgb(red, blue, green);
                }
            }
        }
    }

    pub fn get_color(&mut self, color_num: u8, address: u16) -> Color {
        let mut res = Color::White;
        let palette = self.read_byte(address);
        let mut hi = 0;
        let mut lo = 0;

        match color_num {
            0 => {
                hi = 1;
                lo = 0;
            }
            1 => {
                hi = 3;
                lo = 2;
            }
            2 => {
                hi = 5;
                lo = 4;
            }
            3 => {
                hi = 7;
                lo = 6;
            }
            _ => panic!("unimplemented color_num case"),
        }

        let mut color = 0;
        color = if test_bit(palette, hi) { 1 } else { 0 };
        color <<= 1;
        let b = if test_bit(palette, lo) { 1 } else { 0 };
        color |= b;

        match color {
            0 => res = Color::White,
            1 => res = Color::LightGray,
            2 => res = Color::DarkGray,
            3 => res = Color::Black,
            _ => panic!("Unhandled color"),
        }
        res
    }

    pub fn handle_banking(&mut self, address: usize, mut value: u8) {
        if address < 0x2000 {
            if self._mbc1 || self._mbc2 {
                self.do_ram_bank_enable(address, value);
            }
        } else if (address >= 0x2000) && (address < 0x4000) {
            if self._mbc1 || self._mbc2 {
                self.do_change_lo_rom_bank(value);
            }
        } else if (address >= 0x4000) && (address < 0x6000) {
            if self._mbc1 {
                value &= 0x3;
                if self._ram_mode == 0 {
                    self._current_rom_bank &= 0x1F;
                    self._current_rom_bank |= value << 5;
                } else {
                    self._current_ram_bank = value;
                }
            }
        } else if (address >= 0x6000) && (address < 0x8000) {
            self._ram_mode = value & 1;
            if self._mbc1 {
                // self.do_change_rom_ram_mode(value);
                if self._ram_mode == 0 {
                    self._current_ram_bank = 0;
                } else {
                    self._current_rom_bank &= 0x1F;
                }
            }
        }
    }

    pub fn do_ram_bank_enable(&mut self, address: usize, value: u8) {
        if self._mbc2 {
            if (address & 0b_0000_0001_0000_0000) >> 8 != 1 {
                return;
            }
        }

        let test_val = value & 0xF;

        self._enable_ram = test_val == 0xA;
    }

    pub fn do_change_lo_rom_bank(&mut self, mut value: u8) {
        if self._mbc2 {
            value &= 0xf;
            value = if value == 0 { 1 } else { value };
            self._current_rom_bank = value;
        } else if self._mbc1 {
            value = value & 0x1F;
            value = if value == 0 { 1 } else { value };
            self._current_rom_bank = (self._current_rom_bank & 0xE0) | (value & 0x1F);
            // turning off lower 5 bits

            // if self._current_rom_bank == 0 {
            //     self._current_rom_bank += 1;
            // }
        }
    }

    pub fn do_change_hi_rom_bank(&mut self, mut value: u8) {
        self._current_rom_bank &= 31;

        value &= 224;
        self._current_rom_bank |= value;
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
            ROM_0_BEGIN..=ROM_0_END => {
                return self.memory[address];
            }

            ROM_1_BEGIN..=ROM_1_END => {
                return self._cartridge[address + (self._current_rom_bank - 1) as usize * 0x4000];
            }

            VRAM_BEGIN..=VRAM_END => {
                return self.memory[address];
            }

            //0xA000 - 0XBFFF
            EXTERNAL_RAM_BEGIN..=EXTERNAL_RAM_END => {
                return self._ram_banks
                    [address + ((self._current_ram_bank as usize) * 0x2000) - 0xA000];
            }

            WORKING_RAM_BEGIN..=WORKING_RAM_END | W_SHADOW_RAM_BEGIN..=W_SHADOW_RAM_END => {
                return self.memory[address];
            }

            SPRITE_RAM_BEGIN..=SPRITE_RAM_END => {
                return self.memory[address];
            }

            0xFF00 => {
                return self.get_joypad_state();
            }

            0xFFFF => {
                return self.memory[0xFFFF];
            }

            MM_IO_BEGIN..=MM_IO_END => {
                return self.memory[address];
            }

            ZRAM_BEGIN..=ZRAM_END => {
                return self.memory[address];
            }

            _ => panic!("Reading UnAccesed Memory {:X}", address),
        }
    }

    pub fn write_bytes(&mut self, address: u16, value: u8) {
        let address = address as usize;
        match address {
            ROM_0_BEGIN..=ROM_1_END => {
                // NO WRITING to ROM
                self.handle_banking(address, value);
            }

            VRAM_BEGIN..=VRAM_END => {
                self.memory[address] = value;
            }

            EXTERNAL_RAM_BEGIN..=EXTERNAL_RAM_END => {
                if self._enable_ram {
                    self._ram_banks
                        [address + (self._current_ram_bank as usize) * 0x2000 - 0xA000] = value;
                }
            }

            WORKING_RAM_BEGIN..=WORKING_RAM_END => {
                self.memory[address] = value;
            }

            W_SHADOW_RAM_BEGIN..=W_SHADOW_RAM_END => {
                self.memory[address] = value;
                self.memory[address - 0x2000];
            }

            SPRITE_RAM_BEGIN..=SPRITE_RAM_END => {
                self.memory[address] = value;
            }

            //Divider Register
            0xFF04 => {
                self.memory[address] = 0;
            }

            0xFF07 => {
                let current_freq = self.get_clock_freq();
                self.memory[address] = 0xF8 | value;
                let new_freq = self.get_clock_freq();

                if current_freq != new_freq {
                    self.set_clock_freq();
                }
            }

            0xFF40 => {
                self.memory[address] = value;
            }

            0xFF41 => {
                self.memory[address] = value;
            }

            0xFF44 => {
                self.memory[address] = 0;
            }

            0xFF46 => {
                self.do_dma_transfer(value);
            }

            0xFF50 => {
                println!("Removing bios");
                for i in 0..256 {
                    self.memory[i] = self._first_rom[i];
                }
            }
            0xFFFF => {
                self.memory[0xFFFF] = value;
            }

            MM_IO_BEGIN..=MM_IO_END => {
                self.memory[address] = value;
            }

            ZRAM_BEGIN..=ZRAM_END => {
                self.memory[address] = value;
            }

            _ => {
                self.memory[address] = value;
            }
        }
    }

    pub fn do_dma_transfer(&mut self, value: u8) {
        // multipling with 100H
        let address = (value as u16) << 8;

        for i in 0..0xA0 {
            let v = self.read_byte(address + i);
            self.write_bytes(0xFE00 + i, v);
        }
    }

    pub fn set_clock_freq(&mut self) {
        let freq = self.get_clock_freq();
        match freq {
            0 => self.mem_timer_counter = 1024, // freq 4096
            1 => self.mem_timer_counter = 16,   // freq 262144
            2 => self.mem_timer_counter = 64,   // freq 65536
            3 => self.mem_timer_counter = 256,  // freq 16382
            _ => panic!("Unhandled set_freq arm"),
        }
    }

    pub fn key_pressed(&mut self, key: u8) -> bool {
        let mut previously_unset = false;

        if test_bit(self.joypad_state, key) == false {
            previously_unset = true;
        }

        self.joypad_state = bit_reset(self.joypad_state, key);

        let mut button = true;

        if key > 3 {
            button = true;
        } else {
            button = false;
        }

        let key_req = self.memory[0xFF00];
        let mut request_interupt = false;

        if button && !test_bit(key_req, 5) {
            request_interupt = true;
        } else if !button && !test_bit(key_req, 4) {
            request_interupt = true;
        }

        if request_interupt && !previously_unset {
            return true;
        } else {
            false
        }
    }

    pub fn key_released(&mut self, key: u8) {
        self.joypad_state = bit_set(self.joypad_state, key);
    }

    pub fn get_joypad_state(&self) -> u8 {
        let mut res: u8 = self.memory[0xFF00];
        res ^= 0xFF;

        if !test_bit(res, 4) {
            let mut top_joypad: u8 = self.joypad_state >> 4;
            top_joypad |= 0xF0;
            res &= top_joypad;
        } else if !test_bit(res, 5) {
            let mut bottom_joypad = self.joypad_state & 0xF;
            bottom_joypad |= 0xF0;
            res &= bottom_joypad;
        }
        res
    }

    pub fn clock_enabled(&mut self) -> bool {
        if test_bit(self.read_byte(0xff07), 2) {
            true
        } else {
            false
        }
    }

    pub fn do_divider_register(&mut self, cycles: u32) {
        let (new, overflowed) = self.divider_register.overflowing_add(cycles as u8);

        if overflowed {
            self.divider_register = new;
            self.memory[0xff04] = self.memory[0xff04].wrapping_add(1);
        } else {
            self.divider_register = new;
        }
    }

    pub fn get_clock_freq(&mut self) -> u8 {
        self.read_byte(0xFF07) & 0x3
    }
}
