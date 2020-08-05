// 211. Add and Search Word - Data structure design.
// https://leetcode.com/problems/add-and-search-word-data-structure-design/

use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct TrieNode {
    children: HashMap<char, TrieNode>,
    terminal: bool
}

impl TrieNode {
    pub fn new() -> Self {
        TrieNode::default()
    }

    pub fn insert(&mut self, word: &[char]) {
        if word.is_empty() {
            self.terminal = true;
            return;
        }

        self.children
            .entry(word[0])
            .or_insert(TrieNode::default())
            .insert(&word[1..]);
    }

    pub fn search(&self, word: &[char]) -> bool {
        if word.is_empty() {
            return self.terminal;
        }
        if word[0] == '.' {
            self.children.values().any(|node| node.search(&word[1..]))
        } else {
            if let Some(node) = self.children.get(&word[0]) {
                node.search(&word[1..])
            } else {
                false
            }
        }
    }
}