use std::collections::VecDeque;
use std::fmt::{Debug, Formatter};

use serde::Serialize;
use serde::ser::SerializeStruct;

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

    pub fn bfs(&self) -> BfsIter<'_, T> {
        let mut queue = VecDeque::new();
        queue.push_back(self);
        BfsIter { queue: queue }
    }
}

impl<T: Serialize> Serialize for Node<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Node", 2)?;

        state.serialize_field("value", &self.value)?;
        state.serialize_field("children", &self.children)?;
        state.end()
    }
}

impl<T: PartialEq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
            && self.children.len() == other.children.len()
            && self
                .children
                .iter()
                .zip(other.children.iter())
                .all(|(a, b)| a.eq(b))
    }
}

impl<T: Debug> Node<T> {
    fn fmt_with_ident(&self, f: &mut Formatter<'_>, ident: usize) -> std::fmt::Result {
        for _ in 0..ident {
            write!(f, " ")?;
        }
        writeln!(f, "{:?}", self.value)?;
        for child in &self.children {
            child.fmt_with_ident(f, ident + 1)?;
        }
        Ok(())
    }
}

pub struct DfsIter<'a, T> {
    stack: Vec<&'a Node<T>>,
}

pub struct BfsIter<'a, T> {
    queue: VecDeque<&'a Node<T>>,
}

impl<T: Debug> Debug for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_with_ident(f, 0)
    }
}

impl<T: Clone> Clone for Node<T> {
    fn clone(&self) -> Self {
        let mut node = Node::new(self.value.clone());
        for child in &self.children {
            node.add_child(child.clone());
        }
        node
    }
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
