pub type NodeBox<T> = Option<Box<Node<T>>>;

/// A binary tree with data at every node. Each branch may or may not contain
/// another node.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Node<T> {
    pub value: T,
    pub left: NodeBox<T>,
    pub right: NodeBox<T>,
}

impl<T: Default> Node<T> {
    pub fn new(data: T) -> Node<T> {
        Node { value: data, ..Default::default() }
    }
}

impl<T> Node<T> {
    /// Recursively determine tree depth.
    fn depth(&self) -> usize {
        match *self {
            Node { left: None, right: None, .. } => 0,
            Node { left: Some(ref left_branch), right: None, .. } =>
                1 + left_branch.depth(),
            Node { left: None, right: Some(ref right_branch), .. } =>
                1 + right_branch.depth(),
            Node { left: Some(ref left_branch), right: Some(ref right_branch), .. } =>
                1 + std::cmp::max(left_branch.depth(), right_branch.depth())
        }
    }

    /// Recursively do an idempotent insert.
    pub fn insert(&mut self, data: T)
    where
        T: Default + Ord,
    {
        match self.value.cmp(&data) {
            Ordering::Equal => return,
            Ordering::Greater => {
                match self.left {
                    Some(ref mut left_branch) => left_branch.insert(data),
                    None => self.left = Self::boxer(data),
                }
            },
            Ordering::Less => {
                match self.right {
                    Some(ref mut right_branch) => right_branch.insert(data),
                    None => self.right = Self::boxer(data),
                }
            },
        }
    }

    /// Recursively search for whether value is present.
    pub fn search(&self, data: T) -> bool
    where
        T: Default + Ord,
    {
        match self.value.cmp(&data) {
            Ordering::Equal => true,
            Ordering::Greater => {
                match self.left {
                    Some(ref left_branch) => left_branch.search(data),
                    None => false,
                }
            },
            Ordering::Less => {
                match self.right {
                    Some(ref right_branch) => right_branch.search(data),
                    None => false,
                }
            },
        }
    }

    /// Recursively do an in-order traversal.
    pub fn visit(&self) -> Vec<T>
    where
        T: Copy,
    {
        let mut result: Vec<T> = Vec::new();
        if let Some(ref left_child) = self.left {
            let left_vec = left_child.visit();
            result.extend(left_vec);
        }
        result.push(self.value);
        if let Some(ref right_branch) = self.right {
            let right_vec = right_branch.visit();
            result.extend(right_vec);
        }
        result
    }
}

// TODO: Deal with magic numbers, strings, etc.
// TODO: Figure out cleaner Utils scheme
#[cfg(test)]
mod test {
    use setup_test;

    #[test]
    fn test_depth() {
        setup_test!(root_base,balanced_tree_base,unbalanced_tree_base,);
        let mut tree_base = balanced_tree_base;

        assert_eq!(0, root_base.depth());
        assert_eq!(1, unbalanced_tree_base.depth());
        assert_eq!(1, tree_base.depth());
        tree_base.insert(15);
        assert_eq!(2, tree_base.depth());
        tree_base.insert(35);
        assert_eq!(2, tree_base.depth());
    }

    #[test]
    fn test_insert() {
        setup_test!(root_base,balanced_tree_base,,);
        let mut tree_test = root_base;
        tree_test.insert(25);
        tree_test.insert(75);

        assert_eq!(balanced_tree_base, tree_test);
    }

    #[test]
    fn test_search() {
        setup_test!(,balanced_tree_base,,);

        assert!(balanced_tree_base.search(25));
        assert!(!balanced_tree_base.search(15));
    }

    #[test]
    fn test_visit() {
        setup_test!(,balanced_tree_base,,vec_base);
        let vec_test = balanced_tree_base.visit();

        assert_eq!(vec_base, vec_test);
    }

    // run below with `cargo test -- --ignored --nocapture`

    #[test]
    #[ignore]
    fn test_print() {
        setup_test!(,balanced_tree_base,,);

        println!("arr {:#?}", balanced_tree_base);
    }
}