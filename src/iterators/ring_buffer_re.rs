#[derive(Debug)]
pub struct RingBuffer<T> {
    buffer: Vec<Option<T>>,
    head: usize,
    tail: usize,
    len: usize,
    capacity: usize,
}

pub struct RingBufferIter<'a, T> {
    ring: &'a RingBuffer<T>,
    index: usize,
    remaining: usize,
}

impl<'a, T> IntoIterator for &'a RingBuffer<T> {
    type Item = &'a T;
    type IntoIter = RingBufferIter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct RingBufferIterMut<'a, T> {
    ring: &'a mut RingBuffer<T>,
    index: usize,
    reamaining: usize,
}

impl<'a, T> Iterator for &'a mut RingBufferIterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.reamaining == 0 {
            return None;
        }
        let value = self.ring.buffer[self.index].as_mut()?;
        self.reamaining -= 1;
        self.index = (self.index + 1) % self.ring.capacity;

        let ptr: *mut T = value;
        unsafe { Some(&mut *ptr) }
    }
}

impl<'a, T> Iterator for RingBufferIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        let value = self.ring.buffer[self.index].as_ref();
        self.remaining -= 1;
        self.index = (self.index + 1) % self.ring.capacity;
        value
    }
}

impl<'a, T> RingBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        let buffer: Vec<_> = (0..capacity).into_iter().map(|_| None).collect();
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
            ring: self,
            index: self.head,
            remaining: self.len,
        }
    }

    pub fn iter_mut(&'a mut self) -> RingBufferIterMut<'a, T> {
        let head = (*self).head;
        let len = (*self).len;
        RingBufferIterMut {
            ring: self,
            index: head,
            reamaining: len,
        }
    }

    pub fn push(&mut self, value: T) {
        self.buffer[self.tail] = Some(value);
        if self.len != self.capacity {
            self.len += 1;
        } else {
            self.head = (self.head + 1) % self.capacity;
        }
        self.tail = (self.tail + 1) % self.capacity;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        let value = self.buffer[self.head].take();
        self.head = (self.head + 1) % self.capacity;
        self.len -= 1;
        value
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}
