use super::*;

#[test]
fn dest_city_test() {
    assert_eq!(Solution::dest_city(vec![
        vec!["Moscow".to_string(), "Minsk".to_string()],
        vec!["Perm".to_string(), "Moscow".to_string()]
    ]), "Minsk".to_string());
}

#[test]
fn k_length_apart() {
    assert_eq!(Solution::k_length_apart(vec![1,0,0,0,1,0,0,1], 2), true);
    assert_eq!(Solution::k_length_apart(vec![1,0,0,1,0,1], 2), false);
    assert_eq!(Solution::k_length_apart(vec![0,1,0,1], 1), true);
    assert_eq!(Solution::k_length_apart(vec![1,1,1,1,1], 0), true);
    assert_eq!(Solution::k_length_apart(vec![0, 0, 0, 0], 1), true);
}

#[test]
fn longest_subarray_test() {
    let test_cases = vec![
        (vec![], 2, 0),
        (vec![8], 5, 1),
        (vec![8], 10, 1),
        (vec![8, 2], 0, 1),
        (vec![8, 2, 4, 7], 4, 2),
        (vec![10, 1, 2, 4, 7, 2], 5, 4),
        (vec![4, 2, 2, 2, 4, 4, 2, 2], 0, 3),
        (vec![4, 8, 5, 1, 7, 9], 6, 3),
    ];
    for case in test_cases {
        assert_eq!(Solution::longest_subarray(case.0, case.1), case.2);
    }
}