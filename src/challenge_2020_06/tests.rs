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

fn get_two_city_sched_cost_test_cases() -> Vec<(Vec<Vec<i32>>, i32)>{
    vec![
        (vec![vec![10,10], vec![20,20], vec![30,30], vec![40,40], vec![50,50]], 150),
        (vec![vec![10,20], vec![30,200], vec![400,50], vec![30,20]], 110),
        (vec![vec![20,10], vec![200,30], vec![400,50], vec![30,20]], 130),
    ]
}

#[test]
fn two_city_sched_cost_test() {
    for case in get_two_city_sched_cost_test_cases() {
        assert_eq!(Solution::two_city_sched_cost(case.0), case.1);
    }
}