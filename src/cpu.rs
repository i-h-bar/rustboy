use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::process;
use crate::cartridge::Bus;
use crate::emu::EMU;
use crate::instruction;
use crate::instruction::{AddressMode, ConditionType, Instruction, InstructionType, RegisterType};
use crate::interrupts;
use crate::interrupts::Interrupt;

#[derive(Debug)]
pub struct Register {
    a: u16,
    f: u16,
    b: u16,
    c: u16,
    d: u16,
    e: u16,
    h: u16,
    l: u16,
    sp: u16,
    pc: u16,
}

impl Register {
    fn z_flag(&self) -> bool {
        bit(self.f as u8, 7)
    }

    fn n_flag(&self) -> bool {
        bit(self.f as u8, 6)
    }

    fn h_flag(&self) -> bool {
        bit(self.f as u8, 5)
    }

    fn c_flag(&self) -> bool {
        bit(self.f as u8, 4)
    }

    fn set_z(&mut self, on: bool) {
        self.f = bit_set(self.f as u8, 7, on) as u16;
    }

    fn set_n(&mut self, on: bool) {
        self.f = bit_set(self.f as u8, 6, on) as u16;
    }

    fn set_h(&mut self, on: bool) {
        self.f = bit_set(self.f as u8, 5, on) as u16;
    }

    fn set_c(&mut self, on: bool) {
        self.f = bit_set(self.f as u8, 4, on) as u16;
    }

    fn is_16bit(&self, reg_type: &RegisterType) -> bool {
        match reg_type {
            RegisterType::AF | RegisterType::BC | RegisterType::DE | RegisterType::HL | RegisterType::SP | RegisterType::PC => true,
            _ => false
        }
    }
}

pub struct CPU {
    register: Register,
    bus: Bus,
    fetch_data: u16,
    mem_dest: u16,
    dest_is_mem: bool,
    current_op_code: u8,
    instruction: &'static Instruction,
    halted: bool,
    stepping: bool,
    master_enabled: bool,
    enabling_ime: bool,
    int_flags: u8,
    ie_register: u8,
    cycle: u32,
    log: String
}

impl CPU {
    pub fn from(bus: Bus) -> Self {
        Self {
            register: Register { a: 0x1, f: 0, b: 0, c: 0, d: 0, e: 0, h: 0, l: 0, sp: 0, pc: 0x100 },
            bus,
            fetch_data: 0,
            mem_dest: 0,
            dest_is_mem: false,
            current_op_code: 0,
            instruction: Instruction::from(0).unwrap(),
            halted: false,
            stepping: false,
            master_enabled: false,
            enabling_ime: false,
            int_flags: 0,
            ie_register: 0,
            cycle: 0,
            log: String::new(),
        }
    }

    pub fn test(bus: Bus) -> Self {
        Self {
            register: Register { a: 0x01, f: 0xB0, b: 0, c: 0x13, d: 0, e: 0xD8, h: 0x01, l: 0x4D, sp: 0xFFFE, pc: 0x0100 },
            bus,
            fetch_data: 0,
            mem_dest: 0,
            dest_is_mem: false,
            current_op_code: 0,
            instruction: Instruction::from(0).unwrap(),
            halted: false,
            stepping: false,
            master_enabled: false,
            enabling_ime: false,
            int_flags: 0,
            ie_register: 0,
            cycle: 0,
            log: String::new(),
        }
    }

    fn execute(&mut self) {
        self.process();
    }

