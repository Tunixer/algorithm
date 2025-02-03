use crate::base::List::ListNode;
struct Solution;
impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy_head = Box::new(ListNode::new(0));
        // 这里拿到的是对box的&mut，最多只能一个&mut，允许多个&immutable
        let mut current = &mut dummy_head;
        // 这里的as_ref把 Option<Box<ListNode>> 转换成 Option<&Box<ListNode>>
        // 这样就可以使用 .as_ref() 来获取引用，这样可以同时持有多个对Box的不可变引用，否则就只能有一个可变引用
        // 此外，Box的as_ref需要额外再讨论
        // Option提供nullptr的功能，Box提供堆上的内存。但是，Box本身不能提供nullptr的功能
        let mut list1_head = list1.as_ref();
        let mut list2_head = list2.as_ref();
        while list1_head.is_some() && list2_head.is_some() {
            if list1_head.unwrap().val < list2_head.unwrap().val {
                current.next = Some(Box::new(ListNode::new(list1_head.unwrap().val)));
                list1_head = list1_head.unwrap().next.as_ref();
            } else {
                current.next = Some(Box::new(ListNode::new(list2_head.unwrap().val)));
                list2_head = list2_head.unwrap().next.as_ref();
            }
            current = current.next.as_mut().unwrap();
        }
        if list1_head.is_some() {
            current.next = Some(list1_head.unwrap().clone()); 
        }
        if list2_head.is_some() {
            current.next = Some(list2_head.unwrap().clone()); 
        }
        return dummy_head.next;
    }
}


pub fn main_rs() {
    let arr_list_1 = ListNode::from_vec(vec![1, 2, 3, 6, 8]);
    let arr_list_2 = ListNode::from_vec(vec![3, 5, 7, 9, 11]);
    // 这一步可能是把整个数组都复制了一遍
    let head = Solution::merge_two_lists(arr_list_1, arr_list_2);
    println!("{:?}", head);
}