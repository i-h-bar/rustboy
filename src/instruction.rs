use std::collections::HashMap;
use std::fmt;
use std::fmt::Debug;

use lazy_static::lazy_static;

lazy_static! {
    static ref INSTRUCTION_MAP: HashMap<u8, Instruction> =
        [
            (0x00, Instruction{instruction_type: InstructionType::NOP, address_mode: AddressMode::IMP, register_1: RegisterType::NONE, register_2: RegisterType::NONE, condition_type: ConditionType::NONE, param: 0}),
            (0x01, Instruction{instruction_type: InstructionType::LD,address_mode: AddressMode::RD16,register_1: RegisterType::BC,register_2: RegisterType::NONE,condition_type: ConditionType::NONE,param: 0,}),
            (0x02, Instruction{instruction_type: InstructionType::LD,address_mode: AddressMode::MRR,register_1: RegisterType::BC,register_2: RegisterType::A,condition_type: ConditionType::NONE,param: 0,}),
            
            (0x05, Instruction{instruction_type: InstructionType::DEC, address_mode: AddressMode::R, register_1: RegisterType::B, register_2: RegisterType::NONE, condition_type: ConditionType::NONE, param: 0}),
            (0x06, Instruction{instruction_type: InstructionType::LD,address_mode: AddressMode::RD8,register_1: RegisterType::B,register_2: RegisterType::NONE,condition_type: ConditionType::NONE,param: 0,}),

            (0x08, Instruction{instruction_type: InstructionType::LD,address_mode: AddressMode::A16R,register_1: RegisterType::NONE,register_2: RegisterType::SP,condition_type: ConditionType::NONE,param: 0,}),

            (0x0A, Instruction{instruction_type: InstructionType::LD,address_mode: AddressMode::RMR,register_1: RegisterType::A,register_2: RegisterType::BC,condition_type: ConditionType::NONE,param: 0,}),

            (0x0E, Instruction{instruction_type: InstructionType::LD, address_mode: AddressMode::RD8, register_1: RegisterType::C, register_2: RegisterType::NONE, condition_type: ConditionType::NONE, param: 0}),

            (0x11, Instruction{instruction_type: InstructionType::LD,address_mode: AddressMode::RD16,register_1: RegisterType::DE,register_2: RegisterType::NONE,condition_type: ConditionType::NONE,param: 0,}),
            (0x12, Instruction{instruction_type: InstructionType::LD,address_mode: AddressMode::MRR,register_1: RegisterType::DE,register_2: RegisterType::A,condition_type: ConditionType::NONE,param: 0,}),

            (0x15, Instruction{instruction_type: InstructionType::DEC,address_mode: AddressMode::R,register_1: RegisterType::D,register_2: RegisterType::NONE,condition_type: ConditionType::NONE,param: 0,}),
            (0x16, Instruction{instruction_type: InstructionType::LD,address_mode: AddressMode::R,register_1: RegisterType::D,register_2: RegisterType::NONE,condition_type: ConditionType::NONE,param: 0,}),

            (0x1A, Instruction{instruction_type: InstructionType::LD,address_mode: AddressMode::RMR,register_1: RegisterType::A,register_2: RegisterType::DE,condition_type: ConditionType::NONE,param: 0,}),
            (0x1E, Instruction{instruction_type: InstructionType::LD,address_mode: AddressMode::RD8,register_1: RegisterType::E,register_2: RegisterType::NONE,condition_type: ConditionType::NONE,param: 0,}),

            (0x21, Instruction{instruction_type: InstructionType::LD,address_mode: AddressMode::RD16,register_1: RegisterType::HL,register_2: RegisterType::NONE,condition_type: ConditionType::NONE,param: 0,}),
            (0x22, Instruction{instruction_type: InstructionType::LD,address_mode: AddressMode::HLIR,register_1: RegisterType::HL,register_2: RegisterType::A,condition_type: ConditionType::NONE,param: 0,}),

            (0x25, Instruction{instruction_type: InstructionType::DEC,address_mode: AddressMode::R,register_1: RegisterType::H,register_2: RegisterType::NONE,condition_type: ConditionType::NONE,param: 0,}),
            (0x26, Instruction{instruction_type: InstructionType::LD,address_mode: AddressMode::RD8,register_1: RegisterType::H,register_2: RegisterType::NONE,condition_type: ConditionType::NONE,param: 0,}),

            (0x2A, Instruction{instruction_type: InstructionType::LD,address_mode: AddressMode::RHLI,register_1: RegisterType::A,register_2: RegisterType::HL,condition_type: ConditionType::NONE,param: 0,}),

            (0x2E, Instruction{instruction_type: InstructionType::LD,address_mode: AddressMode::RD8,register_1: RegisterType::L,register_2: RegisterType::NONE,condition_type: ConditionType::NONE,param: 0,}),

            (0x31, Instruction{instruction_type: InstructionType::LD,address_mode: AddressMode::RD16,register_1: RegisterType::SP,register_2: RegisterType::NONE,condition_type: ConditionType::NONE,param: 0,}),
            (0x32, Instruction{instruction_type: InstructionType::LD,address_mode: AddressMode::HLDR,register_1: RegisterType::HL,register_2: RegisterType::A,condition_type: ConditionType::NONE,param: 0,}),

            (0x35, Instruction{instruction_type: InstructionType::DEC,address_mode: AddressMode::R,register_1: RegisterType::HL,register_2: RegisterType::NONE,condition_type: ConditionType::NONE,param: 0,}),
            (0x36, Instruction{instruction_type: InstructionType::LD,address_mode: AddressMode::MRD8,register_1: RegisterType::HL,register_2: RegisterType::NONE,condition_type: ConditionType::NONE,param: 0,}),

            (0x3A, Instruction{instruction_type: InstructionType::LD,address_mode: AddressMode::RHLD,register_1: RegisterType::A,register_2: RegisterType::HL,condition_type: ConditionType::NONE,param: 0,}),

            (0x3E, Instruction{instruction_type: InstructionType::LD,address_mode: AddressMode::RD8,register_1: RegisterType::A,register_2: RegisterType::NONE,condition_type: ConditionType::NONE,param: 0,}),

            (0xAF, Instruction{instruction_type: InstructionType::XOR, address_mode: AddressMode::R, register_1: RegisterType::A, register_2: RegisterType::NONE, condition_type: ConditionType::NONE, param: 0}),
            (0xC3, Instruction{instruction_type: InstructionType::JUMP, address_mode: AddressMode::D16, register_1: RegisterType::NONE, register_2: RegisterType::NONE, condition_type: ConditionType::NONE, param: 0}),
            (0xF3, Instruction{instruction_type: InstructionType::DI, address_mode: AddressMode::NONE, register_1: RegisterType::NONE, register_2: RegisterType::NONE, condition_type: ConditionType::NONE, param: 0}),
        ]
        .into_iter()
        .collect();
}

