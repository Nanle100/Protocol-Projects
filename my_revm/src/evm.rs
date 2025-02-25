
use primitive_types::U256;
use crate::execution::stack::Stack;
use crate::execution::memory::Memory;
use crate::execution::storage::Storage;
use crate::execution::gas::Gas;
use crate::state::world_state::WorldState;
use crate::opcodes::*;  // Import all opcode functions
use anyhow::Result;

#[derive(Debug)]
pub struct EVM {
    pub stack: Stack,
    pub memory: Memory,
    pub storage: Storage,
    pub gas: Gas,
    pub state: WorldState,
    pub pc: usize, // Program Counter
}

impl EVM {
    pub fn new() -> Self {
        EVM {
            stack: Stack::new(),
            memory: Memory::new(),
            storage: Storage::new(),
            gas: Gas::new(1_000_000),
            state: WorldState::new(),
            pc: 0,
        }
    }

    pub fn execute_opcode(&mut self, bytecode: &[u8]) -> Result<()> {
        while self.pc < bytecode.len() {
            let opcode = bytecode[self.pc];
            self.pc += 1; // Move to next opcode

            match opcode {
                0x00 => stop(), // STOP

                // Arithmetic Opcodes
                0x01 => add(&mut self.stack)?,
                0x02 => mul(&mut self.stack)?,
                0x03 => sub(&mut self.stack)?,
                0x04 => div(&mut self.stack)?,

                // Stack Opcodes
                0x60 => { // PUSH1
                    let value = bytecode[self.pc]; // Read next byte as immediate value
                    push1(&mut self.stack, value)?;
                    self.pc += 1; // Move past the immediate value
                }
                0x80 => dup1(&mut self.stack)?,
                0x90 => swap1(&mut self.stack)?,

                _ => return Err(anyhow::anyhow!("Unknown opcode: 0x{:02x}", opcode)),
            }
        }
        Ok(())
    }
}
