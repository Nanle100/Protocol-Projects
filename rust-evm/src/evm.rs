use primitive_types::U256;

mod stack;
mod memory;
mod storage;
mod opcode;

use stack::Stack;
use memory::Memory;
use storage::Storage;
use opcode::{Opcode, decode_bytecode};

pub struct EVM {
    stack: Stack,
    memory: Memory,
    storage: Storage,
    pc: usize,
    gas: U256,
    bytecode: Vec<u8>,
}

impl EVM {
    pub fn new(bytecode: Vec<u8>) -> Self {
        EVM {
            stack: Stack::new(),
            memory: Memory::new(),
            storage: Storage::new(),
            pc: 0,
            gas: U256::from(10_000_000),
            bytecode,
        }
    }

    pub fn run(&mut self) {
        let instructions = decode_bytecode(&self.bytecode);
        while self.pc < instructions.len() {
            self.execute_opcode(&instructions[self.pc]);
            self.pc += 1;
        }
    }

    fn execute_opcode(&mut self, opcode: &Opcode) {
        match opcode {
            Opcode::PUSH1(val) => {
                self.stack.push(*val).unwrap();
                println!("PUSH1 is active");
            }
            Opcode::ADD => {
                let a = self.stack.pop().unwrap();
                let b = self.stack.pop().unwrap();
                self.stack.push(a + b).unwrap();
                println!("ADD is active");
            }
            Opcode::STOP => {
                println!("Execution Stopped");
                self.pc = self.bytecode.len(); // Stop execution
              //  println!("was active");
            }
            opcode::POP => {
                
            }
        }
    }
}
