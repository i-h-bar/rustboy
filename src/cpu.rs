use crate::bus::Bus;
use crate::cpu::{conditions::ConditionType, register::RegisterType};
use crate::interrupts;
use crate::interrupts::Interrupt;
use crate::tpu::Timer;
use instructions::Instruction;
use register::Register;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};

mod actions;
mod addresses;
mod conditions;
mod instructions;
mod register;

pub struct CPU {
    register: Register,
    fetch_data: u16,
    mem_dest: u16,
    dest_is_mem: bool,
    current_op_code: u8,
    instruction: Instruction,
    halted: bool,
    stepping: bool,
    master_enabled: bool,
    enabling_ime: bool,
    pub int_flags: u8,
    ie_register: u8,
    cycle: u32,
    log: String,
    debug_message: String,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            register: Register {
                a: 0x1,
                f: 0,
                b: 0,
                c: 0,
                d: 0,
                e: 0,
                h: 0,
                l: 0,
                sp: 0,
                pc: 0x100,
            },
            fetch_data: 0,
            mem_dest: 0,
            dest_is_mem: false,
            current_op_code: 0,
            instruction: Instruction::from(0),
            halted: false,
            stepping: false,
            master_enabled: false,
            enabling_ime: false,
            int_flags: 0,
            ie_register: 0,
            cycle: 0,
            log: String::new(),
            debug_message: String::new(),
        }
    }

    pub fn test() -> Self {
        Self {
            register: Register {
                a: 0x01,
                f: 0xB0,
                b: 0,
                c: 0x13,
                d: 0,
                e: 0xD8,
                h: 0x01,
                l: 0x4D,
                sp: 0xFFFE,
                pc: 0x0100,
            },
            fetch_data: 0,
            mem_dest: 0,
            dest_is_mem: false,
            current_op_code: 0,
            instruction: Instruction::from(0),
            halted: false,
            stepping: false,
            master_enabled: false,
            enabling_ime: false,
            int_flags: 0,
            ie_register: 0,
            cycle: 0,
            log: String::new(),
            debug_message: String::new(),
        }
    }

    pub fn step(&mut self) {
        self.log();
        self.debug_update();
        self.debug_print();
        // self.log_to_stdout();
        if !self.halted {
            let instruction = self.fetch_instruction();
            instruction.execute(self);

            self.cycle += 1;
        } else {
            Timer::get().emu_cycles(1, self);

            if self.int_flags != 0 {
                self.halted = false;
            }
        }

        if self.master_enabled {
            self.handle_interrupts();
            self.enabling_ime = false;
        }

        if self.enabling_ime {
            self.master_enabled = true;
        }
    }

    pub fn request_interrupt(&mut self, it_timer: u8) {
        todo!()
    }

    fn log_to_stdout(&self) {
        let z: &str;
        let n: &str;
        let h: &str;
        let c: &str;
        if self.register.z_flag() {
            z = "Z";
        } else {
            z = "-";
        }
        if self.register.n_flag() {
            n = "N";
        } else {
            n = "-";
        }
        if self.register.h_flag() {
            h = "H";
        } else {
            h = "-";
        }
        if self.register.c_flag() {
            c = "C";
        } else {
            c = "-";
        }

        println!(
            "{:#08x}: {:#04x} {: <4} | PC: {:#06x} | DATA {:#06x} | a: {:#04x}; bc: {:#06x}; de: {:#06x}; sp: {:#06x}; hl: {:#06x} | {}{}{}{}",
            self.cycle,
            self.current_op_code,
            self.instruction.action.to_string(),
            self.register.pc,
            self.fetch_data,
            self.register.a,
            self.read_register(&RegisterType::BC),
            self.read_register(&RegisterType::DE),
            self.register.sp,
            self.read_register(&RegisterType::HL),
            z,
            n,
            h,
            c
        );
    }

    fn log(&mut self) {
        let data_pc: u16 = Bus::get().read(self.register.pc, &self);
        let data_pc_1: u16 = Bus::get().read(self.register.pc + 1, &self);
        let data_pc_2: u16 = Bus::get().read(self.register.pc + 2, &self);
        let data_pc_3: u16 = Bus::get().read(self.register.pc + 3, &self);

        let log = format!(
            "A:{:#04X} F:{:#04X} B:{:#04X} C:{:#04X} D:{:#04X} E:{:#04X} H:{:#04X} L:{:#04X} SP:{:#06X} PC:{:#06X} PCMEM:{:#04X},{:#04X},{:#04X},{:#04X}\n",
            self.register.a,
            self.register.f,
            self.register.b,
            self.register.c,
            self.register.d,
            self.register.e,
            self.register.h,
            self.register.l,
            self.register.sp,
            self.register.pc,
            data_pc,
            data_pc_1,
            data_pc_2,
            data_pc_3,
        ).replace("0x", "");
        self.log.push_str(&log);
    }

    fn save_log(&self) {
        let file = match OpenOptions::new().append(true).open("log.txt") {
            Ok(file) => File::create("log.txt").expect("Could not create log.txt"),
            Err(_) => File::create("log.txt").expect("Could not create log.txt"),
        };
        let mut buffer = BufWriter::new(file);
        buffer
            .write(self.log.as_ref())
            .expect("Could not write to log.txt");
    }

    fn debug_update(&mut self) {
        if Bus::get().read(0xFF02, &self) as u8 == 0x81 {
            self.debug_message.push(Bus::get().read(0xFF01, &self) as u8 as char);
            Bus::get().write(0xFF02, 0x00, self);
        }
    }

    fn debug_print(&self) {
        if !self.debug_message.is_empty() {
            println!("DBG: {}", self.debug_message);
        }
    }

    fn handle_interrupts(&mut self) {
        if self.interrupt_check(0x40, Interrupt::VBlank) {
        } else if self.interrupt_check(0x48, Interrupt::LCDStat) {
        } else if self.interrupt_check(0x50, Interrupt::Timer) {
        } else if self.interrupt_check(0x58, Interrupt::Serial) {
        } else if self.interrupt_check(0x60, Interrupt::JoyPad) {
        }
    }

    fn interrupt_check(&mut self, address: u16, interrupt: Interrupt) -> bool {
        let int_num = interrupts::fetch_interrupt_num(interrupt);

        if self.int_flags & int_num != 0 && self.ie_register & int_num != 0 {
            self.handle_interrupt(address);
            self.int_flags &= !int_num;
            self.halted = false;
            self.master_enabled = false;

            true
        } else {
            false
        }
    }

    fn handle_interrupt(&mut self, address: u16) {
        self.stack_push16(self.register.pc);
        self.register.pc = address;
    }

    fn fetch_instruction(&mut self) -> Instruction {
        self.current_op_code = Bus::get().read(self.register.pc, &self) as u8;
        self.register.pc += 1;
        Instruction::from(self.current_op_code)
    }

    fn read_register(&self, register: &RegisterType) -> u16 {
        match register {
            RegisterType::NONE => 0,
            RegisterType::A => self.register.a,
            RegisterType::F => self.register.f,
            RegisterType::B => self.register.b,
            RegisterType::C => self.register.c,
            RegisterType::D => self.register.d,
            RegisterType::E => self.register.e,
            RegisterType::H => self.register.h,
            RegisterType::L => self.register.l,
            RegisterType::AF => (self.register.a << 8) | self.register.f,
            RegisterType::BC => (self.register.b << 8) | self.register.c,
            RegisterType::DE => (self.register.d << 8) | self.register.e,
            RegisterType::HL => (self.register.h << 8) | self.register.l,
            RegisterType::SP => self.register.sp,
            RegisterType::PC => self.register.pc,
        }
    }

    fn read_register8(&self, register: &RegisterType) -> u8 {
        match register {
            RegisterType::A => self.register.a as u8,
            RegisterType::F => self.register.f as u8,
            RegisterType::B => self.register.b as u8,
            RegisterType::C => self.register.c as u8,
            RegisterType::D => self.register.d as u8,
            RegisterType::E => self.register.e as u8,
            RegisterType::H => self.register.h as u8,
            RegisterType::L => self.register.l as u8,
            RegisterType::HL => Bus::get().read(self.read_register(register), &self) as u8,
            _ => panic!("{:?} is not a valid 8bit register", register),
        }
    }

    fn set_register8(&mut self, register: &RegisterType, value: u8) {
        match register {
            RegisterType::A => self.register.a = (value & 0xFF) as u16,
            RegisterType::F => self.register.f = (value & 0xFF) as u16,
            RegisterType::B => self.register.b = (value & 0xFF) as u16,
            RegisterType::C => self.register.c = (value & 0xFF) as u16,
            RegisterType::D => self.register.d = (value & 0xFF) as u16,
            RegisterType::E => self.register.e = (value & 0xFF) as u16,
            RegisterType::H => self.register.h = (value & 0xFF) as u16,
            RegisterType::L => self.register.l = (value & 0xFF) as u16,
            RegisterType::HL => Bus::get().write(self.read_register(register), value, self),
            _ => panic!("{:?} is not a valid 8bit register", register),
        }
    }

    fn set_register(&mut self, register_type: &RegisterType, value: u16) {
        match register_type {
            RegisterType::NONE => {}
            RegisterType::A => self.register.a = value & 0xFF,
            RegisterType::F => self.register.f = value & 0xFF,
            RegisterType::B => self.register.b = value & 0xFF,
            RegisterType::C => self.register.c = value & 0xFF,
            RegisterType::D => self.register.d = value & 0xFF,
            RegisterType::E => self.register.e = value & 0xFF,
            RegisterType::H => self.register.h = value & 0xFF,
            RegisterType::L => self.register.l = value & 0xFF,
            RegisterType::AF => {
                let value = reverse(value);
                let hi = value & 0xFF;
                let lo = value >> 8;
                self.register.a = hi;
                self.register.f = lo;
            }
            RegisterType::BC => {
                let value = reverse(value);
                let hi = value & 0xFF;
                let lo = value >> 8;
                self.register.b = hi;
                self.register.c = lo;
            }
            RegisterType::DE => {
                let value = reverse(value);
                let hi = value & 0xFF;
                let lo = value >> 8;
                self.register.d = hi;
                self.register.e = lo;
            }
            RegisterType::HL => {
                let value = reverse(value);
                let hi = value & 0xFF;
                let lo = value >> 8;
                self.register.h = hi;
                self.register.l = lo;
            }
            RegisterType::SP => self.register.sp = value,
            RegisterType::PC => self.register.pc = value,
        }
    }

    fn stack_push(&mut self, data: u8) {
        self.register.sp = self.register.sp.wrapping_sub(1);
        Bus::get().write(self.register.sp, data, self);
    }

    fn stack_push16(&mut self, data: u16) {
        self.stack_push(((data >> 8) & 0xFF) as u8);
        self.stack_push((data & 0xFF) as u8);
    }

    fn stack_pop(&mut self) -> u16 {
        let data = Bus::get().read(self.register.sp, &self);
        self.register.sp += 1;

        data
    }

    fn stack_pop16(&mut self) -> u16 {
        let lo: u16 = self.stack_pop();
        let hi: u16 = self.stack_pop();

        (hi << 8) | lo
    }

    fn go_to(&mut self, address: u16, push_pc: bool, instruction: &Instruction) {
        if instruction.condition.check(self) {
            if push_pc {
                Timer::get().emu_cycles(2, self);
                self.stack_push16(self.register.pc)
            }

            self.register.pc = address;
            Timer::get().emu_cycles(1, self);
        }
    }

    fn return_from_procedure(&mut self, instruction: &Instruction) {
        match self.instruction.condition {
            ConditionType::NONE => {}
            _ => Timer::get().emu_cycles(1, self),
        }

        if instruction.condition.check(self) {
            let lo = self.stack_pop();
            Timer::get().emu_cycles(1, self);
            let hi = self.stack_pop();
            Timer::get().emu_cycles(1, self);
            self.register.pc = (hi << 8) | lo;
            Timer::get().emu_cycles(1, self);
        }
    }
}

