#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_push_pop() {
        let mut stack = Stack::new();
        stack.push(U256::from(10)).unwrap();
        assert_eq!(stack.pop().unwrap(), U256::from(10));
    }

    #[test]
    fn test_memory_store_load() {
        let mut memory = Memory::new();
        memory.store(0, &[1, 2, 3]);
        assert_eq!(memory.load(0, 3), vec![1, 2, 3]);
    }
}
