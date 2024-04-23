use std::fs;
use crate::cartridge::Cartridge;

mod cartridge;
mod cpu;
mod emu;
mod ppu;
mod register;
mod tpu;


fn main() {
    let cart = Cartridge::from("Tetris.gb");
}
