use super::*;

#[test]
fn max_distance_test() {
    let test_cases = vec![
        (
            vec![
                vec![1, 2, 3],
                vec![4, 5],
                vec![1, 2, 3]
            ],
            4
        ),
        (
            vec![
                vec![-1, 2, 3],
                vec![4, 5],
                vec![-1, 2, 3]
            ],
            6
        ),
        (
            vec![
                vec![-8, -7, -7, -5, 1, 1, 3, 4],
                vec![-2],
                vec![-10, -10, -7, 0, 1, 3],
                vec![2]
            ],
            14
        ),
        (
            vec![
                vec![1, 2, 3, 4, 5, 6],
                vec![3, 4],
                vec![4, 5],
            ],
            4
        ),
    ];
    for case in test_cases {
        assert_eq!(Solution::max_distance(case.0), case.1);
    }
}
