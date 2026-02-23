/*
 * https://leetcode.cn/problems/minimum-moves-to-capture-the-queen/description/
 * 可以使用分类讨论对其进行分析
 * 如果车能够攻击到皇后，答案为1，可以攻击到为在同一水平或者竖直线，线段上没有象
 * 如果象能攻击到皇后，那么答案为1，可以攻击到为在同一水平或者竖直线，线段上没有车
 * 如果车被象挡住了，那么通过移动象一次，车就可以攻击到皇后，答案为2
 * 如果象被车挡住，那么移动一次车，象就可以攻击到皇后，答案为2
 * 如果无法直接攻击到，那么必定可以通过切换一次方向攻击到。
 */
use std::cmp::{max, min};

pub fn min_moves_to_capture_the_queen(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> i32 {
    let bishopOk = |srcX: i32, srcY: i32, dstX: i32, dstY: i32| -> bool {
        if (srcX == dstX) { return false; }
        return ((srcY - dstY).abs() as f32 / (srcX - dstX).abs() as f32 - 1.0).abs() < 0.0001;
    };

    let inUnOrdRange = |left: i32, right: i32, target: i32| -> bool {
        let minNum = min(left, right);
        let maxNum = max(left, right);
        return target <= maxNum && target >= minNum;
    };

    if (a == e || b == f) {
        if (a == e && a == c)
        {
           return if !inUnOrdRange(b,f,d) { 1 } else { 2 };
        }
        if (b == f && b == d)
        {
            return if !inUnOrdRange(a,e,c) { 1 } else { 2 };
        }
        return 1;
    }
    let bishopOkResult = bishopOk(c, d, e, f);
    if (bishopOkResult)
    {
        if (a == e || c == e) { return 2; }
        let k1 = (f - b) as f32 / (e - a) as f32;
        let k2 = (f - d) as f32 / (e - c) as f32;
        if  k1 == k2 && inUnOrdRange(c,e,a) && inUnOrdRange(d,f,b)
        {
            return 2;
        }
        else
        {
            return 1;
        }
    }
    return 2;
}

pub fn main_rs()
{
    println!("Result: {}", min_moves_to_capture_the_queen(1,6,3,3,5,6));
}