pub enum AddressMode {
    NONE,
    IMP,
    RD16,
    RR,
    MRR,
    R,
    RD8,
    RMR,
    RHLI,
    RHLD,
    HLIR,
    HLDR,
    RA8,
    A8R,
    HLSPR,
    D16,
    D8,
    D16R,
    MRD8,
    MR,
    A16R,
    RA16,
}

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

#[derive(Debug)]
pub enum InstructionType {
    NONE,
    NOP,
    LD,
    INC,
    DEC,
    RLCA,
    ADD,
    RRCA,
    STOP,
    RLA,
    JR,
    RRA,
    DAA,
    CPL,
    SCF,
    CCF,
    HALT,
    ADC,
    SUB,
    SBC,
    AND,
    XOR,
    OR,
    CP,
    POP,
    JUMP,
    PUSH,
    RET,
    CB,
    CALL,
    RETI,
    LDH,
    JPHL,
    DI,
    EI,
    RST,
    ERR,
    //CB instructions...
    RLC,
    RRC,
    RL,
    RR,
    SLA,
    SRA,
    SWAP,
    SRL,
    BIT,
    RES,
    SET,
}

impl fmt::Display for InstructionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

pub enum ConditionType {
    NONE,
    NZ,
    Z,
    NC,
    C,
}

pub struct Instruction {
    pub instruction_type: InstructionType,
    pub address_mode: AddressMode,
    pub register_1: RegisterType,
    pub register_2: RegisterType,
    pub condition_type: ConditionType,
    pub param: u8,
}

impl Instruction {
    pub fn from(opcode: u8) -> Option<&'static Self> {
        Some(INSTRUCTION_MAP.get(&opcode)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction_from_op_code() {
        let inst = Instruction::from(0).unwrap();
        match inst.instruction_type {
            InstructionType::NOP => {}
            _ => panic!("Not correct instruction type")
        }

        match &inst.address_mode {
            AddressMode::IMP => {}
            _ => panic!("Not correct instruction type")
        }
        match inst.register_1 {
            RegisterType::NONE => {}
            _ => panic!("Not correct reg type")
        }
        match inst.register_2 {
            RegisterType::NONE => {}
            _ => panic!("Not correct reg type")
        }
        match inst.condition_type {
            ConditionType::NONE => {}
            _ => panic!("Not correct reg type")
        }
        assert_eq!(inst.param, 0);
    }

    #[test]
    fn test_0e() {
        let inst = Instruction::from(0x0E).unwrap();
        match inst.instruction_type {
            InstructionType::LD => {}
            _ => panic!("Not correct instruction type")
        }

        match inst.address_mode {
            AddressMode::RD8 => {}
            _ => panic!("Not correct instruction type")
        }
        match inst.register_1 {
            RegisterType::C => {}
            _ => panic!("Not correct reg type")
        }

        match inst.register_2 {
            RegisterType::NONE => {}
            _ => panic!("Not correct reg type")
        }

        match inst.condition_type {
            ConditionType::NONE => {}
            _ => panic!("Not correct reg type")
        }

        assert_eq!(inst.param, 0);
    }
}