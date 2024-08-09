use std::{str::Chars, slice::Iter};

struct CircularBuffer<T: Clone>
{
    buffer:   Vec<Option<T>>,
    capacity: usize,
    head:     usize,
    tail:     usize,
    size:     usize,
}

impl<T: Clone> CircularBuffer<T>
{
    fn new(capacity: usize) -> Self
    {
        Self
        {
            buffer: vec![None; capacity],
            capacity,
            head: 0,
            tail: 0,
            size: 0,
        }
    }

    pub const fn is_empty(&self) -> bool
    {
        self.size == 0
    }

    const fn is_full(&self) -> bool
    {
        self.size == self.capacity
    }

    pub fn enqueue(&mut self, item: T)
    {
        if self.is_full() {
            panic!("Il buffer circolare è pieno!");
        }
        self.buffer[self.tail] = Some(item);
        self.move_tail();
        self.size += 1;
    }

    pub fn dequeue(&mut self) -> Option<T>
    {
        if self.is_empty() {
            return None;
        }
        let item = self.buffer[self.head].take();
        self.move_head();
        self.size -= 1;
        item
    }

    pub fn peek(&mut self, index: usize) -> Option<&T>
    {
        self.buffer[(self.head + index) % self.capacity].as_ref()
    }

    fn move_head(&mut self)
    {
        self.head = (self.head + 1) % self.capacity;
    }

    fn move_tail(&mut self)
    {
        self.tail = (self.tail + 1) % self.capacity;
    }

    pub const fn size(&self) -> usize
    {
        self.size
    }
}

pub struct NthPeekable<I, T: Clone> where I: Iterator<Item = T>,
{
    iter: I,
    buffer: CircularBuffer<T>,
}

impl<I, T: Clone> NthPeekable<I, T> where I: Iterator<Item = T>,
{
    pub fn new(iter: I, size: usize) -> Self
    {
        Self
        {
            iter,
            buffer: CircularBuffer::new(size),
        }
    }

    pub fn peek(&mut self) -> Option<&T>
    {
        if self.buffer.is_empty()
        {
            if let Some(item) = self.iter.next() {
                self.buffer.enqueue(item);
            }
        }
        self.buffer.peek(0)
    }

    pub fn peek_nth(&mut self, index: usize) -> Option<&T>
    {
        if self.buffer.size() > index {
            return self.buffer.peek(index);
        }
        let mut i: usize = self.buffer.size();
        loop {
            if let Some(item) = self.iter.next() {
                self.buffer.enqueue(item);
            } else {
                return None;
            }
            if i == index {
                return self.buffer.peek(index);
            }
            i += 1;
        }
    }

    pub fn next(&mut self) -> Option<T>
    {
        if self.buffer.is_empty() {
            return self.iter.next();
        }
        self.buffer.dequeue()
    }

}

pub struct Peekable<I, T: Clone> where I: Iterator<Item = T>,
{
    iter: I,
    item: Option<T>,
}

impl<I, T: Clone> Peekable<I, T> where I: Iterator<Item = T>,
{
    pub const fn new(iter: I) -> Self
    {
        Self { iter, item: None }
    }

    pub fn peek(&mut self) -> Option<&T>
    {
        if self.item.is_none() {
            self.item = self.iter.next();
        }
        self.item.as_ref()
    }

    pub fn next(&mut self) -> Option<T>
    {
        if self.item.is_none() {
            self.iter.next()
        } else {
            self.item.take()
        }
    }
}

pub struct Scanner<'a>
{
    iter:   NthPeekable<Chars<'a>, char>,
}

