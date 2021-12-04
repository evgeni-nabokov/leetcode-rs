use crate::common::list_node::ListNode;
use crate::common::linked_list::LinkedList;
use crate::common::tree_node::{TreeNode, BinaryTree};

use super::*;
use super::min_stack::MinStack;
use super::lru_cache::LRUCache;
use super::first_unique::FirstUnique;

fn get_single_number_test_cases() -> Vec<(Vec<i32>, i32)> {
    vec![
        (vec![2,2,1], 1),
        (vec![4, 1, 2, 1, 2], 4),
    ]
}

#[test]
fn single_number_test() {
    for case in get_single_number_test_cases() {
        assert_eq!(Solution::single_number(case.0), case.1);
    }
}

#[test]
fn single_number_v2_test() {
    for case in get_single_number_test_cases() {
        assert_eq!(Solution::single_number_v2(case.0), case.1);
    }
}

#[test]
fn happy_number_test() {
    assert_eq!(Solution::is_happy(19), true);
}

fn get_max_sub_array_test_cases() -> Vec<(Vec<i32>, i32)> {
    vec![
        (vec![0], 0),
        (vec![1], 1),
        (vec![-1], -1),
        (vec![-1,-2,-3,-4], -1),
        (vec![-2,-1,-3,-4], -1),
        (vec![-2,-3,-4,-1], -1),
        (vec![1,-2,-3,-4], 1),
        (vec![-2,1,-3,-4], 1),
        (vec![-2,-3,-4,1], 1),
        (vec![-2,1,-3,4,-1,2,1,-5,4], 6),
        (vec![10,1,-3,4,-1,2,1,-5,4], 14),
        (vec![-2,1,-3,4,-3,2,1,-5,4], 4)
    ]
}

#[test]
fn max_sub_array_test() {
    for case in get_max_sub_array_test_cases() {
        assert_eq!(Solution::max_sub_array(case.0), case.1);
    }
}

#[test]
fn max_sub_array_v2_test() {
    for case in get_max_sub_array_test_cases() {
        assert_eq!(Solution::max_sub_array_v2(case.0), case.1);
    }
}

#[test]
fn move_zeroes_test() {
    let mut actual1 = vec![0, 1, 0, 3, 12];
    Solution::move_zeroes(&mut actual1);
    let expected1 = vec![1, 3, 12, 0, 0];
    assert_eq!(actual1, expected1);

    let mut actual2 = vec![];
    Solution::move_zeroes(&mut actual2);
    let expected2 = vec![];
    assert_eq!(actual2, expected2);

    let mut actual3 = vec![0];
    Solution::move_zeroes(&mut actual3);
    let expected3 = vec![0];
    assert_eq!(actual3, expected3);

    let mut actual4 = vec![0, 1];
    Solution::move_zeroes(&mut actual4);
    let expected4 = vec![1, 0];
    assert_eq!(actual4, expected4);

    let mut actual5 = vec![0, 0, 0, 0];
    Solution::move_zeroes(&mut actual5);
    let expected5 = vec![0, 0, 0, 0];
    assert_eq!(actual5, expected5);

    let mut actual6 = vec![1, 1, 1, 1];
    Solution::move_zeroes(&mut actual6);
    let expected6 = vec![1, 1, 1, 1];
    assert_eq!(actual6, expected6);

    let mut actual7 = vec![0, -1, 0, -3, -12];
    Solution::move_zeroes(&mut actual7);
    let expected7 = vec![-1, -3, -12, 0, 0];
    assert_eq!(actual7, expected7);
}

