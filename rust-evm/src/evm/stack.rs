use primitive_types::U256;

const MAX_STACK_SIZE: usize = 1024;

#[derive(Debug, Default)]
pub struct Stack {
    data: Vec<U256>,
}

#[derive(Debug)]
pub enum StackError {
    StackOverflow,
    StackUnderflow,
}

impl Stack {
    pub fn new() -> Self {
        Stack { data: Vec::new() }
    }

    pub fn push(&mut self, value: U256) -> Result<(), StackError> {
        if self.data.len() >= MAX_STACK_SIZE {
            return Err(StackError::StackOverflow);
        }
        self.data.push(value);
        Ok(())
    }

    pub fn pop(&mut self) -> Result<U256, StackError> {
        self.data.pop().ok_or(StackError::StackUnderflow)
    }

    pub fn peek(&self) -> Result<U256, StackError> {
        self.data.last().copied().ok_or(StackError::StackUnderflow)
    }

    pub fn to_string(&self) -> String {
        if self.data.is_empty() {
            return "[]".to_string();
        }
        let mut d = String::from("[");
        for (i, val) in self.data.iter().rev().enumerate() {
            if i > 0 {
                d.push_str(", ");
            }
            d.push_str(&format!("{:x}", val));
        }
        d.push(']');
        d
    }
}
