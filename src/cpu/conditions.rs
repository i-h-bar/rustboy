use crate::cpu::CPU;

pub enum ConditionType {
    NONE,
    NZ,
    Z,
    NC,
    C,
}

impl ConditionType {
    pub fn check(&self, cpu: &mut CPU) -> bool {
        match self {
            ConditionType::NONE => true,
            ConditionType::NZ => !cpu.register.z_flag(),
            ConditionType::Z => cpu.register.z_flag(),
            ConditionType::NC => !cpu.register.c_flag(),
            ConditionType::C => cpu.register.c_flag(),
        }
    }
}
