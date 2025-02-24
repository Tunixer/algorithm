struct Solution;
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut ans =  vec![];
        let mut current_combination = vec![];
        Self::dfs(&candidates, target, &mut ans, &mut current_combination, 0);
        return ans;
    }
    /* 这里的深度优先搜索是对每个元素，都有选/不选的可能，以及是否选下一个元素。三种可能。
     */
    fn dfs(candidates: &Vec<i32>, target: i32, ans: &mut Vec<Vec<i32>>, comb: &mut Vec<i32>, idx: usize) 
    {
        // 如果没有元素需要选择
        if idx == candidates.len() { return; }
        
        // 当前元素不选
        Self::dfs(candidates, target, ans, comb, idx + 1);

        // 如果选择当前元素
        let currentNum = candidates[idx];
        if target - currentNum > 0
        {
            comb.push(currentNum);
            Self::dfs(candidates, target - currentNum, ans, comb, idx);
            comb.pop();
        }

        if target == currentNum
        {
            comb.push(currentNum);
            ans.push(comb.clone());
            comb.pop();
        }
    }
}

pub fn main_rs() {
    println!("ans: {:?}", Solution::combination_sum(vec![1,  2], 3));
}
