/* Trie树
 */

struct Solution;
#[derive(Clone, Debug)]
struct TrieNode {
    children: Vec<Option<TrieNode>>,
    count: usize,
    is_end: bool,
}


impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: vec![None; 26],
            count: 0,
            is_end: false,
        }
    } 
    fn insert(&mut self, word: &str) {
        let mut node = self;
        for c in word.chars() {
            let i = (c as u8 - b'a') as usize;
            // 当前节点是否为空，为空插入字符
            if node.children[i].is_none() {
                node.children[i] = Some(TrieNode::new()); 
                node.count += 1;
            }
            node = node.children[i].as_mut().unwrap();
        }
        node.is_end = true;
    }
}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut node = TrieNode::new();
        for curr_str in strs {
            if curr_str.len() == 0 {
                return "".to_string();
            }
            node.insert(&curr_str);
        }
        let mut ret_node = &node;
        let mut ret = "".to_string();
        while ret_node.count == 1 {
            for i in 0..26 {
                if let Some(child) = &ret_node.children[i] {
                    // 将当前i对应的英语字母转化为string，push入ret
                    ret.push(char::from(i as u8 + b'a'));
                    if child.is_end {
                        return ret;
                    }
                    // 继续向下遍历
                    ret_node = child;
                    break;
                }
            }
        }
        return ret;
    }
}

pub fn main_rs() {
    // let mut first_node = TrieNode::new();
    // first_node.insert("hello");
    // first_node.insert("helle");
    println!("{:?}", Solution::longest_common_prefix(vec!["ahhc".to_string(), "ahha".to_string(), "ahhb".to_string()]));
}