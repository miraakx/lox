use std::slice::Iter;

pub struct Stack<T> {
    vec: Vec<T>
}

impl <T> Stack<T>
{
    pub const fn new() -> Self {
        Self { vec: Vec::new() }
    }

    pub fn push(&mut self, value: T) {
        self.vec.push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.vec.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        let len = self.vec.len();
        if len == 0 {
            return None;
        }
        Some(&mut self.vec[len-1])
    }

    pub fn peek(&self) -> Option<&T> {
        let len = self.vec.len();
        if len == 0 {
            return None;
        }
        Some(&self.vec[len-1])
    }

    pub fn iter(&self) -> Iter<T> {
        return self.vec.iter();
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }
}

#[cfg(test)]
mod tests
{
    use crate::utils::stack::Stack;

    #[test]
    fn test_stack() {
        let mut stack = Stack::<i32>::new();
        assert_eq!(stack.is_empty(), true);
        assert_eq!(stack.len(), 0);
        assert_eq!(stack.peek(), None);
        assert_eq!(stack.peek_mut(), None);
        assert_eq!(stack.pop(), None);
        assert_eq!(stack.len(), 0);


        stack.push(1);
        assert_eq!(stack.peek().cloned(), Some(1));
        assert_eq!(stack.peek_mut().cloned(), Some(1));
        assert_eq!(stack.is_empty(), false);
        assert_eq!(stack.len(), 1);
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.is_empty(), true);
        assert_eq!(stack.len(), 0);

        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.is_empty(), false);
        assert_eq!(stack.len(), 3);
    }
}