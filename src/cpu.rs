use super::memory;
use super::mapper;

#[derive(Default, Debug)]
struct RegP {
    carry: bool,
    zero: bool,
    int_disable: bool,
    bcd: bool,
    decimal: bool,
    expansion: bool,
    overflow: bool,
    negative: bool
}

pub struct CPU {
    reg_pc: u16,
    reg_sp: u8,
    reg_a: u8,
    reg_x: u8,
    reg_y: u8,
    reg_p: RegP, //Processor Status register: NV-BDIZC
    memory: memory::MemMap
}

impl CPU {
    pub fn new(mapper: mapper::Mapper) -> CPU {
        //TODO: impl Default for CPU
        CPU {
            reg_pc: 0,
            reg_sp: 0xFF,
            reg_a: 0,
            reg_x: 0,
            reg_y: 0,
            reg_p: RegP::default(),
            memory: memory::MemMap::new(mapper)
        }
    }
    pub fn run(&mut self) {
        // TODO: normally, set the PC to the value at the reset vector
        //self.reg_pc = self.memory.readw(0xFFFC);
        // but for nestest.nes, set it to 0xC000
        self.reg_pc = 0xC000;
        self.reg_p.expansion = true;
        self.reg_p.int_disable = true;
        loop {
            print!("{:4X}   ", self.reg_pc);
            let opcode = self.read_inc_pc();
            let v: u16;
            match opcode {
                // Load & Store instructions
                0xA9 => { v = self.read_inc_pc() as u16; self.lda(v as u8) },    // LDA immediate
                0xA2 => { v = self.read_inc_pc() as u16; self.ldx(v as u8) },    // LDX immediate
                0x85 => { v = self.zero_page_read(); self.sta(v) },  // STA zero page
                0x86 => { v = self.zero_page_read(); self.stx(v) },  // STX zero page
                // Jump instructions
                0x4C => { v = self.readw_inc_pc(); self.jmp(v) },   // JMP absolute
                0x20 => { v = self.readw_inc_pc(); self.jsr(v) },   // JSR absolute
                0x60 => { v = 0; self.rts() },   // RTS implied
                // Processor status instructions
                0x08 => { v = 0 ; self.php() },   // PHP implied
                0x28 => { v = 0 ; self.plp() },   // PLP implied
                0x48 => { v = 0 ; self.pha() },   // PHA implied
                0x68 => { v = 0 ; self.pla() },   // PLA implied
                0x18 => { v = 0 ; self.clc() },   // CLC implied
                0xD8 => { v = 0 ; self.cld() },   // CLD implied
                0x58 => { v = 0 ; self.cli() },   // CLI implied
                0xB8 => { v = 0 ; self.clv() },   // CLV implied
                0x38 => { v = 0 ; self.sec() },   // SEC implied
                0xF8 => { v = 0 ; self.sed() },   // SED implied
                0x78 => { v = 0 ; self.sei() },   // SEI implied
                // Test instructions
                0x29 => { v = self.read_inc_pc() as u16 ; self.and(v as u8) },   // AND immediate
                0x24 => { v = self.zero_page_read() ; self.bit(v) },   // BIT zero page
                0x2C => { v = self.readw_inc_pc() ; self.bit(v) },   // BIT absolute
                0xC9 => { v = self.read_inc_pc() as u16 ; self.cmp(v as u8) },   // CMP immediate
                0x09 => { v = self.read_inc_pc() as u16 ; self.ora(v as u8) },   // ORA immediate
                // Branch instructions
                0xB0 => { v = self.read_inc_pc() as u16; self.bcs(v) }, // BCS relative
                0x90 => { v = self.read_inc_pc() as u16; self.bcc(v) }, // BCC relative
                0xF0 => { v = self.read_inc_pc() as u16; self.beq(v) }, // BEQ relative
                0xD0 => { v = self.read_inc_pc() as u16; self.bne(v) }, // BNE relative
                0x30 => { v = self.read_inc_pc() as u16; self.bmi(v) }, // BMI relative
                0x10 => { v = self.read_inc_pc() as u16; self.bpl(v) }, // BPL relative
                0x50 => { v = self.read_inc_pc() as u16; self.bvc(v) }, // BVC relative
                0x70 => { v = self.read_inc_pc() as u16; self.bvs(v) }, // BVS relative
                0xEA => { v = 0 }, // NOP
                _ => panic!("Unknown opcode: {:X}", opcode)
            }
            println!("{:2X} {:4X}     A: {:2X} X: {:2X} Y: {:2X} P:{:2X} SP: {:2X} ",
                opcode, v, self.reg_a, self.reg_x, self.reg_y, self.get_p(), self.reg_sp);
        }
    }

