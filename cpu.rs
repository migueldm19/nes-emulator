use std::ops::Range;
use std::num::Wrapping;

pub struct Rom {
    prg_rom_size: u16,
    chr_rom_size: u16,
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>
}

impl Rom {
    pub fn new(data: Vec<u8>) -> Rom {
        let header = &data[..16];
        let prg_rom_size = (header[4] as u16) * 16384;
        let chr_rom_size = (header[5] as u16) * 8192;
    
        let range_prg = Range {start: 16 as usize, end: (16 + prg_rom_size as usize)};
        let prg_rom: Vec<u8> = Vec::from(&data[range_prg]);
    
        let range_chr = Range {start: (16 + prg_rom_size as usize) as usize, end: ((16 + prg_rom_size + chr_rom_size) as usize)};
        let chr_rom: Vec<u8> = Vec::from(&data[range_chr]);
    
        Rom {
            prg_rom_size: prg_rom_size, 
            chr_rom_size: chr_rom_size, 
            prg_rom: prg_rom,
            chr_rom: chr_rom,
        }
    }

    pub fn print_prg(&self) {
        println!("{:?}", self.prg_rom[0]);
    }

    pub fn read(&self, idx: usize) -> u8 {
        self.prg_rom[idx]
    }
}

struct Memory {
    data: [u8; 0xffff]
}

impl Memory {
    fn read(&self, idx: u16) -> u8 {
        self.data[idx as usize]
    }

    fn write(&mut self, val: u8, idx: u16) {
        self.data[idx as usize] = val;
    }

    pub fn load_rom(&mut self, rom: Rom) {
        let range = Range {start: 0x4020 as usize, end: (0x4020 + rom.prg_rom_size as usize)};
        self.data[range].copy_from_slice(&rom.prg_rom);
    }
}


pub struct Cpu {
    a: u8,
    x: u8,
    y: u8,
    pc: u16,
    sp: u8, //$100 - $1ff
    p: u8,
    memory: Memory
}

impl Cpu {
    pub fn new(rom: Rom) -> Cpu {
        let mut mem = Memory { data: [0; 0xffff] };
        mem.load_rom(rom);

        Cpu {
            a: 0,
            x: 0,
            y: 0,
            pc: 0x4020, // prg rom
            sp: 0xfd,
            p: 0x34,
            memory: mem
        }
    }

    fn next_instruction(&mut self) -> u8 {
        let val = self.memory.read(self.pc);
        self.pc += 1;
        val
    }

    pub fn run(&mut self) {
        loop {
            let opcode = self.memory.read(self.pc);
            self.pc += 1;

            match opcode {
                0x00 => print!(""),//TODO
                0xa9 => {                    
                    let val  = self.get_imm();
                    self.lda(val);
                    println!("lda immediate {val:x?}");
                }
                0xa5 => {                    
                    let val  = self.get_zero_page();
                    self.lda(val);
                    println!("lda zero page {val:x?}");
                }
                0xb5 => {                    
                    let val  = self.get_zero_page_x();
                    self.lda(val);
                    println!("lda zero page, X {val:x?}");
                }
                0xad => {                    
                    let val  = self.get_absolute();
                    self.lda(val);
                    println!("lda absolute {val:x?}");
                }
                0xbd => {                    
                    let val  = self.get_absolute_x();
                    self.lda(val);
                    println!("lda absolute, X {val:x?}");
                }
                0xb9 => {                    
                    let val  = self.get_absolute_y();
                    self.lda(val);
                    println!("lda absolute, Y {val:x?}");
                }
                0xa2 => {                    
                    let val  = self.get_imm();
                    self.ldx(val);
                    println!("ldx immediate {val:x?}");
                }
                0xa6 => {                    
                    let val  = self.get_zero_page();
                    self.ldx(val);
                    println!("ldx zero page {val:x?}");
                }
                0xb6 => {                    
                    let val  = self.get_zero_page_y();
                    self.ldx(val);
                    println!("ldx zero page, Y {val:x?}");
                }
                0xae => {                    
                    let val  = self.get_absolute();
                    self.ldx(val);
                    println!("ldx absolute {val:x?}");
                }
                0xbe => {                    
                    let val  = self.get_absolute_y();
                    self.ldx(val);
                    println!("ldx absolute, Y {val:x?}");
                }
                0xa0 => {                    
                    let val  = self.get_imm();
                    self.ldy(val);
                    println!("ldy immediate {val:x?}");
                }
                0xa4 => {                    
                    let val  = self.get_zero_page();
                    self.ldy(val);
                    println!("ldy zero page {val:x?}");
                }
                0xb4 => {                    
                    let val  = self.get_zero_page_x();
                    self.ldy(val);
                    println!("ldy zero page, X {val:x?}");
                }
                0xac => {                    
                    let val  = self.get_absolute();
                    self.ldy(val);
                    println!("ldy absolute {val:x?}");
                }
                0xbc => {                    
                    let val  = self.get_absolute_x();
                    self.ldy(val);
                    println!("ldy absolute, X {val:x?}");
                }
                0x85 => {
                    self.write_zero_page(self.a);
                    println!("sta zero page {:x?}", self.a);
                }
                0x95 => {
                    self.write_zero_page_x(self.a);
                    println!("sta zero page, X {:x?}", self.a);
                }
                0x8d => {
                    self.write_absolute(self.a);
                    println!("sta absolute {:x?}", self.a);
                }
                0x9d => {
                    self.write_absolute_x(self.a);
                    println!("sta absolute, X {:x?}", self.a);
                }
                0x99 => {
                    self.write_absolute_y(self.a);
                    println!("sta absolute, Y {:x?}", self.a);
                }
                _ => print!("")
            }

            if self.pc >= 0xffff {
                println!("End");
                break;
            }
        }
    }

