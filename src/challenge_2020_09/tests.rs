use super::*;
use super::moving_average::MovingAverage;

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

#[test]
fn rob_test() {
    let test_cases = vec![
        (vec![], 0),
        (vec![1], 1),
        (vec![1, 2, 3, 1], 4),
        (vec![2, 7, 9, 3, 1], 12),
        (vec![2, 1, 1, 2], 4),
    ];
    for case in test_cases {
        assert_eq!(Solution::rob(case.0), case.1);
    }
}

#[test]
fn is_robot_bounded_test() {
    let test_cases = vec![
        ("GGLLGG", true),
        ("GG", false),
        ("GL", true),
        ("GLRLLGLL", true)
    ];
    for case in test_cases {
        assert_eq!(Solution::is_robot_bounded(case.0.to_string()), case.1);
    }
}

#[test]
fn max_profit_test() {
    let test_cases = vec![
        (vec![], 0),
        (vec![1], 0),
        (vec![1, 2], 1),
        (vec![7, 1, 5, 3, 6, 4], 5),
        (vec![7, 6, 4, 3, 1], 0),
    ];
    for case in test_cases {
        assert_eq!(Solution::max_profit(case.0), case.1);
    }
}

fn get_sequential_digits_test_cases() -> Vec<(i32, i32 , Vec<i32>)> {
    vec![
        (100, 300, vec![123, 234]),
        (100, 234, vec![123, 234]),
        (23456789, 123456789, vec![23456789, 123456789]),
        (23456788, 123456790, vec![23456789, 123456789]),
        (1000, 13000, vec![1234, 2345, 3456, 4567, 5678, 6789, 12345]),
    ]
}

#[test]
fn sequential_digits_test() {
    for case in get_sequential_digits_test_cases() {
        assert_eq!(Solution::sequential_digits(case.0, case.1), case.2);
    }
}

#[test]
fn sequential_digits_v2_test() {
    for case in get_sequential_digits_test_cases() {
        assert_eq!(Solution::sequential_digits_v2(case.0, case.1), case.2);
    }
}

#[test]
fn unique_paths_iii_test() {
    let test_cases = vec![
        (
            vec![
                vec![1,2],
            ],
            1,
        ),
        (
            vec![
                vec![1,0],
                vec![0,2],
            ],
            0,
        ),
        (
            vec![
                vec![1,-1],
                vec![-1,2],
            ],
            0,
        ),
        (
            vec![
                vec![1,-1],
                vec![0,2],
            ],
            1,
        ),
        (
            vec![
                vec![1, 0, 0],
                vec![0, 0, 0],
                vec![0, 0, 2],
            ],
            2,
        ),
        (
            vec![
                vec![1,0,0,0],
                vec![0,0,0,0],
                vec![0,0,2,-1]
            ],
            2,
        ),
    ];
    for case in test_cases {
        assert_eq!(Solution::unique_paths_iii(case.0), case.1);
    }
}

fn get_car_pooling_test_cases() -> Vec<(Vec<Vec<i32>>, i32, bool)> {
    vec![
        (vec![vec![2, 1, 5], vec![3, 3, 7]], 4, false),
        (vec![vec![2, 1, 5], vec![3, 3, 7]], 5, true),
        (vec![vec![2, 1, 5], vec![3, 5, 7]], 3, true),
        (vec![vec![3, 2, 7], vec![3, 7, 9], vec![8, 3, 9]], 11, true),
        (vec![vec![4, 5, 6], vec![6, 4, 7], vec![4, 3, 5], vec![2, 3, 5]], 13, true),
        (vec![vec![9, 3, 4], vec![9, 1, 7], vec![4, 2, 4], vec![7, 4, 5]], 23, true)
    ]
}

#[test]
fn car_pooling_test() {
    for case in get_car_pooling_test_cases() {
        assert_eq!(Solution::car_pooling(case.0, case.1), case.2);
    }
}

#[test]
fn car_pooling_v2_test() {
    for case in get_car_pooling_test_cases() {
        assert_eq!(Solution::car_pooling_v2(case.0, case.1), case.2);
    }
}

