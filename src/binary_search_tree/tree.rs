use std::cmp::Ordering;

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
    /// Iteratively do an idempotent insert.
    pub fn insert(&mut self, data: T)
    where
        T: Default + Ord,
    {
        let mut next_node = self;
        loop {
            let current_node = next_node;
            let target_node = match current_node.value.cmp(&data) {
                Ordering::Equal => break,
                Ordering::Greater => &mut current_node.left,
                Ordering::Less => &mut current_node.right,
            };
            match target_node {
                Some(branch) => next_node = branch,
                None => {
                    *target_node = Self::boxer(data);
                    return
                },
            }
        }
    }
}

// TODO: Deal with magic numbers, strings, etc.
// TODO: Figure out cleaner Utils scheme
#[cfg(test)]
mod test {
    use setup_test;

     #[test]
    fn test_insert() {
        setup_test!(root_base,balanced_tree_base,unbalanced_tree_base,);
        let mut balanced_tree_test = root_base.clone();
        balanced_tree_test.insert(25);
        balanced_tree_test.insert(75);

        assert_eq!(balanced_tree_base, balanced_tree_test);

        let mut unbalanced_tree_test = root_base;
        unbalanced_tree_test.insert(25);
        unbalanced_tree_test.insert(0);

        assert_eq!(unbalanced_tree_base, unbalanced_tree_test);
    }

    #[test]
    #[ignore]
    fn test_print() {
        setup_test!(,balanced_tree_base,,);

        println!("arr {:#?}", balanced_tree_base);
    }
}