    // Instructions start here!
    fn lda(&mut self, value: u8) {
        self.reg_a = self.set_zn(value);
    }
    fn ldx(&mut self, value: u8) {
        self.reg_x = self.set_zn(value);
    }
    fn sta(&mut self, address: u16) {
        let value = self.reg_a;
        self.memory.write(address, value);
    }
    fn stx(&mut self, address: u16) {
        let value = self.reg_x;
        self.memory.write(address, value);
    }
    fn jmp(&mut self, address: u16) {
        self.reg_pc = address;
    }
    fn jsr(&mut self, address: u16) {
        self.reg_pc = self.reg_pc - 1;
        let pc_hi: u8 = ((self.reg_pc & 0xFF00) >> 8) as u8;
        let pc_lo: u8 = (self.reg_pc & 0x00FF) as u8;
        self.push(pc_hi);
        self.push(pc_lo);
        self.reg_pc = address;
    }
    fn rts(&mut self) {
        let pc_lo = self.pop();
        let pc_hi = self.pop();
        self.reg_pc = (((pc_hi as u16) << 8) | pc_lo as u16) + 1;
    }
    fn php(&mut self) {
        let value = self.get_p();
        self.push(value);
    }
    fn plp(&mut self) {
        let value = self.pop();
        self.set_p(value);
    }
    fn pha(&mut self) {
        let value = self.reg_a;
        self.push(value);
    }
    fn pla(&mut self) {
        let value = self.pop();
        self.reg_a = self.set_zn(value);
    }
    fn clc(&mut self) {
        self.reg_p.carry = false;
    }
    fn cld(&mut self) {
        self.reg_p.decimal = false;
    }
    fn cli(&mut self) {
        self.reg_p.int_disable = false;
    }
    fn clv(&mut self) {
        self.reg_p.overflow = false;
    }
    fn sec(&mut self) {
        self.reg_p.carry = true;
    }
    fn sed(&mut self) {
        self.reg_p.decimal = true;
    }
    fn sei(&mut self) {
        self.reg_p.int_disable = true;
    }
    fn and(&mut self, value: u8) {
        let v = self.reg_a & value;
        self.reg_a = self.set_zn(v);
    }
    fn bit(&mut self, address: u16) {
        let a = self.reg_a;
        let value = self.memory.read(address);
        self.reg_p.zero = match a & value {
            0 => true,
            _ => false
        };
        self.reg_p.overflow = match value & 0x40 {
            0 => false,
            _ => true
        };
        self.reg_p.negative = match value & 0x80 {
            0 => false,
            _ => true
        };
    }
    fn cmp(&mut self, value: u8) {
        let v: u8 = ((self.reg_a as i16) - (value as i16)) as u8;
        self.set_zn(v);
        if v > 0 { self.reg_p.carry = true; }
        else { self.reg_p.carry = false; }
    }
    fn ora(&mut self, value: u8) {
        let v = self.reg_a | value;
        self.reg_a = self.set_zn(v);
    }
    fn bcs(&mut self, rel: u16) {
        if self.reg_p.carry == true {
            self.branch(rel);
        }
    }
    fn bcc(&mut self, rel: u16) {
        if self.reg_p.carry == false {
            self.branch(rel);
        }
    }
    fn beq(&mut self, rel: u16) {
        if self.reg_p.zero == true {
            self.branch(rel);
        }
    }
    fn bne(&mut self, rel: u16) {
        if self.reg_p.zero == false {
            self.branch(rel);
        }
    }
    fn bmi(&mut self, rel: u16) {
        if self.reg_p.negative == true {
            self.branch(rel);
        }
    }
    fn bpl(&mut self, rel: u16) {
        if self.reg_p.negative == false {
            self.branch(rel);
        }
    }
    fn bvc(&mut self, rel: u16) {
        if self.reg_p.overflow == false {
            self.branch(rel);
        }
    }
    fn bvs(&mut self, rel: u16) {
        if self.reg_p.overflow == true {
            self.branch(rel);
        }
    }

    // Utility functions (not instructions)
    fn get_p(&self) -> u8 {
        let mut value: u8 = 0;
        if self.reg_p.negative { value |= 0x80; }
        if self.reg_p.overflow { value |= 0x40; }
        if self.reg_p.expansion { value  |= 0x20; }
        if self.reg_p.bcd { value |= 0x10; }
        if self.reg_p.decimal { value |= 0x08; }
        if self.reg_p.int_disable { value |= 0x04; }
        if self.reg_p.zero { value |= 0x02; }
        if self.reg_p.carry { value |= 0x01; }
        value
    }
    fn set_p(&mut self, value: u8) {
        self.reg_p.negative = match value & 0x80 { 0 => false, _ => true };
        self.reg_p.overflow = match value & 0x40 { 0 => false, _ => true };
        self.reg_p.expansion = match value & 0x20 { 0 => false, _ => true };
        self.reg_p.bcd = match value & 0x10 { 0 => false, _ => true };
        self.reg_p.decimal = match value & 0x08 { 0 => false, _ => true };
        self.reg_p.int_disable = match value & 0x04 { 0 => false, _ => true };
        self.reg_p.zero = match value & 0x02 { 0 => false, _ => true };
        self.reg_p.carry = match value & 0x01 { 0 => false, _ => true };
    }
    fn branch(&mut self, rel: u16) {
        self.reg_pc = (self.reg_pc as i32 + rel as i32) as u16;
    }
    fn read_inc_pc(&mut self) -> u8 {
        let value = self.memory.read(self.reg_pc);
        self.reg_pc = self.reg_pc + 1;
        value
    }
    fn readw_inc_pc(&mut self) -> u16 {
        let value = self.memory.readw(self.reg_pc);
        self.reg_pc = self.reg_pc + 2;
        value
    }
    fn write_inc_pc(&mut self, address: u16, value: u8) {
        self.memory.write(address, value);
        self.reg_pc = self.reg_pc + 1;
    }
    fn zero_page_read(&mut self) -> u16 {
        (0x00 << 8) | self.read_inc_pc() as u16
    }
    fn set_zn(&mut self, value: u8) -> u8 {
        self.reg_p.zero = (value == 0) as bool;
        self.reg_p.negative = ((value & 0x80) != 0) as bool;
        value
    }
    fn push(&mut self, value: u8) {
        let address = (0x01 << 8) | self.reg_sp as u16;
        self.memory.write(address, value);
        self.reg_sp = self.reg_sp - 1;
    }
    fn pop(&mut self) -> u8 {
        self.reg_sp = self.reg_sp + 1;
        let address = (0x01 << 8) | self.reg_sp as u16;
        self.memory.read(address)
    }
}
