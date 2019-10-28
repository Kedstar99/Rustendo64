use std::fmt;

enum RegConfigEp {
    D,
    DxxDxx,
    RFU
}

impl Default for RegConfigEp {
    fn default() -> RegConfigEp {
        RegConfigEp::D
    }
}

impl fmt::Debug for RegConfigEp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RegConfigEp::D => write!(f, "D"),
            RegConfigEp::DxxDxx => write!(f, "DxxDxx"),
            RegConfigEp::RFU => write!(f, "RFU"),
        }
    }
}


enum RegConfigBe {
    LittleEndian,
    BigEndian
}

impl Default for RegConfigBe {
    fn default() -> RegConfigBe {
        RegConfigBe::BigEndian
    }
}

impl fmt::Debug for RegConfigBe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RegConfigBe::LittleEndian => write!(f, "LittleEndian"),
            RegConfigBe::BigEndian => write!(f, "BigEndian"),
        }
    }
}


#[derive(Default, Debug)]
struct RegConfig {
    reg_config_ep: RegConfigEp,
    reg_config_be: RegConfigBe
}

impl RegConfig {
    fn power_on_reset(&mut self) {
        self.reg_config_ep = RegConfigEp::D;
        self.reg_config_be = RegConfigBe::BigEndian;
    }
}


// Coprocessor 0
#[derive(Default, Debug)]
pub struct CP0 {
    reg_config: RegConfig
}

impl CP0 {
    pub fn new()->Self{
        CP0::default()
    }

    pub fn power_on_reset(&mut self) {
        self.reg_config.power_on_reset();
    }

    pub fn write_cp0_reg(&mut self, index: u32, data: u64) {
        match index {
            12 => {
                //status register
                self.write_status_reg(data)
            },

            _ => panic!("TODO CP0 reg write! {:#?} {:#?}", index, data)
        }
    }

    pub fn write_status_reg(&mut self, data: u64) {
        panic!("Status register write {:#?}", data)
    }
}
