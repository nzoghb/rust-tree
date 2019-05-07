use super::tree::Node;

#[macro_export]
macro_rules! setup_test {
    (
        $($root_base:ident)*,
        $($balanced_tree_base:ident)*,
        $($unbalanced_tree_base:ident)*,
        $($vec_base:ident)*
    ) => {
        use binary_search_tree::utils_test::Utils;

        let setup = Utils::new();
        $(let $root_base = setup.root_base;)*
        $(let $balanced_tree_base = setup.balanced_tree_base;)*
        $(let $unbalanced_tree_base = setup.unbalanced_tree_base;)*
        $(let $vec_base = setup.vec_base;)*
    }
}

//TODO: `struct is never constructed` warning
pub struct Utils {
    pub root_base: Node<i32>,
    pub unbalanced_tree_base: Node<i32>,
    pub balanced_tree_base: Node<i32>,
    pub vec_base: Vec<i32>,
}

impl Utils {
    pub fn new() -> Self {
        Self {
            root_base: Node { value: 50, ..Default::default() },
            unbalanced_tree_base: Node {
                value: 50,
                left: Some(Box::new(Node { value: 25, ..Default::default() })),
                right: None,
            },
            balanced_tree_base: Node {
                value: 50,
                left: Some(Box::new(Node { value: 25, ..Default::default() })),
                right: Some(Box::new(Node { value: 75, ..Default::default() })),
            },
            vec_base: vec![25, 50, 75],
        }
    }
}