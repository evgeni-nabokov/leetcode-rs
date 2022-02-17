use super::bst_iterator::BSTIterator;
use super::snapshot_array::SnapshotArray;
use super::*;
use crate::common::tree_node::BinaryTree;

#[test]
fn unique_paths_with_obstacles_test() {
    let test_cases = vec![
        (vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]], 2),
        (vec![vec![0, 1], vec![0, 0]], 1),
    ];
    for case in test_cases {
        assert_eq!(Solution::unique_paths_with_obstacles(case.0), case.1);
    }
}

fn get_simplify_path_test_cases<'a>() -> Vec<(&'a str, &'a str)> {
    vec![
        ("/", "/"),
        ("/home/", "/home"),
        ("/../", "/"),
        ("/home//foo/", "/home/foo"),
    ]
}

#[test]
fn simplify_path_test() {
    for case in get_simplify_path_test_cases() {
        assert_eq!(
            Solution::simplify_path(case.0.to_string()),
            case.1.to_string()
        );
    }
}

#[test]
fn simplify_path_test_v2() {
    for case in get_simplify_path_test_cases() {
        assert_eq!(
            Solution::simplify_path_v2(case.0.to_string()),
            case.1.to_string()
        );
    }
}

#[test]
fn h_index_test() {
    let test_cases = vec![
        (vec![], 0),
        (vec![0], 0),
        (vec![1], 1),
        (vec![10], 1),
        (vec![10, 10], 2),
        (vec![0, 0, 0, 0], 0),
        (vec![1, 1, 1, 1], 1),
        (vec![2, 2, 2], 2),
        (vec![3, 3, 3], 3),
        (vec![2, 1, 1, 1], 1),
        (vec![6, 3, 1, 5, 0], 3),
        (vec![0, 6, 4, 5, 4], 4),
        (vec![10, 14, 0, 12, 10, 1], 4),
        (vec![14, 1, 1, 12, 1, 0], 2),
    ];
    for case in test_cases {
        assert_eq!(Solution::h_index(case.0), case.1);
    }
}

#[test]
fn remove_leaf_nodes_test() {
    let test_cases = vec![
        (vec![Some(1)], 1, vec![]),
        (vec![Some(1), Some(1), None], 1, vec![]),
        (vec![Some(1), Some(1), Some(1)], 1, vec![]),
        (
            vec![Some(1), Some(1), Some(2)],
            1,
            vec![Some(1), None, Some(2)],
        ),
        (
            vec![Some(1), Some(2), Some(3), Some(2), None, Some(2), Some(4)],
            2,
            vec![Some(1), None, Some(3), None, Some(4)],
        ),
        (
            vec![Some(1), Some(3), Some(3), Some(3), Some(2)],
            3,
            vec![Some(1), Some(3), None, None, Some(2)],
        ),
        (
            vec![Some(1), Some(2), None, Some(2), None, Some(2)],
            2,
            vec![Some(1)],
        ),
        (vec![Some(1), Some(1), Some(1)], 1, vec![]),
        (
            vec![Some(1), Some(2), Some(3)],
            1,
            vec![Some(1), Some(2), Some(3)],
        ),
    ];
    for case in test_cases {
        let tree = TreeNode::from_level_order(&case.0);
        assert_eq!(
            Solution::remove_leaf_nodes(tree, case.1).get_level_order_values(),
            case.2
        );
    }
}

#[test]
fn level_order_test() {
    let test_cases = vec![
        (
            vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)],
            vec![vec![3], vec![9, 20], vec![15, 7]],
        ),
        (
            vec![Some(1), Some(2), Some(3), Some(4), None, None, Some(5)],
            vec![vec![1], vec![2, 3], vec![4, 5]],
        ),
    ];
    for case in test_cases {
        assert_eq!(
            Solution::level_order(TreeNode::from_level_order(&case.0)),
            case.1
        );
    }
}

#[test]
fn build_tree_test() {
    let test_cases = vec![
        (vec![], vec![], vec![]),
        (vec![1, 2], vec![2, 1], vec![Some(1), Some(2)]),
        (
            vec![5, 3, 2, 4, 6],
            vec![2, 4, 3, 6, 5],
            vec![Some(5), Some(3), Some(6), Some(2), Some(4)],
        ),
        (
            vec![1, 2, 4, 5, 3, 6, 7],
            vec![4, 5, 2, 6, 7, 3, 1],
            vec![
                Some(1),
                Some(2),
                Some(3),
                Some(4),
                Some(5),
                Some(6),
                Some(7),
            ],
        ),
    ];
    for case in test_cases {
        assert_eq!(
            Solution::build_tree(case.0, case.1).get_level_order_values(),
            case.2
        );
    }
}

