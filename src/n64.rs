use super::cpu;
use super::interconnect;

pub struct N64{
    cpu: cpu::Cpu,
    interconnect: interconnect::Interconnect
}

impl N64 {
    pub fn new()-> Self {
        N64 {
            cpu: cpu::Cpu::new(),
            interconnect: interconnect::Interconnect::new()
        }
    }

    pub fn power_on_reset(&mut self) {
        self.cpu.power_on_reset();
    }

    pub fn run(&mut self) {
        self.cpu.run();
    }
}