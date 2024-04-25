use crate::bus::Bus;
use crate::cartridge::Cartridge;
use crate::emu::EMU;
use crate::instruction::{AddressMode, Instruction, RegisterType};

pub struct Register {
    a: u8,
    f: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
}

pub struct CPU {
    register: Register,
    bus: Bus,
    fetch_data: u16,
    mem_dest: u16,
    dest_is_mem: bool,
    current_op_code: u8,
    instruction: Instruction,
    halted: bool,
    stepping: bool,
}

impl CPU {
    pub fn from(cartridge: Cartridge) -> Self {
        Self {
            register: Register { a: 0x1, f: 0, b: 0, c: 0, d: 0, e: 0, h: 0, l: 0, sp: 0, pc: 0x100 },
            bus: Bus { cartridge },
            fetch_data: 0,
            mem_dest: 0,
            dest_is_mem: false,
            current_op_code: 0,
            instruction: Instruction::from(&0).unwrap(),
            halted: false,
            stepping: false
        }
    }

    pub fn step(&mut self) {
        if !self.halted {
            let pc = self.register.pc;

            self.fetch_instruction();
            self.fetch_data();
            println!("Executing {:#02x}, PC = {:#04x}", self.current_op_code, pc);
            self.execute();
        }
    }

    fn fetch_instruction(&mut self) {
        self.current_op_code = self.bus.read(self.register.pc) as u8;
        self.register.pc += 1;
        self.instruction = Instruction::from(&self.current_op_code)
            .expect(&format!("Unknown Instruction: {:#02x}", self.current_op_code));
    }

    fn fetch_data(&mut self) {
        self.mem_dest = 0;
        self.dest_is_mem = false;

        match &self.instruction.address_mode {
            None => { return; }
            Some(address_mode) => {
                match address_mode {
                    AddressMode::IMP => { return; }
                    AddressMode::RD16 => {}
                    AddressMode::RR => {}
                    AddressMode::MRR => {}
                    AddressMode::R => {
                        self.fetch_data = self.read_register(&self.instruction.register_1)
                    }
                    AddressMode::RD8 => {
                        self.fetch_data = self.bus.read(self.register.pc);
                        EMU::cycles(1);
                        self.register.pc += 1;
                    }
                    AddressMode::RMR => {}
                    AddressMode::RHLI => {}
                    AddressMode::RHLD => {}
                    AddressMode::HLIR => {}
                    AddressMode::HLDR => {}
                    AddressMode::RA8 => {}
                    AddressMode::A8R => {}
                    AddressMode::HLSPR => {}
                    AddressMode::D16 => {
                        let lo = self.bus.read(self.register.pc);
                        EMU::cycles(1);
                        let hi = self.bus.read(self.register.pc + 1);
                        EMU::cycles(1);
                        self.fetch_data = lo | (hi << 8);
                        self.register.pc += 2;
                    }
                    AddressMode::D8 => {}
                    AddressMode::D16R => {}
                    AddressMode::MRD8 => {}
                    AddressMode::MR => {}
                    AddressMode::A16R => {}
                    AddressMode::RA16 => {}
                    _ => {
                        panic!("Address mode unknown");
                    }
                }
            }
        }
    }

    fn execute(&self) {

    }

    fn read_register(&self, register: &Option<RegisterType>) -> u16 {
        match register {
            None => {
                panic!("Unknown Register Type");
            }
            Some(rt) => {
                match rt {
                    RegisterType::NONE => 0,
                    RegisterType::A => self.register.a as u16,
                    RegisterType::F => self.register.f as u16,
                    RegisterType::B => self.register.b as u16,
                    RegisterType::C => self.register.c as u16,
                    RegisterType::D => self.register.d as u16,
                    RegisterType::E => self.register.e as u16,
                    RegisterType::H => self.register.h as u16,
                    RegisterType::L => self.register.l as u16,
                    RegisterType::AF => reverse(self.register.a),
                    RegisterType::BC => reverse(self.register.b),
                    RegisterType::DE => reverse(self.register.d),
                    RegisterType::HL => reverse(self.register.h),
                    RegisterType::SP => self.register.sp,
                    RegisterType::PC => self.register.pc
                }
            }
        }
    }
}


fn reverse(num: u8) -> u16 {
    let num = num as u16;

    ((num & 0xFF00) >> 8) | ((num & 0x00FF) << 8)
}
