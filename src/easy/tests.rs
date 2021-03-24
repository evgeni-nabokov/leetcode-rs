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
fn is_valid() {
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