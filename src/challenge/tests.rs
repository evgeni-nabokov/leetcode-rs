use super::*;
use super::min_stack::MinStack;

#[test]
fn single_number_test() {
    assert_eq!(Solution::single_number(vec![2,2,1]), 1);
    assert_eq!(Solution::single_number(vec![4,1,2,1,2]), 4);
}

#[test]
fn single_number_v2_test() {
    assert_eq!(Solution::single_number_v2(vec![2, 2, 1]), 1);
    assert_eq!(Solution::single_number_v2(vec![4, 1, 2, 1, 2]), 4);
}

#[test]
fn happy_number_test() {
    assert_eq!(Solution::is_happy(19), true);
}

#[test]
fn max_sub_array_test() {
    assert_eq!(Solution::max_sub_array(vec![0]), 0);
    assert_eq!(Solution::max_sub_array(vec![1]), 1);
    assert_eq!(Solution::max_sub_array(vec![-1]), -1);
    assert_eq!(Solution::max_sub_array(vec![-1,-2,-3,-4]), -1);
    assert_eq!(Solution::max_sub_array(vec![-2,-1,-3,-4]), -1);
    assert_eq!(Solution::max_sub_array(vec![-2,-3,-4,-1]), -1);
    assert_eq!(Solution::max_sub_array(vec![1,-2,-3,-4]), 1);
    assert_eq!(Solution::max_sub_array(vec![-2,1,-3,-4]), 1);
    assert_eq!(Solution::max_sub_array(vec![-2,-3,-4,1]), 1);

    assert_eq!(Solution::max_sub_array(vec![-2,1,-3,4,-1,2,1,-5,4]), 6);
    assert_eq!(Solution::max_sub_array(vec![10,1,-3,4,-1,2,1,-5,4]), 14);
    assert_eq!(Solution::max_sub_array(vec![-2,1,-3,4,-3,2,1,-5,4]), 4);
}

#[test]
fn max_sub_array_v2_test() {
    assert_eq!(Solution::max_sub_array_v2(vec![0]), 0);
    assert_eq!(Solution::max_sub_array_v2(vec![1]), 1);
    assert_eq!(Solution::max_sub_array_v2(vec![-1]), -1);
    assert_eq!(Solution::max_sub_array_v2(vec![-1,-2,-3,-4]), -1);
    assert_eq!(Solution::max_sub_array_v2(vec![-2,-1,-3,-4]), -1);
    assert_eq!(Solution::max_sub_array_v2(vec![-2,-3,-4,-1]), -1);
    assert_eq!(Solution::max_sub_array_v2(vec![1,-2,-3,-4]), 1);
    assert_eq!(Solution::max_sub_array_v2(vec![-2,1,-3,-4]), 1);
    assert_eq!(Solution::max_sub_array_v2(vec![-2,-3,-4,1]), 1);

    assert_eq!(Solution::max_sub_array_v2(vec![-2,1,-3,4,-1,2,1,-5,4]), 6);
    assert_eq!(Solution::max_sub_array_v2(vec![10,1,-3,4,-1,2,1,-5,4]), 14);
    assert_eq!(Solution::max_sub_array_v2(vec![-2,1,-3,4,-3,2,1,-5,4]), 4);
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

#[test]
fn max_profit_test() {
    assert_eq!(Solution::max_profit(vec![]), 0);
    assert_eq!(Solution::max_profit(vec![1]), 0);
    assert_eq!(Solution::max_profit(vec![1, 1, 1, 1, 1]), 0);
    assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
    assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
}

#[test]
fn max_profit_v2_test() {
    assert_eq!(Solution::max_profit_v2(vec![]), 0);
    assert_eq!(Solution::max_profit_v2(vec![1]), 0);
    assert_eq!(Solution::max_profit_v2(vec![1, 1, 1, 1, 1]), 0);
    assert_eq!(Solution::max_profit_v2(vec![7, 1, 5, 3, 6, 4]), 7);
    assert_eq!(Solution::max_profit_v2(vec![1, 2, 3, 4, 5]), 4);
    assert_eq!(Solution::max_profit_v2(vec![7, 6, 4, 3, 1]), 0);
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

fn build_list_from_slice(slice: &[i32]) -> Option<Box<ListNode>> {
    if slice.len() == 0 { return None; }
    let mut head = Some(Box::new(ListNode::new(slice[0])));
    let mut prev = head.as_mut().unwrap();
    for v in slice.into_iter().skip(1) {
        prev.next = Some(Box::new(ListNode::new(*v)));
        prev = prev.next.as_mut().unwrap();
    }
    head
}

fn build_vector_from_list(head: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut res = Vec::new();
    if head.is_none() { return res; }
    let mut cur = head;
    while let Some(node) = cur {
        res.push(node.val);
        cur = &node.next;
    }
    res
}

#[test]
fn list_helpers_test() {
    assert_eq!(build_vector_from_list(&build_list_from_slice(&[])), vec![]);
    assert_eq!(build_vector_from_list(&build_list_from_slice(&[1])), vec![1]);
    assert_eq!(build_vector_from_list(&build_list_from_slice(&[1, 2, 3, 4, 5])), vec![1, 2, 3, 4, 5]);
}

#[test]
fn middle_node_test() {
    assert_eq!(build_vector_from_list(&Solution::middle_node(build_list_from_slice(&[]))), vec![]);
    assert_eq!(build_vector_from_list(&Solution::middle_node(build_list_from_slice(&[1]))), vec![1]);
    assert_eq!(build_vector_from_list(&Solution::middle_node(build_list_from_slice(&[1, 2]))), vec![2]);
    assert_eq!(build_vector_from_list(&Solution::middle_node(build_list_from_slice(&[1, 2, 3]))), vec![2, 3]);
    assert_eq!(build_vector_from_list(&Solution::middle_node(build_list_from_slice(&[1, 2, 3, 4]))), vec![3, 4]);
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
    assert_eq!(obj.get_min(), Some(-3));
    obj.pop();
    assert_eq!(obj.top(), Some(0));
    assert_eq!(obj.get_min(), Some(-2));
}