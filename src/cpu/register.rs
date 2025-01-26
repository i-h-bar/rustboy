use crate::cpu;

#[derive(PartialEq, Debug)]
pub enum RegisterType {
    NONE,
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

pub fn reg_lookup(index: u8) -> &'static RegisterType {
    if index > 0b111 {
        &RegisterType::NONE
    } else {
        REGS.get(index as usize).expect(&format!(
            "index to grab from reg_lookup should never be greater than 7 but was {}",
            index
        ))
    }
}

const REGS: [RegisterType; 8] = [
    RegisterType::B,
    RegisterType::C,
    RegisterType::D,
    RegisterType::E,
    RegisterType::H,
    RegisterType::L,
    RegisterType::HL,
    RegisterType::A,
];

#[derive(Debug)]
pub struct Register {
    pub a: u16,
    pub f: u16,
    pub b: u16,
    pub c: u16,
    pub d: u16,
    pub e: u16,
    pub h: u16,
    pub l: u16,
    pub sp: u16,
    pub pc: u16,
}

impl Register {
    pub fn z_flag(&self) -> bool {
        cpu::bit(self.f as u8, 7)
    }

    pub fn n_flag(&self) -> bool {
        cpu::bit(self.f as u8, 6)
    }

    pub fn h_flag(&self) -> bool {
        cpu::bit(self.f as u8, 5)
    }

    pub fn c_flag(&self) -> bool {
        cpu::bit(self.f as u8, 4)
    }

    pub fn set_z(&mut self, on: bool) {
        self.f = cpu::bit_set(self.f as u8, 7, on) as u16;
    }

    pub fn set_n(&mut self, on: bool) {
        self.f = cpu::bit_set(self.f as u8, 6, on) as u16;
    }

    pub fn set_h(&mut self, on: bool) {
        self.f = cpu::bit_set(self.f as u8, 5, on) as u16;
    }

    pub fn set_c(&mut self, on: bool) {
        self.f = cpu::bit_set(self.f as u8, 4, on) as u16;
    }

    pub fn is_16bit(&self, reg_type: &RegisterType) -> bool {
        match reg_type {
            RegisterType::AF
            | RegisterType::BC
            | RegisterType::DE
            | RegisterType::HL
            | RegisterType::SP
            | RegisterType::PC => true,
            _ => false,
        }
    }
}
