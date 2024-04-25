use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static! {
    static ref INSTRUCTION_MAP: HashMap<u8, (InType, Option<AddressMode>, Option<RegisterType>, Option<RegisterType>, Option<ConditionType>)> =
        [
            (0x00, (InType::NOP, Some(AddressMode::IMP), None, None, None)),
            (0x05, (InType::DEC, Some(AddressMode::R), Some(RegisterType::B), None, None)),
            (0x0E, (InType::LD, Some(AddressMode::RD8), Some(RegisterType::C), None, None)),
            (0xAF, (InType::XOR, Some(AddressMode::R), Some(RegisterType::A), None, None)),
            (0xC3, (InType::JP, Some(AddressMode::D16), None, None, None)),
            (0xF3, (InType::DI, None, None, None, None)),
        ]
        .into_iter()
        .collect();
}

pub enum AddressMode {
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

pub enum InType {
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
    JP,
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

pub enum ConditionType {
    None,
    NZ,
    Z,
    NC,
    C,
}

pub struct Instruction {
    pub in_type: &'static InType,
    pub address_mode: &'static Option<AddressMode>,
    pub register_1: &'static Option<RegisterType>,
    pub register_2: &'static Option<RegisterType>,
    pub condition_type: &'static Option<ConditionType>,
    pub param: u8,
}

impl Instruction {
    pub fn from(opcode: &u8) -> Option<Self> {
        let (
            in_type,
            address_mode,
            register_1,
            register_2,
            condition_type
        ) = INSTRUCTION_MAP.get(opcode)?;


        Some(
            Self {
                in_type,
                address_mode,
                register_1,
                register_2,
                condition_type,
                param: 0
            }
        )
    }

    pub fn default() -> Self {
        Self {
            in_type: &InType::NOP,
            address_mode: &None,
            register_1: &None,
            register_2: &None,
            condition_type: &None,
            param: 0
        }
    }
}