use super::reg_status;
use super::reg_config;

// Coprocessor 0
#[derive(Default, Debug)]
pub struct CP0 {
    reg_config: reg_config::RegConfig,
    reg_status: reg_status::RegStatus,
}

impl CP0 {
    pub fn power_on_reset(&mut self) {
        self.reg_config.power_on_reset();
    }

    pub fn write_cp0_reg(&mut self, index: u32, data: u64) {
        match index {
            12 => {self.reg_status = (data as u32).into()},
            16 => {self.reg_config = (data as u32).into()},

            _ => panic!("TODO CP0 reg write! {:#?} {:#?}", index, data)
        }
    }
}
