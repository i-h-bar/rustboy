use crate::cpu::actions::Action;
use crate::cpu::addresses::AddressMode;
use crate::cpu::conditions::ConditionType;
use crate::cpu::register::RegisterType;
use crate::cpu::CPU;

pub struct Instruction {
    pub action: &'static Action,
    pub address: &'static AddressMode,
    pub register_1: &'static RegisterType,
    pub register_2: &'static RegisterType,
    pub condition: &'static ConditionType,
    pub param: &'static u16,
}

impl Instruction {
    pub fn from(opcode: u8) -> Self {
        let (action, address, register_1, register_2, condition, param) =
            &OP_CODES[opcode as usize];

        Self {
            action,
            address,
            register_1,
            register_2,
            condition,
            param,
        }
    }

    pub fn execute(&self, cpu: &mut CPU) {
        self.address.fetch(cpu, &self);
        self.action.execute(cpu, &self);
    }
}

#[rustfmt::skip]
const OP_CODES: [(
    Action,
    AddressMode,
    RegisterType,
    RegisterType,
    ConditionType,
    u16,
); 256] = [
    (Action::NOP, AddressMode::IMP, RegisterType::NONE, RegisterType::NONE, ConditionType::NONE, 0,), // 0x00
    (Action::LD, AddressMode::RD16, RegisterType::BC, RegisterType::NONE, ConditionType::NONE, 0,), // 0x01
    (Action::LD, AddressMode::MRR, RegisterType::BC, RegisterType::A, ConditionType::NONE, 0,), // 0x02
    (Action::INC, AddressMode::R, RegisterType::BC, RegisterType::NONE, ConditionType::NONE, 0,), // 0x03
    (Action::INC, AddressMode::R, RegisterType::B, RegisterType::NONE, ConditionType::NONE, 0,), // 0x04
    (Action::DEC, AddressMode::R, RegisterType::B, RegisterType::NONE, ConditionType::NONE, 0,), // 0x05
    (Action::LD, AddressMode::RD8, RegisterType::B, RegisterType::NONE, ConditionType::NONE, 0,), // 0x06
    (Action::RLCA, AddressMode::NONE, RegisterType::NONE, RegisterType::NONE, ConditionType::NONE, 0,), // 0x07
    (Action::LD, AddressMode::A16R, RegisterType::NONE, RegisterType::SP, ConditionType::NONE, 0,), // 0x08
    (Action::ADD, AddressMode::RR, RegisterType::HL, RegisterType::BC, ConditionType::NONE, 0,), // 0x09
    (Action::LD, AddressMode::RMR, RegisterType::A, RegisterType::BC, ConditionType::NONE, 0,), // 0x0A
    (Action::DEC, AddressMode::R, RegisterType::BC, RegisterType::NONE, ConditionType::NONE, 0,), // 0x0B
    (Action::INC, AddressMode::R, RegisterType::C, RegisterType::NONE, ConditionType::NONE, 0,), // 0x0C
    (Action::DEC, AddressMode::R, RegisterType::C, RegisterType::NONE, ConditionType::NONE, 0,), // 0x0D
    (Action::LD, AddressMode::RD8, RegisterType::C, RegisterType::NONE, ConditionType::NONE, 0,), // 0x0E
    (Action::RRCA, AddressMode::NONE, RegisterType::NONE, RegisterType::NONE, ConditionType::NONE, 0,), // 0x0F
    (Action::STOP, AddressMode::NONE, RegisterType::NONE, RegisterType::NONE, ConditionType::NONE, 0,), // 0x10
    (Action::LD, AddressMode::RD16, RegisterType::DE, RegisterType::NONE, ConditionType::NONE, 0,), // 0x11
    (Action::LD, AddressMode::MRR, RegisterType::DE, RegisterType::A, ConditionType::NONE, 0,), // 0x12
    (Action::INC, AddressMode::R, RegisterType::DE, RegisterType::NONE, ConditionType::NONE, 0,), // 0x13
    (Action::INC, AddressMode::R, RegisterType::D, RegisterType::NONE, ConditionType::NONE, 0,), // 0x14
    (Action::DEC, AddressMode::R, RegisterType::D, RegisterType::NONE, ConditionType::NONE, 0,), // 0x15
    (Action::LD, AddressMode::R, RegisterType::D, RegisterType::NONE, ConditionType::NONE, 0,), // 0x16
    (Action::RLA, AddressMode::NONE, RegisterType::NONE, RegisterType::NONE, ConditionType::NONE, 0,), // 0x17
    (Action::JR, AddressMode::D8, RegisterType::NONE, RegisterType::NONE, ConditionType::NONE, 0,), // 0x18
    (Action::ADD, AddressMode::RR, RegisterType::HL, RegisterType::DE, ConditionType::NONE, 0,), // 0x19
    (Action::LD, AddressMode::RMR, RegisterType::A, RegisterType::DE, ConditionType::NONE, 0,), // 0x1A
    (Action::DEC, AddressMode::R, RegisterType::DE, RegisterType::NONE, ConditionType::NONE, 0,), // 0x1B
    (Action::INC, AddressMode::R, RegisterType::E, RegisterType::NONE, ConditionType::NONE, 0,), // 0x1C
    (Action::DEC, AddressMode::R, RegisterType::E, RegisterType::NONE, ConditionType::NONE, 0,), // 0x1D
    (Action::LD, AddressMode::RD8, RegisterType::E, RegisterType::NONE, ConditionType::NONE, 0,), // 0x1E
    (Action::RRA, AddressMode::NONE, RegisterType::NONE, RegisterType::NONE, ConditionType::NONE, 0,), // 0x1F
    (Action::JR, AddressMode::D8, RegisterType::NONE, RegisterType::NONE, ConditionType::NZ, 0,), // 0x20
    (Action::LD, AddressMode::RD16, RegisterType::HL, RegisterType::NONE, ConditionType::NONE, 0,), // 0x21
    (Action::LD, AddressMode::HLIR, RegisterType::HL, RegisterType::A, ConditionType::NONE, 0,), // 0x22
    (Action::INC, AddressMode::R, RegisterType::HL, RegisterType::NONE, ConditionType::NONE, 0,), // 0x23
    (Action::INC, AddressMode::R, RegisterType::H, RegisterType::NONE, ConditionType::NONE, 0,), // 0x24
    (Action::DEC, AddressMode::R, RegisterType::H, RegisterType::NONE, ConditionType::NONE, 0,), // 0x25
    (Action::LD, AddressMode::RD8, RegisterType::H, RegisterType::NONE, ConditionType::NONE, 0,), // 0x26
    (Action::DAA, AddressMode::NONE, RegisterType::NONE, RegisterType::NONE, ConditionType::NONE, 0,), // 0x27
    (Action::JR, AddressMode::D8, RegisterType::NONE, RegisterType::NONE, ConditionType::Z, 0,), // 0x28
    (Action::ADD, AddressMode::RR, RegisterType::HL, RegisterType::HL, ConditionType::NONE, 0,), // 0x29
    (Action::LD, AddressMode::RHLI, RegisterType::A, RegisterType::HL, ConditionType::NONE, 0,), // 0x2A
    (Action::DEC, AddressMode::R, RegisterType::HL, RegisterType::NONE, ConditionType::NONE, 0,), // 0x2B
    (Action::INC, AddressMode::R, RegisterType::L, RegisterType::NONE, ConditionType::NONE, 0,), // 0x2C
    (Action::DEC, AddressMode::R, RegisterType::L, RegisterType::NONE, ConditionType::NONE, 0,), // 0x2D
    (Action::LD, AddressMode::RD8, RegisterType::L, RegisterType::NONE, ConditionType::NONE, 0,), // 0x2E
    (Action::CPL, AddressMode::NONE, RegisterType::NONE, RegisterType::NONE, ConditionType::NONE, 0,), // 0x2F
    (Action::JR, AddressMode::D8, RegisterType::NONE, RegisterType::NONE, ConditionType::NC, 0,), // 0x30
    (Action::LD, AddressMode::RD16, RegisterType::SP, RegisterType::NONE, ConditionType::NONE, 0,), // 0x31
    (Action::LD, AddressMode::HLDR, RegisterType::HL, RegisterType::A, ConditionType::NONE, 0,), // 0x32
    (Action::INC, AddressMode::R, RegisterType::SP, RegisterType::NONE, ConditionType::NONE, 0,), // 0x33
    (Action::INC, AddressMode::MR, RegisterType::HL, RegisterType::NONE, ConditionType::NONE, 0,), // 0x34
    (Action::DEC, AddressMode::MR, RegisterType::HL, RegisterType::NONE, ConditionType::NONE, 0,), // 0x35
    (Action::LD, AddressMode::MRD8, RegisterType::HL, RegisterType::NONE, ConditionType::NONE, 0,), // 0x36
    (Action::SCF, AddressMode::NONE, RegisterType::NONE, RegisterType::NONE, ConditionType::NONE, 0,), // 0x37
    (Action::JR, AddressMode::D8, RegisterType::NONE, RegisterType::NONE, ConditionType::C, 0,), // 0x38
    (Action::ADD, AddressMode::RR, RegisterType::HL, RegisterType::SP, ConditionType::NONE, 0,), // 0x39
    (Action::LD, AddressMode::RHLD, RegisterType::A, RegisterType::HL, ConditionType::NONE, 0,), // 0x3A
    (Action::DEC, AddressMode::R, RegisterType::SP, RegisterType::NONE, ConditionType::NONE, 0,), // 0x3B
    (Action::INC, AddressMode::R, RegisterType::A, RegisterType::NONE, ConditionType::NONE, 0,), // 0x3C
    (Action::DEC, AddressMode::R, RegisterType::A, RegisterType::NONE, ConditionType::NONE, 0,), // 0x3D
    (Action::LD, AddressMode::RD8, RegisterType::A, RegisterType::NONE, ConditionType::NONE, 0,), // 0x3E
    (Action::CCF, AddressMode::NONE, RegisterType::NONE, RegisterType::NONE, ConditionType::NONE, 0,), // 0x3F
    (Action::LD, AddressMode::RR, RegisterType::B, RegisterType::B, ConditionType::NONE, 0,), // 0x40
    (Action::LD, AddressMode::RR, RegisterType::B, RegisterType::C, ConditionType::NONE, 0,), // 0x41
    (Action::LD, AddressMode::RR, RegisterType::B, RegisterType::D, ConditionType::NONE, 0,), // 0x42
    (Action::LD, AddressMode::RR, RegisterType::B, RegisterType::E, ConditionType::NONE, 0,), // 0x43
    (Action::LD, AddressMode::RR, RegisterType::B, RegisterType::H, ConditionType::NONE, 0,), // 0x44
    (Action::LD, AddressMode::RR, RegisterType::B, RegisterType::L, ConditionType::NONE, 0,), // 0x45
    (Action::LD, AddressMode::RMR, RegisterType::B, RegisterType::HL, ConditionType::NONE, 0,), // 0x46
    (Action::LD, AddressMode::RR, RegisterType::B, RegisterType::A, ConditionType::NONE, 0,), // 0x47
    (Action::LD, AddressMode::RR, RegisterType::C, RegisterType::B, ConditionType::NONE, 0,), // 0x48
    (Action::LD, AddressMode::RR, RegisterType::C, RegisterType::C, ConditionType::NONE, 0,), // 0x49
    (Action::LD, AddressMode::RR, RegisterType::C, RegisterType::D, ConditionType::NONE, 0,), // 0x4A
    (Action::LD, AddressMode::RR, RegisterType::C, RegisterType::E, ConditionType::NONE, 0,), // 0x4B
    (Action::LD, AddressMode::RR, RegisterType::C, RegisterType::H, ConditionType::NONE, 0,), // 0x4C
    (Action::LD, AddressMode::RR, RegisterType::C, RegisterType::L, ConditionType::NONE, 0,), // 0x4D
    (Action::LD, AddressMode::RMR, RegisterType::C, RegisterType::HL, ConditionType::NONE, 0,), // 0x4E
    (Action::LD, AddressMode::RR, RegisterType::C, RegisterType::A, ConditionType::NONE, 0,), // 0x4F
    (Action::LD, AddressMode::RR, RegisterType::D, RegisterType::B, ConditionType::NONE, 0,), // 0x50
    (Action::LD, AddressMode::RR, RegisterType::D, RegisterType::C, ConditionType::NONE, 0,), // 0x51
    (Action::LD, AddressMode::RR, RegisterType::D, RegisterType::D, ConditionType::NONE, 0,), // 0x52
    (Action::LD, AddressMode::RR, RegisterType::D, RegisterType::E, ConditionType::NONE, 0,), // 0x53
    (Action::LD, AddressMode::RR, RegisterType::D, RegisterType::H, ConditionType::NONE, 0,), // 0x54
    (Action::LD, AddressMode::RR, RegisterType::D, RegisterType::L, ConditionType::NONE, 0,), // 0x55
    (Action::LD, AddressMode::RMR, RegisterType::D, RegisterType::HL, ConditionType::NONE, 0,), // 0x56
    (Action::LD, AddressMode::RR, RegisterType::D, RegisterType::A, ConditionType::NONE, 0,), // 0x57
    (Action::LD, AddressMode::RR, RegisterType::E, RegisterType::B, ConditionType::NONE, 0,), // 0x58
    (Action::LD, AddressMode::RR, RegisterType::E, RegisterType::C, ConditionType::NONE, 0,), // 0x59
    (Action::LD, AddressMode::RR, RegisterType::E, RegisterType::D, ConditionType::NONE, 0,), // 0x5A
    (Action::LD, AddressMode::RR, RegisterType::E, RegisterType::E, ConditionType::NONE, 0,), // 0x5B
    (Action::LD, AddressMode::RR, RegisterType::E, RegisterType::H, ConditionType::NONE, 0,), // 0x5C
    (Action::LD, AddressMode::RR, RegisterType::E, RegisterType::L, ConditionType::NONE, 0,), // 0x5D
    (Action::LD, AddressMode::RMR, RegisterType::E, RegisterType::HL, ConditionType::NONE, 0,), // 0x5E
    (Action::LD, AddressMode::RR, RegisterType::E, RegisterType::A, ConditionType::NONE, 0,), // 0x5F
    (Action::LD, AddressMode::RR, RegisterType::H, RegisterType::B, ConditionType::NONE, 0,), // 0x60
    (Action::LD, AddressMode::RR, RegisterType::H, RegisterType::C, ConditionType::NONE, 0,), // 0x61
    (Action::LD, AddressMode::RR, RegisterType::H, RegisterType::D, ConditionType::NONE, 0,), // 0x62
    (Action::LD, AddressMode::RR, RegisterType::H, RegisterType::E, ConditionType::NONE, 0,), // 0x63
    (Action::LD, AddressMode::RR, RegisterType::H, RegisterType::H, ConditionType::NONE, 0,), // 0x64
    (Action::LD, AddressMode::RR, RegisterType::H, RegisterType::L, ConditionType::NONE, 0,), // 0x65
    (Action::LD, AddressMode::RMR, RegisterType::H, RegisterType::HL, ConditionType::NONE, 0,), // 0x66
    (Action::LD, AddressMode::RR, RegisterType::H, RegisterType::A, ConditionType::NONE, 0,), // 0x67
    (Action::LD, AddressMode::RR, RegisterType::L, RegisterType::B, ConditionType::NONE, 0,), // 0x68
    (Action::LD, AddressMode::RR, RegisterType::L, RegisterType::C, ConditionType::NONE, 0,), // 0x69
    (Action::LD, AddressMode::RR, RegisterType::L, RegisterType::D, ConditionType::NONE, 0,), // 0x6A
    (Action::LD, AddressMode::RR, RegisterType::L, RegisterType::E, ConditionType::NONE, 0,), // 0x6B
    (Action::LD, AddressMode::RR, RegisterType::L, RegisterType::H, ConditionType::NONE, 0,), // 0x6C
    (Action::LD, AddressMode::RR, RegisterType::L, RegisterType::L, ConditionType::NONE, 0,), // 0x6D
    (Action::LD, AddressMode::RMR, RegisterType::L, RegisterType::HL, ConditionType::NONE, 0,), // 0x6E
    (Action::LD, AddressMode::RR, RegisterType::L, RegisterType::A, ConditionType::NONE, 0,), // 0x6F
    (Action::LD, AddressMode::MRR, RegisterType::HL, RegisterType::B, ConditionType::NONE, 0,), // 0x70
    (Action::LD, AddressMode::MRR, RegisterType::HL, RegisterType::C, ConditionType::NONE, 0,), // 0x71
    (Action::LD, AddressMode::MRR, RegisterType::HL, RegisterType::D, ConditionType::NONE, 0,), // 0x72
    (Action::LD, AddressMode::MRR, RegisterType::HL, RegisterType::E, ConditionType::NONE, 0,), // 0x73
    (Action::LD, AddressMode::MRR, RegisterType::HL, RegisterType::H, ConditionType::NONE, 0,), // 0x74
    (Action::LD, AddressMode::MRR, RegisterType::HL, RegisterType::L, ConditionType::NONE, 0,), // 0x75
    (Action::HALT, AddressMode::NONE, RegisterType::NONE, RegisterType::NONE, ConditionType::NONE, 0,), // 0x76
    (Action::LD, AddressMode::MRR, RegisterType::HL, RegisterType::A, ConditionType::NONE, 0,), // 0x77
    (Action::LD, AddressMode::RR, RegisterType::A, RegisterType::B, ConditionType::NONE, 0,), // 0x78
    (Action::LD, AddressMode::RR, RegisterType::A, RegisterType::C, ConditionType::NONE, 0,), // 0x79
    (Action::LD, AddressMode::RR, RegisterType::A, RegisterType::D, ConditionType::NONE, 0,), // 0x7A
    (Action::LD, AddressMode::RR, RegisterType::A, RegisterType::E, ConditionType::NONE, 0,), // 0x7B
    (Action::LD, AddressMode::RR, RegisterType::A, RegisterType::H, ConditionType::NONE, 0,), // 0x7C
    (Action::LD, AddressMode::RR, RegisterType::A, RegisterType::L, ConditionType::NONE, 0,), // 0x7D
    (Action::LD, AddressMode::RMR, RegisterType::A, RegisterType::HL, ConditionType::NONE, 0,), // 0x7E
    (Action::LD, AddressMode::RR, RegisterType::A, RegisterType::A, ConditionType::NONE, 0,), // 0x7F
    (Action::ADD, AddressMode::RR, RegisterType::A, RegisterType::B, ConditionType::NONE, 0,), // 0x80
    (Action::ADD, AddressMode::RR, RegisterType::A, RegisterType::C, ConditionType::NONE, 0,), // 0x81
    (Action::ADD, AddressMode::RR, RegisterType::A, RegisterType::D, ConditionType::NONE, 0,), // 0x82
    (Action::ADD, AddressMode::RR, RegisterType::A, RegisterType::E, ConditionType::NONE, 0,), // 0x83
    (Action::ADD, AddressMode::RR, RegisterType::A, RegisterType::H, ConditionType::NONE, 0,), // 0x84
    (Action::ADD, AddressMode::RR, RegisterType::A, RegisterType::L, ConditionType::NONE, 0,), // 0x85
    (Action::ADD, AddressMode::RMR, RegisterType::A, RegisterType::HL, ConditionType::NONE, 0,), // 0x86
    (Action::ADD, AddressMode::RR, RegisterType::A, RegisterType::A, ConditionType::NONE, 0,), // 0x87
    (Action::ADC, AddressMode::RR, RegisterType::A, RegisterType::B, ConditionType::NONE, 0,), // 0x88
    (Action::ADC, AddressMode::RR, RegisterType::A, RegisterType::C, ConditionType::NONE, 0,), // 0x89
    (Action::ADC, AddressMode::RR, RegisterType::A, RegisterType::D, ConditionType::NONE, 0,), // 0x8A
    (Action::ADC, AddressMode::RR, RegisterType::A, RegisterType::E, ConditionType::NONE, 0,), // 0x8B
    (Action::ADC, AddressMode::RR, RegisterType::A, RegisterType::H, ConditionType::NONE, 0,), // 0x8C
    (Action::ADC, AddressMode::RR, RegisterType::A, RegisterType::L, ConditionType::NONE, 0,), // 0x8D
    (Action::ADC, AddressMode::RMR, RegisterType::A, RegisterType::HL, ConditionType::NONE, 0,), // 0x8E
    (Action::ADC, AddressMode::RR, RegisterType::A, RegisterType::A, ConditionType::NONE, 0,), // 0x8F
    (Action::SUB, AddressMode::RR, RegisterType::A, RegisterType::B, ConditionType::NONE, 0,), // 0x90
    (Action::SUB, AddressMode::RR, RegisterType::A, RegisterType::C, ConditionType::NONE, 0,), // 0x91
    (Action::SUB, AddressMode::RR, RegisterType::A, RegisterType::D, ConditionType::NONE, 0,), // 0x92
    (Action::SUB, AddressMode::RR, RegisterType::A, RegisterType::E, ConditionType::NONE, 0,), // 0x93
    (Action::SUB, AddressMode::RR, RegisterType::A, RegisterType::H, ConditionType::NONE, 0,), // 0x94
    (Action::SUB, AddressMode::RR, RegisterType::A, RegisterType::L, ConditionType::NONE, 0,), // 0x95
    (Action::SUB, AddressMode::RMR, RegisterType::A, RegisterType::HL, ConditionType::NONE, 0,), // 0x96
    (Action::SUB, AddressMode::RR, RegisterType::A, RegisterType::A, ConditionType::NONE, 0,), // 0x97
    (Action::SBC, AddressMode::RR, RegisterType::A, RegisterType::B, ConditionType::NONE, 0,), // 0x98
    (Action::SBC, AddressMode::RR, RegisterType::A, RegisterType::C, ConditionType::NONE, 0,), // 0x99
    (Action::SBC, AddressMode::RR, RegisterType::A, RegisterType::D, ConditionType::NONE, 0,), // 0x9A
    (Action::SBC, AddressMode::RR, RegisterType::A, RegisterType::E, ConditionType::NONE, 0,), // 0x9B
    (Action::SBC, AddressMode::RR, RegisterType::A, RegisterType::H, ConditionType::NONE, 0,), // 0x9C
    (Action::SBC, AddressMode::RR, RegisterType::A, RegisterType::L, ConditionType::NONE, 0,), // 0x9D
    (Action::SBC, AddressMode::RMR, RegisterType::A, RegisterType::HL, ConditionType::NONE, 0,), // 0x9E
    (Action::SBC, AddressMode::RR, RegisterType::A, RegisterType::A, ConditionType::NONE, 0,), // 0x9F
    (Action::AND, AddressMode::RR, RegisterType::A, RegisterType::B, ConditionType::NONE, 0,), // 0xA0
    (Action::AND, AddressMode::RR, RegisterType::A, RegisterType::C, ConditionType::NONE, 0,), // 0xA1
    (Action::AND, AddressMode::RR, RegisterType::A, RegisterType::D, ConditionType::NONE, 0,), // 0xA2
    (Action::AND, AddressMode::RR, RegisterType::A, RegisterType::E, ConditionType::NONE, 0,), // 0xA3
    (Action::AND, AddressMode::RR, RegisterType::A, RegisterType::H, ConditionType::NONE, 0,), // 0xA4
    (Action::AND, AddressMode::RR, RegisterType::A, RegisterType::L, ConditionType::NONE, 0,), // 0xA5
    (Action::AND, AddressMode::RMR, RegisterType::A, RegisterType::HL, ConditionType::NONE, 0,), // 0xA6
    (Action::AND, AddressMode::RR, RegisterType::A, RegisterType::A, ConditionType::NONE, 0,), // 0xA7
    (Action::XOR, AddressMode::RR, RegisterType::A, RegisterType::B, ConditionType::NONE, 0,), // 0xA8
    (Action::XOR, AddressMode::RR, RegisterType::A, RegisterType::C, ConditionType::NONE, 0,), // 0xA9
    (Action::XOR, AddressMode::RR, RegisterType::A, RegisterType::D, ConditionType::NONE, 0,), // 0xAA
    (Action::XOR, AddressMode::RR, RegisterType::A, RegisterType::E, ConditionType::NONE, 0,), // 0xAB
    (Action::XOR, AddressMode::RR, RegisterType::A, RegisterType::H, ConditionType::NONE, 0,), // 0xAC
    (Action::XOR, AddressMode::RR, RegisterType::A, RegisterType::L, ConditionType::NONE, 0,), // 0xAD
    (Action::XOR, AddressMode::RMR, RegisterType::A, RegisterType::HL, ConditionType::NONE, 0,), // 0xAE
    (Action::XOR, AddressMode::RR, RegisterType::A, RegisterType::A, ConditionType::NONE, 0,), // 0xAF
    (Action::OR, AddressMode::RR, RegisterType::A, RegisterType::B, ConditionType::NONE, 0,), // 0xB0
    (Action::OR, AddressMode::RR, RegisterType::A, RegisterType::C, ConditionType::NONE, 0,), // 0xB1
    (Action::OR, AddressMode::RR, RegisterType::A, RegisterType::D, ConditionType::NONE, 0,), // 0xB2
    (Action::OR, AddressMode::RR, RegisterType::A, RegisterType::E, ConditionType::NONE, 0,), // 0xB3
    (Action::OR, AddressMode::RR, RegisterType::A, RegisterType::H, ConditionType::NONE, 0,), // 0xB4
    (Action::OR, AddressMode::RR, RegisterType::A, RegisterType::L, ConditionType::NONE, 0,), // 0xB5
    (Action::OR, AddressMode::RMR, RegisterType::A, RegisterType::HL, ConditionType::NONE, 0,), // 0xB6
    (Action::OR, AddressMode::RR, RegisterType::A, RegisterType::A, ConditionType::NONE, 0,), // 0xB7
    (Action::CP, AddressMode::RR, RegisterType::A, RegisterType::B, ConditionType::NONE, 0,), // 0xB8
    (Action::CP, AddressMode::RR, RegisterType::A, RegisterType::C, ConditionType::NONE, 0,), // 0xB9
    (Action::CP, AddressMode::RR, RegisterType::A, RegisterType::D, ConditionType::NONE, 0,), // 0xBA
    (Action::CP, AddressMode::RR, RegisterType::A, RegisterType::E, ConditionType::NONE, 0,), // 0xBB
    (Action::CP, AddressMode::RR, RegisterType::A, RegisterType::H, ConditionType::NONE, 0,), // 0xBC
    (Action::CP, AddressMode::RR, RegisterType::A, RegisterType::L, ConditionType::NONE, 0,), // 0xBD
    (Action::CP, AddressMode::RMR, RegisterType::A, RegisterType::HL, ConditionType::NONE, 0,), // 0xBE
    (Action::CP, AddressMode::RR, RegisterType::A, RegisterType::A, ConditionType::NONE, 0,), // 0xBF
    (Action::RET, AddressMode::IMP, RegisterType::NONE, RegisterType::NONE, ConditionType::NZ, 0,), // 0xC0
    (Action::POP, AddressMode::R, RegisterType::C, RegisterType::NONE, ConditionType::NONE, 0,), // 0xC1
    (Action::JUMP, AddressMode::D16, RegisterType::NONE, RegisterType::NONE, ConditionType::NZ, 0,), // 0xC2
    (Action::JUMP, AddressMode::D16, RegisterType::NONE, RegisterType::NONE, ConditionType::NONE, 0,), // 0xC3
    (Action::CALL, AddressMode::D16, RegisterType::NONE, RegisterType::NONE, ConditionType::NZ, 0,), // 0xC4
    (Action::PUSH, AddressMode::R, RegisterType::BC, RegisterType::NONE, ConditionType::NONE, 0,), // 0xC5
    (Action::ADD, AddressMode::RA8, RegisterType::A, RegisterType::NONE, ConditionType::NONE, 0,), // 0xC6
    (Action::RST, AddressMode::IMP, RegisterType::NONE, RegisterType::NONE, ConditionType::NONE, 0,), // 0xC7
    (Action::RET, AddressMode::IMP, RegisterType::NONE, RegisterType::NONE, ConditionType::Z, 0,), // 0xC8
    (Action::RET, AddressMode::NONE, RegisterType::NONE, RegisterType::NONE, ConditionType::NONE, 0,), // 0xC9
    (Action::JUMP, AddressMode::D16, RegisterType::NONE, RegisterType::NONE, ConditionType::Z, 0,), // 0xCA
    (Action::CB, AddressMode::D8, RegisterType::NONE, RegisterType::NONE, ConditionType::NONE, 0,), // 0xCB
    (Action::CALL, AddressMode::D16, RegisterType::NONE, RegisterType::NONE, ConditionType::Z, 0,), // 0xCC
    (Action::CALL, AddressMode::D16, RegisterType::NONE, RegisterType::NONE, ConditionType::NONE, 0,), // 0xCD
    (Action::ADC, AddressMode::RD8, RegisterType::A, RegisterType::NONE, ConditionType::NONE, 0,), // 0xCE
    (Action::RST, AddressMode::IMP, RegisterType::NONE, RegisterType::NONE, ConditionType::NONE, 0x08), // 0xCF
    (Action::RET, AddressMode::IMP, RegisterType::NONE, RegisterType::NONE, ConditionType::NC, 0,), // 0xD0
    (Action::POP, AddressMode::R, RegisterType::DE, RegisterType::NONE, ConditionType::NONE, 0,), // 0xD1
    (Action::JUMP, AddressMode::D16, RegisterType::NONE, RegisterType::NONE, ConditionType::NC, 0,), // 0xD2
    (Action::NONE, AddressMode::NONE, RegisterType::NONE, RegisterType::NONE, ConditionType::NONE, 0,),
    (Action::CALL, AddressMode::D16, RegisterType::NONE, RegisterType::NONE, ConditionType::NC, 0,), // 0xD4
    (Action::PUSH, AddressMode::R, RegisterType::DE, RegisterType::NONE, ConditionType::NONE, 0,), // 0xD5
    (Action::SUB, AddressMode::D8, RegisterType::NONE, RegisterType::NONE, ConditionType::NONE, 0,), // 0xD6
    (Action::RST, AddressMode::IMP, RegisterType::NONE, RegisterType::NONE, ConditionType::NONE, 0x10,), // 0xD7
    (Action::RET, AddressMode::IMP, RegisterType::NONE, RegisterType::NONE, ConditionType::C, 0,), // 0xD8
    (Action::RETI, AddressMode::NONE, RegisterType::NONE, RegisterType::NONE, ConditionType::NONE, 0,), // 0xD9
    (Action::JUMP, AddressMode::D16, RegisterType::NONE, RegisterType::NONE, ConditionType::C, 0,), // 0xDA
    (Action::NONE, AddressMode::NONE, RegisterType::NONE, RegisterType::NONE, ConditionType::NONE, 0,),
    (Action::CALL, AddressMode::D16, RegisterType::NONE, RegisterType::NONE, ConditionType::C, 0,), // 0xDC
    (Action::NONE, AddressMode::NONE, RegisterType::NONE, RegisterType::NONE, ConditionType::NONE, 0,),
    (Action::SBC, AddressMode::RD8, RegisterType::A, RegisterType::NONE, ConditionType::NONE, 0,), // 0xDE
    (Action::RST, AddressMode::IMP, RegisterType::NONE, RegisterType::NONE, ConditionType::NONE, 0x18), // 0xDF
    (Action::LDH, AddressMode::A8R, RegisterType::NONE, RegisterType::A, ConditionType::NONE, 0,), // 0xE0
    (Action::POP, AddressMode::R, RegisterType::HL, RegisterType::NONE, ConditionType::NONE, 0,), // 0xE1
    (Action::LD, AddressMode::MRR, RegisterType::C, RegisterType::A, ConditionType::NONE, 0,), // 0xE2
    (Action::NONE, AddressMode::NONE, RegisterType::NONE, RegisterType::NONE, ConditionType::NONE, 0,),
    (Action::NONE, AddressMode::NONE, RegisterType::NONE, RegisterType::NONE, ConditionType::NONE, 0,),
    (Action::PUSH, AddressMode::R, RegisterType::HL, RegisterType::NONE, ConditionType::NONE, 0,), // 0xE5
    (Action::AND, AddressMode::D8, RegisterType::NONE, RegisterType::NONE, ConditionType::NONE, 0,), // 0xE6
    (Action::RST, AddressMode::IMP, RegisterType::NONE, RegisterType::NONE, ConditionType::NONE, 0x20,), // 0xE7
    (Action::ADD, AddressMode::RD8, RegisterType::SP, RegisterType::NONE, ConditionType::NONE, 0,), // 0xE8
    (Action::JUMP, AddressMode::MR, RegisterType::HL, RegisterType::NONE, ConditionType::NONE, 0,), // 0xE9
    (Action::LD, AddressMode::A16R, RegisterType::NONE, RegisterType::A, ConditionType::NONE, 0,), // 0xEA
    (Action::NONE, AddressMode::NONE, RegisterType::NONE, RegisterType::NONE, ConditionType::NONE, 0,),
    (Action::NONE, AddressMode::NONE, RegisterType::NONE, RegisterType::NONE, ConditionType::NONE, 0,),
    (Action::NONE, AddressMode::NONE, RegisterType::NONE, RegisterType::NONE, ConditionType::NONE, 0,),
    (Action::XOR, AddressMode::D8, RegisterType::NONE, RegisterType::NONE, ConditionType::NONE, 0,), // 0xEE
    (Action::RST, AddressMode::IMP, RegisterType::NONE, RegisterType::NONE, ConditionType::NONE, 0x28), // 0xEF
    (Action::LDH, AddressMode::RA8, RegisterType::A, RegisterType::NONE, ConditionType::NONE, 0,), // 0xF0
    (Action::POP, AddressMode::R, RegisterType::AF, RegisterType::NONE, ConditionType::NONE, 0,), // 0xF1
    (Action::LD, AddressMode::RMR, RegisterType::A, RegisterType::C, ConditionType::NONE, 0,), // 0xF2
    (Action::DI, AddressMode::NONE, RegisterType::NONE, RegisterType::NONE, ConditionType::NONE, 0,), // 0xF3
    (Action::NONE, AddressMode::NONE, RegisterType::NONE, RegisterType::NONE, ConditionType::NONE, 0,),
    (Action::PUSH, AddressMode::R, RegisterType::AF, RegisterType::NONE, ConditionType::NONE, 0,), // 0xF5
    (Action::OR, AddressMode::D8, RegisterType::NONE, RegisterType::NONE, ConditionType::NONE, 0,), // 0xF6
    (Action::RST, AddressMode::IMP, RegisterType::NONE, RegisterType::NONE, ConditionType::NONE, 0x30,), // 0xF7
    (Action::LD, AddressMode::HLSPR, RegisterType::HL, RegisterType::SP, ConditionType::NONE, 0,), // 0xF8
    (Action::LD, AddressMode::RR, RegisterType::SP, RegisterType::HL, ConditionType::NONE, 0,), // 0xF9
    (Action::LD, AddressMode::RA16, RegisterType::A, RegisterType::NONE, ConditionType::NONE, 0,), // 0xFA
    (Action::EI, AddressMode::NONE, RegisterType::NONE, RegisterType::NONE, ConditionType::NONE, 0,), // 0xFB
    (Action::NONE, AddressMode::NONE, RegisterType::NONE, RegisterType::NONE, ConditionType::NONE, 0,),
    (Action::NONE, AddressMode::NONE, RegisterType::NONE, RegisterType::NONE, ConditionType::NONE, 0,),
    (Action::CP, AddressMode::D8, RegisterType::NONE, RegisterType::NONE, ConditionType::NONE, 0,), // 0xFE
    (Action::RST, AddressMode::IMP, RegisterType::NONE, RegisterType::NONE, ConditionType::NONE, 0x38), // 0xFF
];

