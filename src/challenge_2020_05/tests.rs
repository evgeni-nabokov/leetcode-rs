use super::*;

#[test]
fn num_jewels_in_stones_test() {
    assert_eq!(Solution::num_jewels_in_stones("aA".to_string(), "aAAbbbb".to_string()), 3);
    assert_eq!(Solution::num_jewels_in_stones("z".to_string(), "ZZ".to_string()), 0);
}

fn get_can_construct_test_cases() -> Vec<(String, String, bool)> {
    vec![
        ("".to_string(), "".to_string(), true),
        ("a".to_string(), "".to_string(), false),
        ("".to_string(), "a".to_string(), true),
        ("a".to_string(), "b".to_string(), false),
        ("aa".to_string(), "ab".to_string(), false),
        ("aa".to_string(), "aab".to_string(), true),
    ]
}

#[test]
fn can_construct_test() {
    for case in get_can_construct_test_cases() {
        assert_eq!(Solution::can_construct(case.0, case.1), case.2);
    }
}

#[test]
fn can_construct_v2_test() {
    for case in get_can_construct_test_cases() {
        assert_eq!(Solution::can_construct_v2(case.0, case.1), case.2);
    }
}

fn get_bitwise_complement_test_cases() -> Vec<(i32, i32)> {
    vec![
        (1, 0),
        (0, 1),
        (8, 7),
        (5, 2),
        (7, 0),
        (10, 5)
    ]
}

#[test]
fn bitwise_complement_test() {
    for case in get_bitwise_complement_test_cases() {
        assert_eq!(Solution::bitwise_complement(case.0), case.1);
    }
}

#[test]
fn bitwise_complement_v2_test() {
    for case in get_bitwise_complement_test_cases() {
        assert_eq!(Solution::bitwise_complement_v2(case.0), case.1);
    }
}

#[test]
fn first_uniq_char_test() {
    assert_eq!(Solution::first_uniq_char("leetcode".to_string()), 0);
    assert_eq!(Solution::first_uniq_char("loveleetcode".to_string()), 2);
}

#[test]
fn majority_element_test() {
    assert_eq!(Solution::majority_element(vec![1]), 1);
    assert_eq!(Solution::majority_element(vec![3,2,3]), 3);
    assert_eq!(Solution::majority_element(vec![2,2,1,1,1,2,2]), 2);
}