    fn process(&mut self) {
        match self.instruction.instruction_type {
            InstructionType::NONE => {}
            InstructionType::NOP => {}
            InstructionType::LD => {
                if self.dest_is_mem {
                    if self.register.is_16bit(&self.instruction.register_2) {
                        EMU::cycles(1);
                        self.bus.write16(self.mem_dest, self.fetch_data);
                    } else {
                        self.bus.write(self.mem_dest, self.fetch_data as u8);
                    }
                } else {
                    if self.instruction.address_mode == AddressMode::HLSPR {
                        self.register.set_h(((self.read_register(&self.instruction.register_2) as u8) & 0x0F + (self.fetch_data as u8 & 0x0F)) >= 0x10);
                        self.register.set_c(((self.read_register(&self.instruction.register_2)) & 0xFF00 + (self.fetch_data & 0xFF00)) >= 0x100);
                        self.register.set_z(false);
                        self.register.set_n(false);

                        self.set_register(
                            &self.instruction.register_1,
                            (self.read_register(&self.instruction.register_2) as i8 + self.fetch_data as i8) as u16,
                        );
                    } else {
                        self.set_register(&self.instruction.register_1, self.fetch_data)
                    }
                }
            }
            InstructionType::INC => {
                if self.register.is_16bit(&self.instruction.register_1) {
                    EMU::cycles(1);
                }

                if self.instruction.register_1 == RegisterType::HL && self.instruction.address_mode == AddressMode::MR {
                    let val = (self.bus.read(self.read_register(&self.instruction.register_1))).wrapping_add(1);
                    let val = val & 0xFF;

                    self.bus.write(self.read_register(&self.instruction.register_1), val as u8)
                } else {
                    let val = self.read_register(&self.instruction.register_1).wrapping_add(1);
                    self.set_register(&self.instruction.register_1, val)
                }

                if (self.current_op_code & 0x03) != 0x03 {
                    let val = self.read_register(&self.instruction.register_1);
                    self.register.set_z(val == 0);
                    self.register.set_n(false);
                    self.register.set_h((val & 0x0F) == 0);
                }
            }
            InstructionType::DEC => {
                if self.register.is_16bit(&self.instruction.register_1) {
                    EMU::cycles(1);
                }

                if self.instruction.register_1 == RegisterType::HL && self.instruction.address_mode == AddressMode::MR {
                    let val = (self.bus.read(self.read_register(&self.instruction.register_1)) as u8).wrapping_sub(1);

                    self.bus.write(self.read_register(&self.instruction.register_1), val)
                } else {
                    let val = (self.read_register(&self.instruction.register_1) as u8).wrapping_sub(1);

                    self.set_register(&self.instruction.register_1, val as u16)
                }

                if (self.current_op_code & 0x0B) != 0x0B {
                    let val = self.read_register(&self.instruction.register_1);
                    self.register.set_z(val == 0);
                    self.register.set_n(true);
                    self.register.set_h((val & 0x0F) == 0x0F);
                }
            }
            InstructionType::RLCA => {
                let u = self.register.a as u8;
                let c = (u >> 7) & 1;

                self.register.a = ((u << 1) | c) as u16;
                self.register.set_z(false);
                self.register.set_n(false);
                self.register.set_h(false);
                self.register.set_c(c != 0);
            }
            InstructionType::ADD => {
                let val: u32;
                let is_16bit = self.register.is_16bit(&self.instruction.register_1);

                if is_16bit {
                    EMU::cycles(1);
                }

                if self.instruction.register_1 == RegisterType::SP {
                    val = (self.read_register(&self.instruction.register_1) as i8).wrapping_add(self.fetch_data as i8) as u32;
                } else {
                    val = self.read_register(&self.instruction.register_1) as u32 + self.fetch_data as u32;
                }

                if self.instruction.register_1 == RegisterType::SP {
                    self.register.set_n(true);
                    self.register.set_z(false);
                    self.register.set_h((self.read_register(&self.instruction.register_1) & 0xF).wrapping_add(self.fetch_data & 0xF) >= 0x10);
                    self.register.set_c((self.read_register(&self.instruction.register_1) & 0xFF).wrapping_add(self.fetch_data & 0xFF) > 0x100);
                } else if is_16bit {
                    self.register.set_n(true);
                    self.register.set_h((self.read_register(&self.instruction.register_1) & 0xFFF).wrapping_add(self.fetch_data & 0xFFF) >= 0x1000);
                    let n = (self.read_register(&self.instruction.register_1) as u32).wrapping_add(self.fetch_data as u32);
                    self.register.set_c(n >= 0x10000);
                } else {
                    self.register.set_n(true);
                    self.register.set_z((val & 0xFF) == 0);
                    self.register.set_h((self.read_register(&self.instruction.register_1) & 0xF).wrapping_add(self.fetch_data & 0xF) >= 0x10);
                    self.register.set_c((self.read_register(&self.instruction.register_1) & 0xFF).wrapping_add(self.fetch_data & 0xFF) >= 0x100);
                }

                self.set_register(&self.instruction.register_1, (val & 0xFFFF) as u16);
            }
            InstructionType::RRCA => {
                let b = (self.register.a & 1) as u8;
                self.register.a >>= 1;
                self.register.a |= (b << 7) as u16;
                self.register.set_z(false);
                self.register.set_n(false);
                self.register.set_h(false);
                self.register.set_c(b != 0);
            }
            InstructionType::STOP => {
                panic!("Stop instruction called!")
            }
            InstructionType::RLA => {
                let u = self.register.a as u8;
                let c = (u >> 7) & 1;

                self.register.a = ((u << 1) | if self.register.c_flag() { 1 } else { 0 }) as u16;
                self.register.set_z(false);
                self.register.set_n(false);
                self.register.set_h(false);
                self.register.set_c(c != 0);
            }
            InstructionType::JR => {
                let rel = (self.fetch_data & 0xFF) as i8;
                let address = (self.register.pc as i16 + rel as i16) as u16;
                self.go_to(address, false);
            }
            InstructionType::RRA => {
                let new_c = (self.register.a & 1) as u8;

                self.register.a >>= 1;
                self.register.a |= (if self.register.c_flag() { 1 } else { 0 } << 7) as u16;

                self.register.set_z(false);
                self.register.set_n(false);
                self.register.set_h(false);
                self.register.set_c(new_c != 0);
            }
            InstructionType::DAA => {
                let mut u: u16 = 0;
                let mut cf: u8 = 0;

                if self.register.h_flag() || (!self.register.n_flag() && self.register.a & 0xF > 9) {
                    u = 6;
                }

                if  self.register.c_flag() || (!self.register.n_flag() && self.register.a > 0x99) {
                    u |= 0x60;
                    cf = 1;
                }

                if self.register.n_flag() {
                    self.register.a.wrapping_sub(u);
                } else {
                    self.register.a.wrapping_add(u);
                }
                self.register.set_z(self.register.a == 0);
                self.register.set_h(false);
                self.register.set_c(cf != 0);
            }
            InstructionType::CPL => {
                self.register.a = !(self.register.a as u8) as u16;
                self.register.set_n(true);
                self.register.set_h(true);
            }
            InstructionType::SCF => {
                self.register.set_n(false);
                self.register.set_h(false);
                self.register.set_c(true);
            }
            InstructionType::CCF => {
                self.register.set_n(false);
                self.register.set_h(false);
                self.register.set_c(self.register.c_flag() ^ true);
            }
            InstructionType::HALT => {
                self.halted = true;
            }
            InstructionType::ADC => {
                let u = self.fetch_data;
                let a = self.register.a;
                let c = self.register.c_flag() as u16;

                self.register.a = (a.wrapping_add(u).wrapping_add(c)) & 0xFF;

                self.register.set_z(self.register.a == 0);
                self.register.set_n(true);
                self.register.set_h((a & 0xF).wrapping_add(u & 0xF).wrapping_add(c) > 0xF);
                self.register.set_c(a.wrapping_add(u).wrapping_add(c) > 0xFF);
            }
            InstructionType::SUB => {
                let val = self.read_register(&self.instruction.register_1).wrapping_sub(self.fetch_data);
                let z = val == 0;
                let h = ((self.read_register(&self.instruction.register_1) & 0xF) as i32).wrapping_sub((self.fetch_data & 0xF) as i32) < 0;
                let c = (self.read_register(&self.instruction.register_1) as i32).wrapping_sub(self.fetch_data as i32) < 0;

                self.set_register(&self.instruction.register_1, val);
                self.register.set_z(z);
                self.register.set_h(h);
                self.register.set_c(c);
                self.register.set_n(true);
            }
            InstructionType::SBC => {
                let val = self.fetch_data + self.register.c_flag() as u16;

                let z = (self.read_register(&self.instruction.register_1).wrapping_sub(val)) == 0;
                let h = (((self.read_register(&self.instruction.register_1) & 0xF) as i32).wrapping_sub((self.fetch_data & 0xF) as i32).wrapping_sub(self.register.c_flag() as i32)) < 0;
                let c = (self.read_register(&self.instruction.register_1) as i32).wrapping_sub(self.fetch_data as i32).wrapping_sub(self.register.c_flag() as i32) < 0;

                self.set_register(&self.instruction.register_1, self.read_register(&self.instruction.register_1).wrapping_sub(val));
                self.register.set_z(z);
                self.register.set_h(h);
                self.register.set_c(c);
                self.register.set_n(true);
            }
            InstructionType::AND => {
                self.register.a &= self.fetch_data;
                self.register.set_z(self.register.a == 0);
                self.register.set_n(false);
                self.register.set_h(true);
                self.register.set_c(false);
            }
            InstructionType::XOR => {
                self.register.a ^= self.fetch_data & 0xFF;
                self.register.set_z(self.register.a == 0);
                self.register.set_n(false);
                self.register.set_h(false);
                self.register.set_c(false);
            }
            InstructionType::OR => {
                self.register.a |= self.fetch_data & 0xFF;
                self.register.set_z(self.register.a == 0);
                self.register.set_n(false);
                self.register.set_h(false);
                self.register.set_c(false);
            }
            InstructionType::CP => {
                let n = self.register.a as i32 - self.fetch_data as i32;
                self.register.set_z(n == 0);
                self.register.set_n(true);
                self.register.set_h(((self.register.a as i32 & 0x0F) - (self.fetch_data as i32 & 0x0F)) < 0);
                self.register.set_c(n < 0);
            }
            InstructionType::POP => {
                let lo = self.stack_pop();
                EMU::cycles(1);
                let hi = self.stack_pop();
                EMU::cycles(1);

                let num = (hi << 8) | lo;

                match self.instruction.register_1 {
                    RegisterType::AF => { self.set_register(&self.instruction.register_1, num & 0xFFF0) }
                    _ => { self.set_register(&self.instruction.register_1, num) }
                }
            }
            InstructionType::JUMP => {
                self.go_to(self.fetch_data, false);
            }
            InstructionType::PUSH => {
                let hi = (self.read_register(&self.instruction.register_1) >> 8) & 0xFF;
                EMU::cycles(1);
                self.stack_push(hi as u8);

                let lo = self.read_register(&self.instruction.register_1) & 0xFF;
                EMU::cycles(1);
                self.stack_push(lo as u8);

                EMU::cycles(1);
            }
            InstructionType::RET => {
                self.return_from_procedure()
            }
            InstructionType::CB => {
                let op = self.fetch_data as u8;
                let reg = instruction::reg_lookup(op & 0b111);
                let bit = (op >> 3) & 0b111;
                let bit_op = (op >> 6) & 0b11;
                let mut reg_val = self.read_register8(&reg);

                EMU::cycles(1);

                if reg == &RegisterType::HL {
                    EMU::cycles(2);
                }

                match bit_op {
                    1 => {
                        self.register.set_z((reg_val & (1 << bit)) != 0);
                        self.register.set_n(false);
                        self.register.set_h(true);
                        return;
                    }
                    2 => {
                        reg_val &= !(1 << bit);
                        self.set_register8(&reg, reg_val);
                        return;
                    }
                    3 => {
                        reg_val |= 1 << bit;
                        self.set_register8(&reg, reg_val);
                        return;
                    }
                    _ => {}
                }

                match bit {
                    0 => {
                        let mut result = (reg_val << 1) & 0xFF;
                        let set_c: bool;

                        if (reg_val & (1 << 7)) != 0 {
                            result |= 1;
                            set_c = true;
                        } else {
                            set_c = false;
                        }

                        self.set_register8(&reg, result);
                        self.register.set_z(result == 0);
                        self.register.set_n(false);
                        self.register.set_h(false);
                        self.register.set_c(set_c);
                        return;
                    }

                    1 => {
                        let old = reg_val.clone();
                        reg_val >>= 1;
                        reg_val |= old << 7;

                        self.set_register8(&reg, reg_val);
                        self.register.set_z(reg_val == 0);
                        self.register.set_n(false);
                        self.register.set_h(false);
                        self.register.set_c((old & 1) != 0);
                        return;
                    }

                    2 => {
                        let old = reg_val.clone();
                        reg_val <<= 1;

                        let c_flag_num = if self.register.c_flag() {
                            1
                        } else {
                            0
                        };

                        reg_val |= c_flag_num;
                        self.set_register8(&reg, reg_val);
                        self.register.set_z(reg_val == 0);
                        self.register.set_n(false);
                        self.register.set_h(false);
                        self.register.set_c((old & 0x80) != 0);
                        return;
                    }

                    3 => {
                        let old = reg_val.clone();
                        reg_val >>= 1;
                        let c_flag_num = if self.register.c_flag() {
                            1
                        } else {
                            0
                        };

                        reg_val |= c_flag_num << 7;
                        self.set_register8(&reg, reg_val);
                        self.register.set_z(reg_val == 0);
                        self.register.set_n(false);
                        self.register.set_h(false);
                        self.register.set_c((old & 1) != 0);
                        return;
                    }

                    4 => {
                        let old = reg_val.clone();
                        reg_val <<= 1;
                        self.set_register8(&reg, reg_val);
                        self.register.set_z(reg_val == 0);
                        self.register.set_n(false);
                        self.register.set_h(false);
                        self.register.set_c((old & 0x80) != 0);
                        return;
                    }

                    5 => {
                        let u = reg_val >> 1;
                        self.set_register8(&reg, u);
                        self.register.set_z(u == 0);
                        self.register.set_n(false);
                        self.register.set_h(false);
                        self.register.set_c((reg_val & 1) != 0);
                        return;
                    }
                    6 => {
                        reg_val = ((reg_val & 0xF0) >> 4) | ((reg_val & 0xF) << 4);
                        self.set_register8(&reg, reg_val);
                        self.register.set_z(reg_val == 0);
                        self.register.set_n(false);
                        self.register.set_h(false);
                        self.register.set_c(false);
                        return;
                    }

                    7 => {
                        let u = reg_val >> 1;
                        self.set_register8(&reg, u);
                        self.register.set_z(u == 0);
                        self.register.set_n(false);
                        self.register.set_h(false);
                        self.register.set_c((reg_val & 1) != 0);
                        return;
                    }

                    _ => { panic!("ERROR: INVALID CB: {:#04x}", op) }
                }
            }
            InstructionType::CALL => {
                self.go_to(self.fetch_data, true);
            }
            InstructionType::RETI => {
                self.master_enabled = true;
                self.return_from_procedure()
            }
            InstructionType::LDH => {
                match self.instruction.register_1 {
                    RegisterType::A => {
                        self.set_register(&self.instruction.register_1, self.bus.read(0xFF00 | self.fetch_data))
                    }
                    _ => {
                        self.bus.write(self.mem_dest, self.register.a as u8)
                    }
                }

                EMU::cycles(1)
            }
            InstructionType::JPHL => {}
            InstructionType::DI => {
                self.master_enabled = false;
            }
            InstructionType::EI => {
                self.enabling_ime = true;
            }
            InstructionType::RST => {
                self.go_to(self.instruction.param, true);
            }
            InstructionType::ERR => {}
            InstructionType::RLC => {}
            InstructionType::RRC => {}
            InstructionType::RL => {}
            InstructionType::RR => {}
            InstructionType::SLA => {}
            InstructionType::SRA => {}
            InstructionType::SWAP => {}
            InstructionType::SRL => {}
            InstructionType::BIT => {}
            InstructionType::RES => {}
            InstructionType::SET => {}
        }
    }

