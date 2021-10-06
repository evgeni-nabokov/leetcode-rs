use super::*;

#[test]
fn invert_tree_test() {
    let test_cases = vec![
        (
            "", 100, "",
        ),
        (
            "Zebra-493?", 3, "Cheud-726?",
        ),
        (
            "abcdefghijklmNOPQRSTUVWXYZ0123456789", 39, "nopqrstuvwxyzABCDEFGHIJKLM9012345678",
        ),
    ];
    for case in test_cases {
        assert_eq!(Solution::rotational_cipher(case.0.to_string(), case.1), case.2);
    }
}

#[test]
fn count_subarrays_test() {
    let test_cases = vec![
        (vec![3, 4, 1, 6, 2], vec![1, 3, 1, 5, 1]),
        (vec![1], vec![1])
    ];
    for case in test_cases {
        assert_eq!(Solution::count_subarrays(case.0), case.1);
    }
}