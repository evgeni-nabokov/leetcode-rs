use super::*;

fn get_smallest_common_element_test_cases() -> Vec<(Vec<Vec<i32>>, i32)> {
    vec![
        (
            vec![
                vec![1, 2, 3, 4, 5],
                vec![2, 4, 5, 8, 10],
                vec![3, 5, 7, 9, 11],
                vec![1, 3, 5, 7, 9]
            ],
            5
        ),
        (
            vec![
                vec![1, 2, 3],
                vec![2, 3, 4],
                vec![2, 3, 5],
            ],
            2
        ),
        (
            vec![
                vec![1, 2, 3],
                vec![4, 5, 6],
                vec![7, 8, 9],
            ],
            -1
        ),
    ]
}

#[test]
fn smallest_common_element_test() {
    for case in get_smallest_common_element_test_cases() {
        assert_eq!(Solution::smallest_common_element(case.0), case.1);
    }
}

#[test]
fn smallest_common_element_v2_test() {
    for case in get_smallest_common_element_test_cases() {
        assert_eq!(Solution::smallest_common_element_v2(case.0), case.1);
    }
}