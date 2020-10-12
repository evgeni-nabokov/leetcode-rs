use crate::common::tree_node::BinaryTree;
use crate::common::linked_list::LinkedList;

use super::*;
use super::recent_counter::RecentCounter;
use super::two_sum::TwoSum;

#[test]
fn max_distance_test() {
    let test_cases = vec![
        (
            vec![
                vec![1, 2, 3],
                vec![4, 5],
                vec![1, 2, 3]
            ],
            4
        ),
        (
            vec![
                vec![-1, 2, 3],
                vec![4, 5],
                vec![-1, 2, 3]
            ],
            6
        ),
        (
            vec![
                vec![-8, -7, -7, -5, 1, 1, 3, 4],
                vec![-2],
                vec![-10, -10, -7, 0, 1, 3],
                vec![2]
            ],
            14
        ),
        (
            vec![
                vec![1, 2, 3, 4, 5, 6],
                vec![3, 4],
                vec![4, 5],
            ],
            4
        ),
    ];
    for case in test_cases {
        assert_eq!(Solution::max_distance(case.0), case.1);
    }
}

#[test]
fn recent_counter_test() {
    let test_cases = vec![
        (1, 1),
        (100, 2),
        (3001, 3),
        (3002, 3),
    ];

    let mut obj = RecentCounter::new();
    for case in test_cases {
        assert_eq!(obj.ping(case.0), case.1);
    }
}

#[test]
fn find_pairs_test() {
    let test_cases = vec![
        (vec![1, 1, 1, 1, 1], 0, 1),
        (vec![3, 1, 4, 1, 5], 2, 2),
        (vec![1, 2, 3, 4, 5], 1, 4),
        (vec![1, 3, 1, 5, 4], 0, 1),
        (vec![1, 2, 4, 4, 3, 3, 0, 9, 2, 3], 3, 2),
        (vec![-1, -2, -3], 1, 2),
    ];

    for case in test_cases {
        assert_eq!(Solution::find_pairs(case.0, case.1), case.2);
    }
}

#[test]
fn remove_covered_intervals_test() {
    let test_cases = vec![
        (vec![vec![1, 4], vec![3, 6], vec![2, 8]], 2),
        (vec![vec![1, 4], vec![2, 3]], 1),
        (vec![vec![0, 10], vec![5, 12]], 2),
        (vec![vec![3, 10], vec![4, 10], vec![5, 11]], 2),
        (vec![vec![1, 2], vec![1, 4], vec![3, 4]], 1),
    ];

    for case in test_cases {
        assert_eq!(Solution::remove_covered_intervals(case.0), case.1);
    }
}

#[test]
fn insert_into_bst_test() {
    let test_cases = vec![
        (
            vec![Some(4), Some(2), Some(7), Some(1), Some(3)],
            5,
            vec![Some(4), Some(2), Some(7), Some(1), Some(3), Some(5), None],
        ),
        (
            vec![Some(40), Some(20), Some(60), Some(10), Some(30), Some(50), Some(70)],
            25,
            vec![Some(40), Some(20), Some(60), Some(10), Some(30), Some(50), Some(70), None, None, Some(25), None, None, None, None, None],
        ),
        (
            vec![Some(4), Some(2), Some(7), Some(1), Some(3), None, None, None, None, None, None],
            5,
            vec![Some(4), Some(2), Some(7), Some(1), Some(3), Some(5), None],
        ),
    ];

    for case in test_cases {
        assert_eq!(Solution::insert_into_bst(TreeNode::from_level_order(&case.0), case.1).get_level_order_values(), case.2);
    }
}

#[test]
fn rotate_right_test() {
    let test_cases = vec![
        (vec![], 4, vec![]),
        (vec![0], 4, vec![0]),
        (vec![0, 1, 2], 0, vec![0, 1, 2]),
        (vec![0, 1, 2], 6, vec![0, 1, 2]),
        (vec![1, 2, 3, 4, 5], 2, vec![4, 5, 1, 2, 3]),
        (vec![0, 1, 2], 4, vec![2, 0, 1]),
    ];

    for case in test_cases {
        assert_eq!(Solution::rotate_right(ListNode::from_slice(&case.0), case.1).to_vec(), case.2);
    }
}

#[test]
fn search_test() {
    let test_cases = vec![
        (vec![], 1, -1),
        (vec![1], 1, 0),
        (vec![-1], 1, -1),
        (vec![5], -5, -1),
        (vec![-1, 0, 3, 5, 9, 12], 9, 4),
        (vec![-1, 0, 3, 5, 9, 12], 2, -1),
    ];

    for case in test_cases {
        assert_eq!(Solution::search(case.0, case.1), case.2);
    }
}

#[test]
fn two_sum_test() {
    let mut obj = TwoSum::new();
    obj.add(0);
    assert_eq!(obj.find(0), false);
    obj.add(1);
    obj.add(3);
    obj.add(5);
    assert_eq!(obj.find(4), true);
    assert_eq!(obj.find(7), false);
}

#[test]
fn find_min_arrow_shots_test() {
    let test_cases = vec![
        (vec![], 0),
        (vec![vec![0, 1]], 1),
        (vec![vec![1, 2], vec![4, 5], vec![1, 5]], 2),
        (vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]], 2),
        (vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]], 4),
        (vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]], 2),
        (vec![vec![9, 12], vec![1, 10], vec![4, 11], vec![8, 12], vec![3, 9], vec![6, 9], vec![6, 7]], 2)
    ];

    for case in test_cases {
        assert_eq!(Solution::find_min_arrow_shots(case.0), case.1);
    }
}

#[test]
fn remove_duplicate_letters_test() {
    let test_cases = vec![
        ("bcabc", "abc"),
        ("cbacdcbc", "acdb"),
        ("abc", "abc"),
    ];

    for case in test_cases {
        assert_eq!(Solution::remove_duplicate_letters(case.0.to_string()), case.1.to_string());
    }
}

#[test]
fn buddy_strings_test() {
    let test_cases = vec![
        ("", "", false),
        ("ab", "ba", true),
        ("ab", "ab", false),
        ("aa", "aa", true),
        ("aab", "aab", true),
        ("abac", "abad", false),
        ("aaaaaaabc", "aaaaaaacb", true),
        ("", "aa", false),
    ];

    for case in test_cases {
        assert_eq!(Solution::buddy_strings(case.0.to_string(), case.1.to_string()), case.2);
    }
}