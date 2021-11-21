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

#[test]
fn watering_plants_test() {
    let test_cases = vec![
        (vec![2, 2, 3, 3], 5, 14),
        (vec![1, 1, 1, 4, 2, 3], 4, 30),
        (vec![7, 7, 7, 7, 7, 7, 7], 8, 49),
    ];
    for case in test_cases {
        assert_eq!(Solution::watering_plants(case.0, case.1), case.2);
    }
}