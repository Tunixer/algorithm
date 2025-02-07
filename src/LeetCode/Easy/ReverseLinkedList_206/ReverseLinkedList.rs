/*
 * https://leetcode.cn/problems/reverse-linked-list/
 */
use crate::base::List::ListNode;
struct Solution;
impl Solution {
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy_head = Box::new(ListNode::new(0));
        let mut current = &mut dummy_head;
        while let Some(mut head_node) = head
        {
            let next_node = head_node.next.take();
            head_node.next = current.next.take();
            current.next = Some(head_node);
            head = next_node;
        }
        return dummy_head.next;
    }
}

pub fn main() {
    let arr_list = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
    let head = Solution::reverse_list(arr_list);
    println!("{:?}", &head.unwrap());
}