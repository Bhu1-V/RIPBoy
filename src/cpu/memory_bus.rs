use std::fmt;
pub struct MemoryBus {
    pub memory: [u8; 0xFFFF],
}

impl fmt::Debug for MemoryBus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("memory").field("mem",&self.memory[0]).finish()
    }
}

impl MemoryBus {
    pub fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn write_bytes(&mut self , target: u16,source:u8){
        self.memory[target as usize] = source;
    }
}