#[cfg(test)]
mod tests {
    use crate::cpu::actions::*;
    use crate::cpu::addresses::AddressMode;
    use crate::cpu::conditions::ConditionType;
    use crate::cpu::instructions::Instruction;
    use crate::cpu::register::RegisterType;

    #[test]
    fn test_instruction_from_op_code() {
        let inst = Instruction::from(0);
        match inst.action {
            Action::NOP => {}
            _ => panic!("Not correct instruction type"),
        }

        match &inst.address {
            AddressMode::IMP => {}
            _ => panic!("Not correct instruction type"),
        }
        match inst.register_1 {
            RegisterType::NONE => {}
            _ => panic!("Not correct reg type"),
        }
        match inst.register_2 {
            RegisterType::NONE => {}
            _ => panic!("Not correct reg type"),
        }
        match inst.condition {
            ConditionType::NONE => {}
            _ => panic!("Not correct reg type"),
        }
        assert_eq!(*inst.param, 0,);
    }

    #[test]
    fn test_0e() {
        let inst = Instruction::from(0x0E);
        match inst.action {
            Action::LD => {}
            _ => panic!("Not correct instruction type"),
        }

        match inst.address {
            AddressMode::RD8 => {}
            _ => panic!("Not correct instruction type"),
        }
        match inst.register_1 {
            RegisterType::C => {}
            _ => panic!("Not correct reg type"),
        }

        match inst.register_2 {
            RegisterType::NONE => {}
            _ => panic!("Not correct reg type"),
        }

        match inst.condition {
            ConditionType::NONE => {}
            _ => panic!("Not correct reg type"),
        }

        assert_eq!(*inst.param, 0,);
    }
}
