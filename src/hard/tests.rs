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

#[test]
fn number_to_words_test() {
    let test_cases = vec![
        (0, "Zero"),
        (1, "One"),
        (20, "Twenty"),
        (100, "One Hundred"),
        (1000000, "One Million"),
        (123, "One Hundred Twenty Three"),
        (12345, "Twelve Thousand Three Hundred Forty Five"),
        (1234567, "One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven"),
        (1234567891, "One Billion Two Hundred Thirty Four Million Five Hundred Sixty Seven Thousand Eight Hundred Ninety One"),
    ];
    for case in test_cases {
        assert_eq!(Solution::number_to_words(case.0), case.1.to_string());
    }
}