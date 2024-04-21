mod cartridge;
mod cpu;
mod emu;
mod ppu;
mod register;
mod tpu;

fn rom_size(value: u16) -> u16 {
    assert!(value <= 8);
    32 * (1 << value)
}

fn main() {
    let x = rom_size(8);
    println!("{}", x);
}
