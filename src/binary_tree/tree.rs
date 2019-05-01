pub type NodeBox<T> = Option<Box<Node<T>>>;

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

impl<T: Default + PartialOrd> Node<T> {
    /// Recursively do an idempotent insert.
    pub fn insert(&mut self, data: T) {
        if data == self.value { return }
        if data < self.value {
            match self.left {
                Some(ref mut left_child) => left_child.insert(data),
                None => self.left = Self::boxer(data),
            }
        } else {
            match self.right {
                Some(ref mut right_child) => right_child.insert(data),
                None => self.right = Self::boxer(data),
            }
        }
    }

    /// Recursively search for whether value is present.
    pub fn search(&self, data: T) -> bool {
        if data == self.value { return true }
        if data < self.value {
            match self.left {
                Some(ref left_child) => left_child.search(data),
                None => false,
            }
        } else {
            match self.right {
                Some(ref right_child) => right_child.search(data),
                None => false,
            }
        }
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

    /// Recursively do an in-order traversal.
    pub fn visit(&self) -> Vec<&T> {
        let mut result: Vec<&T> = Vec::new();
        if let Some(ref left_child) = self.left {
            let left_vec = left_child.visit();
            result.extend(left_vec);
        }
        result.push(&self.value);
        if let Some(ref right_child) = self.right {
            let right_vec = right_child.visit();
            result.extend(right_vec);
        }
        result
    }
}


// TODO: Deal with magic numbers, strings, etc.
// TODO: Figure out cleaner Utils scheme
#[cfg(test)]
mod tests {
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

        fn root<T: Default + PartialOrd>(value: T) -> Node<T> {
            Node::new(value)
        }
    }

    #[test]
    fn test_insert() {
        let setup = Utils::new();
        let tree_base = setup.tree_base;
        let mut tree_test = Utils::root(50);
        tree_test.insert(25);
        tree_test.insert(75);

        assert_eq!(tree_base, tree_test);
    }

    #[test]
    fn test_search() {
        let setup = Utils::new();
        let tree_base = setup.tree_base;

        assert!(tree_base.search(25));
        assert!(!tree_base.search(15));
    }

    #[test]
    fn test_depth() {
        let setup = Utils::new();
        let mut tree_base = setup.tree_base;
        let root_base = Utils::root(1);

        assert_eq!(1, tree_base.depth());
        assert_eq!(0, root_base.depth());
        tree_base.insert(15);
        assert_eq!(2, tree_base.depth());
        tree_base.insert(35);
        assert_eq!(2, tree_base.depth());
    }

    #[test]
    fn test_visit() {
        let setup = Utils::new();
        let tree_base = setup.tree_base;
        let vec_base = setup.vec_base;
        let vec_test = tree_base.visit();

        assert_eq!(vec_base, vec_test);
    }

    /// run below with `cargo test -- --ignored --nocapture`

    #[test]
    #[ignore]
    fn test_print() {
        let mut tree = Utils::root(1);
        tree.insert(2);
        tree.insert(3);

        println!("arr {:#?}", tree);
    }
}