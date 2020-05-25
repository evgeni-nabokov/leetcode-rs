use super::*;

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