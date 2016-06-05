mod rom;

use std::env;
use std::path::Path;
use std::io::Read;
use std::fs::File;

fn usage() {
    println!("Usage: futilenes <rom>");
}

fn read_rom<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let mut file = File::open(path).unwrap();
    let mut buf: Vec<u8> = Vec::new();
    match file.read_to_end(&mut buf) {
        Ok(bytes) => println!("Read {} bytes from ROM.", bytes),
        Err(_) => panic!("problem reading from ROM.")
    }
    buf
}

fn main() {
    let mut args = env::args();
    if args.len() != 2 {
        usage();
        return;
    }
    let rom_filename = match args.nth(1) {
        Some(a) => a,
        None => panic!("problem in argument parsing."),
    };
    println!("ROM: {}", rom_filename);

    let rom_data = read_rom(rom_filename);
    let rom = rom::iNESFile::load(rom_data);
    rom.info();
    rom.dump_prg_rom();
}
