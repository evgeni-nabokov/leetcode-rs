use super::*;
use crate::common::tree_node::BinaryTree;

#[test]
fn three_sum_test() {
    assert_eq!(Solution::three_sum(vec![]), vec![] as Vec<Vec<i32>>);
    assert_eq!(Solution::three_sum(vec![0]), vec![] as Vec<Vec<i32>>);
    assert_eq!(Solution::three_sum(vec![0,0,0]), vec![vec![0, 0, 0]]);
    assert_eq!(Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]), vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
}

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
        let tree = TreeNode::create_from_level_order(&case.0);
        assert_eq!(Solution::remove_leaf_nodes(tree, case.1).get_level_order_values(), case.2);
    }
}

