// tests/stack_test.
use crate::execution::stack::Stack;



#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    use super::*;
    use primitive_types::U256;

    #[test]
    fn test_push_pop() {
        let mut stack = Stack::new();
        stack.push(U256::from(42)).unwrap();
        assert_eq!(stack.pop().unwrap(), U256::from(42));
    }

    #[test]
    fn test_swap() {
        let mut stack = Stack::new();
        stack.push(U256::from(1)).unwrap();
        stack.push(U256::from(2)).unwrap();
        stack.swap(1).unwrap();
        assert_eq!(stack.pop().unwrap(), U256::from(1));
        assert_eq!(stack.pop().unwrap(), U256::from(2));
    }

    #[test]
    fn test_dup() {
        let mut stack = Stack::new();
        stack.push(U256::from(42)).unwrap();
        stack.dup(1).unwrap();
        assert_eq!(stack.pop().unwrap(), U256::from(42));
        assert_eq!(stack.pop().unwrap(), U256::from(42));
    }
}