use crate::cartridge::{Bus, Cartridge};
use crate::cpu::CPU;
use crate::ppu::PPU;
use crate::tpu::Timer;
use std::thread;
use std::time::Duration;

pub struct EMU {
    cpu: CPU,
    ppu: PPU,
    timer: Timer,
    running: bool,
    paused: bool,
    ticks: u64,
}

impl EMU {
    pub fn from(file: &str) -> Self {
        let cartridge = Cartridge::from(file);
        let bus = Bus::from(cartridge);
        let cpu = CPU::from(bus);
        let ppu = PPU {};
        let timer = Timer {};

        EMU {
            cpu,
            ppu,
            timer,
            running: false,
            paused: false,
            ticks: 0,
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
            self.ticks += 1;
        }
    }

    pub fn cycles(cycle: u8) {

    }
}
