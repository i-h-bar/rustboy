use crate::cartridge::Bus;
use crate::emu::EMU;
use crate::instruction::{AddressMode, ConditionType, Instruction, InstructionType, RegisterType};


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

    fn set_flags(&mut self, z: i8, n: i8, h: i8, c: i8) {
        if z != -1 {
            self.f = bit_set(self.f as u8, 7, z) as u16
        }
        if n != -1 {
            self.f = bit_set(self.f as u8, 6, n) as u16
        }
        if h != -1 {
            self.f = bit_set(self.f as u8, 5, h) as u16
        }
        if c != -1 {
            self.f = bit_set(self.f as u8, 4, c) as u16
        }
    }

    fn is_16bit(&self, reg_type: &RegisterType) -> bool{
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
    cycle: u32
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
            cycle: 0
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
                        let h = (((self.read_register(&self.instruction.register_2) as u8 ) & 0x0F + (self.fetch_data as u8 & 0x0F)) >= 0x10) as i8;
                        let c = (((self.read_register(&self.instruction.register_2)) & 0xFF00 + (self.fetch_data & 0xFF00)) >= 0x100) as i8;
                        self.register.set_flags(0, 0, h, c);
                        self.set_register(
                            &self.instruction.register_1,
                            (self.read_register(&self.instruction.register_2) as i8 + self.fetch_data as i8) as u16
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
                    let val = (self.bus.read(self.read_register(&self.instruction.register_1)) as u8).wrapping_add(1);
                    let val = val & 0xFF;

                    self.bus.write(self.read_register(&self.instruction.register_1), val)
                } else {
                    let val= (self.read_register(&self.instruction.register_1) as u8).wrapping_add(1);
                    self.set_register(&self.instruction.register_1, val as u16)
                }

                if (self.current_op_code & 0x03) != 0x03 {
                    let val = self.read_register(&self.instruction.register_1);
                    self.register.set_flags((val == 0) as i8, 0, ((val & 0x0F) == 0) as i8, -1)
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
                    self.register.set_flags((val == 0) as i8, 1, ((val & 0x0F) == 0x0F) as i8, -1)
                }
            }
            InstructionType::RLCA => {}
            InstructionType::ADD => {
                let val: u32;
                let z: i8;
                let h: i8;
                let c: i8;
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
                    z = 0;
                    h = ((self.read_register(&self.instruction.register_1) & 0xF).wrapping_add(self.fetch_data & 0xF) >= 0x10) as i8;
                    c = ((self.read_register(&self.instruction.register_1) & 0xFF).wrapping_add(self.fetch_data & 0xFF) > 0x100) as i8;
                } else if is_16bit {
                    z = -1;
                    h = ((self.read_register(&self.instruction.register_1) & 0xFFF).wrapping_add(self.fetch_data & 0xFFF) >= 0x1000) as i8;
                    let n = (self.read_register(&self.instruction.register_1) as u32).wrapping_add(self.fetch_data as u32);
                    c = (n >= 0x10000) as i8;
                } else {
                    z = ((val & 0xFF) == 0) as i8;
                    h = ((self.read_register(&self.instruction.register_1) & 0xF).wrapping_add(self.fetch_data & 0xF) >= 0x10) as i8;
                    c = ((self.read_register(&self.instruction.register_1) & 0xFF).wrapping_add(self.fetch_data & 0xFF) >= 0x100) as i8;
                }

                self.set_register(&self.instruction.register_1, (val & 0xFFFF) as u16);
                self.register.set_flags(z, 0, h, c);
            }
            InstructionType::RRCA => {}
            InstructionType::STOP => {}
            InstructionType::RLA => {}
            InstructionType::JR => {
                let rel = (self.fetch_data & 0xFF) as i8;
                let address = (self.register.pc as i16 + rel as i16) as u16;
                self.go_to(address, false);
            }
            InstructionType::RRA => {}
            InstructionType::DAA => {}
            InstructionType::CPL => {}
            InstructionType::SCF => {}
            InstructionType::CCF => {}
            InstructionType::HALT => {}
            InstructionType::ADC => {
                let u = self.fetch_data;
                let a = self.register.a;
                let c = self.register.c_flag() as u16;

                self.register.a = (a.wrapping_add(u).wrapping_add(c)) & 0xFF;

                self.register.set_flags(
                    (self.register.a == 0) as i8,
                    0,
                    ((a & 0xF).wrapping_add(u & 0xF).wrapping_add(c) > 0xF) as i8,
                    (a.wrapping_add(u).wrapping_add(c) > 0xFF) as i8
                );
            }
            InstructionType::SUB => {
                let val = self.read_register(&self.instruction.register_1).wrapping_sub(self.fetch_data);

                let z = (val == 0) as i8;
                let h = (((self.read_register(&self.instruction.register_1) & 0xF) as i32).wrapping_sub((self.fetch_data & 0xF) as i32) < 0) as i8;
                let c = ((self.read_register(&self.instruction.register_1) as i32).wrapping_sub(self.fetch_data as i32) < 0) as i8;

                self.set_register(&self.instruction.register_1, val);
                self.register.set_flags(z, 1, h, c);
            }
            InstructionType::SBC => {
                let val = self.fetch_data + self.register.c_flag() as u16;

                let z = ((self.read_register(&self.instruction.register_1).wrapping_sub(val)) == 0) as i8;
                let h = ((((self.read_register(&self.instruction.register_1) & 0xF) as i32).wrapping_sub((self.fetch_data & 0xF) as i32).wrapping_sub(self.register.c_flag() as i32)) < 0) as i8;
                let c = ((self.read_register(&self.instruction.register_1) as i32).wrapping_sub(self.fetch_data as i32).wrapping_sub(self.register.c_flag() as i32) < 0) as i8;

                self.set_register(&self.instruction.register_1, self.read_register(&self.instruction.register_1).wrapping_sub(val));
                self.register.set_flags(z, 1, h, c);
            }
            InstructionType::AND => {}
            InstructionType::XOR => {
                self.register.a ^= self.fetch_data & 0xFF;
                let set_z = if self.register.a == 0 {
                    1
                } else {
                    0
                };
                self.register.set_flags(set_z, 0, 0, 0);
            }
            InstructionType::OR => {}
            InstructionType::CP => {}
            InstructionType::POP => {
                let lo = self.stack_pop();
                EMU::cycles(1);
                let hi = self.stack_pop();
                EMU::cycles(1);

                let num = (hi << 8) | lo;

                match self.instruction.register_1 {
                    RegisterType::AF => { self.set_register(&self.instruction.register_1, num & 0xFFF0) },
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
            InstructionType::CB => {}
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
                    },
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
            InstructionType::EI => {}
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
        if !self.halted {
            self.fetch_instruction();
            self.fetch_data();
            self.execute();
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
                "{:#08x}: {:#04x} {: <4} | PC: {:#06x} | a: {:#04x}; bc: {:#06x}; de: {:#06x}; sp: {:#06x}; hl: {:#06x} | {}{}{}{}",
                self.cycle,
                self.current_op_code,
                self.instruction.instruction_type.to_string(),
                self.register.pc,
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

            self.cycle += 1;
        }
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
        // Im sus about these registers
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
            RegisterType::AF => reverse(self.register.a),
            RegisterType::BC => reverse(self.register.b),
            RegisterType::DE => reverse(self.register.d),
            RegisterType::HL => reverse(self.register.h),
            RegisterType::SP => self.register.sp,
            RegisterType::PC => self.register.pc
        }
    }

    fn set_register(&mut self, register_type: &RegisterType, value: u16) {
        // Sus about these too
        match register_type {
            RegisterType::NONE => {}
            RegisterType::A => {self.register.a = value & 0xFF}
            RegisterType::F => {self.register.f = value & 0xFF}
            RegisterType::B => {self.register.b = value & 0xFF}
            RegisterType::C => {self.register.c = value & 0xFF}
            RegisterType::D => {self.register.d = value & 0xFF}
            RegisterType::E => {self.register.e = value & 0xFF}
            RegisterType::H => {self.register.h = value & 0xFF}
            RegisterType::L => {self.register.l = value & 0xFF}
            RegisterType::AF => {self.register.a = reverse(value)}
            RegisterType::BC => {self.register.b = reverse(value)}
            RegisterType::DE => {self.register.d = reverse(value)}
            RegisterType::HL => {self.register.h = reverse(value)}
            RegisterType::SP => {self.register.sp = value}
            RegisterType::PC => {self.register.pc = value}
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
        self.register.sp -= 1;
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
            ConditionType::NONE => {},
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

fn bit(a: u8, n: u8) -> bool {
    if (a & (1 << n)) != 0 {
        true
    } else {
        false
    }
}

fn bit_set(a: u8, n: u8, on: i8) -> u8 {
    if on != 0 {
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
        reg.set_flags(1, 0, 0, 0);
        assert!(reg.z_flag())
    }

    #[test]
    fn test_not_set_and_read_flag() {
        let mut reg = Register { a: 0x1, f: 0, b: 0, c: 0, d: 0, e: 0, h: 0, l: 0, sp: 0, pc: 0x100 };
        assert!(!reg.n_flag());
        reg.set_flags(-1, 0, 0, 0);
        assert!(!reg.z_flag())
    }

    #[test]
    fn test_not_set_and_read_flag_again() {
        let mut reg = Register { a: 0x1, f: 0, b: 0, c: 0, d: 0, e: 0, h: 0, l: 0, sp: 0, pc: 0x100 };
        assert!(!reg.n_flag());
        reg.set_flags(0, 0, 0, 0);
        assert!(!reg.z_flag())
    }

    #[test]
    fn test_set_and_read_and_reset_z_flag() {
        let mut reg = Register { a: 0x1, f: 0, b: 0, c: 0, d: 0, e: 0, h: 0, l: 0, sp: 0, pc: 0x100 };
        assert!(!reg.z_flag());
        reg.set_flags(1, 0, 0, 0);
        assert!(reg.z_flag());
        reg.set_flags(-1, 0, 0, 0);
        assert!(reg.z_flag());
        reg.set_flags(0, 0, 0, 0);
        assert!(!reg.z_flag());
    }

    #[test]
    fn test_set_and_read_and_reset_n_flag() {
        let mut reg = Register { a: 0x1, f: 0, b: 0, c: 0, d: 0, e: 0, h: 0, l: 0, sp: 0, pc: 0x100 };
        assert!(!reg.n_flag());
        reg.set_flags(0, 1, 0, 0);
        assert!(reg.n_flag());
        reg.set_flags(0, -1, 0, 0);
        assert!(reg.n_flag());
        reg.set_flags(0, 0, 0, 0);
        assert!(!reg.n_flag());
    }

    #[test]
    fn test_set_and_read_and_reset_h_flag() {
        let mut reg = Register { a: 0x1, f: 0, b: 0, c: 0, d: 0, e: 0, h: 0, l: 0, sp: 0, pc: 0x100 };
        assert!(!reg.h_flag());
        reg.set_flags(0, 0, 1, 0);
        assert!(reg.h_flag());
        reg.set_flags(0, 0, -1, 0);
        assert!(reg.h_flag());
        reg.set_flags(0, 0, 0, 0);
        assert!(!reg.h_flag());
    }

    #[test]
    fn test_set_and_read_and_reset_c_flag() {
        let mut reg = Register { a: 0x1, f: 0, b: 0, c: 0, d: 0, e: 0, h: 0, l: 0, sp: 0, pc: 0x100 };
        assert!(!reg.c_flag());
        reg.set_flags(0, 0, 0, 1);
        assert!(reg.c_flag());
        reg.set_flags(0, 0, 0, -1);
        assert!(reg.c_flag());
        reg.set_flags(0, 0, 0, 0);
        assert!(!reg.c_flag());
    }
}