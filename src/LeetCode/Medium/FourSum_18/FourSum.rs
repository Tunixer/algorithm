struct Solution;
impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut sorted_nums = nums.clone();
        let mut ret = vec![];
        sorted_nums.sort();
        for i in 0..sorted_nums.len() {
            // nums[i]已经使用过了，那么nums[i+1]如果和nums[i]相等，j以后的数组就不需要使用了nums[i+1]了
            if i > 0 && sorted_nums[i] == sorted_nums[i - 1] { continue; }
            for j in i + 1..sorted_nums.len() {
                if j > i + 1 && sorted_nums[j] == sorted_nums[j - 1] { continue; }
                let mut left = j + 1;
                let mut right = sorted_nums.len() - 1;
                while left < right {
                    let sum =
                        sorted_nums[i] as i64 + sorted_nums[j] as i64 + sorted_nums[left] as i64 + sorted_nums[right] as i64;
                    if sum == target as i64 {
                        // println!("i: {}, j: {}, left: {}, right: {}", i, j, left, right);
                        ret.push(vec![
                            sorted_nums[i],
                            sorted_nums[j],
                            sorted_nums[left],
                            sorted_nums[right],
                        ]);
                        let left_value = sorted_nums[left];
                        while left <= right && sorted_nums[left] == left_value {
                            left += 1;
                        }
                        let right_value = sorted_nums[right];
                        while left <= right && sorted_nums[right] == right_value {
                            right -= 1;
                        }
                    } else if sum < target  as i64 {
                        left += 1;
                    } else {
                        right -= 1;
                    }
                }
            }
        }
        return ret;
    }
}

pub fn main_rs() {
    println!("ans: {:?}", Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0));
    println!("ans: {:?}", Solution::four_sum(vec![2, 2, 2, 2, 2], 8));
    println!("ans: {:?}", Solution::four_sum(vec![1000000000,1000000000,1000000000,1000000000], -294967296));
}