    pub fn step(&mut self) {
        self.log();
        // self.log_to_stdout();
        if !self.halted {
            self.fetch_instruction();
            self.fetch_data();
            self.execute();

            self.cycle += 1;
        } else {
            EMU::cycles(1);

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
            self.instruction.instruction_type.to_string(),
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
            self.bus.read(self.register.pc),
            self.bus.read(self.register.pc + 1),
            self.bus.read(self.register.pc + 2),
            self.bus.read(self.register.pc + 3),
        ).replace("0x", "");
        self.log.push_str(&log);
    }

    fn save_log(&self) {
        let file = match OpenOptions::new()
            .append(true)
            .open("log.txt") {
            Ok(file) => {file}
            Err(_) => {File::create("log.txt").unwrap()}
        };
        let mut buffer = BufWriter::new(file);
        buffer.write(self.log.as_ref()).unwrap();
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

    fn fetch_instruction(&mut self) {
        self.current_op_code = self.bus.read(self.register.pc) as u8;
        self.register.pc += 1;
        self.instruction = match Instruction::from(self.current_op_code) {
            None => {
                println!("\x1b[91mUnknown Instruction: \x1b[1;31;40m{:#02x}\x1b[0m", self.current_op_code);
                std::process::exit(1);
            }
            Some(instruction) => instruction
        }
    }

    fn fetch_data(&mut self) {
        self.mem_dest = 0;
        self.dest_is_mem = false;

        match self.instruction.address_mode {
            AddressMode::NONE => {}
            AddressMode::IMP => {}
            AddressMode::RD16 | AddressMode::D16 => {
                let lo = self.bus.read(self.register.pc);
                EMU::cycles(1);
                let hi = self.bus.read(self.register.pc + 1);
                EMU::cycles(1);
                self.register.pc += 2;

                self.fetch_data = lo | (hi << 8);
            }
            AddressMode::RR => {
                self.fetch_data = self.read_register(&self.instruction.register_2)
            }
            AddressMode::MRR => {
                self.fetch_data = self.read_register(&self.instruction.register_2);
                self.mem_dest = self.read_register(&self.instruction.register_1);
                self.dest_is_mem = true;

                match self.instruction.register_1 {
                    RegisterType::C => self.mem_dest |= 0xFF00,
                    _ => {}
                }
            }
            AddressMode::R => {
                self.fetch_data = self.read_register(&self.instruction.register_1)
            }
            AddressMode::RD8 => {
                self.fetch_data = self.bus.read(self.register.pc);
                EMU::cycles(1);
                self.register.pc += 1;
            }
            AddressMode::RMR => {
                let mut address = self.read_register(&self.instruction.register_2);
                match self.instruction.register_1 {
                    RegisterType::C => address |= 0xFF00,
                    _ => {}
                }
                self.fetch_data = self.bus.read(address);
                EMU::cycles(1);
            }
            AddressMode::RHLI => {
                self.fetch_data = self.bus.read(self.read_register(&self.instruction.register_2));
                EMU::cycles(1);
                self.set_register(&RegisterType::HL, self.read_register(&RegisterType::HL) + 1)
            }
            AddressMode::RHLD => {
                self.fetch_data = self.bus.read(self.read_register(&self.instruction.register_2));
                EMU::cycles(1);
                self.set_register(&RegisterType::HL, self.read_register(&RegisterType::HL) - 1)
            }
            AddressMode::HLIR => {
                self.fetch_data = self.read_register(&self.instruction.register_2);
                self.mem_dest = self.read_register(&self.instruction.register_1);
                self.dest_is_mem = true;
                self.set_register(&RegisterType::HL, self.read_register(&RegisterType::HL) + 1);
            }
            AddressMode::HLDR => {
                self.fetch_data = self.read_register(&self.instruction.register_2);
                self.mem_dest = self.read_register(&self.instruction.register_1);
                self.dest_is_mem = true;
                self.set_register(&RegisterType::HL, self.read_register(&RegisterType::HL) - 1);
            }
            AddressMode::RA8 => {
                self.fetch_data = self.bus.read(self.register.pc);
                EMU::cycles(1);
                self.register.pc += 1;
            }
            AddressMode::A8R => {
                self.mem_dest = self.bus.read(self.register.pc) | 0xFF00;
                self.dest_is_mem = true;
                EMU::cycles(1);
                self.register.pc += 1;
            }
            AddressMode::HLSPR => {
                self.fetch_data = self.bus.read(self.register.pc);
                EMU::cycles(1);
                self.register.pc += 1;
            }
            AddressMode::D8 => {
                self.fetch_data = self.bus.read(self.register.pc);
                EMU::cycles(1);
                self.register.pc += 1;
            }
            AddressMode::D16R | AddressMode::A16R => {
                let lo = self.bus.read(self.register.pc);
                EMU::cycles(1);

                let hi = self.bus.read(self.register.pc + 1);
                EMU::cycles(1);

                self.mem_dest = lo | (hi << 8);
                self.dest_is_mem = true;

                self.register.pc += 2;
                self.fetch_data = self.read_register(&self.instruction.register_2);
            }
            AddressMode::MRD8 => {
                self.fetch_data = self.bus.read(self.register.pc);
                EMU::cycles(1);
                self.register.pc += 1;
                self.mem_dest = self.read_register(&self.instruction.register_1);
                self.dest_is_mem = true;
            }
            AddressMode::MR => {
                self.mem_dest = self.read_register(&self.instruction.register_1);
                self.dest_is_mem = true;
                self.fetch_data = self.bus.read(self.read_register(&self.instruction.register_1));
                EMU::cycles(1);
            }
            AddressMode::RA16 => {
                let lo = self.bus.read(self.register.pc);
                EMU::cycles(1);

                let hi = self.bus.read(self.register.pc + 1);
                EMU::cycles(1);

                let address = lo | (hi << 8);

                self.register.pc += 2;
                self.fetch_data = self.bus.read(address);
                EMU::cycles(1);
            }
        }
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
            RegisterType::AF => {
                (self.register.a << 8) | self.register.f
            },
            RegisterType::BC => {
                (self.register.b << 8) | self.register.c
            },
            RegisterType::DE => {
                (self.register.d << 8) | self.register.e
            },
            RegisterType::HL => {
                (self.register.h << 8) | self.register.l
            },
            RegisterType::SP => self.register.sp,
            RegisterType::PC => self.register.pc
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
            RegisterType::HL => self.bus.read(self.read_register(register)) as u8,
            _ => panic!("{:?} is not a valid 8bit register", register)
        }
    }

