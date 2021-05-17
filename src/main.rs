#![feature(flt2dec)]

use serde::{Serialize, Deserialize};

// types
type Register = u64;
type StackElem = u64;

// constants
const NUM_REGISTERS: usize = 64; // max number of registers
const RESERVED_REGISTERS: usize = 5; // max number

const IP_REGISTER: usize = 0; // instruction pointer
const SP_REGISTER: usize = 1; // stack pointer

const MAX_STACKSIZE: usize = 200; // max stack size

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum Instruction {
    // Move src -> dst
    MOV(Register, Register),
    // Move imm -> dst
    MOVI(u64, Register),
    // Push reg -> stack[sp]
    PUSH(Register),
    // Pop stack[sp] -> reg
    POP(Register),
    JMP(u64),
    OUTPUT(u16),
    HALT()
}

pub struct VM {
    code: Vec<Instruction>,
    registers: Vec<Register>,
    stack: Vec<StackElem>,
}

impl VM {
    fn gpr(reg: usize) -> usize {
        RESERVED_REGISTERS + reg
    }

    fn ip(&self) -> Register {
        self.registers[IP_REGISTER]
    }
  
    fn incr_ip(&mut self) -> Register {
        self.registers[IP_REGISTER] += 1;
        self.ip()
    }

    fn incr_reg(&mut self, reg: Register) -> Register {
        self.registers[reg as usize] += 1;
        self.registers[reg as usize]
    }

    fn decr_reg(&mut self, reg: Register) -> Register {
        self.registers[reg as usize] -= 1;
        self.registers[reg as usize]
    }

    fn step(&mut self) -> () {
        loop {
            if self.ip() >= self.code.len() as u64 {
                panic!("instruction pointer exceeded length of code (ip: {})", self.ip())
            }

            let current_instruction: &Instruction = &self.code[self.ip() as usize];

            match current_instruction {
                Instruction::MOV(src, dst) => {
                    self.registers[VM::gpr(*dst as usize)] = self.registers[VM::gpr(*src as usize)];
                }
                Instruction::MOVI(src, imm) => {
                    self.registers[VM::gpr(*src as usize)] = *imm;
                }
                Instruction::PUSH(register) => {
                    self.stack[self.registers[SP_REGISTER] as usize] = self.registers[*register as usize];
                    self.incr_reg(SP_REGISTER as u64);
                }
                Instruction::POP(register) => {
                    //self.decr_reg(SP_REGISTER as u64);
                    //self.registers[*register as usize] = self.stack[self.registers[SP_REGISTER] as usize];
                }
                Instruction::OUTPUT(reg) => {
                    let general_purpose_register = VM::gpr(*reg as usize);
                    println!("Register {}: {}", general_purpose_register, self.registers[general_purpose_register])
                }
                Instruction::JMP(addr) => {
                    self.registers[IP_REGISTER] = *addr;
                    continue;
                }
                Instruction::HALT() => {
                    return
                }
            }

            self.incr_ip();
        }
    }

    pub fn new(instrs: Vec<Instruction>) -> VM {
        let mut vm = VM {
            code: instrs,
            registers: Vec::with_capacity(RESERVED_REGISTERS),
            stack: Vec::with_capacity(MAX_STACKSIZE),
        };

        vm.registers.resize(NUM_REGISTERS, 0);
        vm.stack.resize(MAX_STACKSIZE, 0);

        vm
    }

    pub fn execute(&mut self) -> () {
        self.step();
    }
}

fn main() {
    let mut instrs: Vec<Instruction> = Vec::new();
    instrs.push(Instruction::OUTPUT(0));
    instrs.push(Instruction::OUTPUT(1));
    instrs.push(Instruction::MOVI(0, 100));
    instrs.push(Instruction::PUSH(0));
    //instrs.push(Instruction::JMP(3));
    instrs.push(Instruction::OUTPUT(0));
    instrs.push(Instruction::OUTPUT(1));
    instrs.push(Instruction::MOV(0, 1));
    instrs.push(Instruction::OUTPUT(0));
    instrs.push(Instruction::OUTPUT(1));
    instrs.push(Instruction::HALT());

    let encoded: Vec<u8> = bincode::serialize(&instrs).unwrap();
    println!("{} {:?}", encoded.len(), encoded);

    let decoded: Vec<Instruction> = bincode::deserialize(&encoded[..]).unwrap();
    println!("{:?}", decoded[2]);

    let mut vm: VM = VM::new(decoded);
    vm.execute();

    println!("Hello, world!");
}
