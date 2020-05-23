use crate::common::tree_node::{TreeNode, BinaryTree};

#[test]
fn create_from_level_order_test() {
    let test_cases = vec![
        vec![Some(1)],
        vec![Some(1), Some(2), Some(3)],
        vec![Some(1), None, Some(3)],
    ];
    for case in test_cases {
        let tree = TreeNode::create_from_level_order(&case);
        assert_eq!(tree.get_level_order_values(), case);
    }
}