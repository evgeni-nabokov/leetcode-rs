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