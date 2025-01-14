/*
 * https://leetcode.cn/problems/middle-of-the-linked-list
 */
use std::fmt::{self, Debug};

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
    pub fn from_vec(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        for &val in vec.iter().rev() {
            let mut node = ListNode::new(val);
            node.next = head;
            head = Some(Box::new(node));
        }
        head
    }
}

impl fmt::Display for ListNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ret = String::from("");
        let mut current = Some(self);
        while let Some(node) = current {
            match node.next {
                Some(_) => {
                    ret += format!("{}->", node.val).as_str();
                }
                None => {
                    ret += format!("{}", node.val).as_str();
                }
            }
            current = node.next.as_deref();
        }
        write!(f, "{} ", ret)
    }
}

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
