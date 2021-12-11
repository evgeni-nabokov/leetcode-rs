use super::*;

#[test]
fn count_vowel_substrings_test() {
    let test_cases = vec![
        ("eiou", 0),
        ("aeiouu", 2),
        ("unicornarihan", 0),
        ("cuaieuouac", 7),
        ("bbaeixoubb", 0),
        ("duuebuaeeeeeeuaoeiueaoui", 81),
    ];
    for case in test_cases {
        assert_eq!(Solution::count_vowel_substrings(case.0.to_string()), case.1);
    }
}

#[test]
fn count_vowels_test() {
    let test_cases = vec![("aba", 6), ("abc", 3), ("ltcd", 0), ("noosabasboosa", 237)];
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

#[test]
fn maximal_path_quality_test() {
    let test_cases = vec![
        (
            vec![0, 32, 10, 43],
            vec![vec![0, 1, 10], vec![1, 2, 15], vec![0, 3, 10]],
            49,
            75,
        ),
        (
            vec![5, 10, 15, 20],
            vec![vec![0, 1, 10], vec![1, 2, 10], vec![0, 3, 10]],
            30,
            25,
        ),
        (
            vec![1, 2, 3, 4],
            vec![
                vec![0, 1, 10],
                vec![1, 2, 11],
                vec![2, 3, 12],
                vec![1, 3, 13],
            ],
            50,
            7,
        ),
        (vec![0, 1, 2], vec![vec![1, 2, 10]], 10, 0),
    ];
    for case in test_cases {
        assert_eq!(
            Solution::maximal_path_quality(case.0, case.1, case.2),
            case.3
        );
    }
}
