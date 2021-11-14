use super::*;
use crate::common::linked_list::LinkedList;

#[test]
fn two_sum_test() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
}

#[test]
fn reverse_test() {
    let test_cases = vec![
        (123, 321),
        (-123, -321),
        (120, 21),
        (1534236469, 0),
    ];
    for case in test_cases {
        assert_eq!(Solution::reverse(case.0), case.1);
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
    let test_cases = vec![
        (14, 6),
        (8, 4),
    ];
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
        assert_eq!(Solution::reverse_list(ListNode::from_slice(&case.0)).to_vec(), case.1);
    }
}

#[test]
fn reverse_list_v2_test() {
    for case in get_reverse_list_test_cases() {
        assert_eq!(Solution::reverse_list_v2(ListNode::from_slice(&case.0)).to_vec(), case.1);
    }
}

#[test]
fn merge_two_lists_test() {
    let test_cases = vec![
        (vec![], vec![], vec![]),
        (vec![1, 3, 5], vec![], vec![1, 3, 5]),
        (vec![], vec![2, 4, 6], vec![2, 4, 6]),
        (vec![1, 3, 5], vec![2, 4, 6], vec![1, 2, 3, 4, 5, 6]),
        (vec![1, 3, 5, 7, 9], vec![2, 4, 6], vec![1, 2, 3, 4, 5, 6, 7, 9]),
        (vec![1, 3, 5], vec![2, 4, 6, 8, 10], vec![1, 2, 3, 4, 5, 6, 8, 10]),
        (vec![1,2,4], vec![1,3,4], vec![1,1,2,3,4,4]),
    ];
    for case in test_cases {
        assert_eq!(Solution::merge_two_lists(ListNode::from_slice(&case.0), ListNode::from_slice(&case.1)).to_vec(), case.2);
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
        ("cupuufxoohdfpgjdmysgvhmvffcnqxjjxqncffvmhvgsymdjgpfdhooxfuupucu", true),
    ];
    for case in test_cases {
        assert_eq!(Solution::valid_palindrome(case.0.to_string()), case.1);
    }
}

#[test]
fn remove_duplicates_test() {
    let test_cases = vec![
        ("abbaca", "ca"),
        ("azxxzy", "ay"),
    ];
    for case in test_cases {
        assert_eq!(Solution::remove_duplicates(case.0.to_string()), case.1.to_string());
    }
}

#[test]
fn add_strings_test() {
    let test_cases = vec![
        ("11", "123", "134"),
        ("456", "77", "533"),
        ("0", "0", "0"),
    ];
    for case in test_cases {
        assert_eq!(Solution::add_strings(case.0.to_string(), case.1.to_string()), case.2.to_string());
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
        assert_eq!(Solution::diameter_of_binary_tree(TreeNode::from_level_order(&case.0)), case.1);
    }
}