// src/opcodes/stack.rs
use crate::execution::stack::Stack;
use anyhow::Result;
use primitive_types::U256;

pub fn push1(stack: &mut Stack, value: u8) -> Result<()> {
    stack.push(U256::from(value))?;
    Ok(())
}

pub fn dup1(stack: &mut Stack) -> Result<()> {
    stack.dup(1)?;
    Ok(())
}

pub fn swap1(stack: &mut Stack) -> Result<()> {
    stack.swap(1)?;
    Ok(())
}