    fn set_register8(&mut self, register: &RegisterType, value: u8) {
        match register {
            RegisterType::A => { self.register.a = (value & 0xFF) as u16 }
            RegisterType::F => { self.register.f = (value & 0xFF) as u16 }
            RegisterType::B => { self.register.b = (value & 0xFF) as u16 }
            RegisterType::C => { self.register.c = (value & 0xFF) as u16 }
            RegisterType::D => { self.register.d = (value & 0xFF) as u16 }
            RegisterType::E => { self.register.e = (value & 0xFF) as u16 }
            RegisterType::H => { self.register.h = (value & 0xFF) as u16 }
            RegisterType::L => { self.register.l = (value & 0xFF) as u16 }
            RegisterType::HL => { self.bus.write(self.read_register(register), value) }
            _ => panic!("{:?} is not a valid 8bit register", register)
        }
    }

    fn set_register(&mut self, register_type: &RegisterType, value: u16) {
        match register_type {
            RegisterType::NONE => {}
            RegisterType::A => { self.register.a = value & 0xFF }
            RegisterType::F => { self.register.f = value & 0xFF }
            RegisterType::B => { self.register.b = value & 0xFF }
            RegisterType::C => { self.register.c = value & 0xFF }
            RegisterType::D => { self.register.d = value & 0xFF }
            RegisterType::E => { self.register.e = value & 0xFF }
            RegisterType::H => { self.register.h = value & 0xFF }
            RegisterType::L => { self.register.l = value & 0xFF }
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
            RegisterType::SP => { self.register.sp = value }
            RegisterType::PC => { self.register.pc = value }
        }
    }

