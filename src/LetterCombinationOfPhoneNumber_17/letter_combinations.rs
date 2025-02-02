/*
 * https://leetcode.cn/problems/letter-combinations-of-a-phone-number/
 */
use std::collections::HashMap;
struct Solution;
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        return Self::letter_combinations_loop(digits);
    }
    // 循环版本
    fn letter_combinations_loop( digits: String) -> Vec<String> {
        let mut ret = vec![];
        let data_map = HashMap::from([
           (2, vec!['a', 'b', 'c']),
           (3, vec!['d', 'e', 'f']),
           (4, vec!['g', 'h', 'i']),
           (5, vec!['j', 'k', 'l']),
           (6, vec!['m', 'n', 'o']),
           (7, vec!['p', 'q', 'r', 's']),
           (8, vec!['t', 'u', 'v']), 
           (9, vec!['w', 'x', 'y', 'z']),
        ]);
        let digits_vec = digits.chars().collect::<Vec<char>>();
        // println!("digits_vec: {:?}", digits_vec);
        for i in 0..digits_vec.len() {
            let current_digit = digits_vec[i] as i32 - '0' as i32;
            let current_digit_vec = data_map.get(&current_digit).unwrap();
            if ret.len() == 0 {
                for j in 0..current_digit_vec.len() {
                    ret.push(current_digit_vec[j].to_string());
                } 
            }
            else {
                // 依次扩展当前ret中的每个元素
                let current_ret_len = ret.len();
                for j in 0..current_ret_len {
                    for k in 0..current_digit_vec.len() {
                        ret.push(ret[j].clone() + &current_digit_vec[k].to_string());
                    }
                }
                // 删掉ret中前current_ret_len个元素
                ret.drain(0..current_ret_len);
            }
        }
        return ret;
    }
}

pub fn main_rs() {
    println!("ans: {:?}", Solution::letter_combinations("234".to_string())); 
}