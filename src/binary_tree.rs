type NodeBox<T> = Option<Box<Node<T>>>;

#[derive(Debug, Default, PartialEq)]
struct Node<T> {
    value: T,
    left: NodeBox<T>,
    right: NodeBox<T>
}

impl<T: Default + PartialOrd> From<T> for Node<T> {
    fn from(val: T) -> Self {
        Node::new(val)
    }
}

impl<T: Default + PartialOrd> Node<T> {
    fn new(val: T) -> Node<T> {
        Node { value: val, ..Default::default() }
    }

    fn boxer(node: Node<T>) -> NodeBox<T> {
        Some(Box::new(node))
    }

    fn left(mut self, node: T) -> Self
    where
        Node<T>: From<T>,
    {
        self.left = Self::boxer(Self::from(node));
        self
    }

    fn right(mut self, node: T) -> Self
    where
        Node<T>: From<T>,
    {
        self.right = Self::boxer(Self::from(node));
        self
    }

    fn left_node(mut self, node: Node<T>) -> Self {
        self.left = Self::boxer(node);
        self
    }

    fn right_node(mut self, node: Node<T>) -> Self {
        self.right = Self::boxer(node);
        self
    }

    fn left_set(&mut self, node: Node<T>) {
        self.left = Self::boxer(node);
    }

    fn right_set(&mut self, node: Node<T>) {
        self.right = Self::boxer(node);
    }

    fn insert(&mut self, data: T) {
        if data < self.value {
            match self.left {
                Some(ref mut left_child) => left_child.insert(data),
                None => self.left_set(Self::new(data)),
            }
        } else {
            match self.right {
                Some(ref mut right_child) => right_child.insert(data),
                None => self.right_set(Self::new(data)),
            }
        }
    }

    fn visit_inorder(&self) -> Vec<&T> {
        let mut result: Vec<&T> = Vec::new();
        if let Some(ref left_child) = self.left {
            let left_vec = left_child.visit_inorder();
            result.extend(left_vec);
        }
        result.push(&self.value);
        if let Some(ref right_child) = self.right {
            let right_vec = right_child.visit_inorder();
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
    fn test_sanity() {
        let setup = Utils::new();
        let tree_base = setup.tree_base;
        let tree_test_one = Utils::root(50)
                    .left(25)
                    .right(75);
        let tree_test_two = Node::new(50)
                    .left_node(Node::new(25))
                    .right_node(Node::new(75));
        let mut tree_test_three = Node::new(50);
        tree_test_three.left_set(Node::new(25));
        tree_test_three.right_set(Node::new(75));

        assert_eq!(tree_base, tree_test_one);
        assert_eq!(tree_base, tree_test_two);
        assert_eq!(tree_base, tree_test_three);
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
        let vec_test = tree_base.visit_inorder();

        assert_eq!(vec_base, vec_test);
    }

    /// run below with `cargo test -- --ignored --nocapture`

    #[test]
    #[ignore]
    fn test_print_one() {
        let tree = Utils::root(1)
                    .left(2)
                    .right(3);

        println!("arr {:#?}", tree);
    }

    #[test]
    #[ignore]
    fn test_print_two() {
        let tree = Node::new(1.0)
                    .left_node(Node::new(1.0))
                    .right_node(Node::new(1.0));

        println!("arr {:#?}", tree);
    }

    #[test]
    #[ignore]
    fn test_print_three() {
        let mut tree = Node::new("root");
        tree.left_set(Node::new("left"));
        tree.right_set(Node::new("right"));

        println!("arr {:#?}", tree);
    }
}