use crate::cpu::instructions::Instruction;
use crate::cpu::register::RegisterType;
use crate::cpu::CPU;
use crate::tpu::Timer;

#[derive(PartialEq)]
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

impl AddressMode {
    pub fn fetch(&self, cpu: &mut CPU, instruction: &Instruction) {
        cpu.mem_dest = 0;
        cpu.dest_is_mem = false;

        match instruction.address {
            AddressMode::NONE => {}
            AddressMode::IMP => {}
            AddressMode::RD16 | AddressMode::D16 => {
                let lo = cpu.bus.read(cpu.register.pc);
                Timer::get().emu_cycles(1, cpu);
                let hi = cpu.bus.read(cpu.register.pc + 1);
                Timer::get().emu_cycles(1, cpu);
                cpu.register.pc += 2;

                cpu.fetch_data = lo | (hi << 8);
            }
            AddressMode::RR => cpu.fetch_data = cpu.read_register(instruction.register_2),
            AddressMode::MRR => {
                cpu.fetch_data = cpu.read_register(instruction.register_2);
                cpu.mem_dest = cpu.read_register(instruction.register_1);
                cpu.dest_is_mem = true;

                match instruction.register_1 {
                    RegisterType::C => cpu.mem_dest |= 0xFF00,
                    _ => {}
                }
            }
            AddressMode::R => cpu.fetch_data = cpu.read_register(instruction.register_1),
            AddressMode::RD8 => {
                cpu.fetch_data = cpu.bus.read(cpu.register.pc);
                Timer::get().emu_cycles(1, cpu);
                cpu.register.pc += 1;
            }
            AddressMode::RMR => {
                let mut address = cpu.read_register(instruction.register_2);
                match instruction.register_1 {
                    RegisterType::C => address |= 0xFF00,
                    _ => {}
                }
                cpu.fetch_data = cpu.bus.read(address);
                Timer::get().emu_cycles(1, cpu);
            }
            AddressMode::RHLI => {
                cpu.fetch_data = cpu.bus.read(cpu.read_register(instruction.register_2));
                Timer::get().emu_cycles(1, cpu);
                cpu.set_register(&RegisterType::HL, cpu.read_register(&RegisterType::HL) + 1)
            }
            AddressMode::RHLD => {
                cpu.fetch_data = cpu.bus.read(cpu.read_register(instruction.register_2));
                Timer::get().emu_cycles(1, cpu);
                cpu.set_register(&RegisterType::HL, cpu.read_register(&RegisterType::HL) - 1)
            }
            AddressMode::HLIR => {
                cpu.fetch_data = cpu.read_register(instruction.register_2);
                cpu.mem_dest = cpu.read_register(instruction.register_1);
                cpu.dest_is_mem = true;
                cpu.set_register(&RegisterType::HL, cpu.read_register(&RegisterType::HL) + 1);
            }
            AddressMode::HLDR => {
                cpu.fetch_data = cpu.read_register(instruction.register_2);
                cpu.mem_dest = cpu.read_register(instruction.register_1);
                cpu.dest_is_mem = true;
                cpu.set_register(&RegisterType::HL, cpu.read_register(&RegisterType::HL) - 1);
            }
            AddressMode::RA8 => {
                cpu.fetch_data = cpu.bus.read(cpu.register.pc);
                Timer::get().emu_cycles(1, cpu);
                cpu.register.pc += 1;
            }
            AddressMode::A8R => {
                cpu.mem_dest = cpu.bus.read(cpu.register.pc) | 0xFF00;
                cpu.dest_is_mem = true;
                Timer::get().emu_cycles(1, cpu);
                cpu.register.pc += 1;
            }
            AddressMode::HLSPR => {
                cpu.fetch_data = cpu.bus.read(cpu.register.pc);
                Timer::get().emu_cycles(1, cpu);
                cpu.register.pc += 1;
            }
            AddressMode::D8 => {
                cpu.fetch_data = cpu.bus.read(cpu.register.pc);
                Timer::get().emu_cycles(1, cpu);
                cpu.register.pc += 1;
            }
            AddressMode::D16R | AddressMode::A16R => {
                let lo = cpu.bus.read(cpu.register.pc);
                Timer::get().emu_cycles(1, cpu);

                let hi = cpu.bus.read(cpu.register.pc + 1);
                Timer::get().emu_cycles(1, cpu);

                cpu.mem_dest = lo | (hi << 8);
                cpu.dest_is_mem = true;

                cpu.register.pc += 2;
                cpu.fetch_data = cpu.read_register(instruction.register_2);
            }
            AddressMode::MRD8 => {
                cpu.fetch_data = cpu.bus.read(cpu.register.pc);
                Timer::get().emu_cycles(1, cpu);
                cpu.register.pc += 1;
                cpu.mem_dest = cpu.read_register(instruction.register_1);
                cpu.dest_is_mem = true;
            }
            AddressMode::MR => {
                cpu.mem_dest = cpu.read_register(instruction.register_1);
                cpu.dest_is_mem = true;
                cpu.fetch_data = cpu.bus.read(cpu.read_register(instruction.register_1));
                Timer::get().emu_cycles(1, cpu);
            }
            AddressMode::RA16 => {
                let lo = cpu.bus.read(cpu.register.pc);
                Timer::get().emu_cycles(1, cpu);

                let hi = cpu.bus.read(cpu.register.pc + 1);
                Timer::get().emu_cycles(1, cpu);

                let address = lo | (hi << 8);

                cpu.register.pc += 2;
                cpu.fetch_data = cpu.bus.read(address);
                Timer::get().emu_cycles(1, cpu);
            }
        }
    }
}
