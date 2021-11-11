use super::english_trie::*;

#[test]
fn english_trie_test() {
    let mut trie = Trie::new();
    assert_eq!(trie.contains(""), true);
    assert_eq!(trie.starts_with(""), true);
    assert_eq!(trie.contains("apple"), false);
    assert_eq!(trie.starts_with("apple"), false);
    trie.insert("apple");
    assert_eq!(trie.contains("apple"), true);
    assert_eq!(trie.contains("app"), false);
    assert_eq!(trie.starts_with("app"), true);
    trie.insert("app");
    assert_eq!(trie.contains("app"), true);
}