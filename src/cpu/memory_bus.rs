use std::fmt;

use VRAM::VRAM_BEGIN;

use crate::gpu::*;
use VRAM::*;
pub struct MemoryBus {
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
    pub fn read_byte(&self, address: u16) -> u8 {
        let address = address as usize;
        match address {
            VRAM_BEGIN..=VRAM_END => {
                return self.gpu.read_vram(address - VRAM_BEGIN);
            }
            _ => self.memory[address as usize],
        }
    }

    pub fn write_bytes(&mut self, address: u16, value: u8) {
        let address = address as usize;
        match address {
            VRAM_BEGIN...VRAM_END => self.gpu.write_vram(address - VRAM_BEGIN, value),
            _ => self.memory[address as usize] = value,
        }
    }

}
