use super::tree::{Node, NodeBox};

impl<T> Node<T> 
where
  Node<T>: From<T>,
{
    pub fn boxer(data: T) -> NodeBox<T> {
        Some(Box::new(Self::from(data)))
    }
}

impl<T: Default + PartialOrd> From<T> for Node<T> {
    fn from(val: T) -> Self {
        Node::new(val)
    }
}