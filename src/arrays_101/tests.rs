use crate::arrays_101::Solution;

#[test]
fn find_max_consecutive_ones_test() {
    let test_cases = vec![
        (vec![], 0),
        (vec![0], 0),
        (vec![1], 1),
        (vec![1,0,1,1,0,1], 2),
        (vec![1,1,0,1,1,1], 3),
    ];
    for case in test_cases {
        assert_eq!(Solution::find_max_consecutive_ones(case.0), case.1);
    }
}

#[test]
fn find_numbers_test() {
    let test_cases = vec![
        (vec![], 0),
        (vec![0, 1], 0),
        (vec![-42], 1),
        (vec![1, 22], 1),
        (vec![100000], 1),
        (vec![555,901,482,1771], 1),
    ];
    for case in test_cases {
        assert_eq!(Solution::find_numbers(case.0), case.1);
    }
}