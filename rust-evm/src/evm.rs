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
           //gas: U256::from(10_000_000),
            gas: U256::from(25),
            bytecode,
        }
    }

    // EVM interpreter loop
    pub fn run(&mut self) {
        let instructions = decode_bytecode(&self.bytecode);
        while self.pc < instructions.len() {
            if self.gas == U256::zero() {
                println!("Out of gas! Halting execution.");
                break;
            }

            self.execute_opcode(&instructions[self.pc]);
            self.pc += 1;
        }

        println!("Execution finished. Remaining gas: {}", self.gas);

    }

    fn execute_opcode(&mut self, opcode: &Opcode) {

        let gas_cost = opcode.gas_cost();

        // Check if there's enough gas
        if self.gas < gas_cost {
            println!("Out of gas! Execution halted.");
            self.pc = self.bytecode.len(); // Stop execution
            return;
        }
    
        // Deduct gas
        self.gas -= gas_cost;

        
        //This is where you execute instructions and manipulate the EVM state (stack, memory, gas, etc.).
        //Each Opcode arm implements part of the EVM instruction set.
        match opcode {
            Opcode::PUSH1(val) => {
                self.stack.push(*val).unwrap();
                println!("PUSH1 executed, remaining gas: {}", self.gas);
               
            }
            Opcode::ADD => {
                let a = self.stack.pop().unwrap();
                let b = self.stack.pop().unwrap();
                self.stack.push(a + b).unwrap();
                println!("ADD executed, remaining gas: {}", self.gas);
            }
            Opcode::STOP => {
                println!("Execution stopped, remaining gas: {}, at a pc:{}", self.gas, self.pc);
                self.pc = self.bytecode.len(); // Stop execution
            //   println!("checking the value of pc {}", );
            }

            Opcode::POP => {
                match self.stack.pop() {
                    Ok(val) => println!("POP executed: removed {:?}, remaining gas: {}", val, self.gas),
                    Err(_) => println!("POP failed: Stack underflow"),
                }
            }

            Opcode::DUP(n) => {
                if self.stack.len() >= *n {
                    // if let Some(val) = self.stack.get(self.stack.len() - n) {
                        if let Ok(val) = self.stack.peek() {
                        self.stack.push(val).unwrap(); 
                        println!("DUP{} executed: duplicated value {:?}, Remaining gas:{}", n, val, self.gas);
                    }
                }
            }

            Opcode::SWAP(n) => {
                if let Err(_) = self.stack.swap(*n) {
                    println!("SWAP{} failed: Stack underflow", n);
                } else {
                    println!("SWAP{} executed: swapped top with stack[{}], remaining gas: {}", n, n, self.gas);
                }
            }

            Opcode::SUB => {
                let a = self.stack.pop().unwrap();
                let b = self.stack.pop().unwrap();
                self.stack.push(a - b).unwrap();
                println!("SUB executed, remaining gas: {}", self.gas);

            }

            Opcode::MUL => {
                let a = self.stack.pop().unwrap();
                let b = self.stack.pop().unwrap();
                if a.is_zero() || b.is_zero() {
                    self.stack.push(U256::zero()).unwrap();
                    println!("Multiplication with zero returns zero");
                    return; // Stop execution of this opcode

                }
               
                let result = a * b;
                self.stack.push(result).unwrap();
                println!("MUL executed, remaining gas: {}", self.gas);

            }

            Opcode::DIV => {
                let a = self.stack.pop().unwrap();
                let b = self.stack.pop().unwrap();
                if b.is_zero() {
                    self.stack.push(U256::zero()).unwrap(); // Division by zero returns 0 in EVM
                    println!("DIV executed: {} / {} = 0 (division by zero)", a, b);
                } else {
                    let result = a / b;
                    self.stack.push(result).unwrap();
                    println!("DIV executed: {} / {} = {}, remaining gas: {}", a, b, result, self.gas);
                }
            }

            Opcode::MOD => {
                let a = self.stack.pop().unwrap();
                let b = self.stack.pop().unwrap();
                if b.is_zero() {
                    self.stack.push(U256::zero()).unwrap(); // Modulo by zero returns 0
                    println!("MOD executed: {} % {} = 0 (modulo by zero)", a, b);
                } else {
                    let result = a % b;
                    self.stack.push(result).unwrap();
                    println!("MOD executed: {} % {} = {}, remaining gas: {}", a, b, result, self.gas);
                }
            }

            Opcode::LT => {
                let a = self.stack.pop().unwrap();
                let b = self.stack.pop().unwrap();
                let result = if a < b { U256::one() } else { U256::zero() };
                self.stack.push(result).unwrap();
                println!("LT executed: {} < {} = {}, remaining gas: {}", a, b, result, self.gas);
            }

            Opcode::GT => {
                let a = self.stack.pop().unwrap();
                let b = self.stack.pop().unwrap();
                let result = if a > b { U256::one() } else { U256::zero() };
                self.stack.push(result).unwrap();
                println!("GT executed: {} > {} = {}, remaining gas: {}", a, b, result, self.gas);
            }

            Opcode::EQ => {
                let a = self.stack.pop().unwrap();
                let b = self.stack.pop().unwrap();
                let result = if a == b { U256::one() } else { U256::zero() };
                self.stack.push(result).unwrap();
                println!("EQ executed: {} == {} = {}, remaining gas: {}", a, b, result, self.gas);
            }
            
         }
    }
}
