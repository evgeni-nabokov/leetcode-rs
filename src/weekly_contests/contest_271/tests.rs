use super::*;

#[test]
fn max_total_fruits_test() {
    let test_cases = vec![("B0B6G0R6R0R6G9", 1), ("B0R0G0R9R0B0G0", 1), ("G4", 0)];
    for case in test_cases {
        assert_eq!(Solution::count_points(case.0.to_string()), case.1)
    }
}

#[test]
fn sub_array_ranges_test() {
    let test_cases = vec![
        (vec![1, 2, 3], 4),
        (vec![1, 3, 3], 4),
        (vec![4, -2, -3, 4, 1], 59),
    ];
    for case in test_cases {
        assert_eq!(Solution::sub_array_ranges(case.0), case.1)
    }
}
