/* https://leetcode.cn/problems/unique-binary-search-trees
 */

struct Solution;
impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        return Self::num_trees_recursive(n);
    }
    /* recursive
     * f(1) = 1
     * f(2) = 2
     * f(3) = f(2)*f(0) + f(1)*f(1) + f(0)*f(2)
     * f(n) = sum_{i = 0}{n-1} f(i)*f(n-i-1)
     */
    pub fn num_trees_recursive(n: i32) -> i32 {
        if n == 1 || n == 0 { return 1; }
        if n == 2 { return 2; }
        let mut result = vec![1, 1, 2];
        let mut sum = 0;
        for i in 3..=n {
            sum = 0;
            for j in 0..i {
                // println!("i: {}, j: {}, i-j-1: {}",i, j, i-j-1);
                sum += result[j as usize] * result[(i-j-1) as usize];
            }
            result.push(sum);
        }
        return result[n as usize];
    }
    /* 卡特兰数有一个O(n)的递推，可以比O(n^2)
     */
}

pub fn main() {
    let ans = Solution::num_trees(18);
    println!("{}",ans);
}