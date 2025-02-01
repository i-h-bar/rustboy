use crate::cartridge::Cartridge;
use crate::cpu::CPU;
use crate::tpu::Timer;
use std::sync::{Mutex, MutexGuard, OnceLock};

static BUS: OnceLock<Mutex<Bus>> = OnceLock::new();

struct RAM {
    wram: Vec<u8>,
    hram: Vec<u8>,
}

impl RAM {
    fn new() -> Self {
        let wram = vec![0; 0x2000];
        let hram = vec![0; 0x80];

        Self { wram, hram }
    }

    fn wram_read(&self, address: u16) -> u8 {
        let address = address - 0xC000;
        self.wram[address as usize]
    }

    fn wram_write(&mut self, address: u16, value: u8) {
        let address = address - 0xC000;
        self.wram[address as usize] = value;
    }

    fn hram_read(&self, address: u16) -> u8 {
        let address = address - 0xFF80;
        self.hram[address as usize]
    }

    fn hram_write(&mut self, address: u16, value: u8) {
        let address = address - 0xFF80;
        self.hram[address as usize] = value;
    }
}

pub struct Bus {
    cartridge: Cartridge,
    ram: RAM,
    ie_register: u8,
    serial_data: [u8; 2],
}

impl Bus {
    pub fn init(cartridge: Cartridge) {
        BUS.get_or_init(|| Mutex::new(Bus::from(cartridge)));
    }

    pub fn get() -> MutexGuard<'static, Bus> {
        BUS.get()
            .expect("Bus is not initialised")
            .lock()
            .expect("Could not get lock on Bus")
    }

    fn from(cartridge: Cartridge) -> Self {
        let ram = RAM::new();
        Self {
            cartridge,
            ram,
            ie_register: 0,
            serial_data: [0; 2],
        }
    }

    pub fn read(&self, address: u16, cpu: &CPU) -> u16 {
        if address < 0x8000 {
            self.cartridge.read(address) as u16
        } else if address < 0xA000 {
            println!("Reading unimplemented address {:04X}", address);
            todo!()
        } else if address < 0xC000 {
            self.cartridge.read(address) as u16
        } else if address < 0xE000 {
            self.ram.wram_read(address) as u16
        } else if address < 0xFE00 {
            println!("Reading reserved address {:04X}", address);
            0
        } else if address < 0xFEA0 {
            println!("Reading unimplemented address {:04X}", address);
            todo!()
        } else if address < 0xFF00 {
            println!("Reading reserved address {:04X}", address);
            0
        } else if address < 0xFF80 {
            self.io_read(address, &cpu) as u16
        } else if address == 0xFFFF {
            self.ie_register as u16
        } else {
            self.ram.hram_read(address) as u16
        }
    }

    pub fn write(&mut self, address: u16, value: u8, cpu: &mut CPU) {
        if address < 0x8000 {
            self.cartridge.write(address, value)
        } else if address < 0xA000 {
            println!("Unsupported write to address: {:#05x}", address);
            todo!()
        } else if address < 0xC000 {
            self.cartridge.write(address, value)
        } else if address < 0xE000 {
            self.ram.wram_write(address, value)
        } else if address < 0xFE00 {
            panic!(
                "Cannot write to address: {:#05x} as it is in a reserved section",
                address
            )
        } else if address < 0xFEA0 {
            println!("Unsupported write to address: {:#05x}", address);
            todo!()
        } else if address < 0xFF00 {
            panic!(
                "Cannot write to address: {:#05x} as it is in a reserved section",
                address
            )
        } else if address < 0xFF80 {
            self.io_write(address, value, cpu)
        } else if address == 0xFFFF {
            self.ie_register = value;
        } else {
            self.ram.hram_write(address, value)
        }
    }

    fn io_write(&mut self, address: u16, value: u8, cpu: &mut CPU) {
        if address == 0xFF01 {
            self.serial_data[0] = value;
            return;
        }
        if address == 0xFF02 {
            self.serial_data[1] = value;
            return;
        }

        if (0xFF04 <= address) && (address <= 0xFF07) {
            println!("{:#05x}", address);
            Timer::get().write(address, value);
            return;
        }

        if address == 0xFF0F {
            cpu.int_flags = value;
            return;
        }

        if (0xFF10 <= address) && (address <= 0xFF3F) {
            println!("Ignore sound {:#05x}", address);
            return;
        }

        println!("Unsupported IO write {:#05x}", address);
    }

    fn io_read(&self, address: u16, cpu: &CPU) -> u8 {
        if address == 0xFF01 {
            return self.serial_data[0];
        }
        if address == 0xFF02 {
            return self.serial_data[1];
        }

        if (0xFF04 <= address) && (address <= 0xFF07) {
            return Timer::get().read(address);
        }

        if address == 0xFF0F {
            return cpu.int_flags;
        }

        // println!("Unsupported IO read {:#05x}", address);
        0
    }

    pub fn read16(&self, address: u16, cpu: &CPU) -> u16 {
        let lo = self.read(address, &cpu);
        let hi = self.read(address + 1, &cpu);

        lo | (hi << 8)
    }

    pub fn write16(&mut self, address: u16, value: u16, cpu: &mut CPU) {
        self.write(address + 1, ((value >> 8) & 0xFF) as u8, cpu);
        self.write(address, (value & 0xFF) as u8, cpu);
    }
}
