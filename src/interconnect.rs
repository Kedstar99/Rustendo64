use super::byteorder::{BigEndian, ByteOrder};
use std::fmt;

use super::cpu::mem_map as mm;
use super::cpu::rsp as rsp;

const RAM_SIZE: usize = 4 * 1024 * 1024;

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
        match mm::mem_map(addr) {
            mm::Addr::PifRom(offset) => BigEndian::read_u32(&self.pif_rom[offset as usize..]),
            mm::Addr::SpStatusReg => self.rsp.read_sp_reg(addr),
            mm::Addr::SpDMABusyReg => self.rsp.read_dma_busy_reg(),
            mm::Addr::PiStatusReg => unimplemented!("Handle pi status reg!")
        }
    }
    
    pub fn write_word(&mut self, addr: u32, data:u32) {
        match mm::mem_map(addr) {
            mm::Addr::PifRom(_) => panic!("Cannot write to PIF ROM"),
            mm::Addr::SpStatusReg => self.rsp.write_sp_reg(data),
            mm::Addr::SpDMABusyReg => self.rsp.write_dma_busy_reg(data),
            mm::Addr::PiStatusReg => unimplemented!("Handle pi status reg!")
        }
    }
    
}

impl fmt::Debug for Interconnect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //TODO figure out sensible way to print RAM
        write!(f, "TODO implement format for interconnect.")
    }
}