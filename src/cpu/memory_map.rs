// BIOS / ROM - 0

pub const ROM_0_BEGIN: usize = 0x0000;
pub const ROM_0_END: usize = 0x3FFF;
pub const ROM_0_SIZE: usize = ROM_0_END - ROM_0_BEGIN + 1;

pub const BIOS_BEGIN: usize = 0x0000;
pub const BIOS_END: usize = 0x00FF;
pub const BIOS_SIZE: usize = BIOS_END - BIOS_BEGIN + 1;

// pub const CART_HEAD_BEGIN : usize = 0x0100;
// pub const CART_HEAD_END : usize = 0x014F;
// pub const CART_HEAD_SIZE : usize = CART_HEAD_END - CART_HEAD_BEGIN + 1;

// ROM - 1

pub const ROM_1_BEGIN: usize = 0x4000;
pub const ROM_1_END: usize = 0x7FFF;
pub const ROM_1_SIZE: usize = ROM_1_END - ROM_1_BEGIN + 1;

// VRAM

pub const VRAM_BEGIN: usize = 0x8000;
pub const VRAM_END: usize = 0x9FFF;
pub const VRAM_SIZE: usize = VRAM_END - VRAM_BEGIN + 1;

// EXTERNAL RAM

pub const EXTERNAL_RAM_BEGIN: usize = 0xA000;
pub const EXTERNAL_RAM_END: usize = 0xBFFF;
pub const EXTERNAL_RAM_SIZE: usize = EXTERNAL_RAM_END - EXTERNAL_RAM_BEGIN + 1;

// WORKING RAM

pub const WORKING_RAM_BEGIN: usize = 0xC000;
pub const WORKING_RAM_END: usize = 0xDFFF;
pub const WORKING_RAM_SIZE: usize = WORKING_RAM_END - WORKING_RAM_BEGIN + 1;

// WORKING_SHADOW RAM

pub const W_SHADOW_RAM_BEGIN: usize = 0xE000;
pub const W_SHADOW_RAM_END: usize = 0xFDFF;
pub const W_SHADOW_RAM_SIZE: usize = W_SHADOW_RAM_END - W_SHADOW_RAM_BEGIN + 1;

// GRAPHICS : SPRITE INFO.

pub const SPRITE_RAM_BEGIN: usize = 0xFE00;
pub const SPRITE_RAM_END: usize = 0xFE9F;
pub const SPRITE_RAM_SIZE: usize = SPRITE_RAM_END - SPRITE_RAM_BEGIN + 1;

// I/O

pub const MM_IO_BEGIN: usize = 0xFF00;
pub const MM_IO_END: usize = 0xFF7F;
pub const MM_IO_SIZE: usize = MM_IO_END - MM_IO_BEGIN + 1;

// ZERO - PAGE RAM

pub const ZRAM_BEGIN: usize = 0xFE80;
pub const ZRAM_END: usize = 0xFFFE;
pub const ZRAM_SIZE: usize = ZRAM_END - ZRAM_BEGIN + 1;

// INTERUPT ENABLE REGISTER
pub const INTERUPT_REG: usize = 0xFFFF;
