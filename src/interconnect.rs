const RAM_SIZE: usize = 4 * 1024 * 1024;

pub struct Interconnect {
    ram: [u16; RAM_SIZE]
}

impl Default for Interconnect {
    fn default() -> Self {
        Interconnect{
            ram: [0;RAM_SIZE]
        }
    }
}

impl Interconnect {
    pub fn new() -> Self {
        Interconnect::default()
    }

}