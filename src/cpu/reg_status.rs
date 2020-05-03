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
pub struct RegStatus {
    // Page 166 User manual Exception Processoring Register
    coprocessor_usability: [bool, 4], //CU
    low_power: bool, //RP
    additional_fp_regs: bool, //FR
    reverse_endian: bool, //RE
    diagnostic_status: DiagnosticsStatus, //DS
    interrupt_mask: InterruptMask, //IM
    kernel_mode_64bit_addr: bool, //KX
    supervisor_mode__64bit_addr_: bool, //SX
    user_mode__64bit_addr: bool  //UX
    mode: Mode, //KSU
    error_level: bool, //ERL
    exception_level: bool, //EXL
    global_interrupt: bool //IE

}