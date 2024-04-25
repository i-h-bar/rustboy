use crate::cartridge::Cartridge;
use crate::cpu::{Register, CPU};
use crate::ppu::PPU;
use crate::tpu::Timer;
use std::thread;
use std::time::Duration;

pub struct EMU {
    cartridge: Cartridge,
    cpu: CPU,
    register: Register,
    ppu: PPU,
    timer: Timer,
    running: bool,
    paused: bool,
    ticks: u64,
}

impl EMU {
    pub fn new(cartridge: Cartridge) -> Self {
        todo!()
        // let cpu = CPU {};
        // let register = Register {};
        // let ppu = PPU {};
        // let timer = Timer {};
        //
        // EMU {
        //     cartridge,
        //     cpu,
        //     register,
        //     ppu,
        //     timer,
        //     running: false,
        //     paused: false,
        //     ticks: 0,
        // }
    }

    pub fn run(&mut self) {
        self.running = true;

        while self.running {
            if self.paused {
                thread::sleep(Duration::from_millis(10));
            }
        }
    }
}
