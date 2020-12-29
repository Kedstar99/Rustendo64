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

pub struct Instruction {
    op_word:u32
}

impl Instruction {

    pub fn opcode(&self) -> CPUI {
        ((self.op_word >> 26) & 0b111111).into()
    }

    pub fn rs(&self) -> u32 {
        (self.op_word >> 21) & 0b11111
    }

    pub fn rt(&self) -> u32 {
        (self.op_word >> 16) & 0b11111
    }

    pub fn imm(&self) -> u32 {
        self.op_word & 0xffff
    }

    pub fn offset(&self) -> u32 {
        self.imm()
    }

    pub fn base(&self) -> u32 {
        self.rs()
    }
}

impl From<u32> for Instruction {
    fn from(data:u32) -> Self {
        Instruction {
            op_word: data
        }
    }
}