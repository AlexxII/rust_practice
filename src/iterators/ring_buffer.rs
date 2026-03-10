#[derive(Debug)]
pub struct RingBuffer<T> {
    buffer: Vec<Option<T>>,
    head: usize,
    tail: usize,
    len: usize,
    capacity: usize,
}

impl<T> RingBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        let buffer: Vec<Option<T>> = (0..capacity).map(|_| None).collect();
        RingBuffer {
            buffer,
            head: 0,
            tail: 0,
            len: 0,
            capacity,
        }
    }

    pub fn push(&mut self, value: T) {
        self.buffer[self.tail] = Some(value);
        if self.capacity != self.len {
            self.len += 1;
        } else {
            self.head = (self.head + 1) % self.capacity;
        }
        self.tail = (self.tail + 1) % self.capacity;
    }

    pub fn pop(&mut self) -> Option<T> {
        let result = self.buffer[self.head].take();
        self.buffer[self.head] = None;
        self.head = (self.head + 1) % self.capacity;
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        result
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}
