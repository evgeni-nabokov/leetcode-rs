use super::*;
use super::trie::Trie;

#[test]
fn num_jewels_in_stones_test() {
    assert_eq!(Solution::num_jewels_in_stones("aA".to_string(), "aAAbbbb".to_string()), 3);
    assert_eq!(Solution::num_jewels_in_stones("z".to_string(), "ZZ".to_string()), 0);
}

fn get_can_construct_test_cases() -> Vec<(String, String, bool)> {
    vec![
        ("".to_string(), "".to_string(), true),
        ("a".to_string(), "".to_string(), false),
        ("".to_string(), "a".to_string(), true),
        ("a".to_string(), "b".to_string(), false),
        ("aa".to_string(), "ab".to_string(), false),
        ("aa".to_string(), "aab".to_string(), true),
    ]
}

#[test]
fn can_construct_test() {
    for case in get_can_construct_test_cases() {
        assert_eq!(Solution::can_construct(case.0, case.1), case.2);
    }
}

#[test]
fn can_construct_v2_test() {
    for case in get_can_construct_test_cases() {
        assert_eq!(Solution::can_construct_v2(case.0, case.1), case.2);
    }
}

fn get_bitwise_complement_test_cases() -> Vec<(i32, i32)> {
    vec![
        (1, 0),
        (0, 1),
        (8, 7),
        (5, 2),
        (7, 0),
        (10, 5)
    ]
}

#[test]
fn bitwise_complement_test() {
    for case in get_bitwise_complement_test_cases() {
        assert_eq!(Solution::bitwise_complement(case.0), case.1);
    }
}

#[test]
fn bitwise_complement_v2_test() {
    for case in get_bitwise_complement_test_cases() {
        assert_eq!(Solution::bitwise_complement_v2(case.0), case.1);
    }
}

#[test]
fn first_uniq_char_test() {
    assert_eq!(Solution::first_uniq_char("leetcode".to_string()), 0);
    assert_eq!(Solution::first_uniq_char("loveleetcode".to_string()), 2);
}

#[test]
fn majority_element_test() {
    assert_eq!(Solution::majority_element(vec![1]), 1);
    assert_eq!(Solution::majority_element(vec![3,2,3]), 3);
    assert_eq!(Solution::majority_element(vec![2,2,1,1,1,2,2]), 2);
}

#[test]
fn check_straight_line_test() {
    assert_eq!(Solution::check_straight_line(vec![
        vec![1,2],
        vec![2,3],
        vec![3,4],
        vec![4,5],
        vec![5,6],
    ]), true);

    assert_eq!(Solution::check_straight_line(vec![
        vec![1,1],
        vec![2,2],
        vec![3,4],
        vec![4,5],
        vec![5,6],
        vec![7,7],
    ]), false);
}

fn get_perfect_square_test_cases() -> Vec<(i32, bool)>{
    vec![
        (1, true),
        (2, false),
        (4, true),
        (9, true),
        (16, true),
        (14, false),
        (2147483647, false),
    ]
}

#[test]
fn is_perfect_square_test() {
    for case in get_perfect_square_test_cases() {
        assert_eq!(Solution::is_perfect_square(case.0), case.1);
    }
}

#[test]
fn is_perfect_square_v2_test() {
    for case in get_perfect_square_test_cases() {
        assert_eq!(Solution::is_perfect_square_v2(case.0), case.1);
    }
}

fn get_find_judge_test_cases() -> Vec<(i32, Vec<Vec<i32>>, i32)> {
    vec![
        (2, vec![vec![1, 2]], 2),
        (3, vec![vec![1, 2], vec![2, 3]], -1),
        (3, vec![vec![1, 3], vec![2, 3]], 3),
        (3, vec![vec![1, 3], vec![2, 3], vec![3, 1]], -1),
        (3, vec![vec![1, 2], vec![2, 3]], -1),
        (4, vec![vec![1, 3], vec![1, 4], vec![2, 3], vec![2, 4], vec![4, 3]], 3),
    ]
}

#[test]
fn find_judge_test() {
    for case in get_find_judge_test_cases() {
        assert_eq!(Solution::find_judge(case.0, case.1), case.2);
    }
}

#[test]
fn find_judge_v2_test() {
    for case in get_find_judge_test_cases() {
        assert_eq!(Solution::find_judge_v2(case.0, case.1), case.2);
    }
}

#[test]
fn flood_fill_test() {
    let test_cases = vec![
        (vec![
            vec![0,0,0],
            vec![0,1,1],
        ], 1, 1, 1,
         vec![
             vec![0,0,0],
             vec![0,1,1],
         ]),
        (vec![
            vec![0,0,0],
            vec![0,1,0],
        ], 1, 0, 2,
         vec![
             vec![2,2,2],
             vec![2,1,2],
        ]),
        (vec![
            vec![1,1,1],
            vec![1,1,0],
            vec![1,0,1]
        ], 1, 1, 1,
         vec![
            vec![1,1,1],
            vec![1,1,0],
            vec![1,0,1]
        ]),
        (vec![
            vec![1,1,1],
            vec![1,1,1],
            vec![1,1,1]
        ], 1, 1, 2,
        vec![
            vec![2,2,2],
            vec![2,2,2],
            vec![2,2,2]
        ]),
        (vec![
            vec![1,1,1],
            vec![1,1,0],
            vec![1,0,1]
        ], 1, 1, 2,
        vec![
            vec![2,2,2],
            vec![2,2,0],
            vec![2,0,1]
        ]),
    ];
    for case in test_cases {
        assert_eq!(Solution::flood_fill(case.0, case.1, case.2, case.3), case.4);
    }
}

#[test]
fn single_non_duplicate_test() {
    let test_cases = vec![
        (vec![1,2,2], 1),
        (vec![1,1,2], 2),
        (vec![1,1,2,3,3,4,4,8,8], 2),
        (vec![1,1,2,3,3,4,4], 2),
        (vec![1,1,3,3,4,4,7,8,8], 7),
        (vec![1,1,2,3,3], 2),
        (vec![1,1,2,2,3], 3),
        (vec![3,3,7,7,10,11,11], 10),
    ];
    for case in test_cases {
        assert_eq!(Solution::single_non_duplicate(case.0), case.1);
    }
}

#[test]
fn remove_kdigits_test() {
    let test_cases = vec![
        ("1432219".to_string(), 3, "1219".to_string()),
        ("10200".to_string(), 1, "200".to_string()),
        ("10".to_string(), 2, "0".to_string()),
    ];

    for case in test_cases {
        assert_eq!(Solution::remove_k_digits(case.0, case.1), case.2);
    }
}

#[test]
fn trie_test() {
    let mut trie = Trie::new();
    trie.insert("apple".to_string());
    assert_eq!(trie.search("apple".to_string()), true);
    assert_eq!(trie.search("app".to_string()), false);
    assert_eq!(trie.starts_with("app".to_string()), true);
    trie.insert("app".to_string());
    assert_eq!(trie.search("app".to_string()), true);
}