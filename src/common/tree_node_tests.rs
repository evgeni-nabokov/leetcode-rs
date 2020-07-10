use crate::common::tree_node::{TreeNode, BinaryTree};

#[test]
fn create_from_level_order_test() {
    let test_cases = vec![
        vec![],
        vec![Some(1)],
        vec![Some(1), Some(2), Some(3)],
        vec![Some(1), None, Some(3)],
        vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)],
    ];
    for case in test_cases {
        let tree = TreeNode::from_level_order(&case);
        assert_eq!(tree.get_level_order_values(), case);
    }
}