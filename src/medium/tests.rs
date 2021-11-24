use super::*;
use crate::common::tree_node::BinaryTree;
use super::bst_iterator::BSTIterator;

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
        (
            vec![Some(1)],
            1,
            vec![]
        ),
        (
            vec![Some(1), Some(1), None],
            1,
            vec![]
        ),
        (
            vec![Some(1), Some(1), Some(1)],
            1,
            vec![]
        ),
        (
            vec![Some(1), Some(1), Some(2)],
            1,
            vec![Some(1), None, Some(2)]
        ),
        (
            vec![Some(1), Some(2), Some(3), Some(2), None, Some(2), Some(4)],
            2,
            vec![Some(1), None, Some(3), None, Some(4)]
        ),
        (
            vec![Some(1), Some(3), Some(3), Some(3), Some(2)],
            3,
            vec![Some(1), Some(3), None, None, Some(2)]
        ),
        (
            vec![Some(1), Some(2), None, Some(2), None, Some(2)],
            2,
            vec![Some(1)]
        ),
        (
            vec![Some(1), Some(1), Some(1)],
            1,
            vec![]
        ),
        (
            vec![Some(1), Some(2), Some(3)],
            1,
            vec![Some(1), Some(2), Some(3)],
        ),
    ];
    for case in test_cases {
        let tree = TreeNode::from_level_order(&case.0);
        assert_eq!(Solution::remove_leaf_nodes(tree, case.1).get_level_order_values(), case.2);
    }
}

#[test]
fn level_order_test() {
    let test_cases = vec![
        (
            vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)],
            vec![
                vec![3],
                vec![9, 20],
                vec![15, 7],
            ]
        ),
        (
            vec![Some(1), Some(2), Some(3), Some(4), None, None, Some(5)],
            vec![
                vec![1],
                vec![2, 3],
                vec![4, 5],
            ]
        ),
    ];
    for case in test_cases {
        assert_eq!(Solution::level_order(TreeNode::from_level_order(&case.0)), case.1);
    }
}

#[test]
fn build_tree_test() {
    let test_cases = vec![
        (vec![], vec![], vec![]),
        (vec![1, 2], vec![2, 1], vec![Some(1), Some(2)]),
        (vec![5, 3, 2, 4, 6], vec![2, 4, 3, 6, 5], vec![Some(5), Some(3), Some(6), Some(2), Some(4)]),
        (vec![1, 2, 4, 5, 3, 6, 7], vec![4, 5, 2, 6, 7, 3, 1], vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7)])
    ];
    for case in test_cases {
        assert_eq!(Solution::build_tree(case.0, case.1).get_level_order_values(), case.2);
    }
}

#[test]
fn build_tree_ii_test() {
    let test_cases = vec![
        (vec![], vec![], vec![]),
        (vec![1, 2], vec![2, 1], vec![Some(1), Some(2)]),
        (vec![5, 3, 2, 4, 6], vec![2, 3, 4, 5, 6], vec![Some(5), Some(3), Some(6), Some(2), Some(4)]),
        (vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7], vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)])
    ];
    for case in test_cases {
        assert_eq!(Solution::build_tree_ii(case.0, case.1).get_level_order_values(), case.2);
    }
}

#[test]
fn combination_sum_test() {
    let test_cases = vec![
        (vec![2, 3, 6, 7], 7, vec![vec![3, 2, 2], vec![7]]),
        (vec![2, 3, 5], 8, vec![vec![2, 2, 2, 2], vec![3, 3, 2], vec![5, 3]]),
        (vec![8, 7, 4, 3], 11, vec![vec![4, 4, 3], vec![8, 3], vec![7, 4]]),
    ];
    for case in test_cases {
        assert_eq!(Solution::combination_sum(case.0, case.1), case.2);
    }
}