#[test]
fn move_zeroes_v2_test() {
    let mut actual1 = vec![0, 1, 0, 3, 12];
    Solution::move_zeroes_v2(&mut actual1);
    let expected1 = vec![1, 3, 12, 0, 0];
    assert_eq!(actual1, expected1);

    let mut actual2 = vec![];
    Solution::move_zeroes_v2(&mut actual2);
    let expected2 = vec![];
    assert_eq!(actual2, expected2);

    let mut actual3 = vec![0];
    Solution::move_zeroes_v2(&mut actual3);
    let expected3 = vec![0];
    assert_eq!(actual3, expected3);

    let mut actual4 = vec![0, 1];
    Solution::move_zeroes_v2(&mut actual4);
    let expected4 = vec![1, 0];
    assert_eq!(actual4, expected4);

    let mut actual5 = vec![0, 0, 0, 0];
    Solution::move_zeroes_v2(&mut actual5);
    let expected5 = vec![0, 0, 0, 0];
    assert_eq!(actual5, expected5);

    let mut actual6 = vec![1, 1, 1, 1];
    Solution::move_zeroes_v2(&mut actual6);
    let expected6 = vec![1, 1, 1, 1];
    assert_eq!(actual6, expected6);

    let mut actual7 = vec![0, -1, 0, -3, -12];
    Solution::move_zeroes_v2(&mut actual7);
    let expected7 = vec![-1, -3, -12, 0, 0];
    assert_eq!(actual7, expected7);
}

fn get_max_profit_test_cases() -> Vec<(Vec<i32>, i32)> {
    vec![
        (vec![], 0),
        (vec![1], 0),
        (vec![1, 1, 1, 1, 1], 0),
        (vec![7, 1, 5, 3, 6, 4], 7),
        (vec![1, 2, 3, 4, 5], 4),
        (vec![7, 6, 4, 3, 1], 0),
    ]
}

#[test]
fn max_profit_test() {
    for case in get_max_profit_test_cases() {
        assert_eq!(Solution::max_profit(case.0), case.1);
    }
}

#[test]
fn max_profit_v2_test() {
    for case in get_max_profit_test_cases() {
        assert_eq!(Solution::max_profit_v2(case.0), case.1);
    }
}

#[test]
fn group_anagrams_test() {
    assert_eq!(Solution::group_anagrams(vec![]), Vec::<Vec<String>>::new());
    assert_eq!(Solution::group_anagrams(
        vec!["eat".to_string(), "tea".to_string(), "tan".to_string(),
                  "ate".to_string(), "nat".to_string(), "bat".to_string()]),
           vec![
               vec!["ate".to_string(), "eat".to_string(), "tea".to_string()],
               vec!["nat".to_string(), "tan".to_string()],
               vec!["bat".to_string()],
           ]);
}

#[test]
fn group_anagrams_v2_test() {
    assert_eq!(Solution::group_anagrams_v2(vec![]), Vec::<Vec<String>>::new());
    assert_eq!(Solution::group_anagrams_v2(
        vec!["eat".to_string(), "tea".to_string(), "tan".to_string(),
             "ate".to_string(), "nat".to_string(), "bat".to_string()]),
               vec![
                   vec!["ate".to_string(), "eat".to_string(), "tea".to_string()],
                   vec!["nat".to_string(), "tan".to_string()],
                   vec!["bat".to_string()],
               ]);
}

#[test]
fn count_elements_test() {
    assert_eq!(Solution::count_elements(vec![1, 2, 3]), 2);
    assert_eq!(Solution::count_elements(vec![1, 1, 3, 3, 5, 5, 7, 7]), 0);
    assert_eq!(Solution::count_elements(vec![1, 3, 2, 3, 5, 0]), 3);
}

#[test]
fn list_helpers_test() {
    assert_eq!(ListNode::from_slice(&[]).to_vec(), vec![]);
    assert_eq!(ListNode::from_slice(&[1]).to_vec(), vec![1]);
    assert_eq!(ListNode::from_slice(&[1, 2, 3, 4, 5]).to_vec(), vec![1, 2, 3, 4, 5]);
}

#[test]
fn middle_node_test() {
    let test_cases = vec![
        (
            vec![],
            vec![],
        ),
        (
            vec![1],
            vec![1],
        ),
        (
            vec![1, 2, 3],
            vec![2, 3],
        ),
        (
            vec![1, 2, 3, 4],
            vec![3, 4],
        ),
    ];

    for case in test_cases {
        assert_eq!(Solution::middle_node(ListNode::from_slice(&case.0)).to_vec(), case.1);
    }
}

fn apply_backspaces_for_str(s: &str) -> String {
    let mut iter = s.chars().rev();
    let char = apply_backspaces(&mut iter);
    if char.is_none() { return String::new(); }
    let mut res = char.unwrap().to_string();
    res.push_str(&iter.collect::<String>());
    res
}