impl Drop for CPU {
    fn drop(&mut self) {
        self.save_log();
    }
}

fn bit(a: u8, n: u8) -> bool {
    if (a & (1 << n)) != 0 {
        true
    } else {
        false
    }
}

fn bit_set(a: u8, n: u8, on: bool) -> u8 {
    if on {
        a | (1 << n)
    } else {
        a & !(1 << n)
    }
}

fn reverse(num: u16) -> u16 {
    ((num & 0xFF00) >> 8) | ((num & 0x00FF) << 8)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        assert_eq!(reverse(0x00EE), 0xEE00);
        assert_eq!(reverse(0xEE00), 0x00EE)
    }

    #[test]
    fn test_set_and_read_flag() {
        let mut reg = Register {
            a: 0x1,
            f: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0x100,
        };
        assert!(!reg.z_flag());
        reg.set_z(true);
        assert!(reg.z_flag())
    }

    #[test]
    fn test_set_and_read_and_reset_z_flag() {
        let mut reg = Register {
            a: 0x1,
            f: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0x100,
        };
        assert!(!reg.z_flag());
        reg.set_z(true);
        assert!(reg.z_flag());
        reg.set_z(false);
        assert!(!reg.z_flag());
    }

    #[test]
    fn test_set_and_read_and_reset_n_flag() {
        let mut reg = Register {
            a: 0x1,
            f: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0x100,
        };
        assert!(!reg.n_flag());
        reg.set_n(true);
        assert!(reg.n_flag());
        reg.set_n(false);
        assert!(!reg.n_flag());
    }

    #[test]
    fn test_set_and_read_and_reset_h_flag() {
        let mut reg = Register {
            a: 0x1,
            f: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0x100,
        };
        assert!(!reg.h_flag());
        reg.set_h(true);
        assert!(reg.h_flag());
        reg.set_h(false);
        assert!(!reg.h_flag());
    }

    #[test]
    fn test_set_and_read_and_reset_c_flag() {
        let mut reg = Register {
            a: 0x1,
            f: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0x100,
        };
        assert!(!reg.c_flag());
        reg.set_c(true);
        assert!(reg.c_flag());
        reg.set_c(false);
        assert!(!reg.c_flag());
    }
}
