use super::*;
use crate::challenge_2020_08::logger_v1::Logger as LoggerV1;
use crate::challenge_2020_08::logger_v2::Logger as LoggerV2;
use crate::challenge_2020_08::hash_set::HashSet;
use crate::challenge_2020_08::word_dictionary::WordDictionary;
use crate::common::linked_list::LinkedList;

fn get_logger_test_cases<'a>() -> Vec<(i32, &'a str, bool)>{
    vec![
        (1, "foo", true),
        (2, "bar", true),
        (3, "foo", false),
        (8, "bar", false),
        (10, "foo", false),
        (11, "foo", true),
    ]
}

#[test]
fn logger_v1_test() {
    let mut logger = LoggerV1::new();
    for case in get_logger_test_cases() {
        assert_eq!(logger.should_print_message(case.0, case.1.to_string()), case.2);
    }
}

#[test]
fn logger_v2_test() {
    let mut logger = LoggerV2::new();
    for case in get_logger_test_cases() {
        assert_eq!(logger.should_print_message(case.0, case.1.to_string()), case.2);
    }
}

#[test]
fn detect_capital_use_test() {
    let test_cases = vec![
        ("USA".to_string(), true),
        ("USa".to_string(), false),
        ("leetcode".to_string(), true),
        ("LeetCode".to_string(), false),
        ("Google".to_string(), true),
        ("GooglE".to_string(), false),
    ];
    for case in test_cases {
        assert_eq!(Solution::detect_capital_use(case.0), case.1);
    }
}

#[test]
fn myhashset_test() {
    let mut set = HashSet::new();
    set.add(1);
    set.add(2);
    assert_eq!(set.contains(1), true);
    assert_eq!(set.contains(3), false);
    set.add(2);
    assert_eq!(set.contains(2), true);
    set.remove(2);
    assert_eq!(set.contains(2), false);
}

#[test]
fn is_palindrome_test() {
    let test_cases = vec![
        ("", true),
        ("a", true),
        ("aa", true),
        ("ab", false),
        ("a.", true),
        ("race a car", false),
        ("A man, a plan, a canal: Panama", true),
    ];
    for case in test_cases {
        assert_eq!(Solution::is_palindrome(case.0.to_string()), case.1);
    }
}

fn get_is_power_of_four_test_cases() -> Vec<(i32, bool)> {
    vec![
        (-4, false),
        (0, false),
        (1, true),
        (2, false),
        (3, false),
        (4, true),
        (5, false),
        (6, false),
        (7, false),
        (8, false),
        (9, false),
        (10, false),
        (12, false),
        (12, false),
        (13, false),
        (14, false),
        (15, false),
        (16, true),
        (17, false),
        (64, true),
        (65, false),
    ]
}

#[test]
fn is_power_of_four_test() {
    for case in get_is_power_of_four_test_cases() {
        assert_eq!(Solution::is_power_of_four(case.0), case.1);
    }
}

#[test]
fn is_power_of_four_v2_test() {
    for case in get_is_power_of_four_test_cases() {
        assert_eq!(Solution::is_power_of_four_v2(case.0), case.1);
    }
}

#[test]
fn is_power_of_four_v3_test() {
    for case in get_is_power_of_four_test_cases() {
        assert_eq!(Solution::is_power_of_four_v3(case.0), case.1);
    }
}

#[test]
fn word_dictionary_test() {
    let mut obj = WordDictionary::new();
    obj.add_word("bad".to_string());
    obj.add_word("dad".to_string());
    obj.add_word("mad".to_string());
    assert_eq!(obj.search("pad".to_string()), false);
    assert_eq!(obj.search("bad".to_string()), true);
    assert_eq!(obj.search(".ad".to_string()), true);
    assert_eq!(obj.search("b..".to_string()), true);
}

#[test]
fn find_duplicates_test() {
    let test_cases = vec![
        (vec![], vec![]),
        (vec![4, 3, 2, 7, 8, 2, 3, 1], vec![2, 3]),
    ];
    for case in test_cases {
        assert_eq!(Solution::find_duplicates(case.0), case.1);
    }
}

#[test]
fn find_duplicates_v2_test() {
    let test_cases = vec![
        (vec![], vec![]),
        (vec![4, 3, 2, 7, 8, 2, 3, 1], vec![2, 3]),
    ];
    for case in test_cases {
        assert_eq!(Solution::find_duplicates_v2(case.0), case.1);
    }
}

fn get_vertical_traversal_test_cases() -> Vec<(Vec<Option<i32>>, Vec<Vec<i32>>)> {
    vec![
        (
            vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)],
            vec![vec![9], vec![3, 15], vec![20], vec![7]]
        ),
        (
            vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7)],
            vec![vec![4], vec![2], vec![1, 5, 6], vec![3], vec![7]]
        ),
    ]
}

#[test]
fn vertical_traversal_test() {
    for case in get_vertical_traversal_test_cases() {
        assert_eq!(Solution::vertical_traversal(TreeNode::from_level_order(&case.0)), case.1);
    }
}

#[test]
fn vertical_traversal_v2_test() {
    for case in get_vertical_traversal_test_cases() {
        assert_eq!(Solution::vertical_traversal_v2(TreeNode::from_level_order(&case.0)), case.1);
    }
}

fn get_closest_value_test_cases() -> Vec<(Vec<Option<i32>>, f64, i32)>{
    vec![
        (
            vec![Some(4), Some(2), Some(5), Some(1), Some(3)],
            3.714286,
            4
        ),
        (
            vec![Some(4), Some(2), Some(5), Some(1), Some(3)],
            1.1,
            1
        ),
        (
            vec![Some(1), None, Some(2)],
            3.428571,
            2
        ),
        (
            vec![Some(8), Some(1)],
            6.0,
            8
        ),
    ]
}

