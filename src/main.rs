use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use clap::Parser;

mod configuration;
use configuration::{ConfigurationParameter, PARAMETERS};

fn read_from_file<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let mut file = File::open(&path).expect("no file found");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("read error");
    buffer
}

fn print_param(data: &Vec<u8>, param: &ConfigurationParameter) {
    let slice = &data[param.address.into()..];

    let str_val = param.value_as_string(slice);

    println!(
        "{:31} ({:#04x}): {:6} [ {:>12} {:3} ] - {}",
        param.name, param.address, str_val.hex, str_val.usr, str_val.units, param.description
    );
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Binary config file
    config: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    let data = read_from_file(cli.config);

    for param in &PARAMETERS {
        print_param(&data, &param);
    }
}
