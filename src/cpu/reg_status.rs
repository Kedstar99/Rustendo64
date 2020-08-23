#[derive(Debug)]
enum TLBGeneralExceptionVectorLocation {
    Normal,
    Bootstrap,
}

impl Default for TLBGeneralExceptionVectorLocation {
    fn default() -> Self {
        TLBGeneralExceptionVectorLocation::Normal
    }
}

impl From<u32> for TLBGeneralExceptionVectorLocation {
    fn from(data: u32) -> Self {
        match data & 0x200000 {
            0 => TLBGeneralExceptionVectorLocation::Normal,
            _ => TLBGeneralExceptionVectorLocation::Bootstrap,
        }
    }
}

#[derive(Debug, Default)]
struct DiagnosticStatus {
    instruction_trace_support: bool,                         //ITS
    tlb_miss_gpe_vectors: TLBGeneralExceptionVectorLocation, //BEV
    tlb_shutdown: bool,                                      //TS
    soft_reset_or_nmi_occured: bool,                         //SR
    condition_bit: bool,                                     //CH
}

impl From<u32> for DiagnosticStatus {
    fn from(data: u32) -> Self {
        let panic_bit_twenty_three = data & 0x400000 == 1;
        let panic_bit_nineteen = data & 0x40000 == 1;
        if panic_bit_twenty_three || panic_bit_nineteen {
            panic!("Detected anomalie in DiagnosticStatus. Bit 19 or 23 were set when they shouldn't have been. {:#b}", data);
        }
        DiagnosticStatus {
            instruction_trace_support: data & 0x800000 != 0,
            tlb_miss_gpe_vectors: data.into(),
            tlb_shutdown: data & 0x100000 != 0,
            soft_reset_or_nmi_occured: data & 0x80000 != 0,
            condition_bit: data & 0x20000 != 0,
        }
    }
}

#[derive(Debug, Default)]
struct InterruptMask {
    enabled: bool,
    external_interrupts_or_write_requests: [bool; 5], //IM(6, 2)
    software_interrupt_cause_reg: [bool; 2],          //IM(1, 0)
}

impl From<u32> for InterruptMask {
    fn from(data: u32) -> Self {
        InterruptMask {
            enabled: data & 0x8000 != 0,
            external_interrupts_or_write_requests: [
                data & (1 << 10) != 0,
                data & (1 << 11) != 0,
                data & (1 << 12) != 0,
                data & (1 << 13) != 0,
                data & (1 << 14) != 0,
            ],
            software_interrupt_cause_reg: [data & (1 << 8) != 0, data & (1 << 9) != 0],
        }
    }
}

#[derive(Debug)]
enum Mode {
    Kernel,     // 00
    Supervisor, // 01
    User,       // 10
}

impl Default for Mode {
    fn default() -> Self {
        Self::Kernel
    }
}

impl From<u32> for Mode {
    fn from(value: u32) -> Self {
        match value & 0x18 {
            0b00000 => Mode::Kernel,
            0b01000 => Mode::Supervisor,
            0b10000 => Mode::User,
            _ => panic!("Unable to determine RegStatus mode"),
        }
    }
}

#[derive(Debug, Default)]
pub struct RegStatus {
    // Page 166 User manual Exception Processoring Register
    coprocessor_usability: [bool; 4],    //CU
    low_power: bool,                     //RP
    additional_fp_regs: bool,            //FR
    reverse_endian: bool,                //RE
    diagnostic_status: DiagnosticStatus, //DS
    interrupt_mask: InterruptMask,       //IM
    kernel_mode_64bit_addr: bool,        //KX
    supervisor_mode_64bit_addr: bool,    //SX
    user_mode_64bit_addr: bool,          //UX
    mode: Mode,                          //KSU
    error_level: bool,                   //ERL
    exception_level: bool,               //EXL
    global_interrupt: bool,              //IE
}

impl From<u32> for RegStatus {
    fn from(data: u32) -> Self {
        RegStatus {
            coprocessor_usability: [
                data & 0x10000000 != 0,
                data & 0x20000000 != 0,
                data & 0x40000000 != 0,
                data & 0x80000000 != 0,
            ],

            low_power: data & 0x08000000 != 0,

            additional_fp_regs: data & 0x04000000 != 0,

            reverse_endian: data & 0x02000000 != 0,
            diagnostic_status: data.into(),
            interrupt_mask: data.into(),
            kernel_mode_64bit_addr: data & 0x80 != 0,
            supervisor_mode_64bit_addr: data & 0x40 != 0,
            user_mode_64bit_addr: data & 0x20 != 0,
            mode: data.into(),
            error_level: data & 0x4 != 0,
            exception_level: data & 0x2 != 0,
            global_interrupt: data & 0x1 != 0,
        }
    }
}
