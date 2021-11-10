use super::*;

#[test]
fn count_vowel_substrings_test() {
    let test_cases = vec![
        ("eiou", 0),
        ("aeiouu", 2),
        ("unicornarihan", 0),
        ("cuaieuouac", 7),
        ("bbaeixoubb", 0),
        ("duuebuaeeeeeeuaoeiueaoui", 81)
    ];
    for case in test_cases {
        assert_eq!(Solution::count_vowel_substrings(case.0.to_string()), case.1);
    }
}

#[test]
fn count_vowels_test() {
    let test_cases = vec![
        ("aba", 6),
        ("abc", 3),
        ("ltcd", 0),
        ("noosabasboosa", 237),
    ];
    for case in test_cases {
        assert_eq!(Solution::count_vowels(case.0.to_string()), case.1);
    }
}

#[test]
fn minimized_maximum_test() {
    let test_cases = vec![
        (6, vec![11, 6], 3),
        (7, vec![15, 10, 10], 5),
        (1, vec![10_000], 10_000),
        (2, vec![1], 1),
    ];
    for case in test_cases {
        assert_eq!(Solution::minimized_maximum(case.0, case.1), case.2);
    }
}