use super::*;
use crate::common::tree_node::{BinaryTree, TreeNode};

#[test]
fn invert_tree_test() {
    let test_cases = vec![
        (
            vec![Some(4), Some(2), Some(7), Some(1), Some(3), Some(6), Some(9)],
            vec![Some(4), Some(7), Some(2), Some(9), Some(6), Some(3), Some(1)]
        ),
        (
            vec![Some(1), Some(2)],
            vec![Some(1), None, Some(2)],
        ),
    ];
    for case in test_cases {
        let tree = TreeNode::create_from_level_order(&case.0);
        assert_eq!(Solution::invert_tree(tree).get_level_order_values(), case.1);
    }
}