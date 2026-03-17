pub struct Flatten<I>
where
    I: Iterator,
    I::Item: IntoIterator,
{
    outer: I,
    inner: Option<<I::Item as IntoIterator>::IntoIter>,
}

impl<I> Flatten<I>
where
    I: Iterator,
    I::Item: IntoIterator,
{
    pub fn new(iter: I) -> Self {
        Self {
            outer: iter,
            inner: None
        }
    }
}

impl<I> Iterator for Flatten<I>
where
    I: Iterator,
    I::Item: IntoIterator,
{
    type Item = <I::Item as IntoIterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(inner) = &mut self.inner {
                if let Some(item) = inner.next() {
                    return Some(item)
                }
                self.inner = None;
            }
            let next_inner = self.outer.next()?;
            self.inner = Some(next_inner.into_iter());
        }
    }
}
