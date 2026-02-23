/*
LeetCode 26
https://leetcode.cn/problems/remove-duplicates-from-sorted-array/description/
*/
struct Solution {}
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut unique_nums = 1;
        let mut last_num = nums[0];
        for i in 1..nums.len() {
            if nums[i] != last_num {
                last_num = nums[i];
                nums[unique_nums] = nums[i];
                unique_nums += 1;
            }
        }
        return unique_nums as i32;
    }
}

#[cfg(test)]
mod tests {
    // 注意这个惯用法：在 tests 模块中，从外部作用域导入所有名字。
    use super::*;

    #[test]
    fn test_remove_duplicates() {
        let mut nums = vec![1, 1, 2];
        assert_eq!(Solution::remove_duplicates(&mut nums), 2);
        assert_eq!(nums[0], 1);
        assert_eq!(nums[1], 2);
    }
}
