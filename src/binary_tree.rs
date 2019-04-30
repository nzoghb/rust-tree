type NodeBox<T> = Option<Box<Node<T>>>;

#[derive(Clone, Debug, Default, PartialEq)]
struct Node<T> {
    value: T,
    left: NodeBox<T>,
    right: NodeBox<T>,
}

impl<T: Default + PartialOrd> From<T> for Node<T> {
    fn from(val: T) -> Self {
        Node::new(val)
    }
}

impl<T: Default + PartialOrd> Node<T> {
    fn new(data: T) -> Node<T> {
        Node { value: data, ..Default::default() }
    }

    fn set_left(mut self, data: T) -> Self
    where
        Node<T>: From<T>,
    {
        self.left = Self::boxer(Self::from(data));
        self
    }

    fn set_right(mut self, data: T) -> Self
    where
        Node<T>: From<T>,
    {
        self.right = Self::boxer(Self::from(data));
        self
    }

    fn set_left_no_return(&mut self, data: T)
    where
        Node<T>: From<T>,
    {
        self.left = Self::boxer(Self::from(data));
    }

    fn set_right_no_return(&mut self, data: T)
    where
        Node<T>: From<T>,
    {
        self.right = Self::boxer(Self::from(data));
    }

    /// Idempotent insert.
    pub fn insert(&mut self, data: T) {
        if data == self.value { return }
        if data < self.value {
            match self.left {
                Some(ref mut left_child) => left_child.insert(data),
                None => self.set_left_no_return(data),
            }
        } else {
            match self.right {
                Some(ref mut right_child) => right_child.insert(data),
                None => self.set_right_no_return(data),
            }
        }
    }
}

impl<T> Node<T> {
    fn boxer(node: Node<T>) -> NodeBox<T> {
        Some(Box::new(node))
    }

    /// Iterator over Node and all its children.
    // pub fn iter(&self) -> NodeIterator<T> {
    //     NodeIterator::new(self)
    // }

    /// Recursive in-order traversal.
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

    // Search if value is present
    // pub fn search(&self, data: T) -> bool {
    //     while let mut node = self {
    //         if data == self.value {
    //             return true
    //         } else if data < self.value {
    //             node = node.left;
    //         } else {
    //             self = node.right;
    //         }
    //     }
    //     false
    // }
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
    fn test_sanity() {
        let setup = Utils::new();
        let tree_base = setup.tree_base;
        let tree_test_one = Utils::root(50)
                    .set_left(25)
                    .set_right(75);
        let mut tree_test_two = Node::new(50);
        tree_test_two.set_left_no_return(25);
        tree_test_two.set_right_no_return(75);

        assert_eq!(tree_base, tree_test_one);
        assert_eq!(tree_base, tree_test_two);
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
    fn test_visit_inorder() {
        let setup = Utils::new();
        let vec_base = setup.vec_base;
        let tree_base = setup.tree_base;
        let vec_test = tree_base.visit();

        assert_eq!(vec_base, vec_test);
    }

    /// run below with `cargo test -- --ignored --nocapture`

    #[test]
    #[ignore]
    fn test_print_one() {
        let tree = Utils::root(1)
                    .set_left(2)
                    .set_right(3);

        println!("arr {:#?}", tree);
    }

    #[test]
    #[ignore]
    fn test_print_two() {
        let mut tree = Node::new("root");
        tree.set_left_no_return("left");
        tree.set_right_no_return("right");

        println!("arr {:#?}", tree);
    }
}

/// Iterator type for a binary tree.
/// This is a generator that progresses through an in-order traversal.
struct NodeIterator<T> {
    branch_stack: Vec<Node<T>>,
}

impl<T: std::fmt::Display> NodeIterator<T>
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
        while let Node { left: Some(left_branch), .. } = root.clone() {
            // println!("{}", root.clone().value);
            root = *left_branch;
            self.branch_stack.push(root.clone());
        }
    }
}

impl<T: std::fmt::Display> Iterator for NodeIterator<T>
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

impl<T: std::fmt::Display> IntoIterator for Node<T>
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