#[test]
fn apply_backspaces_test() {
    assert_eq!(apply_backspaces_for_str(""), "".to_string());
    assert_eq!(apply_backspaces_for_str("#"), "".to_string());
    assert_eq!(apply_backspaces_for_str("#####"), "".to_string());
    assert_eq!(apply_backspaces_for_str("a#"), "".to_string());
    assert_eq!(apply_backspaces_for_str("ab#"), "a".to_string());
    assert_eq!(apply_backspaces_for_str("ab##"), "".to_string());
    assert_eq!(apply_backspaces_for_str("a#b#"), "".to_string());
    assert_eq!(apply_backspaces_for_str("ab###c####"), "".to_string());
}

#[test]
fn backspace_compare_test() {
    assert_eq!(Solution::backspace_compare("ab#c".to_string(), "ad#c".to_string()), true);
    assert_eq!(Solution::backspace_compare("ab##".to_string(), "c#d#".to_string()), true);
    assert_eq!(Solution::backspace_compare("a##c".to_string(), "#a#c".to_string()), true);
    assert_eq!(Solution::backspace_compare("a#c".to_string(), "b".to_string()), false);
}

#[test]
fn min_stack_test() {
    let mut obj = MinStack::new();
    obj.push(-2);
    obj.push(0);
    obj.push(-3);
    assert_eq!(obj.get_min(), -3);
    obj.pop();
    assert_eq!(obj.top(), 0);
    assert_eq!(obj.get_min(), -2);
}

#[test]
fn last_stone_weight_test() {
    assert_eq!(Solution::last_stone_weight(vec![]), 0);
    assert_eq!(Solution::last_stone_weight(vec![1, 1]), 0);
    assert_eq!(Solution::last_stone_weight(vec![2, 7, 4, 1, 8, 1]), 1);
}

#[test]
fn find_max_length_test() {
    assert_eq!(Solution::find_max_length(vec![]), 0);
    assert_eq!(Solution::find_max_length(vec![1]), 0);
    assert_eq!(Solution::find_max_length(vec![1, 0]), 2);
    assert_eq!(Solution::find_max_length(vec![1, 1]), 0);
    assert_eq!(Solution::find_max_length(vec![0,1,0]), 2);
    assert_eq!(Solution::find_max_length(vec![0, 1, 1, 0, 1, 1, 1, 0]), 4);
    assert_eq!(Solution::find_max_length(vec![0,0,1,0,0,0,1,1]), 6);
}

#[test]
fn find_max_length_v2_test() {
    assert_eq!(Solution::find_max_length_v2(vec![]), 0);
    assert_eq!(Solution::find_max_length_v2(vec![1]), 0);
    assert_eq!(Solution::find_max_length_v2(vec![1, 0]), 2);
    assert_eq!(Solution::find_max_length_v2(vec![1, 1]), 0);
    assert_eq!(Solution::find_max_length_v2(vec![0,1,0]), 2);
    assert_eq!(Solution::find_max_length_v2(vec![0, 1, 1, 0, 1, 1, 1, 0]), 4);
    assert_eq!(Solution::find_max_length_v2(vec![0,0,1,0,0,0,1,1]), 6);
}

#[test]
fn string_shift_test() {
    assert_eq!(Solution::string_shift("abc".to_string(), vec![vec![0, 1], vec![1, 2]]), "cab".to_string());
    assert_eq!(Solution::string_shift("abcdefg".to_string(), vec![vec![1, 1], vec![1, 1], vec![0, 2], vec![1, 3]]), "efgabcd".to_string());
}

#[test]
fn product_except_self_test() {
    assert_eq!(Solution::product_except_self(vec![]), vec![]);
    assert_eq!(Solution::product_except_self(vec![42]), vec![1]);
    assert_eq!(Solution::product_except_self(vec![1, 2, 3, 4]), vec![24 , 12, 8, 6]);
    assert_eq!(Solution::product_except_self(vec![2, 3, 4, 5]), vec![60 , 40, 30, 24]);
}

#[test]
fn product_except_self_v2_test() {
    assert_eq!(Solution::product_except_self_v2(vec![]), vec![]);
    assert_eq!(Solution::product_except_self_v2(vec![42]), vec![1]);
    assert_eq!(Solution::product_except_self_v2(vec![1, 2, 3, 4]), vec![24 , 12, 8, 6]);
    assert_eq!(Solution::product_except_self_v2(vec![2, 3, 4, 5]), vec![60 , 40, 30, 24]);
}

