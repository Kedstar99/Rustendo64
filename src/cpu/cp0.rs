use super::reg_config;
use super::reg_status;

// Coprocessor 0
#[derive(Default, Debug)]
pub struct CP0 {
    reg_config: reg_config::RegConfig,
    reg_status: reg_status::RegStatus,
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
        self.reg_status.write(data as u32);
        panic!("Status register write {:#?}", data)
    }
}
