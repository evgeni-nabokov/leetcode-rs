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

#[test]
fn level_order_bottom_test() {
    let test_cases = vec![
        (
            vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)],
            vec![
                vec![15, 7],
                vec![9, 20],
                vec![3],
            ]
        )
    ];
    for case in test_cases {
        assert_eq!(Solution::level_order_bottom(TreeNode::from_level_order(&case.0)), case.1);
    }
}

