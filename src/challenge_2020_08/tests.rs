use super::*;
use crate::challenge_2020_08::logger_v1::Logger as LoggerV1;
use crate::challenge_2020_08::logger_v2::Logger as LoggerV2;

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