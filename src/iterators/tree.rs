pub struct Node<T> {
    pub value: T,
    children: Vec<Node<T>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            children: vec![],
        }
    }

    pub fn add_child(&mut self, node: Node<T>) {
        self.children.push(node);
    }

    pub fn dfs(&self) -> DfsIter<'_, T> {
        DfsIter { stack: vec![self] }
    }
}

pub struct DfsIter<'a, T> {
    stack: Vec<&'a Node<T>>,
}

impl<'a, T> Iterator for DfsIter<'a, T> {
    type Item = &'a Node<T>;
    fn next(&mut self) -> Option<Self::Item> {
        let node = self.stack.pop()?;

        for child in node.children.iter().rev() {
            self.stack.push(child);
        }

        Some(node)
    }
}
