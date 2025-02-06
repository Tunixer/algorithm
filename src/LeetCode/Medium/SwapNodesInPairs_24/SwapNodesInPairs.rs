/*
 * https://leetcode.cn/problems/swap-nodes-in-pairs
 */
use crate::base::List::ListNode;
struct Solution;
impl Solution {
    pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // 当前指针
        let mut dummy_head = Box::new(ListNode::new(0));
        let mut tmp = &mut dummy_head ; // 返回链表的最末尾
        let mut nxt_nxt = None;
        while let Some(mut head_node) = head {
            let mut nxt = head_node.next.take();

            if let Some(mut next_node) = nxt {
                nxt_nxt = next_node.next.take();
                next_node.next = Some(head_node);
                tmp.next = Some(next_node); // 将逆序好的一对，插入返回链表的末尾
                // tmp重新走两格，指向返回链表的末尾
                tmp = tmp.next.as_mut().unwrap(); // tmp的类型是&Box<ListNode>，所以这里需要as_mut以后再unwrap
                tmp = tmp.next.as_mut().unwrap();
            } //最后一个单独判断
            else {
                // 无法成对，所以直接把head_node接到tmp末尾就行
                nxt_nxt = None;
                tmp.next = Some(head_node);
            }
            head = nxt_nxt;
        }
        return dummy_head.next;
    }
}

pub fn main() {
    let arr_list = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
    // 这一步可能是把整个数组都复制了一遍
    println!("{:?}", Solution::swap_pairs(arr_list));
}