fn get_majority_element_ii_test_cases() -> Vec<(Vec<i32>, Vec<i32>)> {
    vec![
        (vec![1, 2], vec![1, 2]),
        (vec![2, 2], vec![2]),
        (vec![3, 2, 3], vec![3]),
        (vec![1, 1, 1, 3, 3, 2, 2, 2], vec![1, 2]),
    ]
}

#[test]
fn majority_element_ii_test() {
    for case in get_majority_element_ii_test_cases() {
        assert_eq!(Solution::majority_element_ii(case.0), case.1);
    }
}

#[test]
fn majority_element_ii_v2_test() {
    for case in get_majority_element_ii_test_cases() {
        assert_eq!(Solution::majority_element_ii_v2(case.0), case.1);
    }
}

fn get_can_complete_circuit_test_cases() -> Vec<(Vec<i32>, Vec<i32>, i32)>{
    vec![
        (vec![1, 2, 3, 4, 5], vec![3,4,5,1,2], 3),
        (vec![2, 3, 4], vec![3, 4, 3], -1),
        (vec![5, 1, 2, 3, 4], vec![4, 4, 1, 5, 1], 4),
        (vec![3, 1, 1], vec![1, 2, 2], 0),
    ]
}

#[test]
fn can_complete_circuit_test() {
    for case in get_can_complete_circuit_test_cases() {
        assert_eq!(Solution::can_complete_circuit(case.0, case.1), case.2);
    }
}

#[test]
fn can_complete_circuit_v2_test() {
    for case in get_can_complete_circuit_test_cases() {
        assert_eq!(Solution::can_complete_circuit_v2(case.0, case.1), case.2);
    }
}

fn get_the_difference_test_cases<'a>() -> Vec<(&'a str, &'a str, char)> {
    vec![
        ("", "a", 'a'),
        ("a", "aa", 'a'),
        ("aa", "aab", 'b'),
        ("abcd", "abcde", 'e'),
    ]
}

#[test]
fn find_the_difference_test() {
    for case in get_the_difference_test_cases() {
        assert_eq!(Solution::find_the_difference(case.0.to_string(), case.1.to_string()), case.2);
    }
}

#[test]
fn find_the_difference_v2_test() {
    for case in get_the_difference_test_cases() {
        assert_eq!(Solution::find_the_difference_v2(case.0.to_string(), case.1.to_string()), case.2);
    }
}

#[test]
fn find_the_difference_v3_test() {
    for case in get_the_difference_test_cases() {
        assert_eq!(Solution::find_the_difference_v3(case.0.to_string(), case.1.to_string()), case.2);
    }
}

fn get_largest_number_test_cases<'a>() -> Vec<(Vec<i32>, &'a str)> {
    vec![
        (vec![0, 0], "0"),
        (vec![1, 1], "11"),
        (vec![8, 91], "918"),
        (vec![10, 2], "210"),
        (vec![3, 30], "330"),
        (vec![30, 3], "330"),
        (vec![121, 12], "12121"),
        (vec![121, 12], "12121"),
        (vec![3, 30, 32, 5, 9], "9533230"),
        (vec![3, 30, 34, 5, 9], "9534330"),
        (vec![128, 12], "12812"),
        (vec![7543, 5328, 9834, 1940, 9387, 871, 5208, 7, 543], "9834938787177543543532852081940"),
        (vec![7543, 7], "77543"),
    ]
}

#[test]
fn largest_number_test() {
    for case in get_largest_number_test_cases() {
        assert_eq!(Solution::largest_number(case.0), case.1.to_string());
    }
}

#[test]
fn largest_number_v2_test() {
    for case in get_largest_number_test_cases() {
        assert_eq!(Solution::largest_number_v2(case.0), case.1.to_string());
    }
}

#[test]
fn find_poisoned_duration_test() {
    let test_cases = vec![
        (vec![1], 2, 2),
        (vec![], 2, 0),
        (vec![1], 0, 0),
        (vec![], 0, 0),
        (vec![1, 4], 2, 4),
        (vec![1, 2], 2, 3),
    ];
    for case in test_cases {
        assert_eq!(Solution::find_poisoned_duration(case.0, case.1), case.2);
    }
}

