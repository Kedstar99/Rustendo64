
use super::super::interconnect;
use super::cp0;
use super::instruction as cpu_i;

//see Google drive for CPU spec
#[derive(Debug)]
pub struct Cpu { 
    gpr: [u64; 32],
    fpr: [f64; 32],
    pc: u64,
    high: u64,
    low: u64,
    llbit: bool,
    fcr0: u32,
    fcr32: u32,

    cp0:cp0::CP0,
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

            cp0: cp0::CP0::default(),
            interconnect: interconnect,
        }
    }

    pub fn power_on_reset(&mut self) {
        self.cp0.power_on_reset();

        //This value comes from n64maps.txt see tag 01 in my google doc for details.
        self.pc = 0xffff_ffff_bfc0_0000;
    }

    pub fn run(&mut self) {
        loop {
            self.run_one_instruction()
        } 

    }

    fn read_word(&self, virt_addr: u64)->u32{
        let phys_addr = self.virt_phys_addr_mapping(virt_addr);
        self.interconnect.read_word(phys_addr as u32)
    }

    fn virt_phys_addr_mapping(&self, virt_addr: u64) -> u64 {
        // see table 5-3 of processor VR4300 user manual
        
        let addr_bit_values = (virt_addr >> 29) & 0b111;

        if addr_bit_values == 0b101 {
            // kseg1 case   
            virt_addr - 0xffff_ffff_a000_0000
        } else {
            panic!("Unrecognized virtual address: {:#x}", virt_addr)
        }

    }

    pub fn run_one_instruction(&mut self) {
        let op_word = self.read_word(self.pc);
        let op_code = (op_word >> 26) & 0b111111;
        let op:cpu_i::CPUI = op_code.into();
        let rs = (op_word >> 21) & 0b11111;
        let rt = (op_word >> 16) & 0b11111;
        let imm = op_word & 0xffff;
        match op {
            cpu_i::CPUI::ORI => {
                let res = self.read_gpr(rs as usize) | (imm as u64);
                self.write_gpr(rt as usize, res)
            }
            cpu_i::CPUI::LUI => {
                let value = ((imm << 16) as i32) as u64;
                self.write_gpr(rt as usize, value)
            },
            cpu_i::CPUI::MTC0 => {
                let rd = (op_word >> 11) & 0b11111;
                let data = self.read_gpr(rt as usize);
                self.cp0.write_cp0_reg(rd, data)
            },
            cpu_i::CPUI::LW => {
                //TODO: Handle LW TLB Miss Exception, invalid exception , bus error exception, address error excpetion
                let base = rs;
                let offset = imm;

                let sign_extended_offset = (offset as i16) as u64;
                let virt_addr = sign_extended_offset.wrapping_add(self.read_gpr(base as usize));
                let word = self.read_word(virt_addr);
                let value = (word as i32) as u64;
                self.write_gpr(rt as usize, value)
                
            }
        }
        self.pc += 4;
    }

    fn write_gpr(&mut self, index: usize, value: u64) {
        if index != 0{
            self.gpr[index] = value;
        }
    }

    fn read_gpr(&self, index: usize) -> u64 {
        match index {
            0 => 0,
            _ => self.gpr[index]
        }
    }
}

