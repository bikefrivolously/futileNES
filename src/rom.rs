use std::io::Write;
use std::fs::File;

#[allow(dead_code)]
pub struct INesFile {
    magic: [u8; 4],
    has_trainer: bool,
    pub mapper: u8,
    pub prg_rom_cnt:    u8,
    prg_rom_size:   u32,
    chr_rom_size:   u32,
    prg_ram_size:   u32,
    flags6:         u8,
    flags7:         u8,
    flags9:         u8,
    flags10:        u8,
    zeros:          [u8; 5],
    trainer: [u8; 0x200],
    pub prg_rom: Vec<[u8; 0x4000]>, // TODO make this a Vec of [u8; 0x4000] (Vector of 16kB pages)
    chr_rom: Vec<u8>,
    pc_inst_rom: Vec<u8>,
    pc_prom: Vec<u8>,
    title: Vec<u8>,
}

impl INesFile {
    pub fn load(bin: Vec<u8>) -> INesFile {
        let header = &bin[0..16];
        let m = [ header[0], header[1], header[2], header[3] ];
        if m[0] != 'N' as u8 || m[1] != 'E' as u8 || m[2] != 'S' as u8 || m[3] != 0x1A {
            panic!("Invalid ROM!");
        }
        let prg_rom_cnt = header[4];
        let prg_rom_size = prg_rom_cnt as u32 * 16384;

        let chr_rom_cnt = header[5];
        let chr_rom_size = chr_rom_cnt as u32 * 8192;

        let flags6 = header[6];
        let flags7 = header[7];
        let prg_ram_size = header[8] as u32 * 8192;
        let flags9 = header[9];
        let flags10 = header[10];
        let zeros = [ header[11], header[12], header[13], header[14], header[15]];

        let mapper = (flags7 & 0xF0) | ((flags6 & 0xF0) >> 4);

        let mut pos: usize = 16;

        let has_trainer: bool;
        let mut trainer = [0u8; 0x200];
        let mut prg_rom: Vec<[u8; 0x4000]> = Vec::new();
        if (flags6 & 0x04) > 0 {
            has_trainer = true;
            for i in 0..0x200 {
                trainer[i] = bin[pos];
                pos = pos + 1;
            }
        }
        else {
            has_trainer = false;
        }

        for _i in 0..prg_rom_cnt {
            let mut page = [0u8; 0x4000];
            for j in 0..0x4000 {
                page[j] = bin[pos];
                pos = pos + 1;
            }
            prg_rom.push(page);
        }



        INesFile {
            magic: m,
            has_trainer: has_trainer,
            mapper: mapper,
            prg_rom_size: prg_rom_size,
            prg_rom_cnt: prg_rom_cnt,
            chr_rom_size: chr_rom_size,
            flags6: flags6,
            flags7: flags7,
            prg_ram_size: prg_ram_size,
            flags9: flags9,
            flags10: flags10,
            zeros: zeros,
            trainer: trainer,
            prg_rom: prg_rom,
            //TODO finish initializing these properly
            chr_rom: vec![0],
            pc_inst_rom: vec![0],
            pc_prom: vec![0],
            title: vec![0],
        }
    }
    #[allow(dead_code)]
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
    #[allow(dead_code)]
    pub fn dump_prg_rom(&self) {
        let mut f = File::create("prg0.rom").unwrap();
        let buf = &self.prg_rom[0][..];
        f.write_all(buf).expect("failed to write to file");
    }
}
