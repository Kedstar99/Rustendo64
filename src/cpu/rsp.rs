use super::mem_map as mm;

pub struct RSP {
    signal_registers: [bool; 8],
    halt: bool,
    broke: bool,
    intr: bool,
    sstep: bool
}

impl RSP {

    pub fn new() -> RSP {
        RSP {
            halt: true,
            intr: false,
            signal_registers: [false, false, false, false, false, false, false, false],
            sstep: false,
            broke: false,
        }
    }

    pub fn read_sp_reg(&self, addr:u32)->u32{
        match addr {
            mm::SP_STATUS_REG_START..=mm::SP_STATUS_REG_END => {
                self.read_status_register()
            }
            _ => panic!("RSP found unknown addr{:#x}", addr)
        }
    
    }

    pub fn write_sp_reg(&mut self, data:u32) {
       for i in 0..=24 {
           match data & (1 << i) {
               1 => self.halt = false,
               2 => self.halt = true,
               4 => self.broke = false,
               8 => self.intr = false,
               16 => self.intr = true,
               32 => self.sstep = false,
               64 => self.sstep = true,
               128 => panic!("not implemented yet!"),
               256 => panic!("not implemented yet!"),
               512..=16777216 => self.signal_registers[(i as usize - 10) / 2] = (i % 2) == 0,
               _ => (),
          }
       }
    }

    fn read_status_register(&self) -> u32 {
        (if self.halt {1} else {0} << 0) | (if self.intr {1} else {0} << 1)
        //TODO properly emulate RSP
    }
}