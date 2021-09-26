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
