pub const PIF_ROM_START:u32 = 0x1fc0_0000;
pub const PIF_ROM_LEN:u32 = 2048;
pub const PIF_ROM_END:u32 = PIF_ROM_START + PIF_ROM_LEN;

pub const SP_BASE_REG_START:u32 = 0x04000000;
pub const SP_BASE_REG_END:u32 = 0x0400FFFF;

pub const SP_STATUS_REG_START:u32 = 0x4040010;
pub const SP_STATUS_REG_END:u32 = 0x4040013;

pub const SP_DMA_BUSY_REG:u32 = 0x4040018;