use std::fmt;

enum TransferDataPattern {
    D, // 0 -> D(default on cold reset)
    DxxDxx,// 6 -> DxxDxx: 2 doublewords/6 cycles
}

impl Default for TransferDataPattern {
    fn default() -> Self {
        TransferDataPattern::D
    }
}

impl From<u32> for TransferDataPattern{
    fn from(data:u32) -> Self {
        match (data >> 24) & 0b111 {
            0 => TransferDataPattern::D,
            6 => TransferDataPattern::DxxDxx,
            _=> panic!("Anything other than this value isn't supported by this emulator")
        }
    }
}

impl fmt::Debug for TransferDataPattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TransferDataPattern::D => write!(f, "D"),
            TransferDataPattern::DxxDxx => write!(f, "DxxDxx"),
        }
    }
}


enum EndianMemory {
    Little,
    Big
}

impl From<u32> for EndianMemory {
    fn from(data:u32) -> Self {
        match data & (1 << 15) != 0 {
            true => EndianMemory::Big,
            false => EndianMemory::Little
        }
    }
}

impl Default for EndianMemory {
    fn default() -> Self {
        EndianMemory::Big
    }
}

impl fmt::Debug for EndianMemory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EndianMemory::Little => write!(f, "LittleEndian"),
            EndianMemory::Big => write!(f, "BigEndian"),
        }
    }
}

enum OperatingFrequencyRatio {
    TwoThree,//1:1.5
    OneTwo,//1:2
    OneThree//1:3
}

impl From<u32> for OperatingFrequencyRatio {
    fn from(data:u32) -> Self {
        match (data >> 28) & 0b111 {
            0b000 => OperatingFrequencyRatio::OneTwo,
            0b001 => OperatingFrequencyRatio::OneTwo,
            0b111 => OperatingFrequencyRatio::TwoThree,
            _ => panic!("This operating frequency ratio isn't supported by this emulator")
        }
    }
}

impl fmt::Debug for OperatingFrequencyRatio {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            OperatingFrequencyRatio::TwoThree => write!(f, "1:1.5"),
            OperatingFrequencyRatio::OneTwo => write!(f, "1:2"),
            OperatingFrequencyRatio::OneThree => write!(f, "1:3"),
        }
    }
}

impl Default for OperatingFrequencyRatio {
    fn default() -> Self {
        OperatingFrequencyRatio::OneTwo
    }
}


#[derive(Default, Debug)]
pub struct RegConfig {
    operating_frequency_ratio: OperatingFrequencyRatio,
    transfer_data_pattern: TransferDataPattern,
    endian_mem: EndianMemory,
    cu_bit: bool,
    coherency_algo_of_kseg0: bool

}

impl RegConfig {
    pub fn power_on_reset(&mut self) {
        self.operating_frequency_ratio = OperatingFrequencyRatio::OneTwo;
        self.transfer_data_pattern = TransferDataPattern::D;
        self.endian_mem = EndianMemory::Big;
        self.cu_bit = false;
        self.coherency_algo_of_kseg0 = true; //if this is false, than the coherency cache isn't used
    }
}

impl From<u32> for RegConfig{
    fn from(data:u32) -> Self {
        
        //check 32nd bit
        if data & (1 << 31) == 1 {
            panic!("Bit 31 should always be 0")
        }
        
        //check pattern
        if data & 0x66460 != 418912 {
            panic!("RegConfig data not matching the specified patterns on Figure 5-16 of config register!")
        }

        RegConfig {
            operating_frequency_ratio: data.into(),
            transfer_data_pattern: data.into(),
            endian_mem: data.into(),
            cu_bit: data & (1 << 3) != 0,
            coherency_algo_of_kseg0: (data & 0b111) != 2,

        }
        


    }
}
