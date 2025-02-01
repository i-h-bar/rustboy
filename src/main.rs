#![allow(dead_code)]
#![allow(unused_variables)]

use crate::emu::EMU;

mod bus;
mod cartridge;
mod cpu;
mod emu;
mod interrupts;
mod ppu;
pub mod tpu;

fn main() {
    let mut emulation = EMU::test(1);
    emulation.run();
}
