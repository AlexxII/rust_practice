use std::collections::VecDeque;

pub struct Node<T> {
    pub value: T,
    pub children: Vec<Node<T>>,
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

    pub fn bfs(&self) -> BfsIter<'_, T> {
        let mut queue = VecDeque::new();
        queue.push_back(self);
        BfsIter { queue: queue }
    }
}


pub struct DfsIter<'a, T> {
    stack: Vec<&'a Node<T>>,
}

pub struct BfsIter<'a, T> {
    queue: VecDeque<&'a Node<T>>,
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

impl<'a, T> Iterator for BfsIter<'a, T> {
    type Item = &'a Node<T>;
    fn next(&mut self) -> Option<Self::Item> {
        let node = self.queue.pop_front()?;

        for child in &node.children {
            self.queue.push_back(child);
        }
        Some(node)
    }
}
