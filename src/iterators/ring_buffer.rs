pub struct RingBuffer<T> {
    pub buffer: Vec<Option<T>>,
    pub head: usize,
    pub tail: usize,
    pub len: usize,
    pub capacity: usize,
}

pub struct RingBufferIter<'a, T> {
    pub buffer: &'a RingBuffer<T>,
    pub index: usize,
    pub remaining: usize,
}

impl<'a, T> Iterator for RingBufferIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let ring = self.buffer;
        if self.remaining == 0 {
            return None;
        }
        let value = ring.buffer[self.index].as_ref();
        self.index = (self.index + 1) % ring.capacity;
        self.remaining -= 1;
        value
    }
}


pub struct RingBufferIterMut<'a, T> {
    buffer: &'a mut RingBuffer<T>,
    index: usize,
    remaining: usize,
}


impl<'a, T> Iterator for RingBufferIterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        let idx = self.index;
        self.index = (self.index + 1) % self.buffer.capacity;
        self.remaining -= 1;

        let value = self.buffer.buffer[idx].as_mut()?;
        // расширяем lifetime до 'a
        let ptr: *mut T = value;
        unsafe { Some(&mut *ptr) }
    }
}

impl<'a, T> IntoIterator for &'a RingBuffer<T> {
    type Item = &'a T;
    type IntoIter = RingBufferIter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
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

    pub fn iter(&self) -> RingBufferIter<'_, T> {
        RingBufferIter {
            buffer: self,
            index: self.head,
            remaining: self.len,
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
