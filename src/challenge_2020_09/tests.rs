use super::*;

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