    fn lda(&mut self, val: u8) {
        self.a = val;
        self.set_zero_flag(self.a == 0);
        self.set_negative(self.a & 0b10000000 == 0b10000000);
    }

    fn ldx(&mut self, val: u8) {
        self.x = val;
        self.set_zero_flag(self.x == 0);
        self.set_negative(self.x & 0b10000000 == 0b10000000);
    }

    fn ldy(&mut self, val: u8) {
        self.y = val;
        self.set_zero_flag(self.y == 0);
        self.set_negative(self.y & 0b10000000 == 0b10000000);
    }

    fn get_imm(&mut self) -> u8 {
        self.next_instruction()
    }

    fn get_zero_page(&mut self) -> u8 {
        let addr  = self.next_instruction();
        self.memory.read(addr as u16)
    }

    fn get_zero_page_x(&mut self) -> u8 {
        let addr  = Wrapping(self.next_instruction()) + Wrapping(self.x);
        self.memory.read(addr.0 as u16)
    }

    fn get_zero_page_y(&mut self) -> u8 {
        let addr  = Wrapping(self.next_instruction()) + Wrapping(self.y);
        self.memory.read(addr.0 as u16)
    }

    fn get_absolute(&mut self) -> u8 {
        let mut addr  = (self.next_instruction() as u16) << 8;
        addr = addr | (self.next_instruction() as u16);
        self.memory.read(addr)
    }

    fn get_absolute_x(&mut self) -> u8 {
        let mut addr  = (self.next_instruction() as u16) << 8;
        addr = addr | (self.next_instruction() as u16) + (self.x as u16);
        self.memory.read(addr)
    }

    fn get_absolute_y(&mut self) -> u8 {
        let mut addr  = (self.next_instruction() as u16) << 8;
        addr = addr | (self.next_instruction() as u16) + (self.y as u16);
        self.memory.read(addr)
    }

    fn write_zero_page(&mut self, val: u8) {
        let addr  = self.next_instruction();
        self.memory.write(val, addr as u16);
    }

    fn write_zero_page_x(&mut self, val: u8) {
        let addr  = Wrapping(self.next_instruction()) + Wrapping(self.x);
        self.memory.write(val, addr.0 as u16);
    }

    fn write_zero_page_y(&mut self, val: u8) {
        let addr  = Wrapping(self.next_instruction()) + Wrapping(self.y);
        self.memory.write(val, addr.0 as u16);
    }

    fn write_absolute(&mut self, val: u8) {
        let mut addr  = (self.next_instruction() as u16) << 8;
        addr = addr | (self.next_instruction() as u16);
        self.memory.write(val, addr);
    }

    fn write_absolute_x(&mut self, val: u8) {
        let mut addr  = (self.next_instruction() as u16) << 8;
        addr = addr | (self.next_instruction() as u16) + (self.x as u16);
        self.memory.write(val, addr);
    }

    fn write_absolute_y(&mut self, val: u8) {
        let mut addr  = (self.next_instruction() as u16) << 8;
        addr = addr | (self.next_instruction() as u16) + (self.y as u16);
        self.memory.write(val, addr);
    }

    fn set_carry_flag(&mut self, carry: bool) {
        match carry {
            true => self.p = self.p | 0b10000000,
            false => self.p = self.p & 0b01111111
        }
    }

    fn set_zero_flag(&mut self, zero: bool) {
        match zero {
            true => self.p = self.p | 0b01000000,
            false => self.p = self.p & 0b10111111
        }
    }

    fn set_interrupt_disable(&mut self, interrupt_disable: bool) {
        match interrupt_disable {
            true => self.p = self.p | 0b00100000,
            false => self.p = self.p & 0b11011111
        }
    }

    fn set_decimal_mode(&mut self, decimal_mode: bool) {
        match decimal_mode {
            true => self.p = self.p | 0b00010000,
            false => self.p = self.p & 0b11101111
        }
    }

    fn set_break_command(&mut self, break_command: bool) {
        match break_command {
            true => self.p = self.p | 0b00001000,
            false => self.p = self.p & 0b11110111
        }
    }

    fn set_overflow(&mut self, overflow: bool) {
        match overflow {
            true => self.p = self.p | 0b00000100,
            false => self.p = self.p & 0b11111011
        }
    }

    fn set_negative(&mut self, negative: bool) {
        match negative {
            true => self.p = self.p | 0b00000010,
            false => self.p = self.p & 0b11111101
        }
    }

    pub fn print_mem(&self) {
        println!("=======================RAM=======================");
        println!("{:x?}", &self.memory.data[..0x800]);
        println!("=======================RAM MIRRORS=======================");
        println!("{:x?}", &self.memory.data[0x800..0x2000]);
        println!("=======================PPU REGISTERS=======================");
        println!("{:x?}", &self.memory.data[0x2000..0x2008]);
        println!("=======================PPU REGISTERS MIRRORS=======================");
        println!("{:x?}", &self.memory.data[0x2008..0x4000]);
        println!("=======================APU AND I/O REGISTERS=======================");
        println!("{:x?}", &self.memory.data[0x4000..0x4018]);
        println!("=======================APU AND I/O FUNCTIONALITY=======================");
        println!("{:x?}", &self.memory.data[0x4018..0x4020]);
        println!("=======================PRG ROM, PRG RAM AND MAPPER REGISTERS=======================");
        println!("{:x?}", &self.memory.data[0x4020..]);
    }
}