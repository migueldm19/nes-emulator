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
                0xa1 => {                    
                    let val  = self.get_indirect_x();
                    self.lda(val);
                    println!("lda indirect, X {val:x?}");
                }
                0xb1 => {                    
                    let val  = self.get_indirect_y();
                    self.lda(val);
                    println!("lda indirect, Y {val:x?}");
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
                0x81 => {
                    self.write_indirect_x(self.a);
                    println!("sta indirect, X {:x?}", self.a);
                }
                0x91 => {
                    self.write_indirect_y(self.a);
                    println!("sta indirect, Y {:x?}", self.a);
                }

                0x86 => {
                    self.write_zero_page(self.x);
                    println!("stx zero page {:x?}", self.x);
                }
                0x96 => {
                    self.write_zero_page_y(self.x);
                    println!("stx zero page, Y {:x?}", self.x);
                }
                0x8e => {
                    self.write_absolute(self.x);
                    println!("stx absolute {:x?}", self.x);
                }

                0x84 => {
                    self.write_zero_page(self.y);
                    println!("sty zero page {:x?}", self.y);
                }
                0x94 => {
                    self.write_zero_page_x(self.y);
                    println!("sty zero page, X {:x?}", self.y);
                }
                0x8c => {
                    self.write_absolute(self.y);
                    println!("sty absolute {:x?}", self.y);
                }
                
                0xaa => {
                    self.ldx(self.a);
                    println!("tax");
                }
                0xa8 => {
                    self.ldy(self.a);
                    println!("tay");
                }
                0xba => {
                    self.ldx(self.sp);
                    println!("tsx");
                }
                0x8a => {
                    self.lda(self.x);
                    println!("txa");
                }
                0x9a => {
                    self.sp = self.x;
                    println!("txs");
                }
                0x98 => {
                    self.lda(self.y);
                    println!("tya");
                }

                0x48 => {
                    self.stack_push(self.a);
                    println!("pha");
                }
                0x08 => {
                    self.stack_push(self.p);
                    println!("php");
                }
                0x68 => {
                    let val = self.stack_pull();
                    self.lda(val);
                    println!("pla");
                }
                0x28 => {
                    self.p = self.stack_pull();
                    println!("plp");
                }

                0x29 => {
                    let val = self.get_imm() & self.a;
                    self.lda(val);
                    println!("and immediate {:x?}", val);
                }
                0x25 => {
                    let val = self.get_zero_page() & self.a;
                    self.lda(val);
                    println!("and zero page {:x?}", val);
                }
                0x35 => {
                    let val = self.get_zero_page_x() & self.a;
                    self.lda(val);
                    println!("and zero page, X {:x?}", val);
                }
                0x2d => {
                    let val = self.get_absolute() & self.a;
                    self.lda(val);
                    println!("and absolute {:x?}", val);
                }
                0x3d => {
                    let val = self.get_absolute_x() & self.a;
                    self.lda(val);
                    println!("and absolute, X {:x?}", val);
                }
                0x39 => {
                    let val = self.get_absolute_y() & self.a;
                    self.lda(val);
                    println!("and absolute, Y {:x?}", val);
                }
                0x21 => {
                    let val = self.get_indirect_x() & self.a;
                    self.lda(val);
                    println!("and indirect, X {:x?}", val);
                }
                0x31 => {
                    let val = self.get_indirect_y() & self.a;
                    self.lda(val);
                    println!("and indirect, Y {:x?}", val);
                }

                0x49 => {
                    let val = self.get_imm() ^ self.a;
                    self.lda(val);
                    println!("eor immediate {:x?}", val);
                }
                0x45 => {
                    let val = self.get_zero_page() ^ self.a;
                    self.lda(val);
                    println!("eor zero page {:x?}", val);
                }
                0x55 => {
                    let val = self.get_zero_page_x() ^ self.a;
                    self.lda(val);
                    println!("eor zero page, X {:x?}", val);
                }
                0x4d => {
                    let val = self.get_absolute() ^ self.a;
                    self.lda(val);
                    println!("eor absolute {:x?}", val);
                }
                0x5d => {
                    let val = self.get_absolute_x() ^ self.a;
                    self.lda(val);
                    println!("eor absolute, X {:x?}", val);
                }
                0x59 => {
                    let val = self.get_absolute_y() ^ self.a;
                    self.lda(val);
                    println!("eor absolute, Y {:x?}", val);
                }
                0x41 => {
                    let val = self.get_indirect_x() ^ self.a;
                    self.lda(val);
                    println!("eor indirect, X {:x?}", val);
                }
                0x51 => {
                    let val = self.get_indirect_y() ^ self.a;
                    self.lda(val);
                    println!("eor indirect, Y {:x?}", val);
                }

                0x09 => {
                    let val = self.get_imm() | self.a;
                    self.lda(val);
                    println!("ora immediate {:x?}", val);
                }
                0x05 => {
                    let val = self.get_zero_page() | self.a;
                    self.lda(val);
                    println!("ora zero page {:x?}", val);
                }
                0x15 => {
                    let val = self.get_zero_page_x() | self.a;
                    self.lda(val);
                    println!("ora zero page, X {:x?}", val);
                }
                0x0d => {
                    let val = self.get_absolute() | self.a;
                    self.lda(val);
                    println!("ora absolute {:x?}", val);
                }
                0x1d => {
                    let val = self.get_absolute_x() | self.a;
                    self.lda(val);
                    println!("ora absolute, X {:x?}", val);
                }
                0x19 => {
                    let val = self.get_absolute_y() | self.a;
                    self.lda(val);
                    println!("ora absolute, Y {:x?}", val);
                }
                0x01 => {
                    let val = self.get_indirect_x() | self.a;
                    self.lda(val);
                    println!("ora indirect, X {:x?}", val);
                }
                0x11 => {
                    let val = self.get_indirect_y() | self.a;
                    self.lda(val);
                    println!("ora indirect, Y {:x?}", val);
                }

                0x24 => {
                    let val = self.get_zero_page() & self.a;
                    self.bit_test(val);
                    println!("bit zero page {:x?}", val);
                }
                0x2c => {
                    let val = self.get_absolute() & self.a;
                    self.bit_test(val);
                    println!("bit absolute {:x?}", val);
                }

                0x69 => {
                    let val = self.get_imm();
                    self.adc(val);
                    println!("adc immediate {:x?} + {:x?} + {}", self.a, val, self.get_carry_flag());
                }
                0x65 => {
                    let val = self.get_zero_page();
                    self.adc(val);
                    println!("adc zero page {:x?} + {:x?} + {}", self.a, val, self.get_carry_flag());
                }
                0x75 => {
                    let val = self.get_zero_page_x();
                    self.adc(val);
                    println!("adc zero page, X {:x?} + {:x?} + {}", self.a, val, self.get_carry_flag());
                }
                0x6d => {
                    let val = self.get_absolute();
                    self.adc(val);
                    println!("adc absolute {:x?} + {:x?} + {}", self.a, val, self.get_carry_flag());
                }
                0x7d => {
                    let val = self.get_absolute_x();
                    self.adc(val);
                    println!("adc absolute, X {:x?} + {:x?} + {}", self.a, val, self.get_carry_flag());
                }
                0x79 => {
                    let val = self.get_absolute_y();
                    self.adc(val);
                    println!("adc absolute, Y {:x?} + {:x?} + {}", self.a, val, self.get_carry_flag());
                }
                0x61 => {
                    let val = self.get_indirect_x();
                    self.adc(val);
                    println!("adc indirect, X {:x?} + {:x?} + {}", self.a, val, self.get_carry_flag());
                }
                0x71 => {
                    let val = self.get_indirect_y();
                    self.adc(val);
                    println!("adc indirect, Y {:x?} + {:x?} + {}", self.a, val, self.get_carry_flag());
                }

                0xe9 => {
                    let val = self.get_imm();
                    self.sbc(val);
                    println!("sbc immediate {:x?} - {:x?} - {}", self.a, val, self.get_carry_flag());
                }
                0xe5 => {
                    let val = self.get_zero_page();
                    self.sbc(val);
                    println!("sbc zero page {:x?} - {:x?} - {}", self.a, val, self.get_carry_flag());
                }
                0xf5 => {
                    let val = self.get_zero_page_x();
                    self.sbc(val);
                    println!("sbc zero page, X {:x?} - {:x?} - {}", self.a, val, self.get_carry_flag());
                }
                0xed => {
                    let val = self.get_absolute();
                    self.sbc(val);
                    println!("sbc absolute {:x?} - {:x?} - {}", self.a, val, self.get_carry_flag());
                }
                0xfd => {
                    let val = self.get_absolute_x();
                    self.sbc(val);
                    println!("sbc absolute, X {:x?} - {:x?} - {}", self.a, val, self.get_carry_flag());
                }
                0xf9 => {
                    let val = self.get_absolute_y();
                    self.sbc(val);
                    println!("sbc absolute, Y {:x?} - {:x?} - {}", self.a, val, self.get_carry_flag());
                }
                0xe1 => {
                    let val = self.get_indirect_x();
                    self.sbc(val);
                    println!("sbc indirect, X {:x?} - {:x?} - {}", self.a, val, self.get_carry_flag());
                }
                0xf1 => {
                    let val = self.get_indirect_y();
                    self.sbc(val);
                    println!("sbc indirect, Y {:x?} - {:x?} - {}", self.a, val, self.get_carry_flag());
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
    fn bit_test(&mut self, val: u8) {
        self.set_zero_flag(val == 0);
        self.set_negative(val & 0b10000000 == 0b10000000);
        self.set_overflow(val & 0b01000000 == 0b01000000);
    }

    fn adc(&mut self, val: u8) {
        let sum_1 = self.a.overflowing_add(val);
        let sum_2 = sum_1.0.overflowing_add(self.get_carry_flag());        

        self.a = sum_2.0;
        
        self.set_zero_flag(self.a == 0);
        self.set_carry_flag(sum_1.1 || sum_2.1);
        self.set_negative(self.a & 0b10000000 == 0b10000000);
        self.set_overflow(self.a & 0b10000000 == 0b10000000 && (sum_1.1 || sum_2.1));
    }

    fn sbc(&mut self, val: u8) {
        let sub_1 = self.a.overflowing_sub(val);
        let sub_2 = sub_1.0.overflowing_sub(1 -self.get_carry_flag());        

        self.a = sub_2.0;
        
        self.set_zero_flag(self.a == 0);
        self.set_carry_flag(!(sub_1.1 || sub_2.1));
        self.set_negative(self.a & 0b10000000 == 0b10000000);
        self.set_overflow(self.a & 0b10000000 == 0b10000000 && !(sub_1.1 || sub_2.1));
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

    fn get_indirect_x(&mut self) -> u8 {
        let ind_addr  = Wrapping(self.next_instruction()) + Wrapping(self.x);
        let addr = (self.memory.read(ind_addr.0 as u16) as u16) << 8;
        self.memory.read(addr)
    }

    fn get_indirect_y(&mut self) -> u8 {
        let mut addr = self.next_instruction() as u16;
        addr = (self.memory.read(addr) as u16) << 8;
        addr += self.y as u16;
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

    fn write_indirect_x(&mut self, val: u8) {
        let ind_addr  = Wrapping(self.next_instruction()) + Wrapping(self.x);
        let addr = (self.memory.read(ind_addr.0 as u16) as u16) << 8;
        self.memory.write(val, addr);
    }

    fn write_indirect_y(&mut self, val: u8) {
        let mut addr = self.next_instruction() as u16;
        addr = (self.memory.read(addr) as u16) << 8;
        addr += self.y as u16;
        self.memory.write(val, addr);
    }

    fn stack_push(&mut self, val: u8) {
        let addr = 0x0100 + (self.sp as u16);
        self.memory.write(val, addr);
        self.sp -= 1;
    }

    fn stack_pull(&mut self) -> u8 {
        self.sp += 1;
        let addr = 0x0100 + (self.sp as u16);
        self.memory.read(addr)        
    }

    fn set_carry_flag(&mut self, carry: bool) {
        match carry {
            true => self.p = self.p | 0b10000000,
            false => self.p = self.p & 0b01111111
        }
    }

    fn get_carry_flag(&self) -> u8 {
        (self.p & 0b10000000) >> 7
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