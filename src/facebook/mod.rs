#[cfg(test)]
mod tests;

struct Solution;

impl Solution {
    fn rotational_cipher(input: String, rotation_factor: i32) -> String {
        if input.is_empty() || rotation_factor == 0 {
            return input;
        }

        let alpha_rf = rotation_factor as u8 % 26;
        let digit_rf = rotation_factor  as u8 % 10;
        if alpha_rf == 0 && digit_rf == 0 {
            return input;
        }

        let mut res_chars: Vec<char> = Vec::with_capacity(input.len());
        for c in input.chars() {
            let mut new_c = c;
            if c.is_ascii_digit() {
                let d = c as u8 - 48;
                let new_d = (d + digit_rf) % 10;
                new_c = (new_d + 48) as char;
            } else if c.is_ascii_uppercase() {
                let c_idx = c as u8 - 65;
                let new_c_idx = (c_idx + alpha_rf) % 26;
                new_c = (new_c_idx + 65) as char;
            } else if c.is_ascii_lowercase() {
                let c_idx = c as u8 - 97;
                let new_c_idx = (c_idx + alpha_rf) % 26;
                new_c = (new_c_idx + 97) as char;
            }
            res_chars.push(new_c)
        }

        res_chars.into_iter().collect()
    }
}