#[test]
fn combination_sum_ii_test() {
    let test_cases = vec![
        (vec![10, 1, 2, 7, 6, 1, 5], 8, vec![vec![6, 1, 1], vec![5, 2, 1], vec![7, 1], vec![6, 2]]),
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
        (
            vec!['a'],
            vec!['a'],
        ),
        (
            vec!['t', 'h', 'e', ' ', 's', 'k', 'y', ' ', 'i', 's', ' ', 'b', 'l', 'u', 'e'],
            vec!['b', 'l', 'u', 'e', ' ', 'i', 's', ' ', 's', 'k', 'y', ' ', 't', 'h', 'e'],
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
        (
            vec![Some(2), Some(1), Some(3)],
            true,
        ),
        (
            vec![Some(5), Some(1), Some(4), None, None, Some(3), Some(6)],
            false,
        ),
        (
            vec![Some(5), Some(4), Some(6), None, None, Some(3), Some(7)],
            false,
        ),
        (
            vec![Some(1), Some(1)],
            false,
        ),
        (
            vec![Some(2147483647)],
            true,
        ),
    ]
}

#[test]
fn is_valid_bst_test() {
    for case in get_is_valid_bst_test_cases() {
        assert_eq!(Solution::is_valid_bst(TreeNode::from_level_order(&case.0)), case.1);
    }
}

#[test]
fn is_valid_bst_v2_test() {
    for case in get_is_valid_bst_test_cases() {
        assert_eq!(Solution::is_valid_bst_v2(TreeNode::from_level_order(&case.0)), case.1);
    }
}

#[test]
fn is_valid_sudoku_test() {
    let test_cases = vec![
        (
            vec![
                vec!['5','3','.','.','7','.','.','.','.'],
                vec!['6','.','.','1','9','5','.','.','.'],
                vec!['.','9','8','.','.','.','.','6','.'],
                vec!['8','.','.','.','6','.','.','.','3'],
                vec!['4','.','.','8','.','3','.','.','1'],
                vec!['7','.','.','.','2','.','.','.','6'],
                vec!['.','6','.','.','.','.','2','8','.'],
                vec!['.','.','.','4','1','9','.','.','5'],
                vec!['.','.','.','.','8','.','.','7','9'],
            ],
            true
        ),
        (
            vec![
                vec!['8','3','.','.','7','.','.','.','.'],
                vec!['6','.','.','1','9','5','.','.','.'],
                vec!['.','9','8','.','.','.','.','6','.'],
                vec!['8','.','.','.','6','.','.','.','3'],
                vec!['4','.','.','8','.','3','.','.','1'],
                vec!['7','.','.','.','2','.','.','.','6'],
                vec!['.','6','.','.','.','.','2','8','.'],
                vec!['.','.','.','4','1','9','.','.','5'],
                vec!['.','.','.','.','8','.','.','7','9'],
            ],
            false
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
        assert_eq!(Solution::longest_palindrome(case.0.to_string()), case.1.to_string());
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
        ("abcabcde", [6,1,1]),
        ("ababcbacadefegdehijhklij", [9,7,8]),
    ];
    for case in test_cases {
        assert_eq!(Solution::partition_labels(case.0.to_string()), case.1);
    }
}

#[test]
fn all_paths_source_target_test() {
    let test_cases = vec![
        (vec![vec![1,2], vec![3], vec![3], vec![]], vec![vec![0, 1, 3], vec![0, 2, 3]]),
        (
            vec![vec![4, 3, 1], vec![3, 2, 4], vec![3], vec![4], vec![]],
            vec![vec![0, 4], vec![0, 3, 4], vec![0, 1, 3, 4], vec![0, 1, 2, 3, 4], vec![0, 1, 4]],
        ),
        (vec![vec![1], vec![]], vec![vec![0, 1]]),
        (
            vec![vec![1, 2, 3], vec![2], vec![3], vec![]],
            vec![vec![0, 1, 2, 3], vec![0, 2, 3], vec![0, 3]],
        ),
        (
            vec![vec![1, 3], vec![2], vec![3], vec![]],
            vec![vec![0, 1, 2, 3], vec![0, 3]],
        )
    ];
    for case in test_cases {
        assert_eq!(Solution::all_paths_source_target(case.0), case.1);
    }
}

#[test]
fn lowest_common_ancestor_test() {
    let test_cases = vec![
        (vec![Some(3), Some(5), Some(1), Some(6), Some(2), Some(0), Some(8), None, None, Some(7), Some(4)], 5, 1, 3),
        (vec![Some(3), Some(5), Some(1), Some(6), Some(2), Some(0), Some(8), None, None, Some(7), Some(4)], 5, 4, 5),
    ];
    for case in test_cases {
        assert_eq!(Solution::lowest_common_ancestor(TreeNode::from_level_order(&case.0),
                                                    TreeNode::new_with_children(Some(case.1), None, None),
                                                    TreeNode::new_with_children(Some(case.2), None, None)).unwrap().borrow().val,
                   case.3);
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
        assert_eq!(Solution::remove_duplicates(case.0.to_string(), case.1), case.2.to_string());
    }
}

#[test]
fn remove_duplicates_v2_test() {
    for case in get_remove_duplicates_test_cases() {
        assert_eq!(Solution::remove_duplicates_v2(case.0.to_string(), case.1), case.2.to_string());
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
fn good_nodes_test() {
    let test_cases = vec![
        (vec![Some(1)], 1),
        (vec![Some(3), Some(1), Some(4), Some(3), None, Some(1), Some(5)], 4),
        (vec![Some(3), Some(3), None, Some(4), Some(2)], 3),
    ];
    for case in test_cases {
        assert_eq!(Solution::good_nodes(TreeNode::from_level_order(&case.0)), case.1);
    }
}

fn get_suggested_products_cases<'a>() -> Vec<(Vec<&'a str>, &'a str, Vec<Vec<&'a str>>)> {
    vec![
        (
            vec!["mobile","mouse","moneypot","monitor","mousepad"],
            "mouse",
            vec![
                vec!["mobile","moneypot","monitor"],
                vec!["mobile","moneypot","monitor"],
                vec!["mouse","mousepad"],
                vec!["mouse","mousepad"],
                vec!["mouse","mousepad"],
            ]
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
            ]
        ),
    ]
}

#[test]
fn suggested_products_test() {
    for case in get_suggested_products_cases() {
        assert_eq!(Solution::suggested_products(
            case.0.into_iter().map(|x| x.to_string()).collect(), case.1.to_string()),
            case.2.into_iter().map(|x| x.into_iter().map(|y| y.to_string()).collect()).collect::<Vec<Vec<String>>>());
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