use super::*;

#[test]
fn is_same_after_reversals_test() {
    let test_cases = vec![(526, true), (1800, false), (0, true)];
    for case in test_cases {
        assert_eq!(Solution::is_same_after_reversals(case.0), case.1);
    }
}
