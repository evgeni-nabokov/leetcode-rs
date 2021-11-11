static BASE_INDEX: u8 = 97;
static ALPHABET_SIZE: usize = 26;

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
}

pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            root: TrieNode::new(true),
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for b in word.bytes() {
            let idx = Trie::get_index(b);
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
        for b in word.bytes() {
            let idx = Trie::get_index(b);
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
        for b in prefix.bytes() {
            let idx = Trie::get_index(b);
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
        (b - BASE_INDEX) as usize
    }
}