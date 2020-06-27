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

#[derive(Debug, Default)]
struct InterruptMask {
    enabled: bool,
    timer_interrupt: bool, //IM(7)
    external_interrupts_or_write_requests: [bool; 5], //IM(6, 2)
    software_interrupt_cause_reg: [bool; 2] //IM(1, 0)
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
    supervisor_mode__64bit_addr_: bool, //SX
    user_mode__64bit_addr: bool,  //UX
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

        self.diagnostic_status = data & 0x1ff0000;

        
    } 
}