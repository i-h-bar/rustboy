#![allow(dead_code)]
#![allow(unused_variables)]

use crate::emu::EMU;

mod cartridge;
mod cpu;
mod emu;
mod instruction;
mod ppu;
mod tpu;
mod interrupts;


const CPU_INSTRS: &str = "test_roms/cpu_instrs.test";
const DMG_ACID2: &str = "test_roms/dmg-acid2.test";
const MEM_TIMING: &str = "test_roms/mem_timing.test";

fn main() {
    let mut emulation = EMU::test(3);
    emulation.run();
}
