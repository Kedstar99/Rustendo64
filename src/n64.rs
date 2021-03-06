use super::cpu::cpu;
use super::interconnect;

#[derive(Debug)]
pub struct N64{
    cpu: cpu::Cpu,
}

impl N64 {
    pub fn new(pif_rom:Vec<u8>)-> Self {
        N64 {
            cpu: cpu::Cpu::new(interconnect::Interconnect::new(pif_rom)),
        }
    }

    pub fn power_on_reset(&mut self) {
        self.cpu.power_on_reset();
    }

    pub fn run(&mut self) {
        self.cpu.run();
    }

    pub fn run_one_instruction(&mut self) {
        self.cpu.run_one_instruction()
    }
}