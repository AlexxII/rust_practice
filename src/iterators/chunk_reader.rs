use std::io::Read;

pub struct ChunkReader<R> {
    reader: R,
    chunk_size: usize,
    buffer: Vec<u8>,
}

impl<R: Read> Iterator for ChunkReader<R> {
    type Item = Vec<u8>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Ok(n) = self.reader.read(&mut self.buffer[..]) {
            if n == 0 {
                return None;
            }
            return Some(self.buffer[..n].to_vec());
        } else {
            return None;
        }
    }
}

impl<R: Read> ChunkReader<R> {
    pub fn new(reader: R, chunk_size: usize) -> Self {
        Self {
            reader,
            chunk_size,
            buffer: vec![0; chunk_size],
        }
    }
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
