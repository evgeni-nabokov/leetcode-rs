use super::trie_node::TrieNode;

#[derive(Clone, Debug)]
pub struct Trie {
    root: TrieNode,
}

impl Trie {

    /** Initialize your data structure here. */
    pub fn new() -> Self {
        Trie {
            root: TrieNode::default(),
        }
    }

    /** Inserts a word into the trie. */
    pub fn insert(&mut self, word: String) {
        self.root.insert(&word.chars().collect::<Vec<char>>())
    }

    /** Returns if the word is in the trie. */
    pub fn search(&self, word: String) -> bool {
        self.root.contains(&word.chars().collect::<Vec<char>>())
    }

    /** Returns if there is any word in the trie that starts with the given prefix. */
    pub fn starts_with(&self, prefix: String) -> bool {
        self.root.starts_with(&prefix.chars().collect::<Vec<char>>())
    }
}