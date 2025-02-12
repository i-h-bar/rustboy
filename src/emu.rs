use crate::bus::Bus;
use crate::cartridge::Cartridge;
use crate::cpu::CPU;
use crate::ppu::PPU;
#[allow(dead_code)]
use std::thread;
use std::time::Duration;

pub struct EMU {
    cpu: CPU,
    ppu: PPU,
    running: bool,
    paused: bool,
}

impl EMU {
    pub fn from(file: &str) -> Self {
        let cartridge = Cartridge::from(file);
        Bus::init(cartridge);
        let cpu = CPU::new();
        let ppu = PPU {};

        EMU {
            cpu,
            ppu,
            running: false,
            paused: false,
        }
    }

    pub fn test(test_num: u8) -> Self {
        let file = match test_num {
            1 => "..\\gb-test-roms\\cpu_instrs\\individual\\01-special.gb",
            2 => "..\\gb-test-roms\\cpu_instrs\\individual\\02-interrupts.gb",
            3 => "..\\gb-test-roms\\cpu_instrs\\individual\\03-op sp,hl.gb",
            4 => "..\\gb-test-roms\\cpu_instrs\\individual\\04-op r,imm.gb",
            5 => "..\\gb-test-roms\\cpu_instrs\\individual\\05-op rp.gb",
            6 => "..\\gb-test-roms\\cpu_instrs\\individual\\06-ld r,r.gb",
            7 => "..\\gb-test-roms\\cpu_instrs\\individual\\07-jr,jp,call,ret,rst.gb",
            8 => "..\\gb-test-roms\\cpu_instrs\\individual\\08-misc instrs.gb",
            9 => "..\\gb-test-roms\\cpu_instrs\\individual\\09-op r,r.gb",
            10 => "..\\gb-test-roms\\cpu_instrs\\individual\\10-bit ops.gb",
            11 => "..\\gb-test-roms\\cpu_instrs\\individual\\11-op a,(hl).gb",
            _ => panic!("Num not implemented"),
        };

        let cartridge = Cartridge::from(file);
        Bus::init(cartridge);
        let cpu = CPU::test();
        let ppu = PPU {};

        EMU {
            cpu,
            ppu,
            running: false,
            paused: false,
        }
    }

    pub fn run(&mut self) {
        self.running = true;

        while self.running {
            if self.paused {
                thread::sleep(Duration::from_millis(10));
                continue;
            }

            self.cpu.step();
        }
    }
}
