use std::iter;

/* https://leetcode.cn/problems/generate-parentheses/description/
 * [分析]
 * 如果我有一对括号(a)b开头。能在其他地方插入括号的位置，就是a和b。
 * 如果一开始是n个，那么接下来的a+b要等于n-1对括号。
 * (a)b
 * 如果a的长度为0，b的长度为4，那么最后会形成4个
 */
struct Solution;
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        if n == 1 {
            return vec!["()".to_string()];
        }
        if n == 0 { return vec!["".to_string()]; }
        let mut ret = vec![];
        for i in 0..n {
            let res_a  = Self::generate_parenthesis(i);
            let res_b  = Self::generate_parenthesis(n - 1 - i);
            // 按照(a)b的形式，插入括号
            for a in res_a.iter() {
                for b in res_b.iter() {
                    ret.push(format!("({}){}", a, b));
                } 
            }

        }
        return ret;
    }
}

pub fn main_rs() {
    println!("{:?}", Solution::generate_parenthesis(2));
    println!("{:?}", Solution::generate_parenthesis(3));
}