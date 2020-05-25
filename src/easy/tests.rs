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