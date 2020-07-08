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
fn exist_test() {
    let board = vec![
        vec!['A','B','C','E'],
        vec!['S','F','C','S'],
        vec!['A','D','E','E']
    ];

    let test_cases = vec![
        (
            "ABCCED".to_string(),
            true
        ),
        (
            "SEE".to_string(),
            true
        ),
        (
            "ABCB".to_string(),
            false
        ),
    ];
    for case in test_cases {
        assert_eq!(Solution::exist(board.clone(), case.0), case.1);
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
        )
    ];
    for case in test_cases {
        assert_eq!(Solution::level_order(TreeNode::from_level_order(&case.0)), case.1);
    }
}