use super::*;
use crate::challenge_2020_08::logger_v1::Logger as LoggerV1;
use crate::challenge_2020_08::logger_v2::Logger as LoggerV2;
use crate::challenge_2020_08::myhashset::MyHashSet;

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
    let mut set = MyHashSet::new();
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