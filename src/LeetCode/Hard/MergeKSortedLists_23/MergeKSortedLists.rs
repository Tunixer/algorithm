/* https://leetcode.cn/problems/merge-k-sorted-lists/
 */
use crate::base::List::ListNode;
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
struct Solution;
impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap = BinaryHeap::new();
        for list in lists.into_iter() {
            if let Some(node) = list {
                heap.push(Reverse(node));
            }
        }
        let mut dummy_head = None;
        let mut current = &mut dummy_head;
        while heap.len() > 0 {
            let mut top_node = heap.pop().unwrap();
            // 插入堆，这时候top_node.next被moved
            if let Some(next_node) = top_node.0.next.take() {
                heap.push(Reverse(next_node));
            }
            // 将pop出来的节点插入链表
            // 先将next置空，防止partial moved，使用take()会自动置None
            // top_node.0.next = None; // 如果不用take(), 则需要将next手动置空
            // current.next = Some(top_node.0);    // 在此基础上插入链表，否则需要重新new对象，非常好耗时
            // current = current.next.as_mut().unwrap();
            /* 采用Option::insert()方法，直接将top_node.0插入链表
             * 注意：Option::insert()方法会返回一个可变引用，因此需要使用&mut current
             */
            current = &mut (current.insert(top_node.0).next);
        }
        return dummy_head;
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.val.cmp(&other.val);
    } 
}
impl PartialOrd for ListNode  {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


pub fn main()
{
    let arr_list_1 = ListNode::from_vec(vec![1, 2, 3, 6, 8]);
    let arr_list_2 = ListNode::from_vec(vec![3, 5, 7, 9, 11]);
    let arr_list_3 = ListNode::from_vec(vec![6, 8, 10, 11, 13]);
    println!("{:?}", Solution::merge_k_lists(vec![arr_list_1, arr_list_2, arr_list_3]));
}