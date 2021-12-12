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


#[test]
fn minimum_deletions_test() {
    let test_cases = vec![
        (vec![2, 10, 7, 5, 4, 1, 8, 6], 5),
        (vec![0, -4, 19, 1, 8, -2, -3, 5], 3),
        (vec![101], 1),
    ];
    for case in test_cases {
        assert_eq!(Solution::minimum_deletions(case.0), case.1);
    }
}

#[test]
fn find_all_people_test() {
    let test_cases = vec![
        (6, vec![vec![1, 2, 5], vec![2, 3, 8], vec![1, 5, 10]], 1, vec![0, 1, 2, 3, 5]),
        (4, vec![vec![3, 1, 3], vec![1, 2, 2], vec![0, 3, 3]], 3, vec![0, 1, 3]),
        (5, vec![vec![3, 4, 2], vec![1, 2, 1], vec![2, 3, 1]], 1, vec![0, 1, 2, 3, 4]),
        (6, vec![vec![0, 2, 1], vec![1, 3, 1], vec![4, 5, 1]], 1, vec![0, 1, 2, 3]),
    ];
    for case in test_cases {
        assert_eq!(Solution::find_all_people(case.0, case.1, case.2), case.3);
    }
}