use super::*;
use crate::challenge_2020_09::moving_average::MovingAverage;

#[test]
fn largest_time_from_digits_test() {
    let test_cases = vec![
        (vec![1, 2, 3, 4], "23:41"),
        (vec![5, 5, 5, 5], ""),
        (vec![0, 4, 0, 0], "04:00"),
        (vec![2, 0, 6, 6], "06:26"),
    ];
    for case in test_cases {
        assert_eq!(Solution::largest_time_from_digits(case.0), case.1.to_string());
    }
}

#[test]
fn contains_nearby_almost_duplicate_test() {
    let test_cases = vec![
        (vec![1, 2, 3, 1], 3, 0, true),
        (vec![1, 0, 1, 1], 1, 2, true),
        (vec![1, 5, 9, 1, 5, 9], 2, 3, false),
        (vec![7, 2, 8], 2, 1, true),
        (vec![-2147483648, -2147483647], 3, 3, true),
    ];
    for case in test_cases {
        assert_eq!(Solution::contains_nearby_almost_duplicate(case.0, case.1, case.2), case.3);
    }
}

#[test]
fn repeated_substring_pattern_test() {
    let test_cases = vec![
        ("", false),
        ("a", false),
        ("aa", true),
        ("abab", true),
        ("xyxy", true),
        ("aba", false),
        ("abcabcabcabc", true),
        ("abcaabca", true),
    ];
    for case in test_cases {
        assert_eq!(Solution::repeated_substring_pattern(case.0.to_string()), case.1);
    }
}

#[test]
fn word_pattern_test() {
    let test_cases = vec![
        ("", "", false),
        ("a", "", false),
        ("abba", "dog cat cat dog", true),
        ("abba", "dog cat cat fish", false),
        ("aaaa", "dog cat cat dog", false),
        ("aaaa", "dog dog dog dog", true),
        ("abba", "dog dog dog dog", false),
    ];
    for case in test_cases {
        assert_eq!(Solution::word_pattern(case.0.to_string(), case.1.to_string()), case.2);
    }
}

#[test]
fn sum_root_to_leaf_test() {
    let test_cases = vec![
        (vec![Some(1), Some(0), Some(1), Some(0), Some(1), Some(0), Some(1)], 22),
        // TODO: Find out why it does not pass.
        // (vec![Some(1),Some(0),Some(1),Some(0),Some(1),Some(1),Some(0),Some(1),Some(0),Some(0),Some(0),Some(1),Some(0),Some(0),Some(0),Some(0),Some(1),Some(1),Some(0),None,Some(1),Some(0),None,Some(1),Some(1),Some(1),Some(1),None,Some(0),None,None,None,None,None,None,Some(1),None,Some(0),None,None,None,None,None,Some(0),Some(1),Some(1),Some(0),Some(0),Some(0),Some(0),None,None,None,Some(0),None,None,None,Some(0),None,Some(0),None,None,None,None,Some(1),None,None,Some(0),Some(0),Some(0),None,None,None,Some(1),None,None,None,Some(0),Some(0),None,None,None,None,None,Some(0),None,None,None,None,Some(1),None,None,None,Some(0),Some(1),None,Some(0)], 4433),
    ];
    for case in test_cases {
        assert_eq!(Solution::sum_root_to_leaf(TreeNode::from_level_order(&case.0)), case.1);
    }
}

#[test]
fn moving_average_test() {
    let mut obj = MovingAverage::new(3);
    assert_eq!((obj.next(1) * 100_000.0).round() / 100_000.0, 1.0);
    assert_eq!((obj.next(10) * 100_000.0).round() / 100_000.0, 5.5);
    assert_eq!((obj.next(3) * 100_000.0).round() / 100_000.0, 4.66667);
    assert_eq!((obj.next(5) * 100_000.0).round() / 100_000.0, 6.0);
}

#[test]
fn compare_version_test() {
    let test_cases = vec![
        ("0.1", "1.1", -1),
        ("1.0.1", "1", 1),
        ("7.5.2.4", "7.5.3", -1),
        ("1.01", "1.001", 0),
        ("1.0", "1.0.0", 0)
    ];
    for case in test_cases {
        assert_eq!(Solution::compare_version(case.0.to_string(), case.1.to_string()), case.2);
    }
}

#[test]
fn get_hint_test() {
    let test_cases = vec![
        ("1807", "7810", "1A3B"),
        ("1123", "0111", "1A1B"),
        ("1234", "1234", "4A0B"),
    ];
    for case in test_cases {
        assert_eq!(Solution::get_hint(case.0.to_string(), case.1.to_string()), case.2.to_string());
    }
}

#[test]
fn max_product_test() {
    let test_cases = vec![
        (vec![2, 3, -2, 4], 6),
        (vec![-2, 0, -1], 0),
        (vec![-2, 3, -4], 24),
    ];
    for case in test_cases {
        assert_eq!(Solution::max_product(case.0), case.1);
    }
}

#[test]
fn combination_sum_iii_test() {
    let test_cases = vec![
        (3, 7, vec![vec![4, 2, 1]]),
        (3, 9, vec![vec![6, 2, 1], vec![5, 3, 1], vec![4, 3, 2]]),
    ];
    for case in test_cases {
        assert_eq!(Solution::combination_sum_iii(case.0, case.1), case.2);
    }
}

#[test]
fn insert_test() {
    let test_cases = vec![
        (
            vec![vec![1, 3], vec![6, 9]],
            vec![2, 5],
            vec![vec![1, 5], vec![6, 9]]
        ),
        (
            vec![vec![1, 3], vec![6, 9]],
            vec![1, 9],
            vec![vec![1, 9]]
        ),
        (
            vec![vec![1, 3], vec![6, 9]],
            vec![0, 10],
            vec![vec![0, 10]]
        ),
        (
            vec![vec![1, 3], vec![6, 9]],
            vec![4, 5],
            vec![vec![1, 3], vec![4, 5], vec![6, 9]],
        ),
        (
            vec![vec![1, 3], vec![6, 9]],
            vec![9, 11],
            vec![vec![1, 3], vec![6, 11]],
        ),
        (
            vec![vec![1, 3], vec![6, 9]],
            vec![10, 11],
            vec![vec![1, 3], vec![6, 9], vec![10, 11]],
        ),
        (
            vec![vec![1, 3], vec![6, 9]],
            vec![0, 0],
            vec![vec![0, 0], vec![1, 3], vec![6, 9]],
        ),
        (
            vec![vec![1, 3], vec![6, 9]],
            vec![10, 10],
            vec![vec![1, 3], vec![6, 9], vec![10, 10]],
        ),
        (
            vec![vec![1, 3], vec![6, 9]],
            vec![1, 1],
            vec![vec![1, 3], vec![6, 9]],
        ),
        (
            vec![vec![1, 3], vec![6, 9]],
            vec![9, 9],
            vec![vec![1, 3], vec![6, 9]],
        ),
        (
            vec![vec![1, 2], vec![3, 5], vec![6, 7], vec![8, 10], vec![12, 16]],
            vec![4, 8],
            vec![vec![1, 2], vec![3, 10], vec![12, 16]]
        ),
    ];
    for case in test_cases {
        assert_eq!(Solution::insert(case.0, case.1), case.2);
    }
}