use super::*;
use super::pick_index;
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

#[test]
fn reverse_string_test() {
    let test_cases = vec![
        (
            vec![],
            vec![],
        ),
        (
            vec!['h', 'e', 'l', 'l', 'o'],
            vec!['o', 'l', 'l', 'e', 'h'],
        ),
    ];
    for mut case in test_cases {
        Solution::reverse_string(&mut case.0);
        assert_eq!(case.0, case.1);
    }
}

#[test]
fn pick_index_test() {
    let mut obj = pick_index::Solution::new(vec![1]);
    // println!("{}", obj.pick_index());

    obj = pick_index::Solution::new(vec![1, 3]);
    // println!("{}", obj.pick_index());
    // println!("{}", obj.pick_index());
    // println!("{}", obj.pick_index());
    // println!("{}", obj.pick_index());
    // println!("{}", obj.pick_index());
}

fn get_reconstruct_queue_test_cases() -> Vec<(Vec<Vec<i32>>, Vec<Vec<i32>>)>{
    vec![
        (
            vec![vec![3, 0], vec![4, 2], vec![6, 1], vec![8, 0]],
            vec![vec![3, 0], vec![8, 0], vec![6, 1], vec![4, 2]],
        ),
        (
            vec![vec![7, 0], vec![4, 4], vec![7, 1], vec![5, 0], vec![6, 1], vec![5, 2]],
            vec![vec![5, 0], vec![7, 0], vec![5, 2], vec![6, 1], vec![4, 4], vec![7, 1]],
        ),
    ]
}

#[test]
fn reconstruct_queue_test() {
    for case in get_reconstruct_queue_test_cases() {
        assert_eq!(Solution::reconstruct_queue(case.0), case.1);
    }
}

#[test]
fn reconstruct_queue_v2_test() {
    for case in get_reconstruct_queue_test_cases() {
        assert_eq!(Solution::reconstruct_queue_v2(case.0), case.1);
    }
}

fn get_change_test_cases() -> Vec<(i32, Vec<i32>, i32)>{
    vec![
        (1, vec![], 0),
        (0, vec![1, 2], 1),
        (1, vec![1, 2], 1),
        (2, vec![1, 2], 2),
        (2, vec![2], 1),
        (10, vec![2], 1),
        (3, vec![1, 2], 2),
        (5, vec![1, 2, 5], 4),
        (500, vec![2, 7, 13], 717),
    ]
}

#[test]
fn change_test() {
    for case in get_change_test_cases() {
        assert_eq!(Solution::change(case.0, case.1), case.2);
    }
}
