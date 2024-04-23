use std::env;
use std::fs::read;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let binary = read(filename).expect("Failed to read file");

    let mut iter = binary.iter();

    while let Some(&byte) = iter.next() {
        if byte == 0x00 {
            continue;
        }
        decode(byte, &mut iter);
    }
}

fn decode(byte: u8, iter: &mut std::slice::Iter<u8>) {
    match byte {
        0x89 => {
            let modrm = iter.next().unwrap();
            if modrm & 0xC0 != 0xC0 {
                disassemble_reg16_to_mem(modrm);
            } else {
                disassemble_16bit(modrm);
            }
        }
        0x88 => {
            let modrm = iter.next().unwrap();
            if modrm & 0xC0 != 0xC0 {
                disassemble_reg8_to_mem(modrm);
            } else {
                disassemble_8bit(modrm);
            }
        }
        0xB0..=0xB7 => {
            disassemble_imm8_to_reg(byte - 0xB0, iter.next().unwrap());
        }
        0xB8..=0xBF => {
            disassemble_imm16_to_reg(byte - 0xB8, iter);
        }
        0x8A => {
            let modrm = iter.next().unwrap();
            match modrm & 0xC0 {
                0x40 => disassemble_mem8_to_reg_disp8(modrm, iter.next().unwrap()),
                0x80 => disassemble_mem8_to_reg_disp16(modrm, iter),
                _ => disassemble_mem8_to_reg(modrm),
            }
        }
        0x8B => {
            disassemble_mem16_to_reg(iter.next().unwrap());
        }
        _ => println!("Unknown instruction: {:02X}", byte),
    }
}

fn disassemble_16bit(&byte: &u8) {
    let src = byte & 0b111;
    let dst = (byte >> 3) & 0b111;

    let reg_names = ["ax", "cx", "dx", "bx", "sp", "bp", "si", "di"];

    println!(
        "mov {}, {}",
        reg_names[src as usize], reg_names[dst as usize]
    );
}

fn disassemble_8bit(&byte: &u8) {
    let src = byte & 0b111;
    let dst = (byte >> 3) & 0b111;

    let reg_names = ["al", "cl", "dl", "bl", "ah", "ch", "dh", "bh"];

    println!(
        "mov {}, {}",
        reg_names[src as usize], reg_names[dst as usize]
    );
}

fn disassemble_imm8_to_reg(reg: u8, &imm: &u8) {
    let reg_names = ["al", "cl", "dl", "bl", "ah", "ch", "dh", "bh"];
    let imm = imm as i8; // Convert to signed integer

    println!("mov {}, {}", reg_names[reg as usize], imm);
}

fn disassemble_imm16_to_reg(reg: u8, iter: &mut std::slice::Iter<u8>) {
    let reg_names = ["ax", "cx", "dx", "bx", "sp", "bp", "si", "di"];
    let low = *iter.next().unwrap() as u16;
    let high = *iter.next().unwrap() as u16;
    let imm = (high << 8) | low;

    println!("mov {}, {}", reg_names[reg as usize], imm as i16);
}

fn disassemble_mem8_to_reg(&byte: &u8) {
    let reg = (byte >> 3) & 0b111;
    let rm = byte & 0b111;

    let reg_names = ["al", "cl", "dl", "bl", "ah", "ch", "dh", "bh"];
    let rm_names = [
        "bx + si", "bx + di", "bp + si", "bp + di", "si", "di", "bp", "bx",
    ];

    println!(
        "mov {}, [{}]",
        reg_names[reg as usize], rm_names[rm as usize]
    );
}

fn disassemble_mem16_to_reg(&byte: &u8) {
    let reg = (byte >> 3) & 0b111;
    let rm = byte & 0b111;

    let reg_names = ["ax", "cx", "dx", "bx", "sp", "bp", "si", "di"];
    let rm_names = [
        "bx + si", "bx + di", "bp + si", "bp + di", "si", "di", "bp", "bx",
    ];

    println!(
        "mov {}, [{}]",
        reg_names[reg as usize], rm_names[rm as usize]
    );
}

fn disassemble_mem8_to_reg_disp8(&modrm: &u8, &disp: &u8) {
    let reg = (modrm >> 3) & 0b111;
    let rm = modrm & 0b111;

    let reg_names = ["al", "cl", "dl", "bl", "ah", "ch", "dh", "bh"];
    let rm_names = [
        "bx + si", "bx + di", "bp + si", "bp + di", "si", "di", "bp", "bx",
    ];

    println!(
        "mov {}, [{} + {}]",
        reg_names[reg as usize], rm_names[rm as usize], disp as i8
    );
}

fn disassemble_mem8_to_reg_disp16(&modrm: &u8, iter: &mut std::slice::Iter<u8>) {
    let reg = (modrm >> 3) & 0b111;
    let rm = modrm & 0b111;

    let reg_names = ["al", "cl", "dl", "bl", "ah", "ch", "dh", "bh"];
    let rm_names = [
        "bx + si", "bx + di", "bp + si", "bp + di", "si", "di", "bp", "bx",
    ];

    let low = *iter.next().unwrap() as u16;
    let high = *iter.next().unwrap() as u16;
    let disp = (high << 8) | low;

    println!(
        "mov {}, [{} + {}]",
        reg_names[reg as usize], rm_names[rm as usize], disp as i16
    );
}

fn disassemble_reg8_to_mem(&byte: &u8) {
    let reg = (byte >> 3) & 0b111;
    let rm = byte & 0b111;

    let reg_names = ["al", "cl", "dl", "bl", "ah", "ch", "dh", "bh"];
    let rm_names = [
        "bx + si", "bx + di", "bp + si", "bp + di", "si", "di", "bp", "bx",
    ];

    println!(
        "mov [{}], {}",
        rm_names[rm as usize], reg_names[reg as usize]
    );
}

fn disassemble_reg16_to_mem(&byte: &u8) {
    let reg = (byte >> 3) & 0b111;
    let rm = byte & 0b111;

    let reg_names = ["ax", "cx", "dx", "bx", "sp", "bp", "si", "di"];
    let rm_names = [
        "bx + si", "bx + di", "bp + si", "bp + di", "si", "di", "bp", "bx",
    ];

    println!(
        "mov [{}], {}",
        rm_names[rm as usize], reg_names[reg as usize]
    );
}
