#[macro_use]
extern crate clap;

fn main() {
    let matches = clap_app!(myapp =>
        (version: "0.1.0")
        (author: "Krish De Souza <K.DeSouza@Outlook.com>")
        (about: "Rustendo64 emulator attempt")
        (@arg CONFIG: -f --filename +takes_value "Reads a ROM file")
    ).get_matches();
    let config = matches.value_of("config");
    println!("Value for config: {:?}", config)
}
