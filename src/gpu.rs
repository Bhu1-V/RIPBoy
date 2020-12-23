pub mod tile_pixel_value;

// use crate::cpu::memory_map::*;
use crate::useful_func::*;
use tile_pixel_value::TilePixelValue;

type Tile = [[TilePixelValue; 8]; 8];

pub fn empty_tile() -> Tile {
    [[TilePixelValue::Zero; 8]; 8]
}
pub struct GPU {
    // pub vram: [u8; VRAM_SIZE],
    pub tile_set: [Tile; 384],
    _modeclock: u16,
    _mode: u8,
    _line: u8,

    pub buffer: [u32; 160 * 144],
}

// impl defa

impl GPU {
    pub fn new() -> GPU {
        GPU {
            tile_set: [empty_tile(); 384],
            _modeclock: 0,
            _mode: 0,
            _line: 0,

            buffer: [0; 160 * 144],
        }
    }

    pub fn reset(&mut self) {
        self.buffer = [0; 160 * 144];

        for i in 0..(160 * 144) {
            self.buffer[i] = from_u8_rgb(255, 255, 255);
        }
    }

    pub fn step(&mut self, cpu_clock: u16) {
        self._modeclock += cpu_clock;

        match self._mode {
            // OAM Read Mode , Scan_Line Actice
            2 => {
                if self._modeclock >= 80 {
                    self._modeclock = 0;
                    self._mode = 3;
                }
            }

            // VRAM read mode, Scan_Line Active
            3 => {
                if self._modeclock >= 172 {
                    // enter h_blank mode
                    self._modeclock = 0;
                    self._mode = 0;

                    // write scanline to frame buffer
                    self.render_scan();
                }
            }

            0 => {
                if self._modeclock >= 204 {
                    self._modeclock = 0;
                    self._line += 1;

                    if self._line == 143 {
                        // enter v-blank mode
                        self._mode = 1;
                    // todo add gpu screen to frame buffer for display.
                    } else {
                        self._mode = 2;
                    }
                }
            }

            1 => {
                if self._modeclock >= 456 {
                    self._modeclock = 0;
                    self._line += 1;

                    if self._line > 153 {
                        self._mode = 2;
                        self._line = 0;
                    }
                }
            }

            _ => panic!("UNKNOWN GPU MODE"),
        }
    }

    fn render_scan(&self) {}

    // pub fn read_vram(&self, address: usize) -> u8 {
    //     self.vram[address]
    // }

    /*pub fn write_vram(&mut self, index: usize, value: u8) {
        self.vram[index] = value;
        // If our index is greater than 0x1800, we're not writing to the tile set storage
        // so we can just return.
        if index >= 0x1800 {
            return;
        }

        // Tiles rows are encoded in two bytes with the first byte always
        // on an even address. Bitwise ANDing the address with 0xffe
        // gives us the address of the first byte.
        // For example: `12 & 0xFFFE == 12` and `13 & 0xFFFE == 12`
        let normalized_index = index & 0xFFFE;

        // First we need to get the two bytes that encode the tile row.
        let byte1 = self.vram[normalized_index];
        let byte2 = self.vram[normalized_index + 1];

        // A tiles is 8 rows tall. Since each row is encoded with two bytes a tile
        // is therefore 16 bytes in total.
        let tile_index = index / 16;
        // Every two bytes is a new row
        let row_index = (index % 16) / 2;

        // Now we're going to loop 8 times to get the 8 pixels that make up a given row.
        for pixel_index in 0..8 {
            // To determine a pixel's value we must first find the corresponding bit that encodes
            // that pixels value:
            // 1111_1111
            // 0123 4567
            //
            // As you can see the bit that corresponds to the nth pixel is the bit in the nth
            // position *from the left*. Bits are normally indexed from the right.
            //
            // To find the first pixel (a.k.a pixel 0) we find the left most bit (a.k.a bit 7). For
            // the second pixel (a.k.a pixel 1) we first the second most left bit (a.k.a bit 6) and
            // so on.
            //
            // We then create a mask with a 1 at that position and 0s everywhere else.
            //
            // Bitwise ANDing this mask with our bytes will leave that particular bit with its
            // original value and every other bit with a 0.
            let mask = 1 << (7 - pixel_index);
            let lsb = byte1 & mask;
            let msb = byte2 & mask;

            // If the masked values are not 0 the masked bit must be 1. If they are 0, the masked
            // bit must be 0.
            //
            // Finally we can tell which of the four tile values the pixel is. For example, if the least
            // significant byte's bit is 1 and the most significant byte's bit is also 1, then we
            // have tile value `Three`.
            let value = match (lsb != 0, msb != 0) {
                (true, true) => TilePixelValue::Three,
                (false, true) => TilePixelValue::Two,
                (true, false) => TilePixelValue::One,
                (false, false) => TilePixelValue::Zero,
            };

            self.tile_set[tile_index][row_index][pixel_index] = value;
        }
    }
    */
}
