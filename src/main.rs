#[macro_use]
extern crate clap;
use std::fs;
use std::io::Read;
use std::path::Path;

mod cpu;

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
    println!("Value for PIF: {:?}", pif_buf);
    let mut cpu = cpu::Cpu::new();
    cpu.power_on_reset();
}

fn load_bin<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let mut file = fs::File::open(path.as_ref()).unwrap();
    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf).unwrap();
    file_buf
}