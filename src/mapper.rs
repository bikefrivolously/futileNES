use rom;

pub struct Mapper {
    // TODO: investigate the posibility of using
    // slices to reference arrays stored in the rom.prg_rom vector.
    // Are there any pros and cons to this approach?
    upper_bank: [u8; 0x4000],
    lower_bank: [u8; 0x4000],
    rom: rom::INesFile,
}

impl Mapper {
    pub fn new(rom: rom::INesFile) -> Mapper {
        let up = [0u8; 0x4000];
        let lo = [0u8; 0x4000];
        Mapper { upper_bank: up, lower_bank: lo, rom: rom }
    }
    pub fn read(&self, address: u16) -> u8 {
        if address < 0xC000 {
            self.lower_bank[(address - 0x8000) as usize]
        }
        else {
            self.upper_bank[(address - 0xC000) as usize]
        }
    }
    pub fn load(&mut self) {
        if self.rom.mapper == 0 {
            if self.rom.prg_rom_cnt == 1 {
                // load single bank to upper and lower
                self.upper_bank = self.rom.prg_rom[0];
                self.lower_bank = self.rom.prg_rom[0];
            }
            else {
                self.lower_bank = self.rom.prg_rom[0];
                self.upper_bank = self.rom.prg_rom[1];
            }
        }
    }
}
