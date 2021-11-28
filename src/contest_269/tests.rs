use super::*;

#[test]
fn target_indices_tests() {
    let test_cases = vec![
        (vec![1, 2, 5, 2, 3], 2, vec![1, 2]),
        (vec![1, 2, 5, 2, 3], 3, vec![3]),
        (vec![1, 2, 5, 2, 3], 5, vec![4]),
    ];
    for case in test_cases {
        assert_eq!(Solution::target_indices(case.0, case.1), case.2);
    }
}