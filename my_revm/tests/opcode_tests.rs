// tests/opcode_test.rs
#[cfg(test)]
mod tests {
    use super::*;
    use primitive_types::U256;

    #[test]
    fn test_add() {
        let mut stack = Stack::new();
        stack.push(U256::from(2)).unwrap();
        stack.push(U256::from(3)).unwrap();
        add(&mut stack).unwrap();
        assert_eq!(stack.pop().unwrap(), U256::from(5));
    }

    #[test]
    fn test_push1() {
        let mut stack = Stack::new();
        push1(&mut stack, 42).unwrap();
        assert_eq!(stack.pop().unwrap(), U256::from(42));
    }
}