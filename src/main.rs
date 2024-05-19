#![allow(dead_code)]
#![allow(unused_variables)]

use crate::emu::EMU;

mod cartridge;
mod cpu;
mod emu;
mod instruction;
mod ppu;
mod tpu;


fn main() {
    let mut emulation = EMU::from("test_roms/cpu_instrs.test");
    emulation.run();
}
