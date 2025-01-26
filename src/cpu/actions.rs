use crate::cpu::addresses::AddressMode;
use crate::cpu::instructions::Instruction;
use crate::cpu::registers::RegisterType;
use crate::cpu::{registers, CPU};
use crate::emu::EMU;
use std::fmt;
use std::fmt::Debug;

#[derive(Debug)]
pub enum Action {
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

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Action {
    pub fn execute(&self, cpu: &mut CPU, instruction: &Instruction) {
        match self {
            Action::NONE => {}
            Action::NOP => {}
            Action::LD => {
                if cpu.dest_is_mem {
                    if cpu.register.is_16bit(instruction.register_2) {
                        EMU::cycles(1);
                        cpu.bus.write16(cpu.mem_dest, cpu.fetch_data);
                    } else {
                        cpu.bus.write(cpu.mem_dest, cpu.fetch_data as u8);
                    }
                } else {
                    if *instruction.address == AddressMode::HLSPR {
                        cpu.register.set_h(
                            ((cpu.read_register(instruction.register_2) as u8)
                                & 0x0F + (cpu.fetch_data as u8 & 0x0F))
                                >= 0x10,
                        );
                        cpu.register.set_c(
                            ((cpu.read_register(instruction.register_2))
                                & 0xFF00 + (cpu.fetch_data & 0xFF00))
                                >= 0x100,
                        );
                        cpu.register.set_z(false);
                        cpu.register.set_n(false);

                        cpu.set_register(
                            instruction.register_1,
                            (cpu.read_register(instruction.register_2) as i8
                                + cpu.fetch_data as i8) as u16,
                        );
                    } else {
                        cpu.set_register(instruction.register_1, cpu.fetch_data)
                    }
                }
            }
            Action::INC => {
                if cpu.register.is_16bit(instruction.register_1) {
                    EMU::cycles(1);
                }

                if *instruction.register_1 == RegisterType::HL
                    && instruction.address == &AddressMode::MR
                {
                    let val =
                        (cpu.bus.read(cpu.read_register(instruction.register_1))).wrapping_add(1);
                    let val = val & 0xFF;

                    cpu.bus
                        .write(cpu.read_register(instruction.register_1), val as u8)
                } else {
                    let val = cpu.read_register(instruction.register_1).wrapping_add(1);
                    cpu.set_register(instruction.register_1, val)
                }

                if (cpu.current_op_code & 0x03) != 0x03 {
                    let val = cpu.read_register(instruction.register_1);
                    cpu.register.set_z(val == 0);
                    cpu.register.set_n(false);
                    cpu.register.set_h((val & 0x0F) == 0);
                }
            }
            Action::DEC => {
                if cpu.register.is_16bit(instruction.register_1) {
                    EMU::cycles(1);
                }

                if *instruction.register_1 == RegisterType::HL
                    && *instruction.address == AddressMode::MR
                {
                    let val = (cpu.bus.read(cpu.read_register(instruction.register_1)) as u8)
                        .wrapping_sub(1);

                    cpu.bus
                        .write(cpu.read_register(instruction.register_1), val)
                } else {
                    let val = (cpu.read_register(instruction.register_1) as u8).wrapping_sub(1);

                    cpu.set_register(instruction.register_1, val as u16)
                }

                if (cpu.current_op_code & 0x0B) != 0x0B {
                    let val = cpu.read_register(instruction.register_1);
                    cpu.register.set_z(val == 0);
                    cpu.register.set_n(true);
                    cpu.register.set_h((val & 0x0F) == 0x0F);
                }
            }
            Action::RLCA => {
                let u = cpu.register.a as u8;
                let c = (u >> 7) & 1;

                cpu.register.a = ((u << 1) | c) as u16;
                cpu.register.set_z(false);
                cpu.register.set_n(false);
                cpu.register.set_h(false);
                cpu.register.set_c(c != 0);
            }
            Action::ADD => {
                let val: u32;
                let is_16bit = cpu.register.is_16bit(instruction.register_1);

                if is_16bit {
                    EMU::cycles(1);
                }

                if *instruction.register_1 == RegisterType::SP {
                    val = (cpu.read_register(instruction.register_1) as i8)
                        .wrapping_add(cpu.fetch_data as i8) as u32;
                } else {
                    val = cpu.read_register(instruction.register_1) as u32 + cpu.fetch_data as u32;
                }

                if *instruction.register_1 == RegisterType::SP {
                    cpu.register.set_n(true);
                    cpu.register.set_z(false);
                    cpu.register.set_h(
                        (cpu.read_register(instruction.register_1) & 0xF)
                            .wrapping_add(cpu.fetch_data & 0xF)
                            >= 0x10,
                    );
                    cpu.register.set_c(
                        (cpu.read_register(instruction.register_1) & 0xFF)
                            .wrapping_add(cpu.fetch_data & 0xFF)
                            > 0x100,
                    );
                } else if is_16bit {
                    cpu.register.set_n(true);
                    cpu.register.set_h(
                        (cpu.read_register(instruction.register_1) & 0xFFF)
                            .wrapping_add(cpu.fetch_data & 0xFFF)
                            >= 0x1000,
                    );
                    let n = (cpu.read_register(instruction.register_1) as u32)
                        .wrapping_add(cpu.fetch_data as u32);
                    cpu.register.set_c(n >= 0x10000);
                } else {
                    cpu.register.set_n(true);
                    cpu.register.set_z((val & 0xFF) == 0);
                    cpu.register.set_h(
                        (cpu.read_register(instruction.register_1) & 0xF)
                            .wrapping_add(cpu.fetch_data & 0xF)
                            >= 0x10,
                    );
                    cpu.register.set_c(
                        (cpu.read_register(instruction.register_1) & 0xFF)
                            .wrapping_add(cpu.fetch_data & 0xFF)
                            >= 0x100,
                    );
                }

                cpu.set_register(instruction.register_1, (val & 0xFFFF) as u16);
            }
            Action::RRCA => {
                let b = (cpu.register.a & 1) as u8;
                cpu.register.a >>= 1;
                cpu.register.a |= (b << 7) as u16;
                cpu.register.set_z(false);
                cpu.register.set_n(false);
                cpu.register.set_h(false);
                cpu.register.set_c(b != 0);
            }
            Action::STOP => {
                panic!("Stop instruction called!")
            }
            Action::RLA => {
                let u = cpu.register.a as u8;
                let c = (u >> 7) & 1;

                cpu.register.a = ((u << 1) | if cpu.register.c_flag() { 1 } else { 0 }) as u16;
                cpu.register.set_z(false);
                cpu.register.set_n(false);
                cpu.register.set_h(false);
                cpu.register.set_c(c != 0);
            }
            Action::JR => {
                let rel = (cpu.fetch_data & 0xFF) as i8;
                let address = (cpu.register.pc as i16 + rel as i16) as u16;
                cpu.go_to(address, false, &instruction);
            }
            Action::RRA => {
                let new_c = (cpu.register.a & 1) as u8;

                cpu.register.a >>= 1;
                cpu.register.a |= (if cpu.register.c_flag() { 1 } else { 0 } << 7) as u16;

                cpu.register.set_z(false);
                cpu.register.set_n(false);
                cpu.register.set_h(false);
                cpu.register.set_c(new_c != 0);
            }
            Action::DAA => {
                let mut u: u16 = 0;
                let mut cf: u8 = 0;

                if cpu.register.h_flag() || (!cpu.register.n_flag() && cpu.register.a & 0xF > 9) {
                    u = 6;
                }

                if cpu.register.c_flag() || (!cpu.register.n_flag() && cpu.register.a > 0x99) {
                    u |= 0x60;
                    cf = 1;
                }

                if cpu.register.n_flag() {
                    cpu.register.a.wrapping_sub(u);
                } else {
                    cpu.register.a.wrapping_add(u);
                }
                cpu.register.set_z(cpu.register.a == 0);
                cpu.register.set_h(false);
                cpu.register.set_c(cf != 0);
            }
            Action::CPL => {
                cpu.register.a = !(cpu.register.a as u8) as u16;
                cpu.register.set_n(true);
                cpu.register.set_h(true);
            }
            Action::SCF => {
                cpu.register.set_n(false);
                cpu.register.set_h(false);
                cpu.register.set_c(true);
            }
            Action::CCF => {
                cpu.register.set_n(false);
                cpu.register.set_h(false);
                cpu.register.set_c(cpu.register.c_flag() ^ true);
            }
            Action::HALT => {
                cpu.halted = true;
            }
            Action::ADC => {
                let u = cpu.fetch_data;
                let a = cpu.register.a;
                let c = cpu.register.c_flag() as u16;

                cpu.register.a = (a.wrapping_add(u).wrapping_add(c)) & 0xFF;

                cpu.register.set_z(cpu.register.a == 0);
                cpu.register.set_n(true);
                cpu.register
                    .set_h((a & 0xF).wrapping_add(u & 0xF).wrapping_add(c) > 0xF);
                cpu.register.set_c(a.wrapping_add(u).wrapping_add(c) > 0xFF);
            }
            Action::SUB => {
                let val = cpu
                    .read_register(instruction.register_1)
                    .wrapping_sub(cpu.fetch_data);
                let z = val == 0;
                let h = ((cpu.read_register(instruction.register_1) & 0xF) as i32)
                    .wrapping_sub((cpu.fetch_data & 0xF) as i32)
                    < 0;
                let c = (cpu.read_register(instruction.register_1) as i32)
                    .wrapping_sub(cpu.fetch_data as i32)
                    < 0;

                cpu.set_register(instruction.register_1, val);
                cpu.register.set_z(z);
                cpu.register.set_h(h);
                cpu.register.set_c(c);
                cpu.register.set_n(true);
            }
            Action::SBC => {
                let val = cpu.fetch_data + cpu.register.c_flag() as u16;

                let z = (cpu.read_register(instruction.register_1).wrapping_sub(val)) == 0;
                let h = (((cpu.read_register(instruction.register_1) & 0xF) as i32)
                    .wrapping_sub((cpu.fetch_data & 0xF) as i32)
                    .wrapping_sub(cpu.register.c_flag() as i32))
                    < 0;
                let c = (cpu.read_register(instruction.register_1) as i32)
                    .wrapping_sub(cpu.fetch_data as i32)
                    .wrapping_sub(cpu.register.c_flag() as i32)
                    < 0;

                cpu.set_register(
                    instruction.register_1,
                    cpu.read_register(instruction.register_1).wrapping_sub(val),
                );
                cpu.register.set_z(z);
                cpu.register.set_h(h);
                cpu.register.set_c(c);
                cpu.register.set_n(true);
            }
            Action::AND => {
                cpu.register.a &= cpu.fetch_data;
                cpu.register.set_z(cpu.register.a == 0);
                cpu.register.set_n(false);
                cpu.register.set_h(true);
                cpu.register.set_c(false);
            }
            Action::XOR => {
                cpu.register.a ^= cpu.fetch_data & 0xFF;
                cpu.register.set_z(cpu.register.a == 0);
                cpu.register.set_n(false);
                cpu.register.set_h(false);
                cpu.register.set_c(false);
            }
            Action::OR => {
                cpu.register.a |= cpu.fetch_data & 0xFF;
                cpu.register.set_z(cpu.register.a == 0);
                cpu.register.set_n(false);
                cpu.register.set_h(false);
                cpu.register.set_c(false);
            }
            Action::CP => {
                let n = cpu.register.a as i32 - cpu.fetch_data as i32;
                cpu.register.set_z(n == 0);
                cpu.register.set_n(true);
                cpu.register
                    .set_h(((cpu.register.a as i32 & 0x0F) - (cpu.fetch_data as i32 & 0x0F)) < 0);
                cpu.register.set_c(n < 0);
            }
            Action::POP => {
                let lo = cpu.stack_pop();
                EMU::cycles(1);
                let hi = cpu.stack_pop();
                EMU::cycles(1);

                let num = (hi << 8) | lo;

                match instruction.register_1 {
                    RegisterType::AF => cpu.set_register(instruction.register_1, num & 0xFFF0),
                    _ => cpu.set_register(instruction.register_1, num),
                }
            }
            Action::JUMP => {
                cpu.go_to(cpu.fetch_data, false, &instruction);
            }
            Action::PUSH => {
                let hi = (cpu.read_register(instruction.register_1) >> 8) & 0xFF;
                EMU::cycles(1);
                cpu.stack_push(hi as u8);

                let lo = cpu.read_register(instruction.register_1) & 0xFF;
                EMU::cycles(1);
                cpu.stack_push(lo as u8);

                EMU::cycles(1);
            }
            Action::RET => cpu.return_from_procedure(&instruction),
            Action::CB => {
                let op = cpu.fetch_data as u8;
                let reg = registers::reg_lookup(op & 0b111);
                let bit = (op >> 3) & 0b111;
                let bit_op = (op >> 6) & 0b11;
                let mut reg_val = cpu.read_register8(&reg);

                EMU::cycles(1);

                if reg == &RegisterType::HL {
                    EMU::cycles(2);
                }

                match bit_op {
                    1 => {
                        cpu.register.set_z((reg_val & (1 << bit)) != 0);
                        cpu.register.set_n(false);
                        cpu.register.set_h(true);
                        return;
                    }
                    2 => {
                        reg_val &= !(1 << bit);
                        cpu.set_register8(&reg, reg_val);
                        return;
                    }
                    3 => {
                        reg_val |= 1 << bit;
                        cpu.set_register8(&reg, reg_val);
                        return;
                    }
                    _ => {}
                }

                match bit {
                    0 => {
                        let mut result = (reg_val << 1) & 0xFF;
                        let set_c: bool;

                        if (reg_val & (1 << 7)) != 0 {
                            result |= 1;
                            set_c = true;
                        } else {
                            set_c = false;
                        }

                        cpu.set_register8(&reg, result);
                        cpu.register.set_z(result == 0);
                        cpu.register.set_n(false);
                        cpu.register.set_h(false);
                        cpu.register.set_c(set_c);
                        return;
                    }

                    1 => {
                        let old = reg_val.clone();
                        reg_val >>= 1;
                        reg_val |= old << 7;

                        cpu.set_register8(&reg, reg_val);
                        cpu.register.set_z(reg_val == 0);
                        cpu.register.set_n(false);
                        cpu.register.set_h(false);
                        cpu.register.set_c((old & 1) != 0);
                        return;
                    }

                    2 => {
                        let old = reg_val.clone();
                        reg_val <<= 1;

                        let c_flag_num = if cpu.register.c_flag() { 1 } else { 0 };

                        reg_val |= c_flag_num;
                        cpu.set_register8(&reg, reg_val);
                        cpu.register.set_z(reg_val == 0);
                        cpu.register.set_n(false);
                        cpu.register.set_h(false);
                        cpu.register.set_c((old & 0x80) != 0);
                        return;
                    }

                    3 => {
                        let old = reg_val.clone();
                        reg_val >>= 1;
                        let c_flag_num = if cpu.register.c_flag() { 1 } else { 0 };

                        reg_val |= c_flag_num << 7;
                        cpu.set_register8(&reg, reg_val);
                        cpu.register.set_z(reg_val == 0);
                        cpu.register.set_n(false);
                        cpu.register.set_h(false);
                        cpu.register.set_c((old & 1) != 0);
                        return;
                    }

                    4 => {
                        let old = reg_val.clone();
                        reg_val <<= 1;
                        cpu.set_register8(&reg, reg_val);
                        cpu.register.set_z(reg_val == 0);
                        cpu.register.set_n(false);
                        cpu.register.set_h(false);
                        cpu.register.set_c((old & 0x80) != 0);
                        return;
                    }

                    5 => {
                        let u = reg_val >> 1;
                        cpu.set_register8(&reg, u);
                        cpu.register.set_z(u == 0);
                        cpu.register.set_n(false);
                        cpu.register.set_h(false);
                        cpu.register.set_c((reg_val & 1) != 0);
                        return;
                    }
                    6 => {
                        reg_val = ((reg_val & 0xF0) >> 4) | ((reg_val & 0xF) << 4);
                        cpu.set_register8(&reg, reg_val);
                        cpu.register.set_z(reg_val == 0);
                        cpu.register.set_n(false);
                        cpu.register.set_h(false);
                        cpu.register.set_c(false);
                        return;
                    }

                    7 => {
                        let u = reg_val >> 1;
                        cpu.set_register8(&reg, u);
                        cpu.register.set_z(u == 0);
                        cpu.register.set_n(false);
                        cpu.register.set_h(false);
                        cpu.register.set_c((reg_val & 1) != 0);
                        return;
                    }

                    _ => {
                        panic!("ERROR: INVALID CB: {:#04x}", op)
                    }
                }
            }
            Action::CALL => {
                cpu.go_to(cpu.fetch_data, true, &instruction);
            }
            Action::RETI => {
                cpu.master_enabled = true;
                cpu.return_from_procedure(&instruction)
            }
            Action::LDH => {
                match instruction.register_1 {
                    RegisterType::A => cpu.set_register(
                        instruction.register_1,
                        cpu.bus.read(0xFF00 | cpu.fetch_data),
                    ),
                    _ => cpu.bus.write(cpu.mem_dest, cpu.register.a as u8),
                }

                EMU::cycles(1)
            }
            Action::JPHL => {}
            Action::DI => {
                cpu.master_enabled = false;
            }
            Action::EI => {
                cpu.enabling_ime = true;
            }
            Action::RST => {
                cpu.go_to(*instruction.param, true, &instruction);
            }
            Action::ERR => {}
            Action::RLC => {}
            Action::RRC => {}
            Action::RL => {}
            Action::RR => {}
            Action::SLA => {}
            Action::SRA => {}
            Action::SWAP => {}
            Action::SRL => {}
            Action::BIT => {}
            Action::RES => {}
            Action::SET => {}
        }
    }
}
