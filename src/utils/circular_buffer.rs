pub struct CircularBuffer<T: Clone>
{
    buffer:   Vec<Option<T>>,
    capacity: usize,
    head:     usize,
    tail:     usize,
    size:     usize,
}

impl<T: Clone> CircularBuffer<T>
{
    pub fn new(capacity: usize) -> Self
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

#[cfg(test)]
mod tests
{
    use crate::utils::circular_buffer::CircularBuffer;

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
}