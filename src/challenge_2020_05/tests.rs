use super::*;
use super::trie::Trie;
use super::stock_spanner::{StockSpanner, StockSpannerV2};
use crate::common::tree_node::BinaryTree;

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

#[test]
fn max_subarray_sum_circular_test() {
    let test_cases = vec![
        (vec![5,-3,-2,6,-1,4], 14),
        (vec![1,-2,3,-2], 3),
        (vec![5,-3,5], 10),
        (vec![3,-1,2,-1], 4),
        (vec![3,-2,2,-3], 3),
        (vec![-2,-3,-1], -1),
    ];

    for case in test_cases {
        assert_eq!(Solution::max_subarray_sum_circular(case.0), case.1);
    }
}

#[test]
fn odd_even_list_test() {
    let test_cases = vec![
        (vec![1], vec![1]),
        (vec![1, 2], vec![1, 2]),
        (vec![1, 2, 3, 4, 5], vec![1, 3, 5, 2, 4]),
        (vec![2, 1, 3, 5, 6, 4, 7], vec![2, 3, 6, 7, 1, 5, 4]),
    ];
    for case in test_cases {
        assert_eq!(Solution::odd_even_list(ListNode::from_slice(&case.0)).unwrap().to_vec(), case.1);
    }
}

fn get_find_anagrams_test_cases() -> Vec<(String, String, Vec<i32>)> {
    vec![
        ("cbaebabacd".to_string(), "abc".to_string(), vec![0, 6]),
        ("abab".to_string(), "ab".to_string(), vec![0, 1, 2]),
    ]
}

#[test]
fn find_anagrams_test() {
    for case in get_find_anagrams_test_cases() {
        assert_eq!(Solution::find_anagrams(case.0, case.1), case.2);
    }
}

#[test]
fn find_anagrams_v2_test() {
    for case in get_find_anagrams_test_cases() {
        assert_eq!(Solution::find_anagrams_v2(case.0, case.1), case.2);
    }
}

#[test]
fn stock_spanner_test() {
    let mut spanner = StockSpanner::new();
    assert_eq!(spanner.next(29), 1);
    assert_eq!(spanner.next(91), 2);
    assert_eq!(spanner.next(62), 1);
    assert_eq!(spanner.next(76), 2);
    assert_eq!(spanner.next(51), 1);

    spanner = StockSpanner::new();
    assert_eq!(spanner.next(100), 1);
    assert_eq!(spanner.next(80), 1);
    assert_eq!(spanner.next(60), 1);
    assert_eq!(spanner.next(70), 2);
    assert_eq!(spanner.next(60), 1);
    assert_eq!(spanner.next(75), 4);
    assert_eq!(spanner.next(85), 6);
    assert_eq!(spanner.next(84), 1);
}

#[test]
fn stock_spanner_v2_test() {
    let mut spanner = StockSpannerV2::new();
    assert_eq!(spanner.next(29), 1);
    assert_eq!(spanner.next(91), 2);
    assert_eq!(spanner.next(62), 1);
    assert_eq!(spanner.next(76), 2);
    assert_eq!(spanner.next(51), 1);

    spanner = StockSpannerV2::new();
    assert_eq!(spanner.next(100), 1);
    assert_eq!(spanner.next(80), 1);
    assert_eq!(spanner.next(60), 1);
    assert_eq!(spanner.next(70), 2);
    assert_eq!(spanner.next(60), 1);
    assert_eq!(spanner.next(75), 4);
    assert_eq!(spanner.next(85), 6);
    assert_eq!(spanner.next(84), 1);
}

#[test]
fn kth_smallest_test() {
    let test_cases = vec![
        (vec![Some(3), Some(1), Some(4), None, Some(2)], 1, 1),
        (vec![Some(5), Some(3), Some(6), Some(2), Some(4), None, None, Some(1)], 3, 3),
    ];
    for case in test_cases {
        let tree = TreeNode::create_from_level_order(&case.0);
        assert_eq!(Solution::kth_smallest(tree, case.1), case.2);
    }
}

