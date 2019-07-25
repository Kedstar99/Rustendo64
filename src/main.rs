#[macro_use]
extern crate clap;
use std::fs;
use std::io::Read;


fn main() {
    let matches = clap_app!(myapp =>
        (version: "0.1.0")
        (author: "Krish De Souza <K.DeSouza@Outlook.com>")
        (about: "Rustendo64 emulator attempt")
        (@arg file_path: -f --filepath +takes_value "Filepath to to File name")
    ).get_matches();
    let file_path = matches.value_of("file_path").unwrap();
    let mut file = fs::File::open(file_path).unwrap();
    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf).unwrap();
    let file_buf = file_buf;
    println!("Value for config: {:?}", file_buf)
}
