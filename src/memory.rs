use super::mapper;

struct RAM {
    ram: [u8; 0x800]
}

impl RAM {
    fn read(&self, address: u16) -> u8 {
        self.ram[address as usize & 0x7FF]
    }
    fn write(&mut self, address: u16, value: u8) {
        self.ram[address as usize & 0x7FF] = value;
    }
}

pub struct MemMap {
    ram: RAM,
    //ppu: PPU,
    //apu: APU,
    //controllers: Controllers,
    mapper: mapper::Mapper,
}

impl MemMap {
    pub fn new(mapper: mapper::Mapper) -> MemMap {
        MemMap {
            ram: RAM { ram: [0; 0x800] },
            mapper: mapper,
        }
    }
    pub fn read(&self, address: u16) -> u8 {
        if address < 0x2000 {
            self.ram.read(address)
        }
        else if address >= 0x8000 {
            self.mapper.read(address)
        }
        else {
            0
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        if address < 0x2000 {
            self.ram.write(address, value);
        }
    }

    pub fn readw(&self, address: u16) -> u16 {
         ((self.read(address + 1) as u16) << 8) | (self.read(address) as u16)
    }
}
