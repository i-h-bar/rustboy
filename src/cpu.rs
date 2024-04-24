use crate::bus::Bus;
use crate::instruction::Instruction;

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
    pc: u16
}


pub struct CPU {
    register: Register,
    bus: Bus,
    fetch_data: u16,
    mem_dest: u16,
    current_op_code: u8,
    instruction: Instruction,
    halted: bool,
    stepping: bool
}

impl CPU {
    fn step(&self) {
        if !self.halted {

        }
    }

    fn fetch_instruction(&mut self) {
        self.current_op_code = self.bus.read(self.register.pc);
        self.register.pc += 1;
    }

    fn fetch_data(&self) {
        todo!()
    }

    fn execute(&self) {

    }
}
