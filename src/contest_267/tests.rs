use super::*;

#[test]
fn time_required_to_buy_test() {
    let test_cases = vec![
        (vec![2, 3, 2], 2, 6),
        (vec![5, 1, 1, 1], 0, 8),
    ];
    for case in test_cases {
        assert_eq!(Solution::time_required_to_buy(case.0, case.1), case.2);
    }
}