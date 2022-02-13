use super::*;
use crate::common::linked_list::LinkedList;

fn get_two_sum_test_cases() -> Vec<(Vec<i32>, i32, Vec<i32>)> {
    vec![
        (vec![2, 7, 11, 15], 9, vec![0, 1]),
        (vec![3, 2, 4], 6, vec![1, 2]),
        (vec![3, 3], 6, vec![0, 1]),
    ]
}

#[test]
fn two_sum_test() {
    for case in get_two_sum_test_cases() {
        let mut actual = Solution::two_sum(case.0, case.1);
        actual.sort_unstable();
        assert_eq!(actual, case.2);
    }
}

#[test]
fn two_sum_v2_test() {
    for case in get_two_sum_test_cases() {
        let mut actual = Solution::two_sum_v2(case.0, case.1);
        actual.sort_unstable();
        assert_eq!(actual, case.2);
    }
}

#[test]
fn reverse_test() {
    let test_cases = vec![(123, 321), (-123, -321), (120, 21), (1534236469, 0)];
    for case in test_cases {
        assert_eq!(Solution::reverse(case.0), case.1);
    }
}

#[test]
fn roman_to_int_test() {
    let test_cases = vec![
        ("III", 3),
        ("IV", 4),
        ("IX", 9),
        ("LVIII", 58),
        ("MCMXCIV", 1994),
    ];
    for case in test_cases {
        assert_eq!(Solution::roman_to_int(case.0.to_string()), case.1);
    }
}

#[test]
fn remove_duplicates_from_sorted_array_test() {
    let test_cases = vec![
        (vec![], vec![]),
        (vec![1, 1, 2], vec![1, 2]),
        (vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4], vec![0, 1, 2, 3, 4]),
    ];
    for mut case in test_cases {
        let k = Solution::remove_duplicates_from_sorted_array(&mut case.0) as usize;
        assert_eq!(k, case.1.len());
        for i in 0..k {
            assert_eq!(case.0[i], case.1[i]);
        }
    }
}

#[test]
fn inorder_traversal_test() {
    let test_cases = vec![
        (vec![], vec![]),
        (vec![Some(1)], vec![1]),
        (vec![Some(1), Some(2)], vec![2, 1]),
        (vec![Some(1), None, Some(2)], vec![1, 2]),
        (vec![Some(1), None, Some(2), Some(3)], vec![1, 3, 2]),
    ];
    for case in test_cases {
        let tree = TreeNode::from_level_order(&case.0);
        assert_eq!(Solution::inorder_traversal(tree), case.1)
    }
}

#[test]
fn defang_ip_addr_test() {
    let test_cases = vec![
        ("1.1.1.1".to_string(), "1[.]1[.]1[.]1".to_string()),
        ("255.100.50.0".to_string(), "255[.]100[.]50[.]0".to_string()),
    ];
    for case in test_cases {
        assert_eq!(Solution::defang_ip_addr(case.0), case.1);
    }
}

#[test]
fn number_of_steps_test() {
    let test_cases = vec![(14, 6), (8, 4)];
    for case in test_cases {
        assert_eq!(Solution::number_of_steps(case.0), case.1);
    }
}

fn get_reverse_list_test_cases() -> Vec<(Vec<i32>, Vec<i32>)> {
    vec![
        (vec![], vec![]),
        (vec![1], vec![1]),
        (vec![1, 2, 3], vec![3, 2, 1]),
    ]
}

#[test]
fn reverse_list_test() {
    for case in get_reverse_list_test_cases() {
        assert_eq!(
            Solution::reverse_list(ListNode::from_slice(&case.0)).to_vec(),
            case.1
        );
    }
}

#[test]
fn reverse_list_v2_test() {
    for case in get_reverse_list_test_cases() {
        assert_eq!(
            Solution::reverse_list_v2(ListNode::from_slice(&case.0)).to_vec(),
            case.1
        );
    }
}

#[test]
fn merge_two_lists_test() {
    let test_cases = vec![
        (vec![], vec![], vec![]),
        (vec![1, 3, 5], vec![], vec![1, 3, 5]),
        (vec![], vec![2, 4, 6], vec![2, 4, 6]),
        (vec![1, 3, 5], vec![2, 4, 6], vec![1, 2, 3, 4, 5, 6]),
        (
            vec![1, 3, 5, 7, 9],
            vec![2, 4, 6],
            vec![1, 2, 3, 4, 5, 6, 7, 9],
        ),
        (
            vec![1, 3, 5],
            vec![2, 4, 6, 8, 10],
            vec![1, 2, 3, 4, 5, 6, 8, 10],
        ),
        (vec![1, 2, 4], vec![1, 3, 4], vec![1, 1, 2, 3, 4, 4]),
    ];
    for case in test_cases {
        assert_eq!(
            Solution::merge_two_lists(ListNode::from_slice(&case.0), ListNode::from_slice(&case.1))
                .to_vec(),
            case.2
        );
    }
}

