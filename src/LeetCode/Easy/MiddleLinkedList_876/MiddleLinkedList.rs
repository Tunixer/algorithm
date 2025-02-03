/*
 * https://leetcode.cn/problems/middle-of-the-linked-list
 */
use crate::base::List::ListNode;
struct Solution;
impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut fast_ptr = head.clone();
        let mut slow_ptr = head.clone();
        while true {
            if fast_ptr.is_none() {
                return slow_ptr;
            }
            fast_ptr = fast_ptr.unwrap().next;

            if fast_ptr.is_none() {
                return slow_ptr;
            }
            fast_ptr = fast_ptr.unwrap().next;
            slow_ptr = slow_ptr.unwrap().next;
        }
        return None;
    }
}

pub fn main_rs() {
    let arr_list = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
    // 这一步可能是把整个数组都复制了一遍
    println!("{}", arr_list.clone().unwrap());
    let head = Solution::middle_node(arr_list);
    println!("{}", head.unwrap().val);
}
