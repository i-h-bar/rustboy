use std::collections::HashMap;

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
        (0x08, "ROM+RAM 1"),
        (0x09, "ROM+RAM+BATTERY 1"),
        (0x0B, "MMM01"),
        (0x0C, "MMM01+RAM"),
        (0x0D, "MMM01+RAM+BATTERY"),
        (0x0F, "MBC3+TIMER+BATTERY"),
        (0x10, "MBC3+TIMER+RAM+BATTERY 2"),
        (0x11, "MBC3"),
        (0x12, "MBC3+RAM 2"),
        (0x13, "MBC3+RAM+BATTERY 2"),
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
}

fn rom_size(value: u16) -> u16 {
    assert!(value <= 8);
    32 * (1 << value)
}

struct Header {
    entry: u8,
    logo: u8,
    title: String,
    new_lic_code: u16,
    sgb_flag: u8,
    cart_type: u8,
    rom_size: u8,
    ram_size: u8,
    dest_code: u8,
    old_lic_code: u8,
    version: u8,
    checksum: u8,
    global_checksum: u16,
}

pub struct Cartridge {
    filename: String,
    rom_size: u32,
    rom_data: Vec<u8>,
    header: Header,
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
            (8, 8192)
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
