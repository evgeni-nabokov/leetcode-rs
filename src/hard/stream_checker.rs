// 1032. Stream of Characters.
// https://leetcode.com/problems/stream-of-characters/
// Time complexity: O(M), M - max word length.
// Space complexity: O(M), M - max word length.

pub struct StreamChecker {
    trie: TrieNode,
    stream: Vec<u8>,
}

impl StreamChecker {
    pub fn new(words: Vec<String>) -> Self {
        let mut trie = TrieNode::new(false);
        let mut max_len = 0;

        for w in words {
            max_len = max_len.max(w.len());
            trie.insert(w);
        }

        StreamChecker {
            trie,
            stream: Vec::new(),
        }
    }

    pub fn query(&mut self, letter: char) -> bool {
        self.stream.push(letter as u8);

        let mut node = &mut self.trie;
        let l = self.stream.len();
        for i in 0..l {
            let b = self.stream[l - i - 1];
            if node.terminal {
                return true;
            }
            if let Some(inner_node) = node.children[TrieNode::get_index(b)].as_mut() {
                node = inner_node;
            } else {
                return false;
            }
        }
        node.terminal
    }
}

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

    fn insert(&mut self, word: String) {
        let mut node = self;
        for b in word.as_bytes().iter().rev() {
            let idx = TrieNode::get_index(*b);
            if node.children[idx].is_none() {
                node.children[idx] = Some(TrieNode::new(false));
            }
            node = node.children[idx].as_mut().unwrap();
        }
        node.terminal = true;
    }

    #[inline]
    fn get_index(b: u8) -> usize {
        (b - 97) as usize
    }
}