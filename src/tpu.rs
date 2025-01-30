use crate::cpu::CPU;
use crate::interrupts;
use std::sync::{Mutex, MutexGuard, OnceLock};


static TIMER: OnceLock<Mutex<Timer>> = OnceLock::new();


pub struct Timer {
    div: u16,
    tima: u8,
    tma: u8,
    tac: u8,
    ticks: u64,
}

impl Timer {
    fn new() -> Self {
        Self {
            div: 0xAC00,
            tima: 0,
            tma: 0,
            tac: 0,
            ticks: 0,
        }
    }

    pub fn get() -> MutexGuard<'static, Timer> {
        TIMER.get_or_init(|| Mutex::new(Timer::new())).lock().unwrap()
    }

    pub fn emu_cycles(&mut self, n: u8, cpu: &mut CPU) {
        for _ in 0..(n * 4) {
            self.ticks += 1;
            self.tick(cpu)
        }
    }

    fn tick(&mut self, cpu: &mut CPU) {
        let previous_div = self.div;
        self.div = self.div.wrapping_add(1);

        let mut update_timer = false;

        match self.tac & 0b11 {
            0b00 => {
                if (previous_div & (1 << 9)) != 0 && (self.div & (1 << 9)) == 0 {
                    update_timer = true;
                }
            }
            0b01 => {
                if (previous_div & (1 << 3)) != 0 && (self.div & (1 << 3)) == 0 {
                    update_timer = true;
                }
            }
            0b10 => {
                if (previous_div & (1 << 5)) != 0 && (self.div & (1 << 5)) == 0 {
                    update_timer = true;
                }
            }
            0b11 => {
                if (previous_div & (1 << 7)) != 0 && (self.div & (1 << 7)) == 0 {
                    update_timer = true;
                }
            }
            _ => {}
        }

        if update_timer && self.tac & (1 << 2) != 0 {
            if self.tima == 0xFF {
                self.tima = self.tma;

                cpu.request_interrupt(interrupts::TIMER)
            }
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {
            0xFF04 => self.div = 0,
            0xFF05 => self.tima = value,
            0xFF06 => self.tma = value,
            0xFF07 => self.tac = value,
            _ => {}
        }
    }

    fn read(&self, address: u16) -> u8 {
        match address {
            0xFF04 => (self.div >> 8) as u8,
            0xFF05 => self.tima,
            0xFF06 => self.tma,
            0xFF07 => self.tac,
            _ => todo!(),
        }
    }
}
