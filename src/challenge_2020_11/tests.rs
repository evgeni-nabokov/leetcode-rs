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
        assert_eq!(Solution::get_decimal_value(ListNode::from_slice(&case.0)), case.1);
    }
}

#[test]
fn max_power_test() {
    let test_cases = vec![
        ("сс", 2),
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
        (6, vec![vec![3,0], vec![3,1], vec![3,2], vec![3,4], vec![5,4]], vec![3, 4]),
    ];
    for case in test_cases {
        assert_eq!(Solution::find_min_height_trees(case.0, case.1), case.2);
    }
}

#[test]
fn find_tilt_test() {
    let test_cases = vec![
        (vec![Some(1), Some(2), Some(3)], 1),
        (vec![Some(4), Some(2), Some(9), Some(3), Some(5), None, Some(7)], 15),
    ];
    for case in test_cases {
        assert_eq!(Solution::find_tilt(TreeNode::from_level_order(&case.0)), case.1);
    }
}
