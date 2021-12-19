use super::*;

#[test]
fn first_palindrome_test() {
    let test_cases = vec![
        (vec!["abc", "car", "ada", "racecar", "cool"], "ada"),
        (vec!["notapalindrome", "racecar"], "racecar"),
        (vec!["def", "ghi"], ""),
    ];
    for case in test_cases {
        assert_eq!(
            Solution::first_palindrome(
                case.0
                    .into_iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>(),
            ),
            case.1.to_string()
        );
    }
}
