// 0x0000 - 0x3FFF : ROM Bank 0
// 0x4000 - 0x7FFF : ROM Bank 1 - Switchable
// 0x8000 - 0x97FF : CHR RAM
// 0x9800 - 0x9BFF : BG Map 1
// 0x9C00 - 0x9FFF : BG Map 2
// 0xA000 - 0xBFFF : Cartridge RAM
// 0xC000 - 0xCFFF : RAM Bank 0
// 0xD000 - 0xDFFF : RAM Bank 1-7 - switchable - Color only
// 0xE000 - 0xFDFF : Reserved - Echo RAM
// 0xFE00 - 0xFE9F : Object Attribute Memory
// 0xFEA0 - 0xFEFF : Reserved - Unusable
// 0xFF00 - 0xFF7F : I/O Registers
// 0xFF80 - 0xFFFE : Zero Page

use crate::cartridge::Cartridge;

pub struct Bus {
    pub cartridge: Cartridge,
}
impl Bus {
    pub fn read(&self, address: u16) -> u16 {
        if address < 0x8000 {
            return self.cartridge.read(address) as u16;
        } else {
            todo!()
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        if address < 0x8000 {
            self.cartridge.write(address, value)
        } else {
            todo!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read() {
        let bus = Bus {
            cartridge: Cartridge::from("test_roms/01-special.test"),
        };
        assert_eq!(bus.read(0x101), 195)
    }

    #[test]
    fn test_write() {
        let mut bus = Bus {
            cartridge: Cartridge::from("test_roms/01-special.test"),
        };
        assert_ne!(bus.read(0x7999), 255);
        bus.write(0x7999, 255);
        assert_eq!(bus.read(0x7999), 255);
    }
}
