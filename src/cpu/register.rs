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

#[cfg(test)]
mod tests {
    use super::*;

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
