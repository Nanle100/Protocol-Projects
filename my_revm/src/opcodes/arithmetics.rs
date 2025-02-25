use crate::execution::stack::Stack;
use anyhow::Result;
use primitive_types::U256;

pub fn add(stack: &mut Stack) -> Result<()> {
    let a = stack.pop()?;
    let b = stack.pop()?;
    stack.push(a.overflowing_add(b).0)?;
    Ok(())
}

pub fn mul(stack: &mut Stack) -> Result<()> {
    let a = stack.pop()?;
    let b = stack.pop()?;
    stack.push(a.overflowing_mul(b).0)?;
    Ok(())
}

pub fn sub(stack: &mut Stack) -> Result<()> {
    let a = stack.pop()?;
    let b = stack.pop()?;
    stack.push(a.overflowing_sub(b).0)?;
    Ok(())
}

pub fn div(stack: &mut Stack) -> Result<()> {
    let a = stack.pop()?;
    let b = stack.pop()?;
    if b.is_zero() {
        stack.push(U256::zero())?; // EVM returns 0 on division by zero
    } else {
        stack.push(a / b)?;
    }
    Ok(())
}