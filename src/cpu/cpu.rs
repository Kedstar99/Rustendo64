use super::super::interconnect;
use super::cp0;
use super::instruction as cpu_i;

use std::fmt;

//see Google drive for CPU spec
pub struct Cpu {
    gpr: [u64; 32],
    fpr: [f64; 32],
    pc: u64,
    high: u64,
    low: u64,
    llbit: bool,
    fcr0: u32,
    fcr31: u32,

    cp0: cp0::CP0,
    interconnect: interconnect::Interconnect,
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const REGS_PER_LINE: usize = 2;
        const REG_NAMES: [&'static str; 32] = [
            "r0", "at", "v0", "v1", "a0", "a1", "a2", "a3", "t0", "t1", "t2", "t3", "t4", "t5",
            "t6", "t7", "s0", "s1", "s2", "s3", "s4", "s5", "s6", "s7", "t8", "t9", "k0", "k1",
            "gp", "sp", "s8", "ra",
        ];
        let reg_iter = self.gpr.iter().zip(REG_NAMES.iter()).enumerate();
        write!(f, "\nCPU General Purpose Registers:");
        for (i, (reg, reg_name)) in reg_iter {
            if i % REGS_PER_LINE == 0 {
                writeln!(f, "");
            };
            write!(
                f,
                "{reg_name}/gpr{num:02}: {value:#018X} ",
                num = i,
                reg_name = reg_name,
                value = reg,
            );
        }

        write!(f, "\n\nCPU Floating Point Registers:");
        for (i, reg) in self.fpr.iter().enumerate() {
            if i % REGS_PER_LINE == 0 {
                writeln!(f, "");
            };
            write!(f, "fpr{num:02}: {value:21} ", num = i, value = reg,);
        }

        writeln!(f, "\n\nCPU Special Registers:");
        writeln!(
            f,
            "\
            reg_pc: {:#018X}\n\
            reg_hi: {:#018X}\n\
            reg_lo: {:#018X}\n\
            reg_llbit: {}\n\
            reg_fcr0:  {:#010X}\n\
            reg_fcr31: {:#010X}\n\
            ",
            self.pc, self.high, self.low, self.llbit, self.fcr0, self.fcr31
        );

        writeln!(f, "{:#?}", self.cp0);
        writeln!(f, "{:#?}", self.interconnect)
    }
}

impl Cpu {
    pub fn new(interconnect: interconnect::Interconnect) -> Self {
        Cpu {
            gpr: [0; 32],
            fpr: [0.0; 32],
            pc: 0,
            high: 0,
            low: 0,
            llbit: false,
            fcr0: 0,
            fcr31: 0,

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

    fn read_word(&self, virt_addr: u64) -> u32 {
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
        let instruction: cpu_i::Instruction = op_word.into();

        match instruction.opcode() {
            cpu_i::CPUI::ANDI => {
                let res = self.read_gpr(instruction.rs() as usize) & (instruction.imm() as u64);
                self.write_gpr(instruction.rt() as usize, res)
            },
            cpu_i::CPUI::ORI => {
                let res = self.read_gpr(instruction.rs() as usize) | (instruction.imm() as u64);
                self.write_gpr(instruction.rt() as usize, res)
            },
            cpu_i::CPUI::LUI => {
                let value = ((instruction.imm() << 16) as i32) as u64;
                self.write_gpr(instruction.rt() as usize, value)
            }
            cpu_i::CPUI::MTC0 => {
                let data = self.read_gpr(instruction.rt() as usize);
                self.cp0.write_cp0_reg(instruction.rd(), data)
            }
            cpu_i::CPUI::BEQL => {
                let branch = self.read_gpr(instruction.rs() as usize) == self.read_gpr(instruction.rt() as usize);
                if branch {
                    let sign_extended_offset = instruction.sign_extended_offset().wrapping_shl(2);
                    self.pc = self.pc.wrapping_add(sign_extended_offset);
                    self.run_one_instruction();
                }
            },
            cpu_i::CPUI::LW => {
                //TODO: Handle LW TLB Miss Exception, invalid exception , bus error exception, address error excpetion
                let virt_addr = instruction.sign_extended_offset().wrapping_add(self.read_gpr(instruction.base() as usize));
                let word = self.read_word(virt_addr);
                let value = (word as i32) as u64;
                self.write_gpr(instruction.rt() as usize, value)
            }
        }
        self.pc += 4;
    }

    fn write_gpr(&mut self, index: usize, value: u64) {
        if index != 0 {
            self.gpr[index] = value;
        }
    }

    fn read_gpr(&self, index: usize) -> u64 {
        match index {
            0 => 0,
            _ => self.gpr[index],
        }
    }
}
