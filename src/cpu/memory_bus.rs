use std::fmt;

use super::memory_map::*;

use crate::gpu::*;

use super::CPU;
pub struct MemoryBus {

    // bios flag
    _inbios : bool,

    // Memory Regions
    _bios : [u8; BIOS_SIZE],
    _rom  : [u8 ; ROM_0_SIZE],
    _wram : [u8 ; WORKING_RAM_SIZE],
    _eram : [u8 ; EXTERNAL_RAM_SIZE],
    _zram : [u8 ; ZRAM_SIZE],

    pub memory: [u8; 0xFFFF],
    pub gpu: GPU,

}

impl fmt::Debug for MemoryBus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("memory")
            .field("mem", &self.memory[0])
            .finish()
    }
}

impl MemoryBus {

    pub fn read_byte(&mut self, address: u16) -> u8 {

        let address = address as usize;

        match address {

            BIOS_BEGIN..=BIOS_END => {

                if self._inbios {
                    if address < 0x0100 {
                        return self._bios[address - BIOS_BEGIN];
                    }else if address == 0x0100 {
                        self._inbios = false;
                    }
                }

                return self._rom[address - ROM_0_BEGIN];
            }

            ROM_0_BEGIN..=ROM_0_END => {
                return self._rom[address - ROM_0_BEGIN];
            }

            ROM_1_BEGIN..=ROM_1_END => {
                return self._rom[ROM_0_SIZE + (address - ROM_1_BEGIN) ]
            }

            VRAM_BEGIN..=VRAM_END => {

                return self.gpu.read_vram(address - VRAM_BEGIN);

            }

            EXTERNAL_RAM_BEGIN..=EXTERNAL_RAM_END => {
                return self._eram[address - EXTERNAL_RAM_BEGIN];
            }

            WORKING_RAM_BEGIN..=WORKING_RAM_END | W_SHADOW_RAM_BEGIN..=W_SHADOW_RAM_END => {
                return self._wram[address - WORKING_RAM_BEGIN];
            }

            SPRITE_RAM_BEGIN..=SPRITE_RAM_END => {
                // HANDLE CORRECTLY.
                return self.gpu.read_vram(address - VRAM_BEGIN);
            }

            ZRAM_BEGIN..=ZRAM_END => {
                return self._zram[address - ZRAM_BEGIN];
            }
        
            _ => self.memory[address as usize],
        }

    }

    pub fn write_bytes(&mut self, address: u16, value: u8) {
        let address = address as usize;

        match address {

            BIOS_BEGIN..=BIOS_END => {

                if self._inbios {
                    if address < 0x0100 {
                        self._bios[address - BIOS_BEGIN] = value;
                    }
                }

                self._rom[address - ROM_0_BEGIN] = value;
            }

            ROM_0_BEGIN..=ROM_0_END => {
                self._rom[address - ROM_0_BEGIN] = value;
            }

            ROM_1_BEGIN..=ROM_1_END => {
                self._rom[ROM_0_SIZE + (address - ROM_1_BEGIN) ] = value;
            }

            VRAM_BEGIN..=VRAM_END => {

                self.gpu.write_vram(address - VRAM_BEGIN,value);

            }

            EXTERNAL_RAM_BEGIN..=EXTERNAL_RAM_END => {
                self._eram[address - EXTERNAL_RAM_BEGIN] = value;
            }

            WORKING_RAM_BEGIN..=WORKING_RAM_END | W_SHADOW_RAM_BEGIN..=W_SHADOW_RAM_END => {
                self._wram[address - WORKING_RAM_BEGIN] = value;
            }

            SPRITE_RAM_BEGIN..=SPRITE_RAM_END => {
                // HANDLE CORRECTLY.
                //self.gpu.read_vram(address - VRAM_BEGIN);
            }

            ZRAM_BEGIN..=ZRAM_END => {
                self._zram[address - ZRAM_BEGIN] = value;
            }
        
            _ => self.memory[address as usize] = value,
        }
    }

}
