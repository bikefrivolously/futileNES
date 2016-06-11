use super::memory;
use super::mapper;

#[derive(Default)]
struct CpuState {
    reg_pc: u16,
    reg_sp: u8,
    reg_a: u8,
    reg_x: u8,
    reg_y: u8,
    reg_p: RegP, //Processor Status register: NV-BDIZC
}

impl CpuState {
    fn store(&mut self, cpu: &CPU) {
        self.reg_pc = cpu.reg_pc;
        self.reg_sp = cpu.reg_sp;
        self.reg_a = cpu.reg_a;
        self.reg_x = cpu.reg_x;
        self.reg_y = cpu.reg_y;
        self.reg_p = cpu.reg_p;
    }
    fn get_p(&self) -> u8 {
        let mut value: u8 = 0;
        if self.reg_p.negative { value |= 0x80; }
        if self.reg_p.overflow { value |= 0x40; }
        if self.reg_p.expansion { value  |= 0x20; }
        if self.reg_p.branch { value |= 0x10; }
        if self.reg_p.decimal { value |= 0x08; }
        if self.reg_p.int_disable { value |= 0x04; }
        if self.reg_p.zero { value |= 0x02; }
        if self.reg_p.carry { value |= 0x01; }
        value
    }
}

trait AddressingMode {
    fn read(&self, cpu: &mut CPU) -> u8;
    fn write(&self, cpu: &mut CPU, value: u8);
}

struct ImmediateAddressingMode;
impl AddressingMode for ImmediateAddressingMode {
    fn read(&self, cpu: &mut CPU) -> u8 {
        cpu.read_inc_pc()
    }
    fn write(&self, _cpu: &mut CPU, _value: u8) {
        panic!("write is not possible in immediate addressing");
    }
}

struct AccumulatorAddressingMode;
impl AddressingMode for AccumulatorAddressingMode {
    fn read(&self, cpu: &mut CPU) -> u8 {
        cpu.reg_a
    }
    fn write(&self, cpu: &mut CPU, value: u8) {
        cpu.reg_a = value;
    }
}

struct MemoryAddressingMode {
    address: u16,
}

impl AddressingMode for MemoryAddressingMode {
    fn read(&self, cpu: &mut CPU) -> u8 {
        cpu.memory.read(self.address)
    }
    fn write(&self, cpu: &mut CPU, value: u8) {
        cpu.memory.write(self.address, value);
    }
}


