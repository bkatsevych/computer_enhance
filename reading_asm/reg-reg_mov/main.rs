use std::env;
use std::fs::read;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let binary = read(filename).expect("Failed to read file");

    let mut iter = binary.iter();

    while let (Some(&byte1), Some(&byte2)) = (iter.next(), iter.next()) {
        decode(byte1, byte2);
    }
}

fn decode(byte: u8, next_byte: u8) {
    match byte {
        0x89 => {
            disassemble_16bit(next_byte);
        }
        0x88 => {
            disassemble_8bit(next_byte);
        }
        _ => println!("Unknown instruction"),
    }
}

fn disassemble_16bit(byte: u8) {
    let src = byte & 0b111;
    let dst = (byte >> 3) & 0b111;

    let reg_names = ["ax", "cx", "dx", "bx", "sp", "bp", "si", "di"];

    println!(
        "mov {}, {}",
        reg_names[src as usize], reg_names[dst as usize]
    );
}

fn disassemble_8bit(byte: u8) {
    let src = byte & 0b111;
    let dst = (byte >> 3) & 0b111;

    let reg_names = ["al", "cl", "dl", "bl", "ah", "ch", "dh", "bh"];

    println!(
        "mov {}, {}",
        reg_names[src as usize], reg_names[dst as usize]
    );
}
