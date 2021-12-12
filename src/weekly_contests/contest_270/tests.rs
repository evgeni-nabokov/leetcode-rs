use super::*;

fn get_find_even_numbers_test_cases() -> Vec<(Vec<i32>, Vec<i32>)> {
    vec![
        (
            vec![2, 1, 3, 0],
            vec![102, 120, 130, 132, 210, 230, 302, 310, 312, 320],
        ),
        (
            vec![2, 2, 8, 8, 2],
            vec![222, 228, 282, 288, 822, 828, 882],
        ),
        (
            vec![3, 7, 5],
            vec![],
        ),
        (
            vec![0,2,0,0],
            vec![200],
        ),
        (
            vec![0,0,0],
            vec![],
        ),
    ]
}

#[test]
fn find_even_numbers_test() {
    for case in get_find_even_numbers_test_cases() {
        assert_eq!(Solution::find_even_numbers(case.0), case.1);
    }
}

#[test]
fn find_even_numbers_v2_test() {
    for case in get_find_even_numbers_test_cases() {
        assert_eq!(Solution::find_even_numbers_v2(case.0), case.1);
    }
}