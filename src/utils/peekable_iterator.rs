use super::circular_buffer::CircularBuffer;

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

#[cfg(test)]
mod tests
{
    use crate::utils::peekable_iterator::{NthPeekable, Peekable};

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