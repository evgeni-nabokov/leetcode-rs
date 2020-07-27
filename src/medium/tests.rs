use super::*;
use crate::common::tree_node::BinaryTree;

#[test]
fn h_index_test() {
    let test_cases = vec![
        (vec![], 0),
        (vec![0], 0),
        (vec![1], 1),
        (vec![10], 1),
        (vec![10, 10], 2),
        (vec![0, 0, 0, 0], 0),
        (vec![1, 1, 1, 1], 1),
        (vec![2, 2, 2], 2),
        (vec![3, 3, 3], 3),
        (vec![2, 1, 1, 1], 1),
        (vec![6, 3, 1, 5, 0], 3),
        (vec![0, 6, 4, 5, 4], 4),
        (vec![10, 14, 0, 12, 10, 1], 4),
        (vec![14, 1, 1, 12, 1, 0], 2),
    ];
    for case in test_cases {
        assert_eq!(Solution::h_index(case.0), case.1);
    }
}

#[test]
fn remove_leaf_nodes_test() {
    let test_cases = vec![
        (
            vec![Some(1)],
            1,
            vec![]
        ),
        (
            vec![Some(1), Some(1), None],
            1,
            vec![]
        ),
        (
            vec![Some(1), Some(1), Some(1)],
            1,
            vec![]
        ),
        (
            vec![Some(1), Some(1), Some(2)],
            1,
            vec![Some(1), None, Some(2)]
        ),
        (
            vec![Some(1), Some(2), Some(3), Some(2), None, Some(2), Some(4)],
            2,
            vec![Some(1), None, Some(3), None, Some(4)]
        ),
        (
            vec![Some(1), Some(3), Some(3), Some(3), Some(2)],
            3,
            vec![Some(1), Some(3), None, None, Some(2)]
        ),
        (
            vec![Some(1), Some(2), None, Some(2), None, Some(2)],
            2,
            vec![Some(1)]
        ),
        (
            vec![Some(1), Some(1), Some(1)],
            1,
            vec![]
        ),
        (
            vec![Some(1), Some(2), Some(3)],
            1,
            vec![Some(1), Some(2), Some(3)],
        ),
    ];
    for case in test_cases {
        let tree = TreeNode::from_level_order(&case.0);
        assert_eq!(Solution::remove_leaf_nodes(tree, case.1).get_level_order_values(), case.2);
    }
}

#[test]
fn level_order_test() {
    let test_cases = vec![
        (
            vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)],
            vec![
                vec![3],
                vec![9, 20],
                vec![15, 7],
            ]
        ),
        (
            vec![Some(1), Some(2), Some(3), Some(4), None, None, Some(5)],
            vec![
                vec![1],
                vec![2, 3],
                vec![4, 5],
            ]
        ),
    ];
    for case in test_cases {
        assert_eq!(Solution::level_order(TreeNode::from_level_order(&case.0)), case.1);
    }
}

#[test]
fn build_tree_test() {
    let test_cases = vec![
        (vec![], vec![], vec![]),
        (vec![1, 2], vec![2, 1], vec![Some(1), Some(2), None]),
        (vec![5, 3, 2, 4, 6], vec![2, 4, 3, 6, 5], vec![Some(5), Some(3), Some(6), Some(2), Some(4), None, None]),
        (vec![1, 2, 4, 5, 3, 6, 7], vec![4, 5, 2, 6, 7, 3, 1], vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7)])
    ];
    for case in test_cases {
        assert_eq!(Solution::build_tree(case.0, case.1).get_level_order_values(), case.2);
    }
}

#[test]
fn build_tree_ii_test() {
    let test_cases = vec![
        (vec![], vec![], vec![]),
        (vec![1, 2], vec![2, 1], vec![Some(1), Some(2), None]),
        (vec![5, 3, 2, 4, 6], vec![2, 3, 4, 5, 6], vec![Some(5), Some(3), Some(6), Some(2), Some(4), None, None]),
        (vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7], vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)])
    ];
    for case in test_cases {
        assert_eq!(Solution::build_tree_ii(case.0, case.1).get_level_order_values(), case.2);
    }
}
