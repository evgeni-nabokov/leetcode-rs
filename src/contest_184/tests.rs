use super::*;

#[test]
fn string_matching_test() {
    assert_eq!(Solution::string_matching(
        vec!["mass".to_string(), "as".to_string(), "hero".to_string(), "superhero".to_string()]),
               vec!["as".to_string(), "hero".to_string()]);
    assert_eq!(Solution::string_matching(
        vec!["leetcode".to_string(), "et".to_string(), "code".to_string()]),
               vec!["et".to_string(), "code".to_string()]);
    assert_eq!(Solution::string_matching(
        vec!["blue".to_string(), "green".to_string(), "bu".to_string()]),
               Vec::<String>::new());
    assert_eq!(Solution::string_matching(
        vec!["leetcoder".to_string(), "leetcode".to_string(), "od".to_string(), "hamlet".to_string(), "am".to_string()]),
               vec!["od".to_string(), "am".to_string(), "leetcode".to_string()]);
}

#[test]
fn entity_parser_test() {
    assert_eq!(Solution::entity_parser("&amp; is an HTML entity but &ambassador; is not.".to_string()),
               "& is an HTML entity but &ambassador; is not.".to_string());
    assert_eq!(Solution::entity_parser("and I quote: &quot;...&quot;".to_string()),
               "and I quote: \"...\"".to_string());
    assert_eq!(Solution::entity_parser("Stay home! Practice on Leetcode :)".to_string()),
               "Stay home! Practice on Leetcode :)".to_string());
    assert_eq!(Solution::entity_parser("x &gt; y &amp;&amp; x &lt; y is always false".to_string()),
               "x > y && x < y is always false".to_string());
    assert_eq!(Solution::entity_parser("leetcode.com&frasl;problemset&frasl;all".to_string()),
               "leetcode.com/problemset/all".to_string());
}

#[test]
fn process_queries_test() {
    assert_eq!(Solution::process_queries(vec![3, 1, 2, 1], 5), vec![2, 1, 2, 1]);
    assert_eq!(Solution::process_queries(vec![4, 1, 2, 2], 4), vec![3, 1, 2, 0]);
    assert_eq!(Solution::process_queries(vec![7, 5, 5, 8, 3], 8), vec![6, 5, 0, 7, 5]);
}

#[test]
fn num_of_ways_test() {
    assert_eq!(Solution::num_of_ways(0), 1);
    assert_eq!(Solution::num_of_ways(1), 12);
    assert_eq!(Solution::num_of_ways(2), 54);
    assert_eq!(Solution::num_of_ways(3), 246);
    assert_eq!(Solution::num_of_ways(7), 106494);
    assert_eq!(Solution::num_of_ways(5000), 30228214);
}