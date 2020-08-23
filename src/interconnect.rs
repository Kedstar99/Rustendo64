use super::byteorder::{BigEndian, ByteOrder};
use std::fmt;

const RAM_SIZE: usize = 4 * 1024 * 1024;

pub struct Interconnect {
    pif_rom: Vec<u8>,
    ram: Box<[u16]>
}

impl Interconnect {
    pub fn new(pif_rom: Vec<u8>) -> Self {
        Interconnect{
            pif_rom: pif_rom,
            ram: vec![0; RAM_SIZE].into_boxed_slice()
        }
    }

    pub fn read_word(&self, addr:u32) -> u32 {
        if addr >= 0x1fc0_0000 && addr < 0x1fc0_07c0 {
            let rel_addr = addr - 0x1fc0_0000;
            // TODO check endiannness (replace with burntsushi byteorder)
            // currently reading as big endian
            BigEndian::read_u32(&self.pif_rom[rel_addr as usize..])
        } else {
            panic!("Unrecognized physical address: {:#x}", addr)
        }
    }

}

impl fmt::Debug for Interconnect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //TODO figure out sensible way to print RAM
        write!(f, "TODO implement format for interconnect.")
    }
}