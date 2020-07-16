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

fn get_sorted_squares_test_cases() -> Vec<(Vec<i32>, Vec<i32>)> {
    vec![
        (vec![], vec![]),
        (vec![-2, 0], vec![0, 4]),
        (vec![1, 2], vec![1, 4]),
        (vec![-4,-1,0,3,10], vec![0,1,9,16,100]),
        (vec![-7,-3,2,3,11], vec![4,9,9,49,121]),
    ]
}

#[test]
fn sorted_squares_test() {
    for case in get_sorted_squares_test_cases() {
        assert_eq!(Solution::sorted_squares(case.0), case.1);
    }
}

#[test]
fn sorted_squares_v2_test() {
    for case in get_sorted_squares_test_cases() {
        assert_eq!(Solution::sorted_squares_v2(case.0), case.1);
    }
}

#[test]
fn duplicate_zeros_test() {
    let test_cases = vec![
        (vec![], vec![]),
        (vec![0], vec![0]),
        (vec![0, 0], vec![0, 0]),
        (vec![0, 1], vec![0, 0]),
        (vec![1, 0], vec![1, 0]),
        (vec![1, 0, 2, 0, 3, 4], vec![1, 0, 0, 2, 0, 0]),
        (vec![1, 0, 2, 3, 0, 4, 5, 0], vec![1, 0, 0, 2, 3, 0, 0, 4]),
        (vec![8, 0, 3, 0, 1], vec![8, 0, 0, 3, 0]),
        (vec![9,9,9,4,8,0,0,3,7,2,0,0,0,0,9,1,0,0,1,1,0,5,6,3,1,6,0,0,2,3,4,7,0,3,9,3,6,5,8,9,1,1,3,2,0,0,7,3,3,0,5,7,0,8,1,9,6,3,0,8,8,8,8,0,0,5,0,0,0,3,7,7,7,7,5,1,0,0,8,0,0],
         vec![9,9,9,4,8,0,0,0,0,3,7,2,0,0,0,0,0,0,0,0,9,1,0,0,0,0,1,1,0,0,5,6,3,1,6,0,0,0,0,2,3,4,7,0,0,3,9,3,6,5,8,9,1,1,3,2,0,0,0,0,7,3,3,0,0,5,7,0,0,8,1,9,6,3,0,0,8,8,8,8,0])
    ];
    for mut case in test_cases {
        Solution::duplicate_zeros(&mut case.0);
        assert_eq!(case.0, case.1);
    }
}