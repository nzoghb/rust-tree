use super::tree::Node;

pub struct NodeIterator<'a, T>
where
    T: 'a,
{
    branch_stack: Vec<&'a Node<T>>,
}

impl<'a, T> NodeIterator<'a, T> {
    /// Given a reference to a node, return an iterator over it and any of its
    /// branches in an in-order traversal. Does not consume the original node.
    //TODO: Vec::with_capacity()
    fn new(root: &'a Node<T>) -> Self {
        let mut iter = NodeIterator {
            branch_stack: vec![],
        };
        iter.add_left_branches(root);
        iter
    }

    /// Given a node, traverse along its left branches and add all right
    /// branches to the `branch_stack` field.
    /// Set the current node to the left-most child.
    fn add_left_branches(&mut self, mut root: &'a Node<T>) {
        self.branch_stack.push(root);
        while let Node { left: Some(left_branch), .. } = root {
            root = left_branch;
            self.branch_stack.push(root);
        }
    }
}

impl<'a, T> Iterator for NodeIterator<'a, T>
where
    Node<T>: Clone,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if let Some(root) = self.branch_stack.pop() {
            if let Node { right: Some(right_branch), .. } = root {
                self.add_left_branches(right_branch);
            }
            let Node { value: result, .. } = root;
            return Some(result);
        }
        None
    }
}

impl<T> Node<T> {
    /// Returns a borrowing iterator over the leaves of the tree.
    pub fn iter(&self) -> NodeIterator<T> {
        NodeIterator::new(self)
    }
}

#[cfg(test)]
mod test {
    use setup_test;

    #[test]
    fn test_iter() {
        setup_test!(,balanced_tree_base,,vec_base);
        let tree_test = balanced_tree_base.clone();
        let vec_test = tree_test.iter().collect::<Vec<&i32>>();
        let vec_ref_base = vec_base.iter().collect::<Vec<&i32>>();

        assert_eq!(vec_ref_base, vec_test);
        // `tree_test` hasn't moved!
        assert_eq!(balanced_tree_base, tree_test);
    }
}