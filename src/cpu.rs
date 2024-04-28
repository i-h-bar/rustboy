use crate::cartridge::Cartridge;
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
}

pub struct CPU {
    register: Register,
    cartridge: Cartridge,
    fetch_data: u16,
    mem_dest: u16,
    dest_is_mem: bool,
    current_op_code: u8,
    instruction: &'static Instruction,
    halted: bool,
    stepping: bool,
    master_interrupt: bool
}

impl CPU {
    pub fn from(cartridge: Cartridge) -> Self {
        Self {
            register: Register { a: 0x1, f: 0, b: 0, c: 0, d: 0, e: 0, h: 0, l: 0, sp: 0, pc: 0x100 },
            cartridge,
            fetch_data: 0,
            mem_dest: 0,
            dest_is_mem: false,
            current_op_code: 0,
            instruction: Instruction::from(0).unwrap(),
            halted: false,
            stepping: false,
            master_interrupt: false
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
                    match self.instruction.register_2 {
                        RegisterType::AF | RegisterType::BC | RegisterType::DE | RegisterType::HL | RegisterType::PC | RegisterType::SP => {
                            EMU::cycles(1);
                            self.cartridge.write16(self.mem_dest, self.fetch_data);
                        }
                        _ => {
                            self.cartridge.write(self.mem_dest, self.fetch_data as u8);
                        }
                    }
                    return
                }

                match self.instruction.address_mode {
                    AddressMode::HLSPR => {
                        let h_flag = if ((self.read_register(&self.instruction.register_2) as u8 ) & 0x0F + (self.fetch_data as u8 & 0x0F)) >= 0x10 {
                            1
                        } else {
                            0
                        };
                        let c_flag = if ((self.read_register(&self.instruction.register_2)) & 0xFF00 + (self.fetch_data & 0xFF00)) >= 0x100 {
                            1
                        } else {
                            0
                        };

                        self.register.set_flags(0, 0, h_flag, c_flag);

                        self.set_register(
                            &self.instruction.register_1,
                            self.read_register(&self.instruction.register_2) + self.fetch_data
                        );
                    }
                    _ => {
                        self.set_register(&self.instruction.register_1, self.fetch_data)
                    }
                }

            }
            InstructionType::INC => {}
            InstructionType::DEC => {}
            InstructionType::RLCA => {}
            InstructionType::ADD => {}
            InstructionType::RRCA => {}
            InstructionType::STOP => {}
            InstructionType::RLA => {}
            InstructionType::JR => {}
            InstructionType::RRA => {}
            InstructionType::DAA => {}
            InstructionType::CPL => {}
            InstructionType::SCF => {}
            InstructionType::CCF => {}
            InstructionType::HALT => {}
            InstructionType::ADC => {}
            InstructionType::SUB => {}
            InstructionType::SBC => {}
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
            InstructionType::POP => {}
            InstructionType::JUMP => {
                if self.check_condition() {
                    self.register.pc = self.fetch_data;
                    EMU::cycles(1);
                }
            }
            InstructionType::PUSH => {}
            InstructionType::RET => {}
            InstructionType::CB => {}
            InstructionType::CALL => {}
            InstructionType::RETI => {}
            InstructionType::LDH => {}
            InstructionType::JPHL => {}
            InstructionType::DI => {
                self.master_interrupt = false;
            }
            InstructionType::EI => {}
            InstructionType::RST => {}
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
            let pc_before = self.register.pc;
            let a_before = self.register.a;
            let b_before = self.register.b;
            let c_before = self.register.c;
            let f_before = self.register.f;

