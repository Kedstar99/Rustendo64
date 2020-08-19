#[derive(Debug)]
pub enum CPUI {
    ORI,
    LUI,
    MTC0,
}

impl From<u32> for CPUI {
    fn from(opcode:u32) -> Self {
        match opcode {
            0b001101 => CPUI::ORI,
            0b001111 => CPUI::LUI,
            0b010000 => CPUI::MTC0,
            _ => panic!("Unrecognized Opcode: {:#b}", opcode)
        }
    }
}