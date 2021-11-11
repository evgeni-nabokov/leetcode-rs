// 208. Implement Trie (Prefix Tree).
// https://leetcode.com/problems/implement-trie-prefix-tree/

use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct TrieNode {
    children: HashMap<char, TrieNode>,
    terminal: bool
}

impl TrieNode {
    pub fn new(terminal: bool) -> Self {
        TrieNode {
            children: HashMap::new(),
            terminal,
        }
    }

    pub fn insert(&mut self, word: &[char]) {
        if word.is_empty() {
            self.terminal = true;
            return;
        }

        self.children
            .entry(word[0])
            .or_insert(TrieNode::new(false))
            .insert(&word[1..]);
    }

    pub fn contains(&self, word: &[char]) -> bool {
        if word.is_empty() {
            return self.terminal;
        }
        if let Some(node) = self.children.get(&word[0]) {
            node.contains(&word[1..])
        } else {
            false
        }
    }

    pub fn starts_with(&self, word: &[char]) -> bool {
        if word.is_empty() {
            return true;
        }
        if let Some(node) = self.children.get(&word[0]) {
            node.starts_with(&word[1..])
        } else {
            false
        }
    }
}

#[derive(Clone, Debug)]
pub struct Trie {
    root: TrieNode,
}

impl Trie {
    /** Initialize your data structure here. */
    pub fn new() -> Self {
        Trie {
            root: TrieNode::new(true),
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