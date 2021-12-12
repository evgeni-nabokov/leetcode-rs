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

#[test]
fn minimum_refill_test() {
    let test_cases = vec![
        (vec![2, 2, 3, 3], 5, 5, 1),
        (vec![2, 2, 3, 3], 3, 4, 2),
        (vec![5], 10, 8, 0),
        (vec![2, 2, 5, 2, 2], 5, 5, 1),
    ];
    for case in test_cases {
        assert_eq!(Solution::minimum_refill(case.0, case.1, case.2), case.3)
    }
}
