use std::io::Write;
use std::fs::File;


pub struct iNESFile {
    magic: [u8; 4],
    has_trainer: bool,
    mapper: u8,
    prg_rom_size:   u32,
    chr_rom_size:   u32,
    prg_ram_size:   u32,
    flags6:         u8,
    flags7:         u8,
    flags9:         u8,
    flags10:        u8,
    zeros:          [u8; 5],
    trainer: Vec<u8>,
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
    pc_inst_rom: Vec<u8>,
    pc_prom: Vec<u8>,
    title: Vec<u8>,
}

impl iNESFile {
    pub fn load(bin: Vec<u8>) -> iNESFile {
        let header = &bin[0..16];
        let m = [ header[0], header[1], header[2], header[3] ];
        if m[0] != 'N' as u8 || m[1] != 'E' as u8 || m[2] != 'S' as u8 || m[3] != 0x1A {
            panic!("Invalid ROM!");
        }
        let prg_rom_size = header[4] as u32 * 16384;
        let chr_rom_size = header[5] as u32 * 8192;
        let flags6 = header[6];
        let flags7 = header[7];
        let prg_ram_size = header[8] as u32 * 8192;
        let flags9 = header[9];
        let flags10 = header[10];
        let zeros = [ header[11], header[12], header[13], header[14], header[15]];

        let mapper = (flags7 & 0xF0) | ((flags6 & 0xF0) >> 4);

        let has_trainer: bool;
        let mut trainer: Vec<u8> = Vec::new();
        let mut prg_rom: Vec<u8> = Vec::new();
        if (flags6 & 0x04) > 0 {
            has_trainer = true;
            trainer.extend_from_slice(&bin[16..16+512]);
            prg_rom.extend_from_slice(&bin[16+512..16+512+prg_rom_size as usize]);
        }
        else {
            has_trainer = false;
            prg_rom.extend_from_slice(&bin[16..16+prg_rom_size as usize]);
        }


        iNESFile {
            magic: m,
            has_trainer: has_trainer,
            mapper: mapper,
            prg_rom_size: prg_rom_size,
            chr_rom_size: chr_rom_size,
            flags6: flags6,
            flags7: flags7,
            prg_ram_size: prg_ram_size,
            flags9: flags9,
            flags10: flags10,
            zeros: zeros,
            trainer: vec![0],
            prg_rom: prg_rom,
            chr_rom: vec![0],
            pc_inst_rom: vec![0],
            pc_prom: vec![0],
            title: vec![0],
        }
    }
    pub fn info(&self) {
        println!("has_trainer: {}", self.has_trainer);
        println!("mapper: {}", self.mapper);
        println!("prg_rom_size: {}", self.prg_rom_size);
        println!("chr_rom_size: {}", self.chr_rom_size);
        println!("prg_ram_size: {}", self.prg_ram_size);
        println!("flags6: 0x{:x}", self.flags6);
        println!("flags7: 0x{:x}", self.flags7);
        println!("flags9: 0x{:x}", self.flags9);
        println!("flags10: 0x{:x}", self.flags10);
        println!("has_trainer: {}", self.has_trainer);
        //println!("prg_rom: {:?}", self.prg_rom);
    }

    pub fn dump_prg_rom(&self) {
        let mut f = File::create("prg.rom").unwrap();
        let buf = &self.prg_rom[..];
        f.write_all(buf);
    }
}
