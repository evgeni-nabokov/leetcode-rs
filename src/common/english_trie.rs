#[derive(Debug, Clone)]
struct TrieNode {
    children: Vec<Option<TrieNode>>,
    terminal: bool,
}

impl TrieNode {
    const ALPHABET_SIZE: usize = 26;

    pub fn new(terminal: bool) -> Self {
        Self {
            children: vec![None; TrieNode::ALPHABET_SIZE],
            terminal,
        }
    }
}

pub struct Trie {
    root: TrieNode,
}

impl Trie {
    const BASE_INDEX: u8 = 97;

    pub fn new() -> Self {
        Self {
            root: TrieNode::new(true),
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for b in word.as_bytes() {
            let idx = Trie::get_index(*b);
            if node.children[idx].is_none() {
                node.children[idx] = Some(TrieNode::new(false));
            }
            node = node.children[idx].as_mut().unwrap();
        }
        node.terminal = true;
    }

    pub fn contains(&self, word: &str) -> bool {
        if word.is_empty() {
            return self.root.terminal;
        }

        let mut node = &self.root;
        for b in word.as_bytes() {
            let idx = Trie::get_index(*b);
            if let Some(n) = node.children[idx].as_ref() {
                node = n;
            } else {
                return false;
            }
        }
        node.terminal
    }

    pub fn starts_with(&self, prefix: &str) -> bool {
        if prefix.is_empty() {
            return true;
        }

        let mut node = &self.root;
        for b in prefix.as_bytes() {
            let idx = Trie::get_index(*b);
            if let Some(n) = node.children[idx].as_ref() {
                node = n;
            } else {
                return false;
            }
        }
        true
    }

    #[inline]
    fn get_index(b: u8) -> usize {
        (b - Trie::BASE_INDEX) as usize
    }
}