#[test]
fn closest_value_test() {
    for case in get_closest_value_test_cases() {
        assert_eq!(Solution::closest_value(TreeNode::from_level_order(&case.0), case.1), case.2);
    }
}

#[test]
fn closest_value_test_v2() {
    for case in get_closest_value_test_cases() {
        assert_eq!(Solution::closest_value_v2(TreeNode::from_level_order(&case.0), case.1), case.2);
    }
}

#[test]
fn path_sum_test() {
    let test_cases = vec![
        (
            vec![Some(10), Some(5), Some(-3), Some(3), Some(2), None, Some(11), Some(3), Some(-2), None, Some(1)],
            8,
            3,
        ),
    ];
    for case in test_cases {
        assert_eq!(Solution::path_sum(TreeNode::from_level_order(&case.0), case.1), case.2);
    }
}

#[test]
fn oranges_rotting_test() {
    let test_cases = vec![
        (
            vec![],
            -1,
        ),
        (
            vec![vec![]],
            -1,
        ),
        (
            vec![vec![0]],
            0,
        ),
        (
            vec![vec![1]],
            -1,
        ),
        (
            vec![vec![2]],
            0,
        ),
        (
            vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]],
            4,
        ),
        (
            vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]],
            -1,
        ),
    ];
    for case in test_cases {
        assert_eq!(Solution::oranges_rotting(case.0), case.1);
    }
}

#[test]
fn title_to_number_test() {
    let test_cases = vec![
        ("A", 1),
        ("AB", 28),
        ("ZY", 701),
    ];
    for case in test_cases {
        assert_eq!(Solution::title_to_number(case.0.to_string()), case.1);
    }
}

#[test]
fn get_row_test() {
    let test_cases = vec![
        (0, vec![1]),
        (1, vec![1, 1]),
        (2, vec![1, 2, 1]),
        (3, vec![1,3,3,1]),
        (30, vec![1,30,435,4060,27405,142506,593775,2035800,5852925,14307150,30045015,54627300,
                  86493225,119759850,145422675,155117520,145422675,119759850,86493225,54627300,
                  30045015,14307150,5852925,2035800,593775,142506,27405,4060,435,30,1]),
    ];
    for case in test_cases {
        assert_eq!(Solution::get_row(case.0), case.1);
    }
}

#[test]
fn longest_palindrome_test() {
    let test_cases = vec![
        ("abccccdd", 7),
        ("a", 1),
        ("bb", 2),
        ("aaaAaaaa", 7),
    ];
    for case in test_cases {
        assert_eq!(Solution::longest_palindrome(case.0.to_string()), case.1);
    }
}

#[test]
fn to_goat_latin_test() {
    let test_cases = vec![
        (
            "I speak Goat Latin",
            "Imaa peaksmaaa oatGmaaaa atinLmaaaaa"),
        (
            "The quick brown fox jumped over the lazy dog",
            "heTmaa uickqmaaa rownbmaaaa oxfmaaaaa umpedjmaaaaaa overmaaaaaaa hetmaaaaaaaa azylmaaaaaaaaa ogdmaaaaaaaaaa"
        ),
    ];
    for case in test_cases {
        assert_eq!(Solution::to_goat_latin(case.0.to_string()), case.1.to_string());
    }
}

#[test]
fn sort_array_by_parity_test() {
    let test_cases = vec![
        (vec![1], vec![1]),
        (vec![1, 2], vec![2, 1]),
        (vec![3,1,2,4], vec![2, 4, 1, 3]),
    ];
    for case in test_cases {
        assert_eq!(Solution::sort_array_by_parity(case.0), case.1);
    }
}

#[test]
fn sort_array_by_parity_v2_test() {
    let test_cases = vec![
        (vec![1], vec![1]),
        (vec![1, 2], vec![2, 1]),
        (vec![3,1,2,4], vec![2, 4, 3, 1]),
    ];
    for case in test_cases {
        assert_eq!(Solution::sort_array_by_parity_v2(case.0), case.1);
    }
}

#[test]
fn reorder_list_test() {
    let test_cases = vec![
        (vec![], vec![]),
        (vec![1], vec![1]),
        (vec![1, 2], vec![1, 2]),
        (vec![1, 2, 3], vec![1, 3, 2]),
        (vec![1, 2, 3, 4, 5], vec![1, 5, 2, 4, 3]),
    ];

    for case in test_cases {
        let mut head = ListNode::from_slice(&case.0);
        Solution::reorder_list(&mut head);
        assert_eq!(head.to_vec(), case.1);
    }
}

fn get_sum_of_left_leaves_test_cases() -> Vec<(Vec<Option<i32>>, i32)>{
    vec![
        (vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)], 24),
        (vec![Some(1), Some(2), Some(3), Some(4), Some(5)], 4),
    ]
}

#[test]
fn sum_of_left_leaves_test() {
    for case in get_sum_of_left_leaves_test_cases() {
        assert_eq!(Solution::sum_of_left_leaves(TreeNode::from_level_order(&case.0)), case.1);
    }
}

#[test]
fn sum_of_left_leaves_v2_test() {
    for case in get_sum_of_left_leaves_test_cases() {
        assert_eq!(Solution::sum_of_left_leaves_v2(TreeNode::from_level_order(&case.0)), case.1);
    }
}