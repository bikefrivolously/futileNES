use super::memory;
use super::mapper;

#[derive(Default, Debug)]
struct RegP {
    carry: bool,
    zero: bool,
    int_disable: bool,
    bcd: bool,
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
        loop {
            print!("{:4X}   ", self.reg_pc);
            let opcode = self.read_inc_pc();
            let v: u16;
            match opcode {

                0x4C => { v = self.readw_inc_pc(); self.jmp(v) },   // JMP absolute
                0xA2 => { v = self.read_inc_pc() as u16; self.ldx(v as u8) },    // LDX immediate
                _ => panic!("Unknown opcode: {:X}", opcode)
            }
            println!("{:X} {:4X}    PC: {:4X} SP: {:X} A: {:X} X: {:X} Y: {:X}",
                opcode, v, self.reg_pc, self.reg_sp, self.reg_a, self.reg_x, self.reg_y);
        }
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

    fn jmp(&mut self, address: u16) {
        self.reg_pc = address;
    }
    fn ldx(&mut self, value: u8) {
        self.reg_x = value;
    }
}
