use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct TrieNode {
    children: HashMap<char, TrieNode>, 
    terminal: bool
}

impl TrieNode {
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