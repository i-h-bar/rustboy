use crate::cartridge::Cartridge;
use std::fs;

mod cartridge;
mod cpu;
mod emu;
mod ppu;
mod register;
mod tpu;

fn main() {
    let cart = Cartridge::from("01-special.gb");
}
