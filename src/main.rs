#[macro_use]
extern crate clap;
use std::fs;
use std::io::Read;
use std::path::Path;

//see Google drive for CPU spec
#[derive(Default)]
struct Cpu { 
    gpr: [u64; 32],
    fpr: [f64; 32],
    pc: u64,
    high: u64,
    low: u64,
    llbit: bool,
    fcr0: u32,
    fcr32: u32,

    cp0:CP0,
}

impl Cpu {
    fn new() -> Self {
        Cpu::default()
    }

    fn power_on_reset(&mut self) {
        self.cp0.power_on_reset()
    }
}

// Coprocessor 0
#[derive(Default)]
struct CP0 {
    index: u64,
    random: u64,
    entry_lo0: u64,
    entry_lo1: u64,
    context: u64,
    page_mask: u64,
    wired: u64,
    bad_v_addr: i64,
    count: u64,
    entry_hi: u64,
    compare: u64,
    status: u64,
    cause: u64,
    epc: u64,
    prid: u64,
    config: u64,
    lladdr: u64,
    watchlo: u64,
    watchhi: u64,
    xcontext: u64,
    parity_err: u64,
    cache_err: u64,
    tag_lo: u64,
    tag_hi: u64,
    epc_err: u64
}

impl CP0 {
    fn new()->Self{
        CP0::default()
    }

    fn power_on_reset(&mut self) {
        
    }
}

fn main() {
    let matches = clap_app!(myapp =>
        (version: "0.1.0")
        (author: "Krish De Souza <K.DeSouza@Outlook.com>")
        (about: "Rustendo64 emulator attempt")
        (@arg rom_path: -f --filepath +takes_value "Filepath to ROM")
        (@arg pif_path: -p --piopath +takes_value "Filepath to PIF ROM")
    ).get_matches();
    let file_path = matches.value_of("rom_path").unwrap();
    let pif_path = matches.value_of("pif_path").unwrap();
    let file_buf = load_bin(Path::new(file_path));
    let pif_buf = load_bin(Path::new(pif_path));
    println!("Value for ROM: {:x?}", file_buf);
    println!("Value for PIF: {:?}", pif_buf)
}

fn load_bin<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let mut file = fs::File::open(path.as_ref()).unwrap();
    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf).unwrap();
    file_buf
}