            self.fetch_instruction();
            self.fetch_data();
            self.execute();
            println!(
                "{:#04x}: {: <4} | PC: {:#06x} -> {:#06x} | a: {:#04x} -> {:#04x}; b: {:#04x} -> {:#04x}; c: {:#04x} -> {:#04x} | {:#010b} -> {:#010b}",
                self.current_op_code,
                self.instruction.instruction_type.to_string(),
                pc_before,
                self.register.pc,
                a_before,
                self.register.a,
                b_before,
                self.register.b,
                c_before,
                self.register.c,
                f_before,
                self.register.f
            );
        }
    }

    fn fetch_instruction(&mut self) {
        self.current_op_code = self.cartridge.read(self.register.pc) as u8;
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
                let lo = self.cartridge.read(self.register.pc);
                EMU::cycles(1);
                let hi = self.cartridge.read(self.register.pc + 1);
                EMU::cycles(1);
                self.register.pc += 1;

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
                self.fetch_data = self.cartridge.read(self.register.pc);
                EMU::cycles(1);
                self.register.pc += 1;
            }
            AddressMode::RMR => {
                let mut address = self.read_register(&self.instruction.register_2);
                match self.instruction.register_1 {
                    RegisterType::C => address |= 0xFF00,
                    _ => {}
                }
                self.fetch_data = self.cartridge.read(address);
                EMU::cycles(1);
            }
            AddressMode::RHLI => {
                self.fetch_data = self.cartridge.read(self.read_register(&self.instruction.register_2));
                EMU::cycles(1);
                self.set_register(&RegisterType::HL, self.read_register(&RegisterType::HL) + 1)
            }
            AddressMode::RHLD => {
                self.fetch_data = self.cartridge.read(self.read_register(&self.instruction.register_2));
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
                self.fetch_data = self.cartridge.read(self.register.pc);
                EMU::cycles(1);
                self.register.pc += 1;
            }
            AddressMode::A8R => {
                self.mem_dest = self.cartridge.read(self.register.pc) | 0xFF00;
                self.dest_is_mem = true;
                EMU::cycles(1);
                self.register.pc += 1;
            }
            AddressMode::HLSPR => {
                self.fetch_data = self.cartridge.read(self.register.pc);
                EMU::cycles(1);
                self.register.pc += 1;
            }
            AddressMode::D8 => {
                self.fetch_data = self.cartridge.read(self.register.pc);
                EMU::cycles(1);
                self.register.pc += 1;
            }
            AddressMode::D16R | AddressMode::A16R => {
                let lo = self.cartridge.read(self.register.pc);
                EMU::cycles(1);

                let hi = self.cartridge.read(self.register.pc + 1);
                EMU::cycles(1);

                self.mem_dest = lo | (hi << 8);
                self.dest_is_mem = true;

                self.register.pc += 2;
                self.fetch_data = self.read_register(&self.instruction.register_2);
            }
            AddressMode::MRD8 => {
                self.fetch_data = self.cartridge.read(self.register.pc);
                EMU::cycles(1);
                self.register.pc += 1;
                self.mem_dest = self.read_register(&self.instruction.register_1);
                self.dest_is_mem = true;
            }
            AddressMode::MR => {
                self.mem_dest = self.read_register(&self.instruction.register_1);
                self.dest_is_mem = true;
                self.fetch_data = self.cartridge.read(self.read_register(&self.instruction.register_1));
                EMU::cycles(1);
            }
            AddressMode::RA16 => {
                let lo = self.cartridge.read(self.register.pc);
                EMU::cycles(1);

                let hi = self.cartridge.read(self.register.pc + 1);
                EMU::cycles(1);

                let address = lo | (hi << 8);

                self.register.pc += 2;
                self.fetch_data = self.cartridge.read(address);
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
            RegisterType::AF => reverse(self.register.a),
            RegisterType::BC => reverse(self.register.b),
            RegisterType::DE => reverse(self.register.d),
            RegisterType::HL => reverse(self.register.h),
            RegisterType::SP => self.register.sp,
            RegisterType::PC => self.register.pc
        }
    }

    fn set_register(&mut self, register_type: &RegisterType, value: u16) {
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

    pub fn check_condition(&self) -> bool {
        match self.instruction.condition_type {
            ConditionType::NONE => { true }
            ConditionType::NZ => { !self.register.z_flag() }
            ConditionType::Z => { self.register.z_flag() }
            ConditionType::NC => { !self.register.c_flag() }
            ConditionType::C => { self.register.c_flag() }
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
    fn test_set_and_read_and_reset_flag() {
        let mut reg = Register { a: 0x1, f: 0, b: 0, c: 0, d: 0, e: 0, h: 0, l: 0, sp: 0, pc: 0x100 };
        assert!(!reg.z_flag());
        reg.set_flags(1, 0, 0, 0);
        assert!(reg.z_flag());
        reg.set_flags(0, 0, 0, 0);
        assert!(!reg.z_flag());
    }
}