#[test]
fn build_tree_ii_test() {
    let test_cases = vec![
        (vec![], vec![], vec![]),
        (vec![1, 2], vec![2, 1], vec![Some(1), Some(2)]),
        (
            vec![5, 3, 2, 4, 6],
            vec![2, 3, 4, 5, 6],
            vec![Some(5), Some(3), Some(6), Some(2), Some(4)],
        ),
        (
            vec![3, 9, 20, 15, 7],
            vec![9, 3, 15, 20, 7],
            vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)],
        ),
    ];
    for case in test_cases {
        assert_eq!(
            Solution::build_tree_ii(case.0, case.1).get_level_order_values(),
            case.2
        );
    }
}

#[test]
fn combination_sum_test() {
    let test_cases = vec![
        (vec![2, 3, 6, 7], 7, vec![vec![3, 2, 2], vec![7]]),
        (
            vec![2, 3, 5],
            8,
            vec![vec![2, 2, 2, 2], vec![3, 3, 2], vec![5, 3]],
        ),
        (
            vec![8, 7, 4, 3],
            11,
            vec![vec![4, 4, 3], vec![8, 3], vec![7, 4]],
        ),
    ];
    for case in test_cases {
        assert_eq!(Solution::combination_sum(case.0, case.1), case.2);
    }
}

#[test]
fn combination_sum_ii_test() {
    let test_cases = vec![
        (
            vec![10, 1, 2, 7, 6, 1, 5],
            8,
            vec![vec![6, 1, 1], vec![5, 2, 1], vec![7, 1], vec![6, 2]],
        ),
        (vec![2, 5, 2, 1, 2], 5, vec![vec![2, 2, 1], vec![5]]),
    ];
    for case in test_cases {
        assert_eq!(Solution::combination_sum_ii(case.0, case.1), case.2);
    }
}

#[test]
fn coin_change_test() {
    let test_cases = vec![
        (vec![1], 0, 0),
        (vec![1], 1, 1),
        (vec![1, 2, 5], 11, 3),
        (vec![2], 3, -1),
    ];
    for case in test_cases {
        assert_eq!(Solution::coin_change(case.0, case.1), case.2);
    }
}

#[test]
fn my_atoi_test() {
    let test_cases = vec![
        ("42", 42),
        ("   -42", -42),
        ("4193 with words", 4193),
        ("words and 987", 0),
    ];
    for case in test_cases {
        assert_eq!(Solution::my_atoi(case.0.to_string()), case.1);
    }
}

fn get_reverse_words_ii_test_cases() -> Vec<(Vec<char>, Vec<char>)> {
    vec![
        (vec!['a'], vec!['a']),
        (
            vec![
                't', 'h', 'e', ' ', 's', 'k', 'y', ' ', 'i', 's', ' ', 'b', 'l', 'u', 'e',
            ],
            vec![
                'b', 'l', 'u', 'e', ' ', 'i', 's', ' ', 's', 'k', 'y', ' ', 't', 'h', 'e',
            ],
        ),
    ]
}

#[test]
fn reverse_words_ii_test() {
    for mut case in get_reverse_words_ii_test_cases() {
        Solution::reverse_words_ii(&mut case.0);
        assert_eq!(case.0, case.1);
    }
}

#[test]
fn reverse_words_ii_v2_test() {
    for mut case in get_reverse_words_ii_test_cases() {
        Solution::reverse_words_ii_v2(&mut case.0);
        assert_eq!(case.0, case.1);
    }
}

fn get_is_valid_bst_test_cases() -> Vec<(Vec<Option<i32>>, bool)> {
    vec![
        (vec![Some(2), Some(1), Some(3)], true),
        (
            vec![Some(5), Some(1), Some(4), None, None, Some(3), Some(6)],
            false,
        ),
        (
            vec![Some(5), Some(4), Some(6), None, None, Some(3), Some(7)],
            false,
        ),
        (vec![Some(1), Some(1)], false),
        (vec![Some(2147483647)], true),
    ]
}

#[test]
fn is_valid_bst_test() {
    for case in get_is_valid_bst_test_cases() {
        assert_eq!(
            Solution::is_valid_bst(TreeNode::from_level_order(&case.0)),
            case.1
        );
    }
}

