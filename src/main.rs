#![allow(dead_code)]
#![allow(unused_variables)]

use crate::cartridge::Cartridge;

mod cartridge;
mod cpu;
mod emu;
mod ppu;
mod register;
mod tpu;

fn main() {
    let cart = Cartridge::from("test_2.gb");
}
