#![allow(dead_code)]
#![allow(unused_variables)]

use crate::cartridge::Cartridge;

mod cartridge;
mod cpu;
mod emu;
mod ppu;
mod tpu;
mod bus;
mod instruction;

fn main() {
    let cart = Cartridge::from("test2.gb");
}