#[test]
fn is_valid_bst_v2_test() {
    for case in get_is_valid_bst_test_cases() {
        assert_eq!(
            Solution::is_valid_bst_v2(TreeNode::from_level_order(&case.0)),
            case.1
        );
    }
}

#[test]
fn is_valid_sudoku_test() {
    let test_cases = vec![
        (
            vec![
                vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
            ],
            true,
        ),
        (
            vec![
                vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
            ],
            false,
        ),
    ];
    for case in test_cases {
        assert_eq!(Solution::is_valid_sudoku(case.0), case.1);
    }
}

#[test]
fn longest_palindrome_test() {
    let test_cases = vec![
        ("a", "a"),
        ("bb", "bb"),
        ("bab", "bab"),
        ("babad", "aba"),
        ("cbbd", "bb"),
        ("ac", "a"),
    ];
    for case in test_cases {
        assert_eq!(
            Solution::longest_palindrome(case.0.to_string()),
            case.1.to_string()
        );
    }
}

#[test]
fn max_area_test() {
    let test_cases = vec![
        (5, 4, vec![1, 2, 4], vec![1, 3], 4),
        (5, 4, vec![3], vec![3], 9),
        (100000, 100000, vec![1], vec![1], 999799938),
    ];
    for case in test_cases {
        assert_eq!(Solution::max_area(case.0, case.1, case.2, case.3), case.4);
    }
}

#[test]
fn partition_labels_test() {
    let test_cases = vec![
        ("abcabcde", [6, 1, 1]),
        ("ababcbacadefegdehijhklij", [9, 7, 8]),
    ];
    for case in test_cases {
        assert_eq!(Solution::partition_labels(case.0.to_string()), case.1);
    }
}

#[test]
fn all_paths_source_target_test() {
    let test_cases = vec![
        (
            vec![vec![1, 2], vec![3], vec![3], vec![]],
            vec![vec![0, 1, 3], vec![0, 2, 3]],
        ),
        (
            vec![vec![4, 3, 1], vec![3, 2, 4], vec![3], vec![4], vec![]],
            vec![
                vec![0, 4],
                vec![0, 3, 4],
                vec![0, 1, 3, 4],
                vec![0, 1, 2, 3, 4],
                vec![0, 1, 4],
            ],
        ),
        (vec![vec![1], vec![]], vec![vec![0, 1]]),
        (
            vec![vec![1, 2, 3], vec![2], vec![3], vec![]],
            vec![vec![0, 1, 2, 3], vec![0, 2, 3], vec![0, 3]],
        ),
        (
            vec![vec![1, 3], vec![2], vec![3], vec![]],
            vec![vec![0, 1, 2, 3], vec![0, 3]],
        ),
    ];
    for case in test_cases {
        assert_eq!(Solution::all_paths_source_target(case.0), case.1);
    }
}

#[test]
fn lowest_common_ancestor_test() {
    let test_cases = vec![
        (
            vec![
                Some(3),
                Some(5),
                Some(1),
                Some(6),
                Some(2),
                Some(0),
                Some(8),
                None,
                None,
                Some(7),
                Some(4),
            ],
            5,
            1,
            3,
        ),
        (
            vec![
                Some(3),
                Some(5),
                Some(1),
                Some(6),
                Some(2),
                Some(0),
                Some(8),
                None,
                None,
                Some(7),
                Some(4),
            ],
            5,
            4,
            5,
        ),
    ];
    for case in test_cases {
        assert_eq!(
            Solution::lowest_common_ancestor(
                TreeNode::from_level_order(&case.0),
                TreeNode::new_with_children(Some(case.1), None, None),
                TreeNode::new_with_children(Some(case.2), None, None)
            )
            .unwrap()
            .borrow()
            .val,
            case.3
        );
    }
}

fn get_remove_duplicates_test_cases<'a>() -> Vec<(&'a str, i32, &'a str)> {
    vec![
        ("aaaabcdeeef", 2, "bcdef"),
        ("abcd", 2, "abcd"),
        ("deeedbbcccbdaa", 3, "aa"),
        ("pbbcggttciiippooaais", 2, "ps"),
    ]
}

#[test]
fn remove_duplicates_test() {
    for case in get_remove_duplicates_test_cases() {
        assert_eq!(
            Solution::remove_duplicates(case.0.to_string(), case.1),
            case.2.to_string()
        );
    }
}

