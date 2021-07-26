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
            rsp: rsp::RSP::default()
        }
    }

    pub fn read_word(&self, addr:u32) -> u32 {
        match addr {
            mm::PIF_ROM_START..=mm::PIF_ROM_END => {
                let rel_addr = addr - mm::PIF_ROM_START;
                // TODO check endiannness (replace with burntsushi byteorder)
                // currently reading as big endian
                return BigEndian::read_u32(&self.pif_rom[rel_addr as usize..])
            },
            mm::SP_BASE_REG_START..=mm::SP_STATUS_REG_END => {
                self.rsp.read_sp_reg(addr)
            },
            _ => panic!("Unrecognized physical address: {:#x}", addr)
        }
    }

    pub fn write_word(&mut self, addr: u32, data:u64) {
        panic!("TODO!");
    }

}

impl fmt::Debug for Interconnect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //TODO figure out sensible way to print RAM
        write!(f, "TODO implement format for interconnect.")
    }
}