impl <'a> Scanner<'a>
{
    pub fn from_str(str: &'a str, peek_dept: usize) -> Self
    {
        Scanner
        {
            iter: NthPeekable::new(str.chars(), peek_dept),
        }
    }

    pub fn next(&mut self) -> Option<char>
    {
        self.iter.next()
    }

    pub fn peek(&mut self) -> Option<char>
    {
        self.iter.peek().cloned()
    }

    pub fn peek_next(&mut self) -> Option<char>
    {
        self.iter.peek_nth(1).cloned()
    }

    pub fn peek_nth(&mut self, index: usize) -> Option<char>
    {
        self.iter.peek_nth(index).cloned()
    }

    pub fn is_peek(&mut self, ch: char) -> bool
    {
        self.peek().map_or(false, |next_ch| next_ch==ch)
    }

    pub fn is_peek_next(&mut self, ch: char) -> bool
    {
        self.peek_next().map_or(false, |next_ch| next_ch==ch)
    }

    pub fn is_peek_ascii_digit(&mut self) -> bool
    {
        matches!(self.peek(), Some(chr) if chr.is_ascii_digit())
    }

    pub fn is_peek_next_ascii_digit(&mut self) -> bool
    {
        matches!(self.peek_next(), Some(chr) if chr.is_ascii_digit())
    }

    pub fn is_peek_identifier_char(&mut self) -> bool
    {
        matches!(self.peek(), Some(ch) if ch.is_ascii_alphabetic() || ch == '_' || ch.is_ascii_digit())
    }

    pub fn consume_if_peek_is(&mut self, ch: char)
    {
        if self.peek().map_or(false, |next_ch| next_ch==ch) {
            self.next();
        }
    }

    pub fn consume(&mut self)
    {
        self.next();
    }

    pub fn unwrap_next(&mut self) -> char
    {
        self.next().unwrap()
    }

}

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
mod tests {


    use crate::utils::utils::{CircularBuffer, NthPeekable};

    use super::{Peekable, Scanner, Stack};

    #[test]
    fn test_scanner() {
        let mut scanner = Scanner::from_str("test123", 3);
        assert_eq!(scanner.is_peek('t'), true);
        assert_eq!(scanner.is_peek_next('e'), true);
        assert_eq!(scanner.peek(), Some('t'));
        assert_eq!(scanner.peek_next(), Some('e'));
        assert_eq!(scanner.peek_nth(0), Some('t'));
        assert_eq!(scanner.peek_nth(1), Some('e'));
        assert_eq!(scanner.peek_nth(2), Some('s'));

        assert_eq!(scanner.next(), Some('t'));
        assert_eq!(scanner.is_peek('e'), true);
        assert_eq!(scanner.is_peek_next('s'), true);
        assert_eq!(scanner.is_peek_ascii_digit(), false);
        assert_eq!(scanner.is_peek_next_ascii_digit(), false);
        assert_eq!(scanner.is_peek_identifier_char(), true);

        scanner.consume();
        scanner.consume();
        scanner.consume();
        assert_eq!(scanner.peek(), Some('1'));
        assert_eq!(scanner.is_peek_ascii_digit(), true);
        assert_eq!(scanner.is_peek_next_ascii_digit(), true);
        assert_eq!(scanner.is_peek_identifier_char(), true);
    }

    #[test]
    #[should_panic]
    fn test_scanner_panic() {
        let mut scanner = Scanner::from_str("test", 2);
        assert_eq!(scanner.peek_nth(2), Some('e'));
    }

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

    #[test]
    fn test_circular_buffer()
    {
        let mut buffer: CircularBuffer<usize> = CircularBuffer::new(5);

        assert_eq!(buffer.dequeue(), None);

        assert_eq!(buffer.is_empty(), true);
        assert_eq!(buffer.is_full(), false);
        buffer.enqueue(1);
        assert_eq!(buffer.is_empty(), false);
        assert_eq!(buffer.is_full(), false);
        buffer.enqueue(2);
        assert_eq!(buffer.is_empty(), false);
        assert_eq!(buffer.is_full(), false);
        buffer.enqueue(3);
        assert_eq!(buffer.is_empty(), false);
        assert_eq!(buffer.is_full(), false);
        buffer.enqueue(4);
        assert_eq!(buffer.is_empty(), false);
        assert_eq!(buffer.is_full(), false);
        buffer.enqueue(5);
        assert_eq!(buffer.is_empty(), false);
        assert_eq!(buffer.is_full(), true);

        assert_eq!(buffer.dequeue(), Some(1));
        assert_eq!(buffer.is_empty(), false);
        assert_eq!(buffer.is_full(), false);
        assert_eq!(buffer.dequeue(), Some(2));
        assert_eq!(buffer.is_empty(), false);
        assert_eq!(buffer.is_full(), false);
        assert_eq!(buffer.dequeue(), Some(3));
        assert_eq!(buffer.is_empty(), false);
        assert_eq!(buffer.is_full(), false);
        assert_eq!(buffer.dequeue(), Some(4));
        assert_eq!(buffer.is_empty(), false);
        assert_eq!(buffer.is_full(), false);
        assert_eq!(buffer.dequeue(), Some(5));
        assert_eq!(buffer.is_empty(), true);
        assert_eq!(buffer.is_full(), false);
        assert_eq!(buffer.dequeue(), None);
    }

