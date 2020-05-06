use super::*;

#[test]
fn num_jewels_in_stones_test() {
    assert_eq!(Solution::num_jewels_in_stones("aA".to_string(), "aAAbbbb".to_string()), 3);
    assert_eq!(Solution::num_jewels_in_stones("z".to_string(), "ZZ".to_string()), 0);
}

#[test]
fn can_construct_test() {
    assert_eq!(Solution::can_construct("".to_string(), "".to_string()), true);
    assert_eq!(Solution::can_construct("a".to_string(), "".to_string()), false);
    assert_eq!(Solution::can_construct("".to_string(), "a".to_string()), true);
    assert_eq!(Solution::can_construct("a".to_string(), "b".to_string()), false);
    assert_eq!(Solution::can_construct("aa".to_string(), "ab".to_string()), false);
    assert_eq!(Solution::can_construct("aa".to_string(), "aab".to_string()), true);
}

#[test]
fn can_construct_v2_test() {
    assert_eq!(Solution::can_construct_v2("".to_string(), "".to_string()), true);
    assert_eq!(Solution::can_construct_v2("a".to_string(), "".to_string()), false);
    assert_eq!(Solution::can_construct_v2("".to_string(), "a".to_string()), true);
    assert_eq!(Solution::can_construct_v2("a".to_string(), "b".to_string()), false);
    assert_eq!(Solution::can_construct_v2("aa".to_string(), "ab".to_string()), false);
    assert_eq!(Solution::can_construct_v2("aa".to_string(), "aab".to_string()), true);
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