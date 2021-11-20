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

    // 273. Integer to English Words.
    // https://leetcode.com/problems/integer-to-english-words/
    // Time complexity : O(N), N - number of digits.
    // Space complexity: O(N), N - number of digits.
    pub fn number_to_words(mut num: i32) -> String {
        if num == 0 {
            return "Zero".to_string();
        }

        let numerals_1 = ["One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten",
            "Eleven", "Twelve", "Thirteen", "Fourteen", "Fifteen", "Sixteen", "Seventeen",
            "Eighteen", "Nineteen"];
        let numerals_2 = ["Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety"];

        let numerals_3 = ["Thousand", "Million", "Billion", "Trillion", "Quadrillion", "Quintillion", "Sextillion", "Septillion", "Octillion", "Nonillion"];

        let mut res = Vec::new();
        let mut chunk_idx = 0;
        while num > 0 {
            let mut chunk_numerals = Vec::with_capacity(4);
            let chunk = num % 1000;
            let hundreds = chunk / 100;
            if hundreds > 0 {
                chunk_numerals.push(numerals_1[hundreds as usize - 1].to_string());
                chunk_numerals.push("Hundred".to_string());
            }
            let under_hundred = chunk % 100;
            if under_hundred > 0 && under_hundred < 20 {
                chunk_numerals.push(numerals_1[under_hundred as usize - 1].to_string());
            } else {
                let tens = under_hundred / 10;
                if tens > 0 {
                    chunk_numerals.push(numerals_2[tens as usize - 2].to_string());
                }
                let ones = under_hundred % 10;
                if ones > 0 {
                    chunk_numerals.push(numerals_1[ones as usize - 1].to_string());
                }
            }

            if chunk_idx > 0 && !chunk_numerals.is_empty() {
                chunk_numerals.push(numerals_3[chunk_idx - 1].to_string());
            }

            res.push(chunk_numerals);
            chunk_idx += 1;
            num /= 1000;
        }

        res.into_iter().rev().flat_map(|x| x).collect::<Vec<String>>().join(" ")
    }
}
