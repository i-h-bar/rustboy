use std::collections::HashMap;
use std::error::Error;
use std::fs;

use lazy_static::lazy_static;

lazy_static! {
    static ref LIC_MAP: HashMap<&'static str, &'static str> = [
        ("00", "None"),
        ("01", "Nintendo R&D1"),
        ("08", "Capcom"),
        ("13", "Electronic Arts"),
        ("18", "Hudson Soft"),
        ("19", "b-ai"),
        ("20", "kss"),
        ("22", "pow"),
        ("24", "PCM Complete"),
        ("25", "san-x"),
        ("28", "Kemco Japan"),
        ("29", "seta"),
        ("30", "Viacom"),
        ("31", "Nintendo"),
        ("32", "Bandai"),
        ("33", "Ocean/Acclaim"),
        ("34", "Konami"),
        ("35", "Hector"),
        ("37", "Taito"),
        ("38", "Hudson"),
        ("39", "Banpresto"),
        ("41", "Ubi Soft"),
        ("42", "Atlus"),
        ("44", "Malibu"),
        ("46", "angel"),
        ("47", "Bullet-Proof"),
        ("49", "irem"),
        ("50", "Absolute"),
        ("51", "Acclaim"),
        ("52", "Activision"),
        ("53", "American sammy"),
        ("54", "Konami"),
        ("55", "Hi tech entertainment"),
        ("56", "LJN"),
        ("57", "Matchbox"),
        ("58", "Mattel"),
        ("59", "Milton Bradley"),
        ("60", "Titus"),
        ("61", "Virgin"),
        ("64", "LucasArts"),
        ("67", "Ocean"),
        ("69", "Electronic Arts"),
        ("70", "Infogrames"),
        ("71", "Interplay"),
        ("72", "Broderbund"),
        ("73", "sculptured"),
        ("75", "sci"),
        ("78", "THQ"),
        ("79", "Accolade"),
        ("80", "misawa"),
        ("83", "lozc"),
        ("86", "Tokuma Shoten Intermedia"),
        ("87", "Tsukuda Original"),
        ("91", "Chunsoft"),
        ("92", "Video system"),
        ("93", "Ocean/Acclaim"),
        ("95", "Varie"),
        ("96", "Yonezawa/s’pal"),
        ("97", "Kaneko"),
        ("99", "Pack in soft"),
        ("9H", "Bottom Up"),
        ("A4", "Konami (Yu-Gi-Oh!)")
    ]
    .iter()
    .copied()
    .collect();
    static ref CART_TYPE_MAP: HashMap<u8, &'static str> = [
        (0x00, "ROM ONLY"),
        (0x01, "MBC1"),
        (0x02, "MBC1+RAM"),
        (0x03, "MBC1+RAM+BATTERY"),
        (0x05, "MBC2"),
        (0x06, "MBC2+BATTERY"),
        (0x08, "ROM+RAM"),
        (0x09, "ROM+RAM+BATTERY"),
        (0x0B, "MMM01"),
        (0x0C, "MMM01+RAM"),
        (0x0D, "MMM01+RAM+BATTERY"),
        (0x0F, "MBC3+TIMER+BATTERY"),
        (0x10, "MBC3+TIMER+RAM+BATTERY"),
        (0x11, "MBC3"),
        (0x12, "MBC3+RAM"),
        (0x13, "MBC3+RAM+BATTERY"),
        (0x19, "MBC5"),
        (0x1A, "MBC5+RAM"),
        (0x1B, "MBC5+RAM+BATTERY"),
        (0x1C, "MBC5+RUMBLE"),
        (0x1D, "MBC5+RUMBLE+RAM"),
        (0x1E, "MBC5+RUMBLE+RAM+BATTERY"),
        (0x20, "MBC6"),
        (0x22, "MBC7+SENSOR+RUMBLE+RAM+BATTERY"),
        (0xFC, "POCKET CAMERA"),
        (0xFD, "BANDAI TAMA5"),
        (0xFE, "HuC3"),
        (0xFF, "HuC1+RAM+BATTERY"),
    ]
    .iter()
    .copied()
    .collect();
    static ref OLD_LIC_MAP: HashMap<u8, &'static str> = [
        (0x00, "None"),
        (0x01, "Nintendo"),
        (0x08, "Capcom"),
        (0x09, "Hot-B"),
        (0x0A, "Jaleco"),
        (0x0B, "Coconuts Japan"),
        (0x0C, "Elite Systems"),
        (0x13, "EA (Electronic Arts)"),
        (0x18, "Hudsonsoft"),
        (0x19, "ITC Entertainment"),
        (0x1A, "Yanoman"),
        (0x1D, "Japan Clary"),
        (0x1F, "Virgin Interactive"),
        (0x24, "PCM Complete"),
        (0x25, "San-X"),
        (0x28, "Kotobuki Systems"),
        (0x29, "Seta"),
        (0x30, "Infogrames"),
        (0x31, "Nintendo"),
        (0x32, "Bandai"),
        (0x33, "Use new lic map"),
        (0x34, "Konami"),
        (0x35, "HectorSoft"),
        (0x38, "Capcom"),
        (0x39, "Banpresto"),
        (0x3C, ".Entertainment i"),
        (0x3E, "Gremlin"),
        (0x41, "Ubisoft"),
        (0x42, "Atlus"),
        (0x44, "Malibu"),
        (0x46, "Angel"),
        (0x47, "Spectrum Holoby"),
        (0x49, "Irem"),
        (0x4A, "Virgin Interactive"),
        (0x4D, "Malibu"),
        (0x4F, "U.S. Gold"),
        (0x50, "Absolute"),
        (0x51, "Acclaim"),
        (0x52, "Activision"),
        (0x53, "American Sammy"),
        (0x54, "GameTek"),
        (0x55, "Park Place"),
        (0x56, "LJN"),
        (0x57, "Matchbox"),
        (0x59, "Milton Bradley"),
        (0x5A, "Mindscape"),
        (0x5B, "Romstar"),
        (0x5C, "Naxat Soft"),
        (0x5D, "Tradewest"),
        (0x60, "Titus"),
        (0x61, "Virgin Interactive"),
        (0x67, "Ocean Interactive"),
        (0x69, "EA (Electronic Arts)"),
        (0x6E, "Elite Systems"),
        (0x6F, "Electro Brain"),
        (0x70, "Infogrames"),
        (0x71, "Interplay"),
        (0x72, "Broderbund"),
        (0x73, "Sculptered Soft"),
        (0x75, "The Sales Curve"),
        (0x78, "t.hq"),
        (0x79, "Accolade"),
        (0x7A, "Triffix Entertainment"),
        (0x7C, "Microprose"),
        (0x7F, "Kemco"),
        (0x80, "Misawa Entertainment"),
        (0x83, "Lozc"),
        (0x86, "Tokuma Shoten Intermedia"),
        (0x8B, "Bullet-Proof Software"),
        (0x8C, "Vic Tokai"),
        (0x8E, "Ape"),
        (0x8F, "I’Max"),
        (0x91, "Chunsoft Co."),
        (0x92, "Video System"),
        (0x93, "Tsubaraya Productions Co."),
        (0x95, "Varie Corporation"),
        (0x96, "Yonezawa/S’Pal"),
        (0x97, "Kaneko"),
        (0x99, "Arc"),
        (0x9A, "Nihon Bussan"),
        (0x9B, "Tecmo"),
        (0x9C, "Imagineer"),
        (0x9D, "Banpresto"),
        (0x9F, "Nova"),
        (0xA1, "Hori Electric"),
        (0xA2, "Bandai"),
        (0xA4, "Konami"),
        (0xA6, "Kawada"),
        (0xA7, "Takara"),
        (0xA9, "Technos Japan"),
        (0xAA, "Broderbund"),
        (0xAC, "Toei Animation"),
        (0xAD, "Toho"),
        (0xAF, "Namco"),
        (0xB0, "acclaim"),
        (0xB1, "ASCII or Nexsoft"),
        (0xB2, "Bandai"),
        (0xB4, "Square Enix"),
        (0xB6, "HAL Laboratory"),
        (0xB7, "SNK"),
        (0xB9, "Pony Canyon"),
        (0xBA, "Culture Brain"),
        (0xBB, "Sunsoft"),
        (0xBD, "Sony Imagesoft"),
        (0xBF, "Sammy"),
        (0xC0, "Taito"),
        (0xC2, "Kemco"),
        (0xC3, "Squaresoft"),
        (0xC4, "Tokuma Shoten Intermedia"),
        (0xC5, "Data East"),
        (0xC6, "Tonkinhouse"),
        (0xC8, "Koei"),
        (0xC9, "UFL"),
        (0xCA, "Ultra"),
        (0xCB, "Vap"),
        (0xCC, "Use Corporation"),
        (0xCD, "Meldac"),
        (0xCE, ".Pony Canyon or"),
        (0xCF, "Angel"),
        (0xD0, "Taito"),
        (0xD1, "Sofel"),
        (0xD2, "Quest"),
        (0xD3, "Sigma Enterprises"),
        (0xD4, "ASK Kodansha Co."),
        (0xD6, "Naxat Soft"),
        (0xD7, "Copya System"),
        (0xD9, "Banpresto"),
        (0xDA, "Tomy"),
        (0xDB, "LJN"),
        (0xDD, "NCS"),
        (0xDE, "Human"),
        (0xDF, "Altron"),
        (0xE0, "Jaleco"),
        (0xE1, "Towa Chiki"),
        (0xE2, "Yutaka"),
        (0xE3, "Varie"),
        (0xE5, "Epcoh"),
        (0xE7, "Athena"),
        (0xE8, "Asmik ACE Entertainment"),
        (0xE9, "Natsume"),
        (0xEA, "King Records"),
        (0xEB, "Atlus"),
        (0xEC, "Epic/Sony Records"),
        (0xEE, "IGS"),
        (0xF0, "A Wave"),
        (0xF3, "Extreme Entertainment"),
        (0xFF, "LJN")
    ]
    .iter()
    .copied()
    .collect();
}

