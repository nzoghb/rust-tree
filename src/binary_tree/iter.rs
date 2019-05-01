use tree::{Node};

/// Iterator type for a binary tree.
/// This is a generator that progresses through an in-order traversal.
pub struct NodeIterator<T> {
    branch_stack: Vec<Node<T>>,
}

impl<T> NodeIterator<T>
where
    Node<T>: Clone
{
    /// Given a reference to a node, return an iterator over it and any of
    /// its branches in an in order traversal.
    //TODO: Vec::with_capacity()
    fn new(root: Node<T>) -> NodeIterator<T> {
        let mut iter = NodeIterator {
            branch_stack: vec![],
        };
        iter.add_left_branches(root);
        iter
    }

    /// Given a node, traverse along its left branches and add all right
    /// branches to the `branch_stack` field.
    /// Set the current node to the left-most child.
    fn add_left_branches(&mut self, mut root: Node<T>) {
        self.branch_stack.push(root.clone());
        while let Node { left: Some(left_branch), .. } = root {
            root = *left_branch;
            self.branch_stack.push(root.clone());
        }
    }
}

impl<T> Iterator for NodeIterator<T>
where
    Node<T>: Clone
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if let Some(node) = self.branch_stack.pop() {
            if let Some(right_branch) = node.right {
                self.add_left_branches(*right_branch);
            }
            let Node { value: result, .. } = node;
            return Some(result);
        }
        None
    }
}

impl<T> IntoIterator for Node<T>
where
    Node<T>: Clone
{
    type Item = T;
    type IntoIter = NodeIterator<T>;

    fn into_iter(self) -> NodeIterator<T> {
        NodeIterator::new(self)
    }
}

#[cfg(test)]
mod iter_tests {
    use super::Node;

    #[test]
    fn test_iter() {
        let mut tree = Node::new("hello");
        tree.insert("hi");
        tree.insert("bye");
        tree.insert("hey");
        tree.insert("three");
        let mut iter = tree.clone().into_iter();
        println!("{:#?}", tree);
        println!("{:?}", iter.next());
        println!("{:?}", iter.next());
        println!("{:?}", iter.next());
        println!("{:?}", iter.next());
        println!("{:?}", iter.next());
    }
}