use std::fmt;

#[derive(Debug)]
pub enum CPUI {
    ADDI,
    ADDIU,
    ORI,
    LUI,
    ANDI,
    MTC0,
    BEQL,
    BNEL,
    LW,
    SW,
}

impl ToString for CPUI {
    fn to_string(&self) -> String {
        let result = match self {
            CPUI::ADDI => "ADDI",
            CPUI::ADDIU => "ADDIU",
            CPUI::ORI => "ORI",
            CPUI::LUI => "LUI",
            CPUI::ANDI => "ANDI",
            CPUI::MTC0 => "MTC0",
            CPUI::BEQL => "BEQL",
            CPUI::BNEL => "BNEL",
            CPUI::LW => "LW",
            CPUI::SW => "SW",
            _ => panic!("Unrecognized OP")
        };
        result.to_string()
    }
}

impl From<u32> for CPUI {
    fn from(opcode:u32) -> Self {
        match opcode {
            0b001000 => CPUI::ADDI,
            0b001001 => CPUI::ADDIU,
            0b001101 => CPUI::ORI,
            0b001111 => CPUI::LUI,
            0b001100 => CPUI::ANDI,
            0b010101 => CPUI::BNEL,
            0b010000 => CPUI::MTC0,
            0b010100 => CPUI::BEQL,
            0b100011 => CPUI::LW,
            0b101011 => CPUI::SW,
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

    pub fn rs(&self) -> usize {
        ((self.op_word >> 21) & 0b11111) as usize
    }

    pub fn rt(&self) -> usize {
        ((self.op_word >> 16) & 0b11111) as usize
    }

    pub fn rd(&self) -> u32 {
        (self.op_word >> 11) & 0b11111
    }

    pub fn imm(&self) -> u32 {
        self.op_word & 0xffff
    }

    pub fn sign_extended_imm(&self) -> u64 {
        (self.imm() as i16) as u64
    }

    pub fn offset(&self) -> u32 {
        self.imm()
    }

    pub fn sign_extended_offset(&self) -> u64 {
        (self.offset() as i16) as u64
    }

    pub fn base(&self) -> usize {
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

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.opcode().to_string())
    }
}