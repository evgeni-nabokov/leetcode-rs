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

fn get_prison_after_n_days_test_cases() -> Vec<(Vec<i32>, i32, Vec<i32>)> {
    vec![
        (
            vec![],
            7,
            vec![],
        ),
        (
            vec![0, 1, 0, 1, 1, 0, 0, 1],
            0,
            vec![0, 1, 0, 1, 1, 0, 0, 1],
        ),
        (
            vec![0, 1, 0, 1, 1, 0, 0, 1],
            7,
            vec![0, 0, 1, 1, 0, 0, 0, 0],
        ),
        (
            vec![1, 0, 0, 1, 0, 0, 1, 0],
            1000000000,
            vec![0, 0, 1, 1, 1, 1, 1, 0],
        ),
        (
            vec![0, 0, 1, 1, 1, 1, 0, 0],
            8,
            vec![0,0,0,1,1,0,0,0]
        ),
        (
            vec![1, 1, 0, 1, 1, 0, 1, 1],
            6,
            vec![0, 0, 1, 0, 0, 1, 0, 0]
        ),
        (
            vec![1,0,0,1,0,0,0,1],
            826,
            vec![0,1,1,0,1,1,1,0]
        ),
    ]
}

#[test]
fn prison_after_n_days_test() {
    for case in get_prison_after_n_days_test_cases() {
        assert_eq!(Solution::prison_after_n_days(case.0, case.1), case.2);
    }
}

#[test]
fn prison_after_n_days_v2_test() {
    for case in get_prison_after_n_days_test_cases() {
        assert_eq!(Solution::prison_after_n_days_v2(case.0, case.1), case.2);
    }
}

#[test]
fn nth_ugly_number_test() {
    let test_cases = vec![
        (1, 1),
        (2, 2),
        (3, 3),
        (4, 4),
        (5, 5),
        (6, 6),
        (7, 8),
        (8, 9),
        (9, 10),
        (10, 12),
    ];
    for case in test_cases {
        assert_eq!(Solution::nth_ugly_number(case.0), case.1);
    }
}

#[test]
fn hamming_distance_test() {
    let test_cases = vec![
        (1, 4, 2),
    ];
    for case in test_cases {
        assert_eq!(Solution::hamming_distance(case.0, case.1), case.2);
    }
}

#[test]
fn hamming_distance_v2_test() {
    let test_cases = vec![
        (1, 4, 2),
    ];
    for case in test_cases {
        assert_eq!(Solution::hamming_distance_v2(case.0, case.1), case.2);
    }
}

#[test]
fn hamming_distance_v3_test() {
    let test_cases = vec![
        (1, 4, 2),
    ];
    for case in test_cases {
        assert_eq!(Solution::hamming_distance_v3(case.0, case.1), case.2);
    }
}

#[test]
fn plus_one_test() {
    let test_cases = vec![
        (
            vec![0],
            vec![1],
        ),
        (
            vec![1, 2, 3],
            vec![1, 2, 4],
        ),
        (
            vec![4, 3, 2, 1],
            vec![4, 3, 2, 2],
        ),
        (
            vec![9],
            vec![1, 0],
        ),
    ];
    for case in test_cases {
        assert_eq!(Solution::plus_one(case.0), case.1);
    }
}

#[test]
fn island_perimeter_test() {
    let test_cases = vec![
        (
            vec![],
            0
        ),
        (
            vec![vec![0]],
            0
        ),
        (
            vec![vec![1]],
            4
        ),
        (
            vec![
                vec![0,1,0,0],
                vec![1,1,1,0],
                vec![0,1,0,0],
                vec![1,1,0,0],
            ],
            16
        ),
    ];
    for case in test_cases {
        assert_eq!(Solution::island_perimeter(case.0), case.1);
    }
}

fn get_three_sum_test_cases() -> Vec<(Vec<i32>, Vec<Vec<i32>>)>{
    vec![
        (vec![], vec![]),
        (vec![0], vec![]),
        (vec![0, 0, 0], vec![vec![0, 0, 0]]),
        (vec![-1, 0, 1, 2, -1, -4], vec![vec![-1, -1, 2], vec![-1, 0, 1]]),
    ]
}

#[test]
fn three_sum_test() {
    for case in get_three_sum_test_cases() {
        assert_eq!(Solution::three_sum(case.0), case.1);
    }
}

#[test]
fn three_sum_test_v2() {
    for case in get_three_sum_test_cases() {
        assert_eq!(Solution::three_sum_v2(case.0), case.1);
    }
}
