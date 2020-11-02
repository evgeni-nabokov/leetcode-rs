use super::*;

#[test]
fn get_decimal_value_test() {
    let test_cases = vec![
        (vec![1, 0, 1], 5),
        (vec![0], 0),
        (vec![1], 1),
        (vec![0, 0], 0),
        (vec![1, 0, 0, 1, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0], 18880),
    ];
    for case in test_cases {
        assert_eq!(Solution::get_decimal_value(ListNode::from_slice(&case.0)), case.1);
    }
}
