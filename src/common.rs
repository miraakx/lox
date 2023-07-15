struct CircularBuffer<T: Clone> {
    buffer: Vec<Option<T>>,
    capacity: usize,
    head: usize,
    tail: usize,
    size: usize
}

impl<T: Clone> CircularBuffer<T> {

    fn new(capacity: usize) -> Self {
        CircularBuffer {
            buffer: vec![None; capacity],
            capacity,
            head: 0,
            tail: 0,
            size: 0
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    #[inline]
    fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    pub fn enqueue(&mut self, item: T) {
        if self.is_full() {
            panic!("Il buffer circolare è pieno!");
        }
        self.buffer[self.tail] = Some(item);
        self.move_tail();
        self.size = self.size + 1;
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let item = self.buffer[self.head].take();
        self.move_head();
        self.size = self.size - 1;
        item
    }

    #[inline]
    pub fn peek(&mut self, index: usize) -> Option<&T> {
        self.buffer[(self.head + index) % self.capacity].as_ref()
    }

    #[inline]
    fn move_head(&mut self) {
        self.head = (self.head + 1) % self.capacity;
    }

    #[inline]
    fn move_tail(&mut self) {
        self.tail = (self.tail + 1) % self.capacity;
    }

    pub fn tail(&self) -> Option<&T> {
        //If necessario perche' self.tail e self.capacity sono usize
        if self.tail == 0 {
            self.buffer[self.capacity - 1].as_ref()
        } else {
            self.buffer[(self.tail - 1)%self.capacity].as_ref()
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.buffer[self.head].as_ref()
    }

    #[inline]
    pub fn size(&self) -> usize {
        self.size
    }
    
}

pub struct NthPeekable<I, T: Clone> 
where
    I: Iterator<Item = T>
{
    iter: I,
    buffer: CircularBuffer<T>,
}

impl<I, T: Clone> NthPeekable<I, T> 
where
    I: Iterator<Item = T>
{
    pub fn new(iter: I, size: usize) -> Self {
        NthPeekable {
            iter,
            buffer: CircularBuffer::new(size),
        }
    }

    pub fn peek(&mut self) -> Option<&T> {
        if self.buffer.is_empty(){
            if let Some(item) = self.iter.next() {
                self.buffer.enqueue(item);
            }
        }
        self.buffer.peek(0)
    }

    pub fn peek_nth(&mut self, index: usize) -> Option<&T> {
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
            i = i + 1;
        }
    }

    pub fn next(&mut self) -> Option<T> {
        if self.buffer.is_empty() {
            return self.iter.next();
        }
        self.buffer.dequeue()
    }

    pub fn is_last(&mut self) -> bool {
        self.peek().is_none()
    }
    
}


pub struct Peekable<I, T: Clone> 
where
    I: Iterator<Item = T>
{
    iter: I,
    item: Option<T>
}

impl<I, T: Clone> Peekable<I, T> 
where
    I: Iterator<Item = T>
{
    pub fn new(iter: I) -> Self {
        Peekable {
            iter,
            item: None
        }
    }

    pub fn peek(&mut self) -> Option<&T> {
        if self.item.is_none(){
            self.item = self.iter.next();
        }
        self.item.as_ref()
    }

    pub fn next(&mut self) -> Option<T> {
        if self.item.is_none() {
            self.iter.next()
        } else {
            self.item.take()
        }
    }

    pub fn is_last(&mut self) -> bool {
        self.peek().is_none()
    }

}

#[test]
fn test_circular_buffer() {
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
fn test_circular_buffer_panic() {
    let mut buffer: CircularBuffer<usize> = CircularBuffer::new(1);
    buffer.enqueue(1);
    buffer.enqueue(2);
}

#[test]
fn test_peekable_iter() {
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
fn test_peekable_nth() {
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

