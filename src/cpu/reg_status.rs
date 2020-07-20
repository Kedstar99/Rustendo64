#[derive(Debug)]
enum TLBGeneralExceptionVectorLocation {
    Normal,
    Bootstrap
}

impl Default for TLBGeneralExceptionVectorLocation {
    fn default() -> Self { TLBGeneralExceptionVectorLocation::Normal }
}

#[derive(Debug, Default)]
struct DiagnosticStatus {
    instruction_trace_support: bool, //ITS
    tlb_miss_gpe_vectors: TLBGeneralExceptionVectorLocation, //BEV
    tlb_shutdown: bool, //TS
    soft_reset_or_nmi_occured: bool, //SR
    condition_bit: bool, //CH
}

impl DiagnosticStatus {
    fn write(&mut self, data: u32) {
        let panic_bit_twenty_three = data & 0x400000 == 1;
        let panic_bit_nineteen = data & 0x40000 == 1;
        if panic_bit_twenty_three || panic_bit_nineteen {
            panic!("Detected anomalie in DiagnosticStatus. Bit 19 or 23 were set when they shouldn't have been. {:#b}", data);
        }
        
        self.instruction_trace_support = data & 0x800000 != 0;
        self.tlb_miss_gpe_vectors = match data & 0x200000 != 0 {
            true => TLBGeneralExceptionVectorLocation::Bootstrap,
            false => TLBGeneralExceptionVectorLocation::Normal
        };
        self.tlb_shutdown = data & 0x100000 != 0;
        self.soft_reset_or_nmi_occured = data & 0x80000 != 0;
        self.condition_bit = data & 0x20000 != 0;

    }
}

#[derive(Debug, Default)]
struct InterruptMask {
    enabled: bool,
    external_interrupts_or_write_requests: [bool; 5], //IM(6, 2)
    software_interrupt_cause_reg: [bool; 2] //IM(1, 0)
}

impl InterruptMask {
    fn write(&mut self, data:u32) {
        self.enabled = data & 0x8000 != 0;
        for i in (0..5).rev() {
            self.external_interrupts_or_write_requests[i] = data & (1 << (10 + i)) != 0;
        }
        for i in (0..2).rev() {
            self.software_interrupt_cause_reg[i] = data & (1 << (8 + i)) != 0;
        }
    }
}

#[derive(Debug)]
enum Mode {
    Kernel, // 00
    Supervisor, // 01
    User,  // 10
}

impl Default for Mode {
    fn default() -> Self {Mode::Kernel}
}

#[derive(Debug, Default)]
pub struct RegStatus {
    // Page 166 User manual Exception Processoring Register
    coprocessor_usability: [bool; 4], //CU
    low_power: bool, //RP
    additional_fp_regs: bool, //FR
    reverse_endian: bool, //RE
    diagnostic_status: DiagnosticStatus, //DS
    interrupt_mask: InterruptMask, //IM
    kernel_mode_64bit_addr: bool, //KX
    supervisor_mode_64bit_addr: bool, //SX
    user_mode_64bit_addr: bool,  //UX
    mode: Mode, //KSU
    error_level: bool, //ERL
    exception_level: bool, //EXL
    global_interrupt: bool //IE
}

impl RegStatus {
    pub fn write(&mut self, data: u32) {
        self.coprocessor_usability[3] = data & 0x80000000 != 0;
        self.coprocessor_usability[2] = data & 0x40000000 != 0;
        self.coprocessor_usability[1] = data & 0x20000000 != 0;
        self.coprocessor_usability[0] = data & 0x10000000 != 0;

        self.low_power = data & 0x08000000 != 0;

        self.additional_fp_regs = data & 0x04000000 != 0;

        self.reverse_endian = data & 0x02000000 != 0;

        self.diagnostic_status.write(data);
        self.interrupt_mask.write(data);
        self.kernel_mode_64bit_addr = data & 0x80 != 0;
        self.supervisor_mode_64bit_addr = data & 0x40 !=0;
        self.user_mode_64bit_addr = data & 0x20 != 0;
        self.mode = match (data & 0x18) >> 3 {
            0b00 => Mode::Kernel,
            0b01 => Mode::Supervisor,
            0b10 => Mode::User,
            _ => panic!("Unable to determine RegStatus mode")
        };
        self.error_level = data & 0x4 != 0;
        self.exception_level = data & 0x2 != 0;
        self.global_interrupt = data & 0x1 != 0;
        
    } 
}