#[test]
fn remove_duplicates_v2_test() {
    for case in get_remove_duplicates_test_cases() {
        assert_eq!(
            Solution::remove_duplicates_v2(case.0.to_string(), case.1),
            case.2.to_string()
        );
    }
}

fn get_find_kth_largest_test_cases() -> Vec<(Vec<i32>, i32, i32)> {
    vec![
        (vec![3, 2, 1, 5, 6, 4], 2, 5),
        (vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4, 4),
    ]
}

#[test]
fn find_kth_largest_test() {
    for case in get_find_kth_largest_test_cases() {
        assert_eq!(Solution::find_kth_largest(case.0, case.1), case.2);
    }
}

#[test]
fn find_kth_largest_v2_test() {
    for case in get_find_kth_largest_test_cases() {
        assert_eq!(Solution::find_kth_largest_v2(case.0, case.1), case.2);
    }
}

#[test]
fn find_kth_largest_v3_test() {
    for case in get_find_kth_largest_test_cases() {
        assert_eq!(Solution::find_kth_largest_v3(case.0, case.1), case.2);
    }
}

#[test]
fn find_kth_largest_v4_test() {
    for case in get_find_kth_largest_test_cases() {
        assert_eq!(Solution::find_kth_largest_v4(case.0, case.1), case.2);
    }
}

#[test]
fn good_nodes_test() {
    let test_cases = vec![
        (vec![Some(1)], 1),
        (
            vec![Some(3), Some(1), Some(4), Some(3), None, Some(1), Some(5)],
            4,
        ),
        (vec![Some(3), Some(3), None, Some(4), Some(2)], 3),
    ];
    for case in test_cases {
        assert_eq!(
            Solution::good_nodes(TreeNode::from_level_order(&case.0)),
            case.1
        );
    }
}

fn get_suggested_products_cases<'a>() -> Vec<(Vec<&'a str>, &'a str, Vec<Vec<&'a str>>)> {
    vec![
        (
            vec!["mobile", "mouse", "moneypot", "monitor", "mousepad"],
            "mouse",
            vec![
                vec!["mobile", "moneypot", "monitor"],
                vec!["mobile", "moneypot", "monitor"],
                vec!["mouse", "mousepad"],
                vec!["mouse", "mousepad"],
                vec!["mouse", "mousepad"],
            ],
        ),
        (
            vec!["havana"],
            "havana",
            vec![
                vec!["havana"],
                vec!["havana"],
                vec!["havana"],
                vec!["havana"],
                vec!["havana"],
                vec!["havana"],
            ],
        ),
    ]
}

#[test]
fn suggested_products_test() {
    for case in get_suggested_products_cases() {
        assert_eq!(
            Solution::suggested_products(
                case.0.into_iter().map(|x| x.to_string()).collect(),
                case.1.to_string()
            ),
            case.2
                .into_iter()
                .map(|x| x.into_iter().map(|y| y.to_string()).collect())
                .collect::<Vec<Vec<String>>>()
        );
    }
}

#[test]
fn bst_iterator_test() {
    let tree = TreeNode::from_level_order(&vec![Some(1), Some(2), Some(3)]);
    let mut obj = BSTIterator::new(tree);
    assert_eq!(obj.has_next(), true);
    assert_eq!(obj.next(), 2);
    assert_eq!(obj.next(), 1);
    assert_eq!(obj.next(), 3);
    assert_eq!(obj.has_next(), false);
}

#[test]
fn recover_tree_test() {
    let test_cases = vec![
        (
            vec![Some(1), Some(3), None, None, Some(2)],
            vec![Some(3), Some(1), None, None, Some(2)],
        ),
        (
            vec![Some(3), Some(1), Some(4), None, None, Some(2)],
            vec![Some(2), Some(1), Some(4), None, None, Some(3)],
        ),
    ];
    for case in test_cases {
        let mut tree = TreeNode::from_level_order(&case.0);
        Solution::recover_tree(&mut tree);
        assert_eq!(tree.get_level_order_values(), case.1);
    }
}

#[test]
fn num_decodings_test() {
    let test_cases = vec![
        ("0", 0),
        ("12", 2),
        ("226", 3),
        ("06", 0),
        ("00", 0),
        ("10", 1),
        ("2101", 1),
        ("2125", 5),
    ];
    for case in test_cases {
        assert_eq!(Solution::num_decodings(case.0.to_string()), case.1);
    }
}

