use super::*;

#[test]
fn can_attend_meetings_test() {
    let test_cases = vec![
        (vec![vec![0, 30], vec![5, 10], vec![15, 20]], false),
        (vec![vec![7, 10], vec![2, 4]], true),
    ];
    for case in test_cases {
        assert_eq!(Solution::can_attend_meetings(case.0), case.1);
    }
}

#[test]
fn get_decimal_value_test() {
    let test_cases = vec![
        (vec![1, 0, 1], 5),
        (vec![0], 0),
        (vec![1], 1),
        (vec![0, 0], 0),
        (vec![1, 0, 0, 1, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0], 18880),
    ];
    for case in test_cases {
        assert_eq!(
            Solution::get_decimal_value(ListNode::from_slice(&case.0)),
            case.1
        );
    }
}

#[test]
fn max_power_test() {
    let test_cases = vec![
        ("cc", 2),
        ("leetcode", 2),
        ("abbcccddddeeeeedcba", 5),
        ("triplepillooooow", 5),
        ("hooraaaaaaaaaaay", 11),
        ("tourist", 1),
    ];
    for case in test_cases {
        assert_eq!(Solution::max_power(case.0.to_string()), case.1);
    }
}

#[test]
fn find_min_height_trees_test() {
    let test_cases = vec![
        (1, vec![], vec![0]),
        (2, vec![], vec![0, 1]),
        (4, vec![vec![1, 0], vec![1, 2], vec![1, 3]], vec![1]),
        (
            6,
            vec![vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 4], vec![5, 4]],
            vec![3, 4],
        ),
    ];
    for case in test_cases {
        assert_eq!(Solution::find_min_height_trees(case.0, case.1), case.2);
    }
}

#[test]
fn find_tilt_test() {
    let test_cases = vec![
        (vec![Some(1), Some(2), Some(3)], 1),
        (
            vec![Some(4), Some(2), Some(9), Some(3), Some(5), None, Some(7)],
            15,
        ),
    ];
    for case in test_cases {
        assert_eq!(
            Solution::find_tilt(TreeNode::from_level_order(&case.0)),
            case.1
        );
    }
}

fn get_two_sum_less_than_k_test_cases() -> Vec<(Vec<i32>, i32, i32)> {
    vec![
        (vec![0, 0, 1], 1, 0),
        (vec![0, 1, 1], 1, -1),
        (vec![34, 23, 1, 24, 75, 33, 54, 8], 60, 58),
        (vec![10, 20, 30], 15, -1),
        (
            vec![
                803, 468, 292, 154, 924, 424, 197, 277, 753, 86, 984, 144, 105, 450, 287, 265, 655,
                404, 407, 794, 513, 976, 241, 272, 84, 503, 65, 654, 805, 413, 362, 907, 297, 473,
                113, 567, 646, 607, 806, 674, 424, 753, 870, 574, 765, 170, 603, 696, 513, 58,
            ],
            300,
            299,
        ),
    ]
}

#[test]
fn two_sum_less_than_k_test() {
    for case in get_two_sum_less_than_k_test_cases() {
        assert_eq!(Solution::two_sum_less_than_k(case.0, case.1), case.2);
    }
}

#[test]
fn two_sum_less_than_k_v2_test() {
    for case in get_two_sum_less_than_k_test_cases() {
        assert_eq!(Solution::two_sum_less_than_k_v2(case.0, case.1), case.2);
    }
}

#[test]
fn flip_and_invert_image_test() {
    let test_cases = vec![
        (
            vec![vec![1, 1, 0], vec![1, 0, 1], vec![0, 0, 0]],
            vec![vec![1, 0, 0], vec![0, 1, 0], vec![1, 1, 1]],
        ),
        (
            vec![
                vec![1, 1, 0, 0],
                vec![1, 0, 0, 1],
                vec![0, 1, 1, 1],
                vec![1, 0, 1, 0],
            ],
            vec![
                vec![1, 1, 0, 0],
                vec![0, 1, 1, 0],
                vec![0, 0, 0, 1],
                vec![1, 0, 1, 0],
            ],
        ),
    ];
    for case in test_cases {
        assert_eq!(Solution::flip_and_invert_image(case.0), case.1);
    }
}

