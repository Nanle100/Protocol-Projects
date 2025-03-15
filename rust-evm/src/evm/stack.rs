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

    //returns the stack length
    pub fn len(&self) -> usize {
        self.data.len()
    }
   
    // This function safely pushes a U256 value onto a stack, ensuring it doesn’t exceed the maximum allowed size. If the stack is full, it returns a StackOverflow error.
    pub fn push(&mut self, value: U256) -> Result<(), StackError> {
        if self.data.len() >= MAX_STACK_SIZE {
            return Err(StackError::StackOverflow);
        }
        self.data.push(value);

        Ok(())
    }

    //This Rust function, pop, is designed to remove and return the top element from a stack while handling underflow errors.
    pub fn pop(&mut self) -> Result<U256, StackError> {
        self.data.pop().ok_or(StackError::StackUnderflow)
    }

    //swaps the top stack item with the n-th item below it
    pub fn swap(&mut self, n: usize) -> Result<(), StackError> {
        if self.data.len() <= n {
            return Err(StackError::StackUnderflow);
        }

        let top_idx = self.data.len() - 1;
        let swap_idx = self.data.len() - 1 - n;

        self.data.swap(top_idx, swap_idx);
        Ok(())
    }

    // The peek function allows you to look at the top element of the stack without removing it, while also handling underflow errors.
    pub fn peek(&self) -> Result<U256, StackError> {
        self.data.last().copied().ok_or(StackError::StackUnderflow)
    }

    // #[allow(dead_code)]
    //The to_string function converts the stack into a human-readable string representation, formatting the values in hexadecimal.
    // pub fn to_string(&self) -> String {
    //     if self.data.is_empty() {
    //         return "[]".to_string();
    //     }
    //     let mut d = String::from("[");
    //     for (i, val) in self.data.iter().rev().enumerate() {
    //         if i > 0 {
    //             d.push_str(", ");
    //         }
    //         d.push_str(&format!("{:x}", val));
    //     }
    //     d.push(']');
    //     d
    // }
}
