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

}