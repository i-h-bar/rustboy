use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static! {
    static ref INSTRUCTION_MAP: HashMap<u8, (InType, Option<AddressMode>, Option<RegisterType>)> =
        [
            (0x00, (InType::NOP, Some(AddressMode::IMP), None)),
            (0x05, (InType::DEC, Some(AddressMode::R), Some(RegisterType::B))),
            (0x0E, (InType::LD, Some(AddressMode::RD8), Some(RegisterType::C))),
            (0xAF, (InType::XOR, Some(AddressMode::R), Some(RegisterType::A))),
            (0xC3, (InType::JP, Some(AddressMode::D16), None)),
            (0xF3, (InType::DI, None, None)),
        ]
        .into_iter()
        .collect();
}

enum AddressMode {
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

enum RegisterType {
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

enum InType {
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

enum ConditionType {
    None,
    NZ,
    Z,
    NC,
    C,
}

pub struct Instruction {
    in_type: InType,
    address_mode: AddressMode,
    register_1: RegisterType,
    register_2: RegisterType,
    condition_type: ConditionType,
    param: u8,
}