#[test]
fn is_valid_test() {
    let test_cases = vec![
        ("()", true),
        ("()[]{}", true),
        ("(]", false),
        ("([)]", false),
        ("{[]}", true),
    ];
    for case in test_cases {
        assert_eq!(Solution::is_valid(case.0.to_string()), case.1);
    }
}

#[test]
fn pivot_index_test() {
    let test_cases = vec![
        (vec![1, 7, 3, 6, 5, 6], 3),
        (vec![1, 2, 3], -1),
        (vec![2, 1, -1], 0),
        (vec![1, -1, 2], 2),
        (vec![1], 0),
    ];
    for case in test_cases {
        assert_eq!(Solution::pivot_index(case.0), case.1);
    }
}

#[test]
fn find_middle_index_test() {
    let test_cases = vec![
        (vec![2, 3, -1, 8, 4], 3),
        (vec![1, -1, 4], 2),
        (vec![4, 1, -1], 0),
        (vec![2, 5], -1),
        (vec![1], 0),
    ];
    for case in test_cases {
        assert_eq!(Solution::find_middle_index(case.0), case.1);
    }
}

#[test]
fn valid_palindrome_test() {
    let test_cases = vec![
        ("aba", true),
        ("abca", true),
        ("abc", false),
        ("cuppucu", true),
        (
            "cupuufxoohdfpgjdmysgvhmvffcnqxjjxqncffvmhvgsymdjgpfdhooxfuupucu",
            true,
        ),
    ];
    for case in test_cases {
        assert_eq!(Solution::valid_palindrome(case.0.to_string()), case.1);
    }
}

#[test]
fn remove_duplicates_test() {
    let test_cases = vec![("abbaca", "ca"), ("azxxzy", "ay")];
    for case in test_cases {
        assert_eq!(
            Solution::remove_duplicates(case.0.to_string()),
            case.1.to_string()
        );
    }
}

#[test]
fn add_strings_test() {
    let test_cases = vec![("11", "123", "134"), ("456", "77", "533"), ("0", "0", "0")];
    for case in test_cases {
        assert_eq!(
            Solution::add_strings(case.0.to_string(), case.1.to_string()),
            case.2.to_string()
        );
    }
}

#[test]
fn sum_odd_length_subarrays_test() {
    let test_cases = vec![
        (vec![1, 4, 2, 5, 3], 58),
        (vec![1, 2], 3),
        (vec![10, 11, 12], 66),
    ];
    for case in test_cases {
        assert_eq!(Solution::sum_odd_length_subarrays(case.0), case.1);
    }
}

#[test]
fn diameter_of_binary_tree_test() {
    let test_cases = vec![
        (vec![Some(1)], 0),
        (vec![Some(1), Some(2)], 1),
        (vec![Some(1), Some(2), Some(3), Some(4), Some(5)], 3),
    ];
    for case in test_cases {
        assert_eq!(
            Solution::diameter_of_binary_tree(TreeNode::from_level_order(&case.0)),
            case.1
        );
    }
}

fn get_maximum_units_test_cases() -> Vec<(Vec<Vec<i32>>, i32, i32)> {
    vec![
        (vec![vec![1, 3], vec![2, 2], vec![3, 1]], 4, 8),
        (
            vec![vec![5, 10], vec![2, 5], vec![4, 7], vec![3, 9]],
            10,
            91,
        ),
    ]
}

#[test]
fn maximum_units_test() {
    for case in get_maximum_units_test_cases() {
        assert_eq!(Solution::maximum_units(case.0, case.1), case.2);
    }
}

#[test]
fn maximum_units_v2_test() {
    for case in get_maximum_units_test_cases() {
        assert_eq!(Solution::maximum_units_v2(case.0, case.1), case.2);
    }
}

fn get_min_cost_climbing_stairs_test_cases() -> Vec<(Vec<i32>, i32)> {
    vec![
        (vec![10, 15, 20], 15),
        (vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1], 6),
    ]
}

#[test]
fn min_cost_climbing_stairs_test() {
    for case in get_min_cost_climbing_stairs_test_cases() {
        assert_eq!(Solution::min_cost_climbing_stairs(case.0), case.1);
    }
}

#[test]
fn min_cost_climbing_stairs_v2_test() {
    for case in get_min_cost_climbing_stairs_test_cases() {
        assert_eq!(Solution::min_cost_climbing_stairs_v2(case.0), case.1);
    }
}