#[derive(Default, Debug, Copy, Clone)]
struct RegP {
    carry: bool,
    zero: bool,
    int_disable: bool,
    decimal: bool,
    branch: bool,
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
            reg_sp: 0,
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
        self.reg_sp = 0xFD;
        self.reg_p.expansion = true;
        self.reg_p.int_disable = true;
        let mut state = CpuState::default();
        let mut op_cnt = 1;
        loop {
            state.store(&self);
            let opcode = self.read_inc_pc();
            match opcode {
                0x69 => { let v = self.imm(); self.adc(v) },
                0x65 => { let v = self.zp();  self.adc(v) },
                0x75 => { let v = self.zp_x();  self.adc(v) },
                0x6D => { let v = self.abs(); self.adc(v) },
                0x7D => { let v = self.abs_x();  self.adc(v) },
                0x79 => { let v = self.abs_y();  self.adc(v) },
                0x61 => { let v = self.indirect_x();  self.adc(v) },
                0x71 => { let v = self.indirect_y();  self.adc(v) },

                0x29 => { let v = self.imm(); self.and(v) },
                0x25 => { let v = self.zp(); self.and(v) },
                0x35 => { let v = self.zp_x(); self.and(v) },
                0x2D => { let v = self.abs(); self.and(v) },
                0x3D => { let v = self.abs_x(); self.and(v) },
                0x39 => { let v = self.abs_y(); self.and(v) },
                0x21 => { let v = self.indirect_x(); self.and(v) },
                0x31 => { let v = self.indirect_y(); self.and(v) },

                0x0A => { let v = self.acc(); self.asl(v) },
                0x06 => { let v = self.zp();  self.asl(v) },
                0x16 => { let v = self.zp_x();  self.asl(v) },
                0x0E => { let v = self.abs(); self.asl(v) },
                0x1E => { let v = self.abs_x(); self.asl(v) },

                0x90 => { self.bcc() },
                0xB0 => { self.bcs() },
                0xF0 => { self.beq() },
                0x30 => { self.bmi() },
                0xD0 => { self.bne() },
                0x10 => { self.bpl() },
                0x50 => { self.bvc() },
                0x70 => { self.bvs() },

                0x24 => { let v = self.zp() ; self.bit(v) },
                0x2C => { let v = self.abs() ; self.bit(v) },

                0x00 => { self.brk() },

                0x18 => { self.clc() },
                0xD8 => { self.cld() },
                0x58 => { self.cli() },
                0xB8 => { self.clv() },

                0xC9 => { let v = self.imm(); self.cmp(v) },
                0xC5 => { let v = self.zp();  self.cmp(v) },
                0xD5 => { let v = self.zp_x(); self.cmp(v) },
                0xCD => { let v = self.abs(); self.cmp(v) },
                0xDD => { let v = self.abs_x();  self.cmp(v) },
                0xD9 => { let v = self.abs_y(); self.cmp(v) },
                0xC1 => { let v = self.indirect_x(); self.cmp(v) },
                0xD1 => { let v = self.indirect_y();  self.cmp(v) },

                0xE0 => { let v = self.imm(); self.cpx(v) },
                0xE4 => { let v = self.zp();  self.cpx(v) },
                0xEC => { let v = self.abs(); self.cpx(v) },

                0xC0 => { let v = self.imm(); self.cpy(v) },
                0xC4 => { let v = self.zp();  self.cpy(v) },
                0xCC => { let v = self.abs(); self.cpy(v) },

                0xC6 => { let v = self.zp(); self.dec(v) },
                0xD6 => { let v = self.zp_x(); self.dec(v) },
                0xCE => { let v = self.abs(); self.dec(v) },
                0xDE => { let v = self.abs_x(); self.dec(v) },

                0xCA => { self.dex(); },

                0x88 => { self.dey() },

                0x49 => { let v = self.imm(); self.eor(v) },
                0x45 => { let v = self.zp(); self.eor(v) },
                0x55 => { let v = self.zp_x(); self.eor(v) },
                0x4D => { let v = self.abs(); self.eor(v) },
                0x5D => { let v = self.abs_x(); self.eor(v) },
                0x59 => { let v = self.abs_y(); self.eor(v) },
                0x41 => { let v = self.indirect_x(); self.eor(v) },
                0x51 => { let v = self.indirect_y(); self.eor(v) },

                0xE6 => { let v = self.zp(); self.inc(v) },
                0xF6 => { let v = self.zp_x(); self.inc(v) },
                0xEE => { let v = self.abs(); self.inc(v) },
                0xFE => { let v = self.abs_x(); self.inc(v) },

                0xE8 => { self.inx(); },

                0xC8 => { self.iny() },

                0x4C => { self.jmp() },
                0x6C => { self.jmp_indirect() },

                0x20 => { self.jsr() },

                0xA9 => { let v = self.imm(); self.lda(v) },
                0xA5 => { let v = self.zp();  self.lda(v) },
                0xB5 => { let v = self.zp_x();  self.lda(v) },
                0xAD => { let v = self.abs(); self.lda(v) },
                0xBD => { let v = self.abs_x(); self.lda(v) },
                0xB9 => { let v = self.abs_y(); self.lda(v) },
                0xA1 => { let v = self.indirect_x(); self.lda(v) },
                0xB1 => { let v = self.indirect_y(); self.lda(v) },

                0xA2 => { let v = self.imm(); self.ldx(v) },
                0xA6 => { let v = self.zp();  self.ldx(v) },
                0xB6 => { let v = self.zp_y();  self.ldx(v) },
                0xAE => { let v = self.abs(); self.ldx(v) },
                0xBE => { let v = self.abs_y(); self.ldx(v) },

                0xA0 => { let v = self.imm(); self.ldy(v) },
                0xA4 => { let v = self.zp();  self.ldy(v) },
                0xB4 => { let v = self.zp_x();  self.ldy(v) },
                0xAC => { let v = self.abs(); self.ldy(v) },
                0xBC => { let v = self.abs_x(); self.ldy(v) },

                0x4A => { let v = self.acc(); self.lsr(v) },
                0x46 => { let v = self.zp();  self.lsr(v) },
                0x56 => { let v = self.zp_x();  self.lsr(v) },
                0x4E => { let v = self.abs(); self.lsr(v) },
                0x5E => { let v = self.abs_x(); self.lsr(v) },

                0xEA => {  }, // NOP

                0x09 => { let v = self.imm(); self.ora(v) },
                0x05 => { let v = self.zp(); self.ora(v) },
                0x15 => { let v = self.zp_x(); self.ora(v) },
                0x0D => { let v = self.abs(); self.ora(v) },
                0x1D => { let v = self.abs_x(); self.ora(v) },
                0x19 => { let v = self.abs_y(); self.ora(v) },
                0x01 => { let v = self.indirect_x(); self.ora(v) },
                0x11 => { let v = self.indirect_y(); self.ora(v) },

                0x48 => { self.pha() },

                0x08 => { self.php() },

                0x68 => { self.pla() },

                0x28 => { self.plp() },

                0x2A => { let v = self.acc(); self.rol(v) },
                0x26 => { let v = self.zp();  self.rol(v) },
                0x36 => { let v = self.zp_x();  self.rol(v) },
                0x2E => { let v = self.abs(); self.rol(v) },
                0x3E => { let v = self.abs_x(); self.rol(v) },

                0x6A => { let v = self.acc(); self.ror(v) },
                0x66 => { let v = self.zp();  self.ror(v) },
                0x76 => { let v = self.zp_x();  self.ror(v) },
                0x6E => { let v = self.abs(); self.ror(v) },
                0x7E => { let v = self.abs_x(); self.ror(v) },

                0x40 => { self.rti() },

                0x60 => { self.rts() },

                0xE9 => { let v = self.imm(); self.sbc(v) },
                0xE5 => { let v = self.zp();  self.sbc(v) },
                0xF5 => { let v = self.zp_x();  self.sbc(v) },
                0xED => { let v = self.abs(); self.sbc(v) },
                0xFD => { let v = self.abs_x(); self.sbc(v) },
                0xF9 => { let v = self.abs_y(); self.sbc(v) },
                0xE1 => { let v = self.indirect_x(); self.sbc(v) },
                0xF1 => { let v = self.indirect_y(); self.sbc(v) },

                0x38 => { self.sec() },

                0xF8 => { self.sed() },

                0x78 => { self.sei() },

                0x85 => { let v = self.zp(); self.sta(v) },
                0x95 => { let v = self.zp_x(); self.sta(v) },
                0x8D => { let v = self.abs(); self.sta(v) },
                0x9D => { let v = self.abs_x(); self.sta(v) },
                0x99 => { let v = self.abs_y(); self.sta(v) },
                0x81 => { let v = self.indirect_x(); self.sta(v) },
                0x91 => { let v = self.indirect_y(); self.sta(v) },

                0x86 => { let v = self.zp();  self.stx(v) },
                0x96 => { let v = self.zp_y();  self.stx(v) },
                0x8E => { let v = self.abs(); self.stx(v) },

                0x84 => { let v = self.zp();  self.sty(v) },
                0x94 => { let v = self.zp_x();  self.sty(v) },
                0x8C => { let v = self.abs(); self.sty(v) },

                0xAA => { self.tax() },

                0xA8 => { self.tay() },

                0xBA => { self.tsx() },

                0x8A => { self.txa() },

                0x9A => { self.txs() },

                0x98 => { self.tya() },

                0x80 => { self.read_inc_pc(); },

                0x82 => { self.read_inc_pc(); },
                0xC2 => { self.read_inc_pc(); },
                0xE2 => { self.read_inc_pc(); },

                0x04 => { self.read_inc_pc(); },
                0x44 => { self.read_inc_pc(); },
                0x64 => { self.read_inc_pc(); },

                0x89 => { self.read_inc_pc(); },

                0x0C => { self.readw_inc_pc(); },

                0x14 => { self.read_inc_pc(); },
                0x34 => { self.read_inc_pc(); },
                0x54 => { self.read_inc_pc(); },
                0x74 => { self.read_inc_pc(); },
                0xD4 => { self.read_inc_pc(); },
                0xF4 => { self.read_inc_pc(); },

                0x1A => {},
                0x3A => {},
                0x5A => {},
                0x7A => {},
                0xDA => {},
                0xFA => {},

                0x1C => { self.readw_inc_pc(); },
                0x3C => { self.readw_inc_pc(); },
                0x5C => { self.readw_inc_pc(); },
                0x7C => { self.readw_inc_pc(); },
                0xDC => { self.readw_inc_pc(); },
                0xFC => { self.readw_inc_pc(); },

                _ => println!("Unknown opcode: {:X}", opcode)
            }
            println!("{} {:4X} {:2X} UV     A: {:2X} X: {:2X} Y: {:2X} P:{:2X} SP: {:2X} ",
                op_cnt, state.reg_pc, opcode, state.reg_a, state.reg_x, state.reg_y, state.get_p(), state.reg_sp);
            op_cnt = op_cnt + 1;
        }
    }

    // Instructions start here!
    fn brk(&mut self) {
        let pc = self.reg_pc + 1;
        let pch = ((pc & 0xFF00) >> 8) as u8;
        let pcl = (pc & 0x00FF) as u8;
        self.push(pch);
        self.push(pcl);
        self.php();
        self.reg_pc = 0xFFFE;
    }
    fn inc<AM: AddressingMode>(&mut self, am: AM) {
        let value = am.read(self);
        let result = (value as u16 + 1) as u8;
        self.set_zn(result);
        am.write(self, result);
    }
    fn dec<AM: AddressingMode>(&mut self, am: AM) {
        let value = am.read(self);
        let result = (value as i16 - 1) as u8;
        self.set_zn(result);
        am.write(self, result);
    }

    fn asl<AM: AddressingMode>(&mut self, am: AM) {
        let value = am.read(self);
        if value & 0x80 != 0 { self.reg_p.carry = true; }
        else { self.reg_p.carry = false; }
        let result = value << 1;
        self.set_zn(result);
        am.write(self, result);
    }
    fn lsr<AM: AddressingMode>(&mut self, am: AM) {
        let value = am.read(self);
        if value & 0x01 != 0 { self.reg_p.carry = true; }
        else { self.reg_p.carry = false; }
        let result = value >> 1;
        self.set_zn(result);
        am.write(self, result);
    }
    fn rol<AM: AddressingMode>(&mut self, am: AM) {
        let value = am.read(self);
        let bit0 = match self.reg_p.carry {
            true => 0x01,
            false => 0x00,
        };
        if value & 0x80 != 0 { self.reg_p.carry = true; }
        else { self.reg_p.carry = false; }
        let result = (value << 1) | bit0;
        self.set_zn(result);
        am.write(self, result);
    }
    fn ror<AM: AddressingMode>(&mut self, am: AM) {
        let value = am.read(self);
        let bit7 = match self.reg_p.carry {
            true => 0x80,
            false => 0x00,
        };
        if value & 0x01 != 0 { self.reg_p.carry = true; }
        else { self.reg_p.carry = false; }
        let result = (value >> 1) | bit7;
        self.set_zn(result);
        am.write(self, result);
    }
    fn rti(&mut self) {
        let p = self.pop();
        self.set_p(p);
        let pcl = self.pop();
        let pch = self.pop();
        self.reg_pc = (pch as u16) << 8 | pcl as u16;
    }
    fn tax(&mut self) {
        let value = self.reg_a;
        self.reg_x = self.set_zn(value);
    }
    fn tay(&mut self) {
        let value = self.reg_a;
        self.reg_y = self.set_zn(value);
    }
    fn tsx(&mut self) {
        let value = self.reg_sp;
        self.reg_x = self.set_zn(value);
    }
    fn txa(&mut self) {
        let value = self.reg_x;
        self.reg_a = self.set_zn(value);
    }
    fn txs(&mut self) {
        let value = self.reg_x;
        self.reg_sp = value;
    }
    fn tya(&mut self) {
        let value = self.reg_y;
        self.reg_a = self.set_zn(value);
    }
    fn lda<AM: AddressingMode>(&mut self, am: AM) {
        let value = am.read(self);
        self.reg_a = self.set_zn(value);
    }
    fn ldx<AM: AddressingMode>(&mut self, am: AM) {
        let value = am.read(self);
        self.reg_x = self.set_zn(value);
    }
    fn ldy<AM: AddressingMode>(&mut self, am: AM) {
        let value = am.read(self);
        self.reg_y = self.set_zn(value);
    }
    fn sta<AM: AddressingMode>(&mut self, am: AM) {
        let value = self.reg_a;
        am.write(self, value);
    }
    fn stx<AM: AddressingMode>(&mut self, am: AM) {
        let value = self.reg_x;
        am.write(self, value);
    }
    fn sty<AM: AddressingMode>(&mut self, am: AM) {
        let value = self.reg_y;
        am.write(self, value);
    }
    fn inx(&mut self) {
        let result = self.reg_x as u16 + 1;
        self.reg_x = self.set_zn(result as u8);
    }
    fn iny(&mut self) {
        let result = self.reg_y as u16 + 1;
        self.reg_y = self.set_zn(result as u8);
    }
    fn dex(&mut self) {
        let result = self.reg_x as i16 - 1;
        self.reg_x = self.set_zn(result as u8);
    }
    fn dey(&mut self) {
        let result = self.reg_y as i16 - 1;
        self.reg_y = self.set_zn(result as u8);
    }
    fn adc<AM: AddressingMode>(&mut self, am: AM) {
        let value = am.read(self);
        let mut tmp_result = self.reg_a as u16 + value as u16;

        if self.reg_p.carry { tmp_result += 1; }

        if tmp_result & 0x100 != 0 { self.reg_p.carry = true; }
        else { self.reg_p.carry = false; }

        let result = tmp_result as u8;

        if (self.reg_a & 0x80) == 0 && (value & 0x80) == 0 && (result & 0x80) != 0 {
            self.reg_p.overflow = true;
        }
        else if (self.reg_a & 0x80) != 0 && (value & 0x80) != 0 && (result & 0x80) == 0 {
            self.reg_p.overflow = true;
        }
        else {
            self.reg_p.overflow = false;
        }
        self.reg_a = self.set_zn(result);
    }
    fn sbc<AM: AddressingMode>(&mut self, am: AM) {
        let a = self.reg_a;
        let m = am.read(self);
        let mut result = a as i16 - m as i16;
        if !self.reg_p.carry { result -= 1; }

        if result & 0x100 == 0 { self.reg_p.carry = true; }
        else { self.reg_p.carry = false; }

        let result = result as u8;

        if (a & 0x80) == 0 && (m & 0x80) != 0 && (result & 0x80) != 0 {
            self.reg_p.overflow = true;
        }
        else if (a & 0x80) != 0 && (m & 0x80) == 0 && (result & 0x80) == 0 {
            self.reg_p.overflow = true;
        }
        else {
            self.reg_p.overflow = false;
        }
        self.reg_a = self.set_zn(result);
    }
    fn jmp(&mut self) {
        let address = self.memory.readw(self.reg_pc);
        self.reg_pc = address;
    }
    fn jmp_indirect(&mut self) {
        let indirect_address = self.memory.readw(self.reg_pc);
        let address: u16;
        if indirect_address & 0x00FF == 0x00FF {
            // implement CPU bug
            let page = indirect_address & 0xFF00;
            let lsb = self.memory.read(page | indirect_address & 0x00FF);
            let msb = self.memory.read(page);
            address = ((msb as u16) << 8) | lsb as u16;
            println!("JMP CPU BUG: {:4X} {:4X} {:4X} {:4X} {:4X} {:4X}", indirect_address, page, page | indirect_address & 0x00FF, lsb, msb, address);
        }
        else {
            address = self.memory.readw(indirect_address);
        }
        self.reg_pc = address;
    }
    fn jsr(&mut self) {
        let address = self.readw_inc_pc();
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
        self.push(value | 0x10); // always set the break bit on the stack
    }
    fn plp(&mut self) {
        let value = self.pop();
        self.set_p(value & (!0x10)); // always clear the break bit when popping from the stack
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
    fn and<AM: AddressingMode>(&mut self, am: AM) {
        let value = self.reg_a & am.read(self);
        self.reg_a = self.set_zn(value);
    }
    fn bit<AM: AddressingMode>(&mut self, am: AM) {
        let a = self.reg_a;
        let value = am.read(self);
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
    fn cmp<AM: AddressingMode>(&mut self, am: AM) {
        let register = self.reg_a;
        self.compare(register, am);
    }
    fn cpx<AM: AddressingMode>(&mut self, am: AM) {
        let register = self.reg_x;
        self.compare(register, am);
    }
    fn cpy<AM: AddressingMode>(&mut self, am: AM) {
        let register = self.reg_y;
        self.compare(register, am);
    }
    fn compare<AM: AddressingMode>(&mut self, register: u8, am: AM) {
        let value = am.read(self);
        //println!("{} {}", self.reg_a, value);
        let v = (register as i16) - (value as i16);

        if (v & 0x100) == 0 { self.reg_p.carry = true; }
        else { self.reg_p.carry = false; }

        self.set_zn(v as u8);
    }
    fn eor<AM: AddressingMode>(&mut self, am: AM) {
        let value = am.read(self);
        let v = self.reg_a ^ value;
        self.reg_a = self.set_zn(v);
    }
    fn ora<AM: AddressingMode>(&mut self, am: AM) {
        let value = am.read(self);
        let v = self.reg_a | value;
        self.reg_a = self.set_zn(v);
    }
    fn bcs(&mut self) {
        let rel = self.read_inc_pc() as i8;
        if self.reg_p.carry == true {
            self.branch(rel);
        }
    }
    fn bcc(&mut self) {
        let rel = self.read_inc_pc() as i8;
        if self.reg_p.carry == false {
            self.branch(rel);
        }
    }
    fn beq(&mut self) {
        let rel = self.read_inc_pc() as i8;
        if self.reg_p.zero == true {
            self.branch(rel);
        }
    }
    fn bne(&mut self) {
        let rel = self.read_inc_pc() as i8;
        if self.reg_p.zero == false {
            self.branch(rel);
        }
    }
    fn bmi(&mut self) {
        let rel = self.read_inc_pc() as i8;
        if self.reg_p.negative == true {
            self.branch(rel);
        }
    }
    fn bpl(&mut self) {
        let rel = self.read_inc_pc() as i8;
        if self.reg_p.negative == false {
            self.branch(rel);
        }
    }
    fn bvc(&mut self) {
        let rel = self.read_inc_pc() as i8;
        if self.reg_p.overflow == false {
            self.branch(rel);
        }
    }
    fn bvs(&mut self) {
        let rel = self.read_inc_pc() as i8;
        if self.reg_p.overflow == true {
            self.branch(rel);
        }
    }

    // Address mode functions. Each one should return an AddressingMode
    fn acc(&mut self) -> AccumulatorAddressingMode {
        AccumulatorAddressingMode
    }
    fn imm(&mut self) -> ImmediateAddressingMode {
        ImmediateAddressingMode
    }
    fn zp(&mut self) -> MemoryAddressingMode {
        MemoryAddressingMode { address: self.read_inc_pc() as u16 }
    }
    fn zp_x(&mut self) -> MemoryAddressingMode {
        MemoryAddressingMode { address: (self.read_inc_pc() as u16 + self.reg_x as u16) & 0x00FF }
    }
    fn zp_y(&mut self) -> MemoryAddressingMode {
        MemoryAddressingMode { address: (self.read_inc_pc() as u16 + self.reg_y as u16) & 0x00FF }
    }
    fn abs(&mut self) -> MemoryAddressingMode {
        MemoryAddressingMode { address: self.readw_inc_pc() }
    }
    fn abs_x(&mut self) -> MemoryAddressingMode {
        MemoryAddressingMode { address: self.readw_inc_pc() + self.reg_x as u16 }
    }
    fn abs_y(&mut self) -> MemoryAddressingMode {
        MemoryAddressingMode { address: (self.readw_inc_pc() as u32 + self.reg_y as u32) as u16 }
    }
    fn indirect_x(&mut self) -> MemoryAddressingMode {
        let x = self.reg_x;
        let ial = self.read_inc_pc();
        let address = self.memory.readw((ial + x) as u16);
        MemoryAddressingMode { address: address }
    }
    fn indirect_y(&mut self) -> MemoryAddressingMode {
        let y = self.reg_y;
        let ial = self.read_inc_pc();
        println!("Indirect Y:");
        println!("IAL: {:X} Y: {:X} Indirect: {:X}", ial as u16, y as u16, self.memory.readw_zp(ial as u16));
        let address = (self.memory.readw_zp(ial as u16) as u32) + y as u32;
        println!("Indirect Y address: {:X} {:X}", address, address as u16);
        MemoryAddressingMode { address: address as u16 }
    }

    // Utility functions (not instructions)
    fn get_p(&self) -> u8 {
        let mut value: u8 = 0;
        if self.reg_p.negative { value |= 0x80; }
        if self.reg_p.overflow { value |= 0x40; }
        if self.reg_p.expansion { value  |= 0x20; }
        if self.reg_p.branch { value |= 0x10; }
        if self.reg_p.decimal { value |= 0x08; }
        if self.reg_p.int_disable { value |= 0x04; }
        if self.reg_p.zero { value |= 0x02; }
        if self.reg_p.carry { value |= 0x01; }
        value
    }
    fn set_p(&mut self, value: u8) {
        self.reg_p.negative = match value & 0x80 { 0 => false, _ => true };
        self.reg_p.overflow = match value & 0x40 { 0 => false, _ => true };
        self.reg_p.expansion = true;
        self.reg_p.branch = match value & 0x10 { 0 => false, _ => true };
        self.reg_p.decimal = match value & 0x08 { 0 => false, _ => true };
        self.reg_p.int_disable = match value & 0x04 { 0 => false, _ => true };
        self.reg_p.zero = match value & 0x02 { 0 => false, _ => true };
        self.reg_p.carry = match value & 0x01 { 0 => false, _ => true };
    }
    fn branch(&mut self, rel: i8) {
        let newpc = (self.reg_pc as i32 + rel as i32) as u16;
        println!("BRANCH: {:X} {} {:X}", self.reg_pc, rel, newpc);
        self.reg_pc = newpc;
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
    fn set_zn(&mut self, value: u8) -> u8 {
        self.reg_p.zero = (value == 0) as bool;
        self.reg_p.negative = ((value & 0x80) != 0) as bool;
        value
    }
    fn push(&mut self, value: u8) {
        let address = 0x0100 | self.reg_sp as u16;
        self.memory.write(address, value);
        self.reg_sp = self.reg_sp - 1;
    }
    fn pop(&mut self) -> u8 {
        self.reg_sp = self.reg_sp + 1;
        let address = 0x0100 | self.reg_sp as u16;
        self.memory.read(address)
    }
}