fn rom_size(value: u16) -> u16 {
    assert!(value <= 8);
    32 * (1 << value)
}

struct Header {
    entry: Vec<u8>,
    logo: Vec<u8>,
    title: String,
    new_lic_code: Vec<u8>,
    sgb_flag: u8,
    cart_type: u8,
    rom_size: u16,
    ram_size: u8,
    dest_code: u8,
    old_lic_code: u8,
    version: u8,
    checksum: u8,
    global_checksum: Vec<u8>,
}

pub struct Cartridge {
    filename: String,
    rom_size: usize,
    pub rom_data: Vec<u8>,
    header: Header,
}

impl Header {
    fn from(rom_data: &Vec<u8>) -> Result<Self, Box<dyn Error>> {
        let entry = rom_data[0x100..=0x103].to_vec();
        let logo = rom_data[0x104..=0x133].to_vec();
        let title = String::from_utf8(rom_data[0x134..=0x143].to_vec())?;
        let new_lic_code = rom_data[0x144..=0x145].to_vec();
        let sgb_flag = rom_data[0x146];
        let cart_type = rom_data[0x147];
        let rom_size = rom_size(rom_data[0x148] as u16);
        let ram_size = rom_data[0x149];
        let dest_code = rom_data[0x14A];
        let old_lic_code = rom_data[0x14B];
        let version = rom_data[0x14C];
        let checksum = rom_data[0x14D];
        let global_checksum = rom_data[0x14E..=0x14F].to_vec();

        println!("{title}");

        Ok(Self {
            entry,
            logo,
            title,
            new_lic_code,
            sgb_flag,
            cart_type,
            rom_size,
            ram_size,
            dest_code,
            old_lic_code,
            version,
            checksum,
            global_checksum,
        })
    }
}

impl Cartridge {
    pub fn from(rom_file: &str) -> Self {
        let rom_data = fs::read(rom_file).expect("Could not open ROM");
        let rom_size = rom_data.len();
        let filename = rom_file.to_string();
        let header = Header::from(&rom_data).expect("Could not read header correctly");

        Self {
            filename,
            rom_size,
            rom_data,
            header,
        }
    }

    fn checksum(&mut self) -> bool {
        let mut checksum: u16 = 0;
        for num in &mut self.rom_data[0x0134..=0x014C] {
            checksum = checksum - (*num as u16) - 1;
        }

        checksum.to_be_bytes()[1] == self.rom_data[0x014D]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rom_size() {
        let data = [
            (0, 32),
            (1, 64),
            (2, 128),
            (3, 256),
            (4, 512),
            (5, 1024),
            (6, 2048),
            (7, 4096),
            (8, 8192),
        ];

        for (value, answer) in data {
            assert_eq!(rom_size(value), answer);
        }
    }

    #[test]
    #[should_panic]
    fn rom_size_no_great_then_8() {
        let _ = rom_size(9);
    }
}
