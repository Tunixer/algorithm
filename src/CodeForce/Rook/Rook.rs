/*
 * A. Rook: https://codeforces.com/problemset/problem/1907/A
 */
#[derive(Clone)] 
struct RolColIdx {
    pub row:u32,
    pub col:u32,
}

use std::{sync::Mutex, collections::HashMap};
// use once_cell::sync::Lazy;
/*
 * 这里的col2num使用的是Lazy惰性加载，因为static函数不支持
 * once_cell: Rust library for single assignment cells and lazy statics without macros
 */
// static col2num: Lazy<Mutex<HashMap<&str,u32>>> = Lazy::new(|| {
//     let mut m = HashMap::from([
//         ("a", 1),
//         ("b", 2),
//         ("c", 3),
//         ("d", 4),
//         ("e", 5),
//         ("f", 6),
//         ("g", 7),
//         ("h", 8),
//     ]);
//     Mutex::new(m)
// });

// static num2col: Lazy<Mutex<HashMap<u32,&str>>> = Lazy::new(|| {
//     let mut m = HashMap::from([
//         (1, "a"),
//         (2, "b"),
//         (3, "c"),
//         (4, "d"),
//         (5, "e"),
//         (6, "f"),
//         (7, "g"),
//         (8, "h"),
//     ]);
//     Mutex::new(m)
// });

fn solve(rol_col:&RolColIdx, num2col: &HashMap<u32,&str>) {
    for i in 1..9 {
        if i != rol_col.row {
            println!("{}{}", num2col[&rol_col.col], i);
        }
    }
    for j in 1..9 {
        if j != rol_col.col {
            println!("{}{}", num2col[&j], &rol_col.row);
        }
    }
}

pub fn main_rs() {
    let num2col = HashMap::from([
                (1, "a"),
                (2, "b"),
                (3, "c"),
                (4, "d"),
                (5, "e"),
                (6, "f"),
                (7, "g"),
                (8, "h"),
            ]);
    let col2num = HashMap::from([
        ("a", 1),
        ("b", 2),
        ("c", 3),
        ("d", 4),
        ("e", 5),
        ("f", 6),
        ("g", 7),
        ("h", 8),
    ]);

    let mut caseNumStr = String::new();
    std::io::stdin().read_line(&mut caseNumStr).expect("read_line error!");
    let caseNum = caseNumStr.trim().parse::<i32>().unwrap();
    let mut startIdxVec = Vec::new();
    for i in 0..caseNum {
        let mut chessboardLocation = String::new();
        std::io::stdin().read_line(&mut chessboardLocation).expect("read_line error!");
        let mut startIdx = RolColIdx{col:col2num[&chessboardLocation[0..1]],
            row:(&chessboardLocation[1..2]).parse::<u32>().unwrap()};
        // println!("caseNum: {}, Row: ({}, {})", &caseNum, &startIdx.row, &startIdx.col);
        solve(&startIdx, &num2col);
        startIdxVec.push(startIdx);

    }    
}
