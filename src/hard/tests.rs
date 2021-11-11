use super::*;

#[test]
fn lowest_common_ancestor_test() {
    let test_cases = vec![
        (
            vec!["cat","dog","catdog"],
            vec!["catdog"],
        ),
        (
            vec!["cat", "cats", "catsdogcats", "dog", "dogcatsdog", "hippopotamuses", "rat", "ratcatdogcat"],
            vec!["catsdogcats", "dogcatsdog","ratcatdogcat"],
        ),
    ];
    for case in test_cases {
        let mut actual = Solution::find_all_concatenated_words_in_a_dict(
            case.0.into_iter().map(|x| x.to_string()).collect::<Vec<String>>());
        actual.sort_unstable();
        let mut expected = case.1.into_iter().map(|x| x.to_string()).collect::<Vec<String>>();
        expected.sort_unstable();
        assert_eq!(actual, expected);
    }
}