    fn check_condition(&self) -> bool {
        match self.instruction.condition_type {
            ConditionType::NONE => { true }
            ConditionType::NZ => { !self.register.z_flag() }
            ConditionType::Z => { self.register.z_flag() }
            ConditionType::NC => { !self.register.c_flag() }
            ConditionType::C => { self.register.c_flag() }
        }
    }

    fn stack_push(&mut self, data: u8) {
        self.register.sp = self.register.sp.wrapping_sub(1);
        self.bus.write(self.register.sp, data);
    }

    fn stack_push16(&mut self, data: u16) {
        self.stack_push(((data >> 8) & 0xFF) as u8);
        self.stack_push((data & 0xFF) as u8);
    }

    fn stack_pop(&mut self) -> u16 {
        let data = self.bus.read(self.register.sp);
        self.register.sp += 1;

        data
    }

    fn stack_pop16(&mut self) -> u16 {
        let lo: u16 = self.stack_pop();
        let hi: u16 = self.stack_pop();

        (hi << 8) | lo
    }

    fn go_to(&mut self, address: u16, push_pc: bool) {
        if self.check_condition() {
            if push_pc {
                EMU::cycles(2);
                self.stack_push16(self.register.pc)
            }

            self.register.pc = address;
            EMU::cycles(1)
        }
    }

