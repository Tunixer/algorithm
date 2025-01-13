/* https://leetcode.cn/problems/sort-an-array/
 */
struct Solution;
struct SortEachTurn
{
    start: usize,
    end: usize,
}
impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        Self::quick_sort(nums)
        // let mut sorted_arr = nums.clone();
        // sorted_arr.sort();
        // return sorted_arr;
    }
    // 快速排序
    fn quick_sort(nums: Vec<i32>) -> Vec<i32> {
        let mut sorted_arr = nums.clone();
        let mut stack: Vec<SortEachTurn> = Vec::new();
        let init_start = 0 as usize;
        let init_end = sorted_arr.len() - 1;
        stack.push(SortEachTurn { start: init_start, end: init_end });
        while !stack.is_empty() {
            if let Some(top_elem) = stack.pop() {
                let opt_pivot_index = Self::quick_sort_partition(&mut sorted_arr, top_elem.start, top_elem.end);
                if let Some(pivot_index) = opt_pivot_index {
                    if pivot_index > 0 { stack.push(SortEachTurn { start: top_elem.start, end: pivot_index - 1 }); }
                    if pivot_index + 1 <= top_elem.end { stack.push(SortEachTurn { start: pivot_index + 1, end: top_elem.end }); }
                }
            }
        }
        return sorted_arr;
    }
    fn quick_sort_partition(nums: &mut Vec<i32>, start: usize, end: usize) -> Option<usize> {
        if start >= end {
            return None;
        }

        let mut lo = start + 1;
        let mut hi = end;
        let mut pivot_index = start;
        let pivot = nums[pivot_index];

        while lo <= hi
        {
            while lo <= hi && nums[lo] < pivot
            {
                // pivot_index = lo;
                lo += 1;
            }
            while lo <= hi && nums[hi] > pivot
            {
                hi -= 1;
            }
            if lo >= hi { break; }
            nums.swap(lo, hi);
            // pivot_index = lo;
            lo += 1;
            hi -= 1;
        }
        
        /* 关键是这一步，上面已经完成了pivot的其他元素的划分，如何根据lo和hi把pivot放到合适的位置？
         * 我们需要的是小于pivot区间的最后一个元素
         * 本来是借助lo在while循环里追踪的，可以参考上面的注释，但发现都会经历一个lo+=1的过程。
         * 所以lo - 1 就可以
         */ 
        pivot_index = lo - 1;
        nums.swap(pivot_index, start);
        return Some(pivot_index);
    }
}

pub fn main_rs() {
    // let nums = vec![-4,0,7,4,9,-5,-1,0,-7,-1];
    let nums = vec![-7087,12694,-19352,-7660,12052,-11316,-352,18321,15,19967,6331,-1289,6540,-10454,-19309,-10193,15074,8926,510,-11044];
    let result = Solution::sort_array(nums);
    println!("{:?}", result);
}

