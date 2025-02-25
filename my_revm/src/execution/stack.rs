use primitive_types::U256; // Use Ethereum's 256-bit integer type

pub struct Stack {
    data: Vec<U256>,  // Use U256 for 256-bit stack elements
    max_depth: usize, // EVM stack has a max depth of 1024
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            data: Vec::with_capacity(1024), // Preallocate stack space
            max_depth: 1024,
        }
    }

    /// Push a value onto the stack
    pub fn push(&mut self, value: U256) -> Result<(), &'static str> {
        if self.data.len() >= self.max_depth {
            return Err("Stack overflow: max depth reached (1024)");
        }
        self.data.push(value);
        Ok(())
    }

    /// Pop a value from the stack
    pub fn pop(&mut self) -> Result<U256, &'static str> {
        self.data.pop().ok_or("Stack underflow: cannot pop from empty stack")
    }

    /// Peek at the top value without removing it
    pub fn peek(&self) -> Option<U256> {
        self.data.last().copied()
    }

    /// Swap elements (SWAP1, SWAP2, ..., SWAP16)
    pub fn swap(&mut self, index: usize) -> Result<(), &'static str> {
        if index == 0 || index >= self.data.len() {
            return Err("Invalid swap index");
        }
        let top = self.data.len() - 1;
        self.data.swap(top, top - index);
        Ok(())
    }

    /// Duplicate elements (DUP1, DUP2, ..., DUP16)
    pub fn dup(&mut self, index: usize) -> Result<(), &'static str> {
        if index == 0 || index > self.data.len() {
            return Err("Invalid dup index");
        }
        let value = self.data[self.data.len() - index];
        self.push(value)
    }
}
