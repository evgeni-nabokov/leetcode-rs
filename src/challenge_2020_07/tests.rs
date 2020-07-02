use super::*;

#[test]
fn arrange_coins_test() {
    let test_cases = vec![
        (5, 2),
        (8, 3),
        (2146467959, 65519)
    ];
    for case in test_cases {
        assert_eq!(Solution::arrange_coins(case.0), case.1);
    }
}