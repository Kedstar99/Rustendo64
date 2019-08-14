
use super::interconnect;

//see Google drive for CPU spec
pub struct Cpu { 
    gpr: [u64; 32],
    fpr: [f64; 32],
    pc: u64,
    high: u64,
    low: u64,
    llbit: bool,
    fcr0: u32,
    fcr32: u32,

    cp0:CP0,
    interconnect: interconnect::Interconnect,
}

impl Cpu {
    pub fn new(interconnect:interconnect::Interconnect) -> Self {
        Cpu {
            gpr: [0; 32],
            fpr: [0.0; 32],
            pc: 0,
            high: 0,
            low: 0,
            llbit: false,
            fcr0: 0,
            fcr32: 0,

            cp0: CP0::default(),
            interconnect: interconnect,
        }
    }

    pub fn power_on_reset(&mut self) {
        self.cp0.power_on_reset()
    }

    pub fn run(&mut self) {
        loop {

        }

    }
}

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

enum RegConfigBe {
    LittleEndian,
    BigEndian
}

impl Default for RegConfigBe {
    fn default() -> RegConfigBe {
        RegConfigBe::BigEndian
    }
}


#[derive(Default)]
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
#[derive(Default)]
struct CP0 {
    reg_config: RegConfig
}

impl CP0 {
    fn new()->Self{
        CP0::default()
    }

    fn power_on_reset(&mut self) {
        self.reg_config.power_on_reset();
    }
}
