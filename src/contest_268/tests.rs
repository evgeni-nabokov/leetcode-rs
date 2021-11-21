use super::*;

#[test]
fn max_distance_test() {
    let test_cases = vec![
        (vec![0, 1], 1),
        (vec![1, 1, 1, 6, 1, 1, 1], 3),
        (vec![1, 8, 3, 8, 3], 4),
    ];
    for case in test_cases {
        assert_eq!(Solution::max_distance(case.0), case.1);
    }
}