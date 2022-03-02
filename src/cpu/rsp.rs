use super::mem_map as mm;

#[derive(Default, Debug)]
pub struct RSP;

impl RSP {
    pub fn read_sp_reg(&self, addr:u32)->u32{
        match addr {
            mm::SP_STATUS_REG_START..=mm::SP_STATUS_REG_END => {
                self.read_status_register()
            }
            _ => panic!("RSP found unknown addr{:#x}", addr)
        }
    
    }

    pub fn write_sp_reg(&mut self, data:u32) {
        panic!("Write to RSP status reg! {:?}", data);
    }

    fn read_status_register(&self) -> u32 {
        1 //TODO properly emulate RSP
    }
}