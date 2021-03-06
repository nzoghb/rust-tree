use super::tree::Node;

/// Iterator type for a binary tree.
/// This is a generator that progresses through an in-order traversal.
pub struct NodeIterator<T> {
    branch_stack: Vec<Node<T>>,
}

impl<T> NodeIterator<T>
where
    Node<T>: Clone,
{
    /// Given a reference to a node, consume it and return an iterator over it
    /// and any of its branches in an in-order traversal.
    //TODO: Vec::with_capacity()
    fn new(root: Node<T>) -> Self {
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
    Node<T>: Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if let Some(root) = self.branch_stack.pop() {
            if let Node { right: Some(right_branch), .. } = root {
                self.add_left_branches(*right_branch);
            }
            let Node { value: result, .. } = root;
            return Some(result);
        }
        None
    }
}

impl<T> IntoIterator for Node<T>
where
    Node<T>: Clone,
{
    type Item = T;
    type IntoIter = NodeIterator<T>;

    fn into_iter(self) -> NodeIterator<T> {
        NodeIterator::new(self)
    }
}

#[cfg(test)]
mod test {
    use setup_test;

    #[test]
    fn test_iter() {
        setup_test!(,balanced_tree_base,,vec_base);
        let vec_test = balanced_tree_base.into_iter().collect::<Vec<i32>>();

        assert_eq!(vec_base, vec_test);
    }
}