#[test]
fn rob_test() {
    let test_cases = vec![
        (
            vec![Some(3), Some(2), Some(3), None, Some(3), None, Some(1)],
            7,
        ),
        (
            vec![Some(3), Some(4), Some(5), Some(1), Some(3), None, Some(1)],
            9,
        ),
    ];
    for case in test_cases {
        assert_eq!(Solution::rob(TreeNode::from_level_order(&case.0)), case.1);
    }
}

#[test]
fn compress_test() {
    let test_cases = vec![
        (
            vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'],
            vec!['a', '2', 'b', '2', 'c', '3'],
        ),
        (vec!['a'], vec!['a']),
        (
            vec![
                'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b',
            ],
            vec!['a', 'b', '1', '2'],
        ),
        (
            vec![
                'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'c',
            ],
            vec!['a', 'b', '1', '2', 'c'],
        ),
        (
            vec!['a', 'a', 'a', 'b', 'b', 'a', 'a'],
            vec!['a', '3', 'b', '2', 'a', '2'],
        ),
    ];
    for mut case in test_cases {
        let new_size = Solution::compress(&mut case.0) as usize;
        case.0.resize(new_size, '#');
        assert_eq!(case.0, case.1);
    }
}

fn get_can_reach_test_cases() -> Vec<(Vec<i32>, i32, bool)> {
    vec![
        (vec![4, 2, 3, 0, 3, 1, 2], 5, true),
        (vec![4, 2, 3, 0, 3, 1, 2], 0, true),
        (vec![3, 0, 2, 1, 2], 2, false),
    ]
}

#[test]
fn can_reach_test() {
    for case in get_can_reach_test_cases() {
        assert_eq!(Solution::can_reach(case.0, case.1), case.2);
    }
}

#[test]
fn can_reach_v2_test() {
    for case in get_can_reach_test_cases() {
        assert_eq!(Solution::can_reach_v2(case.0, case.1), case.2);
    }
}

fn get_num_tilings_test_cases() -> Vec<(i32, i32)> {
    vec![(3, 5), (4, 11), (5, 24), (30, 312342182)]
}

#[test]
fn num_tilings_test() {
    for case in get_num_tilings_test_cases() {
        assert_eq!(Solution::num_tilings(case.0), case.1);
    }
}

#[test]
fn num_tilings_v2_test() {
    for case in get_num_tilings_test_cases() {
        assert_eq!(Solution::num_tilings_v2(case.0), case.1);
    }
}

#[test]
fn max_score_test() {
    let test_cases = vec![
        (vec![1, 2, 3, 4, 5, 6, 1], 3, 12),
        (vec![2, 2, 2], 2, 4),
        (vec![9, 7, 7, 9, 7, 7, 9], 7, 55),
        (vec![1, 1000, 1], 1, 1),
        (vec![1, 1000, 1], 3, 1002),
        (vec![1, 79, 80, 1, 1, 1, 200, 1], 3, 202),
    ];
    for case in test_cases {
        assert_eq!(Solution::max_score(case.0, case.1), case.2);
    }
}

#[test]
fn can_partition_test() {
    let test_cases = vec![(vec![1, 5, 11, 5], true), (vec![1, 2, 3, 5], false)];
    for case in test_cases {
        assert_eq!(Solution::can_partition(case.0), case.1);
    }
}

#[test]
fn restore_ip_addresses_test() {
    let test_cases = vec![
        ("0000", vec!["0.0.0.0"]),
        ("000125", vec!["0.0.0.125"]),
        ("25525511135", vec!["255.255.11.135", "255.255.111.35"]),
        (
            "101023",
            vec![
                "1.0.10.23",
                "1.0.102.3",
                "10.1.0.23",
                "10.10.2.3",
                "101.0.2.3",
            ],
        ),
    ];
    for case in test_cases {
        assert_eq!(
            Solution::restore_ip_addresses(case.0.to_string()),
            case.1
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
        );
    }
}

fn get_length_of_lis_test_cases() -> Vec<(Vec<i32>, i32)> {
    vec![
        (vec![10, 9, 2, 5, 3, 7, 101, 18], 4),
        (vec![0, 1, 0, 3, 2, 3], 4),
        (vec![7, 7, 7, 7, 7, 7, 7], 1),
    ]
}

#[test]
fn length_of_lis_test() {
    for case in get_length_of_lis_test_cases() {
        assert_eq!(Solution::length_of_lis(case.0), case.1);
    }
}

#[test]
fn length_of_lis_v2_test() {
    for case in get_length_of_lis_test_cases() {
        assert_eq!(Solution::length_of_lis_v2(case.0), case.1);
    }
}

