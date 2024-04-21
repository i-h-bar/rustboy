use crate::cartridge::Cartridge;
use crate::cpu::CPU;
use crate::ppu::PPU;
use crate::register::Register;
use crate::tpu::Timer;

pub struct EMU {
    cartridge: Cartridge,
    cpu: CPU,
    register: Register,
    ppu: PPU,
    timer: Timer,
    context: EMUContext
}

struct EMUContext {
    running: bool,
    paused: bool,
    ticks: u64,
}


impl EMU {
    pub fn run(&self, rom: u32) {}

    pub fn context(&self) -> EMUContext {
        EMUContext { running: true, paused: false, ticks: 0 }
    }
}