#[test]
fn calc_equation_test() {
    let test_cases = vec![
        (
            vec![vec!["a", "b"]],
            vec![0.5],
            vec![vec!["a", "b"], vec!["b", "a"], vec!["a", "c"], vec!["x", "y"]],
            vec![0.5, 2.0, -1.0, -1.0]
        ),
        (
            vec![vec!["a", "b"], vec!["b", "c"]],
            vec![2.0, 3.0],
            vec![vec!["a", "c"], vec!["b", "a"], vec!["a", "e"], vec!["a", "a"], vec!["x", "x"]],
            vec![6.0, 0.5, -1.0, 1.0, -1.0]
        ),
        (
            vec![vec!["a", "b"], vec!["b", "c"], vec!["bc", "cd"]],
            vec![1.5, 2.5, 5.0],
            vec![vec!["a", "c"], vec!["c", "b"], vec!["bc", "cd"], vec!["cd", "bc"]],
            vec![3.75, 0.4, 5.0, 0.2]
        ),
        (
            vec![vec!["x1", "x2"], vec!["x2", "x3"], vec!["x3", "x4"], vec!["x4", "x5"]],
            vec![3.0, 4.0, 5.0, 6.0],
            vec![vec!["x1", "x5"], vec!["x5", "x2"], vec!["x2", "x4"], vec!["x2", "x2"], vec!["x2", "x9"], vec!["x9", "x9"]],
            vec![360.0, 0.00833, 20.0, 1.0, -1.0, -1.0]
        ),
    ];
    for case in test_cases {
        assert_eq!(Solution::calc_equation(
            case.0.iter().map(|x| x.into_iter().map(|y| y.to_string()).collect()).collect(),
            case.1,
            case.2.iter().map(|x| x.into_iter().map(|y| y.to_string()).collect()).collect())
                       .into_iter().map(|x| (x * 100_000.0).round() / 100_000.0).collect::<Vec<f64>>(),
            case.3);
    }
}

#[test]
fn num_subarray_product_less_than_k_test() {
    let test_cases = vec![
        (vec![10, 5, 2, 6], 100, 8),
    ];
    for case in test_cases {
        assert_eq!(Solution::num_subarray_product_less_than_k(case.0, case.1), case.2);
    }
}

#[test]
fn word_break_test() {
    let test_cases = vec![
        ("leetcode", vec!["leet", "code"], true),
        ("applepenapple", vec!["apple", "pen"], true),
        ("catsandog", vec!["cats", "dog", "sand", "and", "cat"], false),
        (
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            vec!["a","aa","aaa","aaaa","aaaaa","aaaaaa","aaaaaaa","aaaaaaaa","aaaaaaaaa","aaaaaaaaaa"],
            true,
        ),
        (
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab",
            vec!["a","aa","aaa","aaaa","aaaaa","aaaaaa","aaaaaaa","aaaaaaaa","aaaaaaaaa","aaaaaaaaaa"],
            false,
        )
    ];
    for case in test_cases {
        assert_eq!(Solution::word_break(case.0.to_string(),
                                        case.1.into_iter().map(|x| x.to_string()).collect()),
                                        case.2);
    }
}

fn get_first_missing_positive_test_cases() -> Vec<(Vec<i32>, i32)> {
    vec![
        (vec![2], 1),
        (vec![1], 2),
        (vec![1, 2, 0], 3),
        (vec![3, 4, -1, 1], 2),
        (vec![7, 8, 9, 11, 12], 1),
        (vec![-1, -2, 0], 1),
        (vec![-1, -2, 1], 2),
        (vec![-1, -2, 1, 2, 5], 3),
        (vec![-1, -2, 1, 2, 3], 4),
    ]
}

#[test]
fn first_missing_positive_test() {
    for case in get_first_missing_positive_test_cases() {
        assert_eq!(Solution::first_missing_positive(case.0), case.1);
    }
}

#[test]
fn first_missing_positive_v2_test() {
    for case in get_first_missing_positive_test_cases() {
        assert_eq!(Solution::first_missing_positive_v2(case.0), case.1);
    }
}
