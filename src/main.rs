#![allow(dead_code)]
#![allow(unused_variables)]

use crate::emu::EMU;

mod cartridge;
mod cpu;
mod emu;
mod interrupts;
mod ppu;
mod tpu;

fn main() {
    let mut emulation = EMU::test(3);
    emulation.run();
}