#[test]
fn check_valid_string_test() {
    assert_eq!(Solution::check_valid_string("".to_string()), true);
    assert_eq!(Solution::check_valid_string("*".to_string()), true);
    assert_eq!(Solution::check_valid_string("()".to_string()), true);
    assert_eq!(Solution::check_valid_string("(*)".to_string()), true);
    assert_eq!(Solution::check_valid_string("**()**".to_string()), true);
    assert_eq!(Solution::check_valid_string("**(**".to_string()), true);
    assert_eq!(Solution::check_valid_string("**)**".to_string()), true);
    assert_eq!(Solution::check_valid_string("(((******))".to_string()), true);
    assert_eq!(Solution::check_valid_string("(*))".to_string()), true);
    assert_eq!(Solution::check_valid_string("(*)())".to_string()), true);
    assert_eq!(Solution::check_valid_string("(*)()()".to_string()), true);
    assert_eq!(Solution::check_valid_string("**))".to_string()), true);
    assert_eq!(Solution::check_valid_string("((**".to_string()), true);
    assert_eq!(Solution::check_valid_string("(*".to_string()), true);
    assert_eq!(Solution::check_valid_string("*(".to_string()), false);
    assert_eq!(Solution::check_valid_string("((*".to_string()), false);
    assert_eq!(Solution::check_valid_string(")(".to_string()), false);
    assert_eq!(Solution::check_valid_string("())".to_string()), false);
    assert_eq!(Solution::check_valid_string("(())((())()()(*)(*()(())())())()()((()())((()))(*".to_string()), false);
}

fn get_char_grid_1() -> Vec<Vec<char>> {
    vec![
        vec!['1', '1', '1', '1', '0'],
        vec!['1', '1', '0', '1', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '0', '0', '0'],
    ]
}


fn get_char_grid_2() -> Vec<Vec<char>> {
    vec![
        vec!['1', '1', '0', '0', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '1', '0', '0'],
        vec!['0', '0', '0', '1', '1'],
    ]
}

#[test]
fn num_islands_test() {
    assert_eq!(Solution::num_islands(vec![]), 0);
    assert_eq!(Solution::num_islands(vec![vec![]]), 0);

    assert_eq!(Solution::num_islands(get_char_grid_1()), 1);
    assert_eq!(Solution::num_islands(get_char_grid_2()), 3);
}

#[test]
fn num_islands_v2_test() {
    assert_eq!(Solution::num_islands_v2(vec![]), 0);
    assert_eq!(Solution::num_islands_v2(vec![vec![]]), 0);

    assert_eq!(Solution::num_islands_v2(get_char_grid_1()), 1);
    assert_eq!(Solution::num_islands_v2(get_char_grid_2()), 3);
}


fn get_number_grid_1() -> Vec<Vec<i32>> {
    vec![
        vec![1, 3, 1],
        vec![1, 5, 1],
        vec![4, 2, 1],
    ]
}

fn get_number_grid_2() -> Vec<Vec<i32>> {
    vec![
        vec![0, 0, 0],
        vec![0, 0, 0],
        vec![0, 0, 0],
    ]
}

fn get_min_path_sum_test_cases() -> Vec<(Vec<Vec<i32>>, i32)>{
    vec![
        (
            vec![
                vec![1, 3, 1],
                vec![1, 5, 1],
                vec![4, 2, 1],
            ],
            7
        ),
        (
            vec![
                vec![0, 0, 0],
                vec![0, 0, 0],
                vec![0, 0, 0],
            ],
            0
        ),
        (
            vec![
                vec![1, 2, 3],
                vec![4, 5, 6],
            ],
            12
        ),
    ]
}

#[test]
fn min_path_sum_test() {
    for case in get_min_path_sum_test_cases() {
        assert_eq!(Solution::min_path_sum(case.0), case.1);
    }
}

#[test]
fn min_path_sum_v2_test() {
    for case in get_min_path_sum_test_cases() {
        assert_eq!(Solution::min_path_sum_v2(case.0), case.1);
    }
}

