#[cfg(test)]
mod tests;

struct Solution;

impl Solution {
    // Rotational Cipher.
    // https://www.facebookrecruiting.com/portal/coding_practice_question/?problem_id=238827593802550
    // Time complexity: O(N).
    // Space complexity: O(N).
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


    // Contiguous Subarrays.
    // https://www.facebookrecruiting.com/portal/coding_practice_question/?problem_id=226517205173943
    // Time complexity: O(N).
    // Space complexity: O(N).
    fn count_subarrays(arr: Vec<i32>) -> Vec<i32> {
        let mut counter = vec![0i32; arr.len()];
        let mut stack = Vec::new();

        for i in 0..arr.len() {
            while !stack.is_empty() && arr[i] > arr[*stack.last().unwrap()] {
                // Dismounted is the index where this one started to travel with us.
                let dismounted = stack.pop().unwrap();
                // Count how many steps this one travelled.
                counter[dismounted] = i as i32 - dismounted as i32;
            }

            stack.push(i);
        }

        while !stack.is_empty() {
            // Dismounted is the index where this one started to travel with us.
            let dismounted = stack.pop().unwrap();
            // Count how many steps this one travelled.
            counter[dismounted] = arr.len() as i32 - dismounted as i32;
        }

        for i in (0..arr.len()).rev() {
            counter[i] -= 1;

            while !stack.is_empty() && arr[i] > arr[*stack.last().unwrap()] {
                let dismounted = stack.pop().unwrap();
                counter[dismounted] += dismounted as i32 - i as i32;
            }

            stack.push(i);
        }

        while !stack.is_empty() {
            // Dismounted is the index where this one started to travel with us.
            let dismounted = stack.pop().unwrap();
            // Count how many steps this one travelled.
            counter[dismounted] += dismounted as i32 + 1;
        }

        counter
    }
}