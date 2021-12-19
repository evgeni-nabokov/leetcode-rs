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

#[test]
fn add_spaces_test() {
    let test_cases = vec![
        (
            "LeetcodeHelpsMeLearn",
            vec![8, 13, 15],
            "Leetcode Helps Me Learn",
        ),
        ("icodeinpython", vec![1, 5, 7, 9], "i code in py thon"),
        ("spacing", vec![0, 1, 2, 3, 4, 5, 6], " s p a c i n g"),
    ];
    for case in test_cases {
        assert_eq!(
            Solution::add_spaces(case.0.to_string(), case.1),
            case.2.to_string()
        );
    }
}
