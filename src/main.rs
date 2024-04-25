#![allow(dead_code)]
#![allow(unused_variables)]

use crate::cartridge::Cartridge;

mod bus;
mod cartridge;
mod cpu;
mod emu;
mod instruction;
mod ppu;
mod tpu;

fn main() {
    let cart = Cartridge::from("test2.gb");
}