#[test]
fn count_squares_test() {
    let test_cases = vec![
        (vec![
        ], 0),
        (vec![
            vec![],
        ], 0),
        (vec![
            vec![0],
        ], 0),
        (vec![
            vec![1],
        ], 1),
        (vec![
            vec![1, 1],
        ], 2),
        (vec![
          vec![0,1,1,1],
          vec![1,1,1,1],
          vec![0,1,1,1]
          ], 15),
        (vec![
            vec![1,0,1],
            vec![1,1,0],
            vec![1,1,0]
        ], 7),
    ];
    for case in test_cases {
        assert_eq!(Solution::count_squares(case.0), case.1);
    }
}

#[test]
fn frequency_sort_test() {
    let test_cases = vec![
        ("tree".to_string(), "eert".to_string()),
        ("cccaaa".to_string(), "aaaccc".to_string()),
        ("Aabb".to_string(), "bbAa".to_string()),
    ];
    for case in test_cases {
        assert_eq!(Solution::frequency_sort(case.0), case.1);
    }
}

#[test]
fn frequency_sort_v2_test() {
    let test_cases = vec![
        ("tree".to_string(), "eert".to_string()),
        ("cccaaa".to_string(), "aaaccc".to_string()),
        ("Aabb".to_string(), "bbAa".to_string()),
        ("2a554442f544asfasssffffasss".to_string(), "sssssssffffff44444aaaa55522".to_string()),
    ];
    for case in test_cases {
        assert_eq!(Solution::frequency_sort_v2(case.0), case.1);
    }
}

#[test]
fn interval_intersection_test() {
    let test_cases = vec![
        (
            vec![],
            vec![vec![1, 2]],
            vec![],
        ),
        (
            vec![vec![1, 2]],
            vec![],
            vec![],
        ),
        (
            vec![vec![0,2],vec![5,10],vec![13,23],vec![24,25]],
            vec![vec![1,5],vec![8,12],vec![15,24],vec![25,26]],
            vec![vec![1,2],vec![5,5],vec![8,10],vec![15,23],vec![24,24],vec![25,25]]
        ),
    ];
    for case in test_cases {
        assert_eq!(Solution::interval_intersection(case.0, case.1), case.2);
    }
}

fn get_bst_from_preorder_test_cases() -> Vec<(Vec<i32>, Vec<Option<i32>>)> {
    vec![
        (vec![], vec![]),
        (vec![1], vec![Some(1)]),
        (vec![10, 5], vec![Some(10), Some(5), None]),
        (vec![10, 20], vec![Some(10), None, Some(20)]),
        (vec![8, 5, 1, 7, 10, 12], vec![Some(8), Some(5), Some(10), Some(1), Some(7), None, Some(12)]),
    ]
}

#[test]
fn bst_from_preorder_test() {
    for case in get_bst_from_preorder_test_cases() {
        assert_eq!(Solution::bst_from_preorder(case.0).get_level_order_values(), case.1);
    }
}

#[test]
fn bst_from_preorder_v2_test() {
    for case in get_bst_from_preorder_test_cases() {
        assert_eq!(Solution::bst_from_preorder_v2(case.0).get_level_order_values(), case.1);
    }
}

#[test]
fn max_uncrossed_lines_test() {
    let test_cases = vec![
        (vec![], vec![2], 0),
        (vec![1], vec![], 0),
        (vec![2,5,1,2,5], vec![10,5,2,1,5,2], 3),
        (vec![1,3,7,1,7,5], vec![1,9,2,5,1], 2),
    ];
    for case in test_cases {
        assert_eq!(Solution::max_uncrossed_lines(case.0, case.1), case.2);
    }
}

#[test]
fn possible_bipartition_test() {
    let test_cases = vec![
        (0, vec![], true),
        (1, vec![], true),
        (2, vec![], true),
        (2, vec![vec![1, 2]], true),
        (4, vec![vec![1,2],vec![1,3],vec![2,4]], true),
        (3, vec![vec![1,2],vec![1,3],vec![2,3]], false),
        (5, vec![vec![1,2],vec![2,3],vec![3,4],vec![4, 5], vec![1, 5]], false),
    ];
    for case in test_cases {
        assert_eq!(Solution::possible_bipartition(case.0, case.1), case.2);
    }
}