    fn return_from_procedure(&mut self) {
        match self.instruction.condition_type {
            ConditionType::NONE => {}
            _ => EMU::cycles(1)
        }

        if self.check_condition() {
            let lo = self.stack_pop();
            EMU::cycles(1);
            let hi = self.stack_pop();
            EMU::cycles(1);
            self.register.pc = (hi << 8) | lo;
            EMU::cycles(1)
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
        let mut reg = Register { a: 0x1, f: 0, b: 0, c: 0, d: 0, e: 0, h: 0, l: 0, sp: 0, pc: 0x100 };
        assert!(!reg.z_flag());
        reg.set_z(true);
        assert!(reg.z_flag())
    }

    #[test]
    fn test_set_and_read_and_reset_z_flag() {
        let mut reg = Register { a: 0x1, f: 0, b: 0, c: 0, d: 0, e: 0, h: 0, l: 0, sp: 0, pc: 0x100 };
        assert!(!reg.z_flag());
        reg.set_z(true);
        assert!(reg.z_flag());
        reg.set_z(false);
        assert!(!reg.z_flag());
    }

    #[test]
    fn test_set_and_read_and_reset_n_flag() {
        let mut reg = Register { a: 0x1, f: 0, b: 0, c: 0, d: 0, e: 0, h: 0, l: 0, sp: 0, pc: 0x100 };
        assert!(!reg.n_flag());
        reg.set_n(true);
        assert!(reg.n_flag());
        reg.set_n(false);
        assert!(!reg.n_flag());
    }

    #[test]
    fn test_set_and_read_and_reset_h_flag() {
        let mut reg = Register { a: 0x1, f: 0, b: 0, c: 0, d: 0, e: 0, h: 0, l: 0, sp: 0, pc: 0x100 };
        assert!(!reg.h_flag());
        reg.set_h(true);
        assert!(reg.h_flag());
        reg.set_h(false);
        assert!(!reg.h_flag());
    }

    #[test]
    fn test_set_and_read_and_reset_c_flag() {
        let mut reg = Register { a: 0x1, f: 0, b: 0, c: 0, d: 0, e: 0, h: 0, l: 0, sp: 0, pc: 0x100 };
        assert!(!reg.c_flag());
        reg.set_c(true);
        assert!(reg.c_flag());
        reg.set_c(false);
        assert!(!reg.c_flag());
    }
}