fn get_right_side_view_test_cases() -> Vec<(Vec<Option<i32>>, Vec<i32>)> {
    vec![
        (
            vec![Some(1), Some(2), Some(3), None, Some(5), None, Some(4)],
            vec![1, 3, 4],
        ),
        (vec![Some(1), None, Some(3)], vec![1, 3]),
    ]
}

#[test]
fn right_side_view_test() {
    for case in get_right_side_view_test_cases() {
        assert_eq!(
            Solution::right_side_view(TreeNode::from_level_order(&case.0)),
            case.1
        );
    }
}

#[test]
fn right_side_view_v2_test() {
    for case in get_right_side_view_test_cases() {
        assert_eq!(
            Solution::right_side_view_v2(TreeNode::from_level_order(&case.0)),
            case.1
        );
    }
}

fn get_daily_temperatures_test_cases() -> Vec<(Vec<i32>, Vec<i32>)> {
    vec![
        (
            vec![73, 74, 75, 71, 69, 72, 76, 73],
            vec![1, 1, 4, 2, 1, 1, 0, 0],
        ),
        (vec![30, 40, 50, 60], vec![1, 1, 1, 0]),
        (vec![30, 60, 90], vec![1, 1, 0]),
        (
            vec![34, 80, 80, 34, 34, 80, 80, 80, 80, 34],
            vec![1, 0, 0, 2, 1, 0, 0, 0, 0, 0],
        ),
    ]
}

#[test]
fn daily_temperatures_test() {
    for case in get_daily_temperatures_test_cases() {
        assert_eq!(Solution::daily_temperatures(case.0), case.1);
    }
}

#[test]
fn daily_temperatures_v2_test() {
    for case in get_daily_temperatures_test_cases() {
        assert_eq!(Solution::daily_temperatures_v2(case.0), case.1);
    }
}

fn get_shortest_path_binary_matrix_test_cases() -> Vec<(Vec<Vec<i32>>, i32)> {
    vec![
        (vec![vec![0, 1], vec![1, 0]], 2),
        (vec![vec![0, 0, 0], vec![1, 1, 0], vec![1, 1, 0]], 4),
        (vec![vec![1, 0, 0], vec![1, 1, 0], vec![1, 1, 0]], -1),
    ]
}

#[test]
fn shortest_path_binary_matrix_test() {
    for case in get_shortest_path_binary_matrix_test_cases() {
        assert_eq!(Solution::shortest_path_binary_matrix(case.0), case.1);
    }
}

#[test]
fn snapshot_array_test() {
    let mut obj = SnapshotArray::new(3);
    obj.set(0, 5);

    assert_eq!(obj.snap(), 0);

    obj.set(0, 6);

    assert_eq!(obj.get(0, 0), 5);
    assert_eq!(obj.get(1, 0), 0);

    obj.set(1, 1);
    assert_eq!(obj.get(1, 0), 0);
}

#[test]
fn smallest_repunit_div_by_k() {
    let test_cases = vec![(1, 1), (2, -1), (3, 3), (11, 2)];
    for case in test_cases {
        assert_eq!(Solution::smallest_repunit_div_by_k(case.0), case.1);
    }
}

#[test]
fn max_ancestor_diff_test() {
    let test_cases = vec![
        (
            vec![
                Some(8),
                Some(3),
                Some(10),
                Some(1),
                Some(6),
                None,
                Some(14),
                None,
                None,
                Some(4),
                Some(7),
                Some(13),
            ],
            7,
        ),
        (vec![Some(1), None, Some(2), None, Some(0), Some(3)], 3),
    ];
    for case in test_cases {
        assert_eq!(
            Solution::max_ancestor_diff(TreeNode::from_level_order(&case.0)),
            case.1
        );
    }
}

#[test]
fn minimum_operations_test() {
    let test_cases = vec![
        (vec![1, 2], 0),
        (vec![1, 1], 1),
        (vec![3, 1, 3, 2, 4, 3], 3),
        (vec![1, 2, 2, 2, 2], 2),
    ];

    for case in test_cases {
        assert_eq!(Solution::minimum_operations(case.0), case.1);
    }
}

#[test]
fn find_min_difference_test() {
    let test_cases = vec![
        (vec!["23:59", "00:00"], 1),
        (vec!["00:00", "23:59", "00:00"], 0),
    ];

    for case in test_cases {
        assert_eq!(
            Solution::find_min_difference(
                case.0
                    .into_iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
            ),
            case.1
        );
    }
}
