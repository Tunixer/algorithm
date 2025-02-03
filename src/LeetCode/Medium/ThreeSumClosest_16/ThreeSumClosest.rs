struct Solution;
impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut sorted_nums = nums.clone();
        sorted_nums.sort();
        let mut ans = i32::MAX;
        for i in 0..sorted_nums.len() {
            let mut left = i + 1;
            let mut right = sorted_nums.len() - 1;
            while left < right {
                let sum = sorted_nums[i] + sorted_nums[left] + sorted_nums[right];
                if (ans - target).abs() > (sum - target).abs() {
                    ans = sum; 
                }
                if sum == target {
                    return sum;
                } else if sum < target {
                    left += 1;
                } else {
                    right -= 1;
                }
            } 
        }
        return ans;
    }
}

pub fn main_rs() {
    println!("ans: {}", Solution::three_sum_closest(vec![-1, 2, 1, -4], 1)); 
}