    #[test]
    #[should_panic]
    fn test_circular_buffer_panic()
    {
        let mut buffer: CircularBuffer<usize> = CircularBuffer::new(1);
        buffer.enqueue(1);
        buffer.enqueue(2);
    }

    #[test]
    fn test_peekable()
    {
        let text = "test";
        let mut buffer = Peekable::new(text.chars());
        assert_eq!(buffer.peek().cloned(), Some('t'));
        assert_eq!(buffer.peek().cloned(), Some('t'));
        assert_eq!(buffer.next(), Some('t'));
        assert_eq!(buffer.peek().cloned(), Some('e'));
        assert_eq!(buffer.peek().cloned(), Some('e'));
        assert_eq!(buffer.peek().cloned(), Some('e'));
        assert_eq!(buffer.next(), Some('e'));
        assert_eq!(buffer.next(), Some('s'));
        assert_eq!(buffer.peek().cloned(), Some('t'));
        assert_eq!(buffer.next(), Some('t'));
        assert_eq!(buffer.peek(), None);
        assert_eq!(buffer.next(), None);
    }

    #[test]
    fn test_nth_peekable_1()
    {
        let text = "testo di prova";
        let mut buffer = NthPeekable::new(text.chars(), 5);
        assert_eq!(buffer.peek().cloned(), Some('t'));
        assert_eq!(buffer.peek().cloned(), Some('t'));
        assert_eq!(buffer.peek().cloned(), Some('t'));
        assert_eq!(buffer.peek().cloned(), Some('t'));
        assert_eq!(buffer.peek().cloned(), Some('t'));

        assert_eq!(buffer.next(), Some('t'));
        assert_eq!(buffer.next(), Some('e'));
        assert_eq!(buffer.next(), Some('s'));
        assert_eq!(buffer.next(), Some('t'));
        assert_eq!(buffer.next(), Some('o'));
        assert_eq!(buffer.next(), Some(' '));

        assert_eq!(buffer.peek().cloned(), Some('d'));
        assert_eq!(buffer.peek().cloned(), Some('d'));
        assert_eq!(buffer.next(), Some('d'));
        assert_eq!(buffer.next(), Some('i'));
        assert_eq!(buffer.next(), Some(' '));
        assert_eq!(buffer.next(), Some('p'));
        assert_eq!(buffer.next(), Some('r'));
        assert_eq!(buffer.next(), Some('o'));
        assert_eq!(buffer.next(), Some('v'));
        assert_eq!(buffer.next(), Some('a'));
        assert_eq!(buffer.peek(), None);
        assert_eq!(buffer.peek(), None);
        assert_eq!(buffer.next(), None);
    }

    #[test]
    fn test_nth_peekable_2()
    {
        let text = "testo di prova";
        let mut buffer = NthPeekable::new(text.chars(), 5);
        assert_eq!(buffer.peek_nth(0).cloned(), Some('t'));
        assert_eq!(buffer.peek_nth(1).cloned(), Some('e'));
        assert_eq!(buffer.peek_nth(2).cloned(), Some('s'));
        assert_eq!(buffer.peek_nth(3).cloned(), Some('t'));
        assert_eq!(buffer.peek_nth(4).cloned(), Some('o'));

        assert_eq!(buffer.next(), Some('t'));
        assert_eq!(buffer.next(), Some('e'));
        assert_eq!(buffer.next(), Some('s'));
        assert_eq!(buffer.next(), Some('t'));
        assert_eq!(buffer.next(), Some('o'));
        assert_eq!(buffer.next(), Some(' '));

        assert_eq!(buffer.peek_nth(0).cloned(), Some('d'));
        assert_eq!(buffer.peek_nth(1).cloned(), Some('i'));
        assert_eq!(buffer.next(), Some('d'));
        assert_eq!(buffer.next(), Some('i'));
        assert_eq!(buffer.next(), Some(' '));
        assert_eq!(buffer.next(), Some('p'));
        assert_eq!(buffer.next(), Some('r'));
        assert_eq!(buffer.next(), Some('o'));
        assert_eq!(buffer.next(), Some('v'));
        assert_eq!(buffer.next(), Some('a'));
        assert_eq!(buffer.peek_nth(0), None);
        assert_eq!(buffer.peek_nth(1), None);
        assert_eq!(buffer.next(), None);
    }
}