fn get_missing_number_test_cases() -> Vec<(Vec<i32>, i32)> {
    vec![
        (vec![3, 0, 1], 2),
        (vec![0, 1], 2),
        (vec![9, 6, 4, 2, 3, 5, 7, 0, 1], 8),
        (vec![0], 1),
    ]
}

#[test]
fn missing_number_test() {
    for case in get_missing_number_test_cases() {
        assert_eq!(Solution::missing_number(case.0), case.1);
    }
}

#[test]
fn missing_number_v2_test() {
    for case in get_missing_number_test_cases() {
        assert_eq!(Solution::missing_number_v2(case.0), case.1);
    }
}

#[test]
fn is_palindrome() {
    let test_cases = vec![
        (0, true),
        (121, true),
        (-121, false),
        (10, false),
        (-101, false),
    ];
    for case in test_cases {
        assert_eq!(Solution::is_palindrome(case.0), case.1);
    }
}

fn get_min_cost_to_move_chips_test_cases() -> Vec<(Vec<i32>, i32)> {
    vec![(vec![1, 2, 3], 1), (vec![2, 2, 2, 3, 3], 2)]
}

#[test]
fn min_cost_to_move_chips_test() {
    for case in get_min_cost_to_move_chips_test_cases() {
        assert_eq!(Solution::min_cost_to_move_chips(case.0), case.1);
    }
}

#[test]
fn min_cost_to_move_chips_v2_test() {
    for case in get_min_cost_to_move_chips_test_cases() {
        assert_eq!(Solution::min_cost_to_move_chips_v2(case.0), case.1);
    }
}

#[test]
fn reorder_log_files_test() {
    let test_cases = vec![
        (
            vec![
                "dig1 8 1 5 1",
                "let1 art can",
                "dig2 3 6",
                "let2 own kit dig",
                "let3 art zero",
            ],
            vec![
                "let1 art can",
                "let3 art zero",
                "let2 own kit dig",
                "dig1 8 1 5 1",
                "dig2 3 6",
            ],
        ),
        (
            vec![
                "a1 9 2 3 1",
                "g1 act car",
                "zo4 4 7",
                "ab1 off key dog",
                "a8 act zoo",
            ],
            vec![
                "g1 act car",
                "a8 act zoo",
                "ab1 off key dog",
                "a1 9 2 3 1",
                "zo4 4 7",
            ],
        ),
        (
            vec![
                "dig1 8 1 5 1",
                "let1 art zero can",
                "dig2 3 6",
                "let2 own kit dig",
                "let3 art zero",
            ],
            vec![
                "let3 art zero",
                "let1 art zero can",
                "let2 own kit dig",
                "dig1 8 1 5 1",
                "dig2 3 6",
            ],
        ),
    ];

    for case in test_cases {
        assert_eq!(
            Solution::reorder_log_files(
                case.0
                    .into_iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
            ),
            case.1
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
        );
    }
}

#[test]
fn is_toeplitz_matrix_test() {
    let test_cases = vec![
        (
            vec![vec![1, 2, 3, 4], vec![5, 1, 2, 3], vec![9, 5, 1, 2]],
            true,
        ),
        (vec![vec![1, 2], vec![2, 2]], false),
    ];

    for case in test_cases {
        assert_eq!(Solution::is_toeplitz_matrix(case.0), case.1);
    }
}

#[test]
fn minimum_abs_difference_test() {
    let test_cases = vec![
        (vec![4, 2, 1, 3], vec![vec![1, 2], vec![2, 3], vec![3, 4]]),
        (vec![1, 3, 6, 10, 15], vec![vec![1, 3]]),
        (
            vec![3, 8, -10, 23, 19, -4, -14, 27],
            vec![vec![-14, -10], vec![19, 23], vec![23, 27]],
        ),
    ];

    for case in test_cases {
        assert_eq!(Solution::minimum_abs_difference(case.0), case.1);
    }
}

#[test]
fn binary_tree_paths_test() {
    let test_cases = vec![
        (
            vec![Some(1), Some(2), Some(3), None, Some(5)],
            vec!["1->2->5", "1->3"],
        ),
        (vec![Some(1)], vec!["1"]),
    ];

    for case in test_cases {
        assert_eq!(
            Solution::binary_tree_paths(TreeNode::from_level_order(&case.0)),
            case.1
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
        );
    }
}

#[test]
fn count_operations_test() {
    let test_cases = vec![(2, 3, 3), (10, 10, 1)];

    for case in test_cases {
        assert_eq!(Solution::count_operations(case.0, case.1), case.2);
    }
}
