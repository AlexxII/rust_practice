use crate::iterators::ring_buffer::RingBuffer;
use std::fmt::{Debug, Display};

impl<T: Clone> Clone for RingBuffer<T> {
    fn clone(&self) -> Self {
        Self {
            buffer: self.buffer.clone(),
            head: self.head,
            tail: self.tail,
            len: self.len,
            capacity: self.capacity,
        }
    }
}

impl<T> From<Vec<T>> for RingBuffer<T> {
    fn from(value: Vec<T>) -> Self {
        let len = value.len();
        Self {
            buffer: value.into_iter().map(Some).collect(),
            head: 0,
            tail: len,
            len: len,
            capacity: len,
        }
    }
}

impl<T: PartialEq> PartialEq for RingBuffer<T> {
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len && self.iter().eq(other.iter())
    }
    // fn eq(&self, other: &Self) -> bool {
    //     self.head == other.head
    //         && self.tail == other.tail
    //         && self.len == other.len
    //         && self.capacity == other.capacity
    //         && self
    //             .buffer
    //             .iter()
    //             .zip(other.buffer.iter())
    //             .all(|(a, b)| a == b)
    // }
}

impl<T: Display> Display for RingBuffer<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        let mut first = true;
        for val in self.iter() {
            if !first {
                write!(f, ", ")?;
            }
            first = false;
            write!(f, "{}", val)?;
        }
        write!(f, "]")?;
        Ok(())
    }
}

impl<T: Debug> Debug for RingBuffer<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RingBuffer(len={}) ", self.len)?;
        f.debug_list().entries(self.iter()).finish()
    }
}
