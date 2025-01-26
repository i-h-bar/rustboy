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
