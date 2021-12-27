use super::*;

#[test]
fn is_same_after_reversals_test() {
    let test_cases = vec![(526, true), (1800, false), (0, true)];
    for case in test_cases {
        assert_eq!(Solution::is_same_after_reversals(case.0), case.1);
    }
}

#[test]
fn get_distances_test() {
    let test_cases = vec![
        (vec![2, 1, 3, 1, 2, 3, 3], vec![4, 2, 7, 2, 4, 4, 5]),
        (vec![10, 5, 10, 10], vec![5, 0, 3, 4]),
    ];
    for case in test_cases {
        assert_eq!(Solution::get_distances(case.0), case.1);
    }
}
