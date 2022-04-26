pub const PIF_ROM_START:u32 = 0x1fc0_0000;
pub const PIF_ROM_LEN:u32 = 2048;
pub const PIF_ROM_END:u32 = PIF_ROM_START + PIF_ROM_LEN;

pub const SP_BASE_REG_START:u32 = 0x04000000;
pub const SP_BASE_REG_END:u32 = 0x0400FFFF;

pub const SP_STATUS_REG_START:u32 = 0x4040010;
pub const SP_STATUS_REG_END:u32 = 0x4040013;

pub const SP_DMA_BUSY_REG:u32 = 0x4040018;

pub const PI_STATUS_REG_START:u32 = 0x04600010;
pub const PI_STATUS_REG_END:u32 = 0x04600013;


pub enum Addr {
    PifRom(u32),
    SpStatusReg,
    SpDMABusyReg,
    PiStatusReg,
}

pub fn mem_map(addr: u32) -> Addr {
    match addr {
        PIF_ROM_START..=PIF_ROM_END =>  Addr::PifRom(addr - PIF_ROM_START),
        SP_BASE_REG_START..=SP_STATUS_REG_END => Addr::SpStatusReg,
        SP_DMA_BUSY_REG =>  Addr::SpDMABusyReg,
        PI_STATUS_REG_START..=PI_STATUS_REG_END => Addr::PiStatusReg,
        _ => panic!("Unrecognized physical address: {:#x}", addr)
    }
}