#[test]
fn search_test() {
    assert_eq!(Solution::search(vec![], 1), -1);
    assert_eq!(Solution::search(vec![1], 1), 0);
    assert_eq!(Solution::search(vec![2], 1), -1);
    assert_eq!(Solution::search(vec![1, 2], 1), 0);
    assert_eq!(Solution::search(vec![1, 2], 2), 1);
    assert_eq!(Solution::search(vec![1, 2], 3), -1);
    assert_eq!(Solution::search(vec![2, 1], 1), 1);
    assert_eq!(Solution::search(vec![2, 1], 2), 0);
    assert_eq!(Solution::search(vec![2, 1], 3), -1);
    assert_eq!(Solution::search(vec![1, 2, 3], 3), 2);
    assert_eq!(Solution::search(vec![3, 1, 2], 3), 0);
    assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
    assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
    assert_eq!(Solution::search(vec![4, 5, 6, 7, 8, 9, 10, 0, 1, 2], 10), 6);
}

fn get_bst_from_preorder_test_cases() -> Vec<(Vec<i32>, Vec<Option<i32>>)> {
    vec![
        (vec![], vec![]),
        (vec![1], vec![Some(1)]),
        (vec![10, 5], vec![Some(10), Some(5)]),
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
fn subarray_sum_test() {
    assert_eq!(Solution::subarray_sum(vec![1, 1, 1], 2), 2);
    assert_eq!(Solution::subarray_sum(vec![2, 5, 4, 1, 0], 2), 1);
    assert_eq!(Solution::subarray_sum(vec![2, 5, 4, 1, 0], 1), 2);
}

#[test]
fn subarray_sum_v2_test() {
    assert_eq!(Solution::subarray_sum_v2(vec![1, 1, 1], 2), 2);
    assert_eq!(Solution::subarray_sum_v2(vec![2, 5, 4, 1, 0], 2), 1);
    assert_eq!(Solution::subarray_sum_v2(vec![2, 5, 4, 1, 0], 1), 2);
}

#[test]
fn subarray_sum_v3_test() {
    assert_eq!(Solution::subarray_sum_v3(vec![1, 1, 1], 2), 2);
    assert_eq!(Solution::subarray_sum_v3(vec![2, 5, 4, 1, 0], 2), 1);
    assert_eq!(Solution::subarray_sum_v3(vec![2, 5, 4, 1, 0], 1), 2);
}

#[test]
fn subarray_sum_v4_test() {
    assert_eq!(Solution::subarray_sum_v4(vec![1, 1, 1], 2), 2);
    assert_eq!(Solution::subarray_sum_v4(vec![2, 5, 4, 1, 0], 2), 1);
    assert_eq!(Solution::subarray_sum_v4(vec![2, 5, 4, 1, 0], 1), 2);
}

#[test]
fn lru_cache_test() {
    let mut cache = LRUCache::new(2);
    cache.put(1, 1);
    cache.put(2, 2);
    assert_eq!(cache.get(1), 1);
    cache.put(3, 3);
    assert_eq!(cache.get(2), -1);
    cache.put(4, 4);
    assert_eq!(cache.get(1), -1);
    assert_eq!(cache.get(3), 3);
    assert_eq!(cache.get(4), 4);
}

#[test]
fn range_bitwise_and_test() {
    assert_eq!(Solution::range_bitwise_and(5, 7), 4);
    assert_eq!(Solution::range_bitwise_and(0, 1), 0);
    assert_eq!(Solution::range_bitwise_and(16, 19), 16);
    assert_eq!(Solution::range_bitwise_and(16, 31), 16);
}

#[test]
fn can_jump_test() {
    assert_eq!(Solution::can_jump(vec![]), true);
    assert_eq!(Solution::can_jump(vec![2,0,0]), true);
    assert_eq!(Solution::can_jump(vec![2,3,1,1,4]), true);
    assert_eq!(Solution::can_jump(vec![3,2,1,0,4]), false);
}

#[test]
fn can_jump_v2_test() {
    assert_eq!(Solution::can_jump_v2(vec![]), true);
    assert_eq!(Solution::can_jump_v2(vec![2,0,0]), true);
    assert_eq!(Solution::can_jump_v2(vec![2,3,1,1,4]), true);
    assert_eq!(Solution::can_jump_v2(vec![3,2,1,0,4]), false);
}

#[test]
fn longest_common_subsequence() {
    assert_eq!(Solution::longest_common_subsequence("".to_string(), "a".to_string()), 0);
    assert_eq!(Solution::longest_common_subsequence("a".to_string(), "a".to_string()), 1);
    assert_eq!(Solution::longest_common_subsequence("a".to_string(), "b".to_string()), 0);
    assert_eq!(Solution::longest_common_subsequence("abcde".to_string(), "ace".to_string()), 3);
    assert_eq!(Solution::longest_common_subsequence("abc".to_string(), "abc".to_string()), 3);
    assert_eq!(Solution::longest_common_subsequence("abc".to_string(), "def".to_string()), 0);
    assert_eq!(Solution::longest_common_subsequence("aggtab".to_string(), "gxtxayb".to_string()), 4);
}

#[test]
fn maximal_square_test() {
    assert_eq!(Solution::maximal_square(vec![vec![]]), 0);
    assert_eq!(Solution::maximal_square(vec![
        vec!['0']
    ]), 0);
    assert_eq!(Solution::maximal_square(vec![
        vec!['1']
    ]), 1);
    assert_eq!(Solution::maximal_square(vec![
        vec!['1', '0', '1', '0', '0'],
        vec!['1', '0', '1', '1', '1'],
        vec!['1', '1', '1', '1', '1'],
        vec!['1', '0', '0', '1', '0'],
    ]), 4);
}

#[test]
fn maximal_square_v2_test() {
    assert_eq!(Solution::maximal_square_v2(vec![vec![]]), 0);
    assert_eq!(Solution::maximal_square_v2(vec![
        vec!['0']
    ]), 0);
    assert_eq!(Solution::maximal_square_v2(vec![
        vec!['1']
    ]), 1);
    assert_eq!(Solution::maximal_square_v2(vec![
        vec!['1', '0', '1', '0', '0'],
        vec!['1', '0', '1', '1', '1'],
        vec!['1', '1', '1', '1', '1'],
        vec!['1', '0', '0', '1', '0'],
    ]), 4);
}

#[test]
fn max_path_sum_test() {
    let test_cases = vec![
        (
            vec![Some(1), Some(2), Some(3)],
            6
        ),
        (
            vec![Some(-10), Some(9), Some(20), None, None, Some(15), Some(7)],
            42
        )
    ];
    for case in test_cases {
        assert_eq!(Solution::max_path_sum(TreeNode::from_level_order(&case.0)), case.1);
    }
}

#[test]
fn is_valid_sequence_test() {
    let test_cases = vec![
        (
            vec![Some(0), Some(1), Some(0), Some(0), Some(1), Some(0), None, None, Some(1), Some(0), Some(0)],
            vec![0, 1, 0, 1],
            true
        ),
        (
            vec![Some(0), Some(1), Some(0), Some(0), Some(1), Some(0), None, None, Some(1), Some(0), Some(0)],
            vec![0, 1, 1],
            false
        ),
        (
            vec![Some(0), Some(1), Some(0), Some(0), Some(1), Some(0), None, None, Some(1), Some(0), Some(0)],
            vec![0, 1, 1],
            false
        ),
        (
            vec![Some(8), Some(3), None, Some(2), Some(1), Some(5), Some(4)],
            vec![8],
            false,
        )
    ];
    for case in test_cases {
        assert_eq!(Solution::is_valid_sequence(TreeNode::from_level_order(&case.0), case.1), case.2);
    }
}

#[test]
fn first_unique_test() {
    let mut obj = FirstUnique::new(vec![2, 3, 5]);
    assert_eq!(obj.show_first_unique(), 2);
    obj.add(5);
    assert_eq!(obj.show_first_unique(), 2);
    obj.add(2);
    assert_eq!(obj.show_first_unique(), 3);
    obj.add(3);
    assert_eq!(obj.show_first_unique(), -1);

    obj = FirstUnique::new(vec![7, 7, 7, 7, 7, 7]);
    assert_eq!(obj.show_first_unique(), -1);
    obj.add(7);
    obj.add(3);
    obj.add(3);
    obj.add(7);
    obj.add(17);
    assert_eq!(obj.show_first_unique(), 17);

    obj = FirstUnique::new(vec![809]);
    assert_eq!(obj.show_first_unique(), 809);
    obj.add(809);
    assert_eq!(obj.show_first_unique(), -1);
}