#[test]
fn valid_square_test() {
    let test_cases = vec![
        (vec![0, 0], vec![1, 1], vec![1, 0], vec![0, 1], true),
        (vec![0, 1], vec![1, 2], vec![2, 1], vec![1, 0], true),
        (vec![1, 0], vec![-1, 0], vec![0, -1], vec![0, 1], true),
        (vec![0, 0], vec![0, 0], vec![0, 0], vec![0, 0], false),
    ];
    for case in test_cases {
        assert_eq!(
            Solution::valid_square(case.0, case.1, case.2, case.3),
            case.4
        );
    }
}

fn get_range_sum_bst_test_cases() -> Vec<(Vec<Option<i32>>, i32, i32, i32)> {
    vec![
        (
            vec![
                Some(10),
                Some(5),
                Some(15),
                Some(3),
                Some(7),
                None,
                Some(18),
            ],
            7,
            15,
            32,
        ),
        (
            vec![
                Some(10),
                Some(5),
                Some(15),
                Some(3),
                Some(7),
                Some(13),
                Some(18),
                Some(1),
                None,
                Some(6),
            ],
            6,
            10,
            23,
        ),
    ]
}

#[test]
fn range_sum_bst_test() {
    for case in get_range_sum_bst_test_cases() {
        assert_eq!(
            Solution::range_sum_bst(TreeNode::from_level_order(&case.0), case.1, case.2),
            case.3
        );
    }
}

#[test]
fn range_sum_bst_v2_test() {
    for case in get_range_sum_bst_test_cases() {
        assert_eq!(
            Solution::range_sum_bst_v2(TreeNode::from_level_order(&case.0), case.1, case.2),
            case.3
        );
    }
}

fn get_remove_interval_test_cases() -> Vec<(Vec<Vec<i32>>, Vec<i32>, Vec<Vec<i32>>)> {
    vec![
        (vec![vec![0, 5]], vec![5, 6], vec![vec![0, 5]]),
        (vec![vec![0, 5]], vec![-1, 0], vec![vec![0, 5]]),
        (
            vec![vec![0, 2], vec![3, 4], vec![5, 7]],
            vec![1, 6],
            vec![vec![0, 1], vec![6, 7]],
        ),
        (
            vec![
                vec![-5, -4],
                vec![-3, -2],
                vec![1, 2],
                vec![3, 5],
                vec![8, 9],
            ],
            vec![-1, 4],
            vec![vec![-5, -4], vec![-3, -2], vec![4, 5], vec![8, 9]],
        ),
        (vec![vec![0, 100]], vec![0, 50], vec![vec![50, 100]]),
        (vec![vec![0, 100]], vec![50, 100], vec![vec![0, 50]]),
    ]
}

#[test]
fn remove_interval_test() {
    for case in get_remove_interval_test_cases() {
        assert_eq!(Solution::remove_interval(case.0, case.1), case.2);
    }
}

#[test]
fn remove_interval_v2_test() {
    for case in get_remove_interval_test_cases() {
        assert_eq!(Solution::remove_interval_v2(case.0, case.1), case.2);
    }
}

#[test]
fn merge_test() {
    let test_cases = vec![
        (
            vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]],
            vec![vec![1, 6], vec![8, 10], vec![15, 18]],
        ),
        (vec![vec![1, 4], vec![4, 5]], vec![vec![1, 5]]),
        (vec![vec![1, 4], vec![0, 4]], vec![vec![0, 4]]),
    ];
    for case in test_cases {
        assert_eq!(Solution::merge(case.0), case.1);
    }
}

#[test]
fn search_test() {
    let test_cases = vec![
        (vec![2, 5, 6, 0, 0, 1, 2], 0, true),
        (vec![2, 5, 6, 0, 0, 1, 2], 3, false),
        (vec![1, 3, 1, 1, 1], 3, true),
        (vec![1, 3], 0, false),
        (vec![3, 1], 0, false),
    ];
    for case in test_cases {
        assert_eq!(Solution::search(case.0, case.1), case.2);
    }
}

#[test]
fn unique_morse_representations_test() {
    let test_cases = vec![(vec!["gin", "zen", "gig", "msg"], 2)];
    for case in test_cases {
        assert_eq!(
            Solution::unique_morse_representations(
                case.0.into_iter().map(|x| x.to_string()).collect()
            ),
            case.1
        );
    }
}

#[test]
fn calculate_test() {
    let test_cases = vec![("3+2*2", 7), ("3 / 2", 1), (" 3+5 / 2 ", 5)];
    for case in test_cases {
        assert_eq!(Solution::calculate(case.0.to_string()), case.1);
    }
}
