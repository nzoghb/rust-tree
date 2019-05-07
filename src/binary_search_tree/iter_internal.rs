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
    use super::Node;

    struct Utils {
        tree_base: Node<i32>,
        vec_base: Vec<&'static i32>,
    }

    impl Utils {
        fn new() -> Self {
            Self {
                tree_base: Node {
                    value: 50,
                    left: Some(Box::new(Node { value: 25, ..Default::default() })),
                    right: Some(Box::new(Node { value: 75, ..Default::default() })),
                },
                vec_base: vec![&25, &50, &75],
            }
        }
    }

    #[test]
    fn test_iter() {
        let setup = Utils::new();
        let tree_base = setup.tree_base;
        let tree_test = tree_base.clone();
        let vec_base = setup.vec_base;
        let vec_test = tree_test.iter().collect::<Vec<&i32>>();

        assert_eq!(vec_base, vec_test);
        // `tree_test` hasn't moved!
        assert_eq!(tree_base, tree_test);
    }
}