pub struct Window<I>
where
    I: Iterator,
{
    iterator: I,
    buffer: Vec<I::Item>,
    window_size: usize,
}

impl<I> Window<I>
where
    I: Iterator,
{
    pub fn new(iter: I, size: usize) -> Self {
        Self {
            iterator: iter,
            buffer: Vec::new(),
            window_size: size,
        }
    }
}

impl<I> Iterator for Window<I>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = Vec<I::Item>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.window_size == 0 {
            return None;
        }

        while self.buffer.len() < self.window_size {
            self.buffer.push(self.iterator.next()?);
        }

        let result = self.buffer.clone();
        match self.iterator.next() {
            Some(next_item) => {
                self.buffer.remove(0);
                self.buffer.push(next_item);
            }
            None => {
                self.buffer.clear();
            }
        }
        Some(result)
    }
}
