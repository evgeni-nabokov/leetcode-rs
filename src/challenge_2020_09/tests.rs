use super::*;

#[test]
fn largest_time_from_digits_test() {
    let test_cases = vec![
        (vec![1, 2, 3, 4], "23:41"),
        (vec![5, 5, 5, 5], ""),
        (vec![0, 4, 0, 0], "04:00"),
        (vec![2, 0, 6, 6], "06:26"),
    ];
    for case in test_cases {
        assert_eq!(Solution::largest_time_from_digits(case.0), case.1.to_string());
    }
}

#[test]
fn contains_nearby_almost_duplicate_test() {
    let test_cases = vec![
        (vec![1, 2, 3, 1], 3, 0, true),
        (vec![1, 0, 1, 1], 1, 2, true),
        (vec![1, 5, 9, 1, 5, 9], 2, 3, false),
        (vec![7, 2, 8], 2, 1, true),
        (vec![-2147483648, -2147483647], 3, 3, true),
    ];
    for case in test_cases {
        assert_eq!(Solution::contains_nearby_almost_duplicate(case.0, case.1, case.2), case.3);
    }
}
