use std::{fs};
mod cpu;

fn read_header(data: &Vec<u8>) {
    println!("File length: {}", data.len());

    let header = &data[..16];

    let id = Vec::from(&header[..4]);
    println!("Id: {:?}", String::from_utf8(id));

    let prg_rom_size = (header[4] as u16) * 16384;
    println!("PRG ROM size: {} bytes", prg_rom_size);

    let chr_rom_size = (header[5] as u16) * 8192;
    println!("CHR ROM size: {} bytes", chr_rom_size);

    let flags6 = header[6];
    let trainer_present = (flags6 & 0b00000100) >> 2;
    println!("Trainer present: {}", trainer_present);
}

fn main() {
    let data = fs::read("nestest.nes");
    match data {
        Ok(_data) => {
            read_header(&_data);
            let r = cpu::Rom::new(_data);
            let mut cpu = cpu::Cpu::new(r);
            cpu.run();
        }
        Err(e) => {
            println!("Error reading ROM file: {e}");
        }
    }
}
