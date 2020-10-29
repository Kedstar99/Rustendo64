#[derive(Debug)]
pub enum CPUI {
    BEQL,
    ANDI,
    ORI,
    LUI,
    MTC0,
    LW,
}

impl From<u32> for CPUI {
    fn from(opcode:u32) -> Self {
        match opcode {
            0b001101 => CPUI::ORI,
            0b001111 => CPUI::LUI,
            0b001100 => CPUI::ANDI,
            0b010000 => CPUI::MTC0,
            0b010100 => CPUI::BEQL,
            0b100011 => CPUI::LW,
            _ => panic!("Unrecognized Opcode: {:#b}", opcode)
        }
    }
}