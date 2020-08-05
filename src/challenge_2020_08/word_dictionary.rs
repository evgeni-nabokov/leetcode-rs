// 211. Add and Search Word - Data structure design.
// https://leetcode.com/problems/add-and-search-word-data-structure-design/

use crate::challenge_2020_08::trie_node::TrieNode;

#[derive(Clone, Debug)]
pub struct WordDictionary {
    root: TrieNode,
}

impl WordDictionary {
    /** Initialize your data structure here. */
    pub fn new() -> Self {
        WordDictionary {
            root: TrieNode::new(),
        }
    }

    /** Adds a word into the data structure. */
    pub fn add_word(&mut self, word: String) {
        self.root.insert(&word.chars().collect::<Vec<char>>());
    }

    /** Returns if the word is in the data structure. A word could contain the dot character '.' to represent any one letter. */
    pub fn search(&self, word: String) -> bool {
        self.root.search(&word.chars().collect::<Vec<char>>())
    }
}