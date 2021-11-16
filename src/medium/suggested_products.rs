#[derive(Clone, Debug)]
struct TrieNode {
    children: Vec<Option<TrieNode>>,
    terminal: bool,
}


impl TrieNode {
    pub fn new(terminal: bool) -> Self {
        TrieNode {
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

    pub fn search(&self, word: &str, n: usize) -> Vec<String> {
        let mut res = Vec::new();

        let mut prefix = Vec::with_capacity(word.len());
        let mut node = self;
        for b in word.as_bytes() {
            let idx = TrieNode::get_index(*b);
            if node.children[idx].is_none() {
                return res;
            }
            prefix.push(*b);
            node = node.children[idx].as_ref().unwrap();
        }

        if node.terminal {
            res.push(word.to_string());
        }

        if res.len() == n {
            return res;
        }


        fn dfs(node: &TrieNode, path: Vec<u8>, res: &mut Vec<String>, n: usize) {
            for i in 0..26 {
                if node.children[i].is_some() {
                    let new_node = node.children[i].as_ref().unwrap();
                    let mut new_path = path.clone();
                    new_path.push(TrieNode::get_byte(i));
                    if new_node.terminal {
                        res.push(String::from_utf8(new_path.clone()).unwrap());
                        if res.len() == n {
                            return;
                        }
                    }
                    dfs(new_node, new_path, res, n);
                    if res.len() == n {
                        return;
                    }
                }
            }
        }

        dfs(node, prefix, &mut res, n);

        res
    }

    fn get_index(b: u8) -> usize {
        b as usize - 97
    }

    fn get_byte(i: usize) -> u8 {
        i as u8 + 97
    }
}

impl super::Solution {
    // Trie method.
    pub fn suggested_products(products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        let mut words = TrieNode::new(true);

        for p in products {
            words.insert(&p);
        }

        let mut res = Vec::with_capacity(search_word.len());
        for i in 1..=search_word.len() {
            res.push(words.search(&search_word[0..i], 3));
        }

        res
    }
}