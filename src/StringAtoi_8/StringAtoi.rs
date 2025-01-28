use core::num;
use std::backtrace;

struct Solution;
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let bytes_s = s.into_bytes();
        let mut is_minus = false;
        let mut is_positive = false;
        let mut ret = 0 as i64;
        let mut num_start = false;
        let mut i = 0;
        // 去除空格
        while i < bytes_s.len() {
            match bytes_s[i] {
                b' ' => {
                    i += 1;
                    continue;
                }
                _ => { break; }
            }
        }
        // 正负号
        if i < bytes_s.len() {
            match bytes_s[i] {
                b'-' => {
                    is_minus = true;
                    i += 1;
                }
                b'+' => {
                    is_positive = true;
                    i += 1;
                }
                _ => { is_positive = true; }
            }
        }
        // 数字0
        while i < bytes_s.len() {
            match bytes_s[i] {
                b'0' => {
                    i += 1;
                }
                _ => { break; }
            }
        }
        // 数字
        for j in i..bytes_s.len() {
            match bytes_s[j] {
                b'0'..=b'9' => {
                    ret = (ret * 10) + (bytes_s[j] - b'0') as i64; 
                    if is_minus && ret > -(i32::MIN as i64) { return i32::MIN; }
                    if is_positive && ret > (i32::MAX as i64) { return i32::MAX; }
                }
                _ => { break; }
            } 
        }

        return if is_minus { -ret as i32} else { ret as i32 };
    }
}

pub fn main_rs() {
    println!("{}", Solution::my_atoi("+-12".to_string()));
    println!("{}", Solution::my_atoi("1-12".to_string()));
    println!("{}", Solution::my_atoi("   -042".to_string()));  
    println!("{}", Solution::my_atoi("1337c0d3".to_string())); 
    println!("{}", Solution::my_atoi("9223372036854775808".to_string()));
}
