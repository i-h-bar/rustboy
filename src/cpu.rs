use crate::cartridge::Cartridge;
use crate::emu::EMU;
use crate::instruction::{AddressMode, ConditionType, Instruction, InType, RegisterType};

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

    fn c_flag(&self) -> bool {
        bit(self.f as u8, 4)
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
        }
    }

    fn execute(&mut self) {
        self.process();
    }

    fn process(&mut self) {
        match self.instruction.in_type {
            InType::NONE => {}
            InType::NOP => {}
            InType::LD => {}
            InType::INC => {}
            InType::DEC => {}
            InType::RLCA => {}
            InType::ADD => {}
            InType::RRCA => {}
            InType::STOP => {}
            InType::RLA => {}
            InType::JR => {}
            InType::RRA => {}
            InType::DAA => {}
            InType::CPL => {}
            InType::SCF => {}
            InType::CCF => {}
            InType::HALT => {}
            InType::ADC => {}
            InType::SUB => {}
            InType::SBC => {}
            InType::AND => {}
            InType::XOR => {}
            InType::OR => {}
            InType::CP => {}
            InType::POP => {}
            InType::JP => {
                if self.check_condition() {
                    self.register.pc = self.fetch_data;
                    EMU::cycles(1);
                }
            }
            InType::PUSH => {}
            InType::RET => {}
            InType::CB => {}
            InType::CALL => {}
            InType::RETI => {}
            InType::LDH => {}
            InType::JPHL => {}
            InType::DI => {}
            InType::EI => {}
            InType::RST => {}
            InType::ERR => {}
            InType::RLC => {}
            InType::RRC => {}
            InType::RL => {}
            InType::RR => {}
            InType::SLA => {}
            InType::SRA => {}
            InType::SWAP => {}
            InType::SRL => {}
            InType::BIT => {}
            InType::RES => {}
            InType::SET => {}
        }
    }

    pub fn step(&mut self) {
        if !self.halted {
            let pc = self.register.pc;

            self.fetch_instruction();
            self.fetch_data();
            println!("Executing {:#02x}, PC = {:#04x}", self.current_op_code, pc);
            self.execute();
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

        match &self.instruction.address_mode {
            AddressMode::NONE => { return }
            AddressMode::IMP => { return }
            AddressMode::RD16 => {}
            AddressMode::RR => {}
            AddressMode::MRR => {}
            AddressMode::R => {
                self.fetch_data = self.read_register(&self.instruction.register_1)
            }
            AddressMode::RD8 => {
                self.fetch_data = self.cartridge.read(self.register.pc);
                EMU::cycles(1);
                self.register.pc += 1;
            }
            AddressMode::RMR => {}
            AddressMode::RHLI => {}
            AddressMode::RHLD => {}
            AddressMode::HLIR => {}
            AddressMode::HLDR => {}
            AddressMode::RA8 => {}
            AddressMode::A8R => {}
            AddressMode::HLSPR => {}
            AddressMode::D16 => {
                let lo = self.cartridge.read(self.register.pc);
                EMU::cycles(1);
                let hi = self.cartridge.read(self.register.pc + 1);
                EMU::cycles(1);
                self.fetch_data = lo | (hi << 8);
                self.register.pc += 2;
            }
            AddressMode::D8 => {}
            AddressMode::D16R => {}
            AddressMode::MRD8 => {}
            AddressMode::MR => {}
            AddressMode::A16R => {}
            AddressMode::RA16 => {}
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

    pub fn check_condition(&self) -> bool {
        match &self.instruction.condition_type {
            ConditionType::NONE => {true}
            ConditionType::NZ => {!self.register.z_flag()}
            ConditionType::Z => {self.register.z_flag()}
            ConditionType::NC => {!self.register.c_flag()}
            ConditionType::C => {self.register.c_flag()}
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
}