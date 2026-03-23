use std::fmt::{Debug, Formatter};
use crate::iterators::tree::Node;

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
