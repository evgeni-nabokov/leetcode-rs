#[cfg(test)]
mod tests;

struct Solution;

impl Solution {
    // 472. Concatenated Words.
    // https://leetcode.com/problems/concatenated-words/
    pub fn find_all_concatenated_words_in_a_dict(mut words: Vec<String>) -> Vec<String> {
        // Auxiliary structure - Trie.
        #[derive(Debug, Clone)]
        struct TrieNode {
            children: Vec<Option<TrieNode>>,
            terminal: bool,
        }

        impl TrieNode {
            pub fn new(terminal: bool) -> Self {
                Self {
                    children: vec![None; 26],
                    terminal,
                }
            }

            pub fn insert(&mut self, word: &str) {
                let mut node = self;
                for b in word.as_bytes() {
                    let idx = TrieNode::get_index(*b);
                    if node.children[idx].is_none() {
                        node.children[idx] = Some(TrieNode::new(false));
                    }
                    node = node.children[idx].as_mut().unwrap();
                }
                node.terminal = true;
            }

            pub fn is_concatenated(&self, word: &str) -> bool {
                if word.is_empty() {
                    return false;
                }

                let bytes = word.as_bytes();
                let mut node = self;

                for i in 0..bytes.len() {
                    let idx = TrieNode::get_index(bytes[i]);
                    if let Some(n) = node.children[idx].as_ref() {
                        if n.terminal {
                            if i == bytes.len() - 1 {
                                return true;
                            }

                            if self.is_concatenated(&word[i + 1..]) {
                                return true;
                            }
                        }
                        node = n;
                    } else {
                        return false;
                    }
                }

                false
            }

            #[inline]
            fn get_index(b: u8) -> usize {
                (b - 97) as usize
            }
        }

        // Solution.
        if words.len() < 2 {
            return vec![];
        }

        words.sort_unstable_by_key(|x| x.len());
        let mut res = Vec::new();
        let mut trie = TrieNode::new(true);
        for w in words {
            if trie.is_concatenated(&w) {
                res.push(w);
            } else {
                trie.insert(&w);
            }
        }

        res
    }
}
