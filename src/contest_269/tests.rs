use super::*;

#[test]
fn target_indices_test() {
    let test_cases = vec![
        (vec![1, 2, 5, 2, 3], 2, vec![1, 2]),
        (vec![1, 2, 5, 2, 3], 3, vec![3]),
        (vec![1, 2, 5, 2, 3], 5, vec![4]),
    ];
    for case in test_cases {
        assert_eq!(Solution::target_indices(case.0, case.1), case.2);
    }
}

fn get_get_averages_test_cases() -> Vec<(Vec<i32>, i32, Vec<i32>)> {
    vec![
        (vec![7, 4, 3, 9, 1, 8, 5, 2, 6], 3, vec![-1, -1, -1, 5, 4, 4, -1, -1, -1]),
        (vec![7, 4, 3, 9, 1, 8, 5, 2, 6], 0, vec![7, 4, 3, 9, 1, 8, 5, 2, 6]),
        (vec![10000], 0, vec![10000]),
        (vec![1], 100, vec![-1]),
    ]
}

#[test]
fn get_averages_test() {
    for case in get_get_averages_test_cases() {
        assert_eq!(Solution::get_averages(case.0, case.1), case.2);
    }
}

#[test]
fn get_averages_v2_test() {
    for case in get_get_averages_test_cases() {
        assert_eq!(Solution::get_averages_v2(case.0, case.1), case.2);
    }
}