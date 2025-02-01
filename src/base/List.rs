use std::fmt::{self, Debug};

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
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