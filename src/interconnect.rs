use super::byteorder::{BigEndian, ByteOrder};
use std::fmt;

use super::cpu::mem_map as mm;
use super::cpu::rsp as rsp;

const RAM_SIZE: usize = 4 * 1024 * 1024;

enum Addr {
    PifRom(u32),
    SpStatusReg,
    SpDMABusyReg,
}

pub struct Interconnect {
    pif_rom: Box<[u8]>,
    ram: Box<[u16]>,
    rsp: rsp::RSP,
}

impl Interconnect {
    pub fn new(pif_rom: Box<[u8]>) -> Self {
        Interconnect{
            pif_rom: pif_rom,
            ram: vec![0; RAM_SIZE].into_boxed_slice(),
            rsp: rsp::RSP::new()
        }
    }

    pub fn read_word(&self, addr:u32) -> u32 {
        match self.mem_map(addr) {
            Addr::PifRom(offset) => BigEndian::read_u32(&self.pif_rom[offset as usize..]),
            Addr::SpStatusReg => self.rsp.read_sp_reg(addr),
            Addr::SpDMABusyReg => self.rsp.read_dma_busy_reg(),
        }
    }
    
    pub fn write_word(&mut self, addr: u32, data:u32) {
        match self.mem_map(addr) {
            Addr::PifRom(_) => panic!("Cannot write to PIF ROM"),
            Addr::SpStatusReg => self.rsp.write_sp_reg(data),
            Addr::SpDMABusyReg => self.rsp.write_dma_busy_reg(data),
        }
    }
    
    fn mem_map(&self, addr: u32) -> Addr {
        match addr {
            mm::PIF_ROM_START..=mm::PIF_ROM_END =>  Addr::PifRom(addr - mm::PIF_ROM_START),
            mm::SP_BASE_REG_START..=mm::SP_STATUS_REG_END => Addr::SpStatusReg,
            mm::SP_DMA_BUSY_REG =>  Addr::SpDMABusyReg,
            _ => panic!("Unrecognized physical address: {:#x}", addr)
        }
    }

}

impl fmt::Debug for Interconnect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //TODO figure out sensible way to print RAM
        write!(f, "TODO implement format for interconnect.")
    }
}