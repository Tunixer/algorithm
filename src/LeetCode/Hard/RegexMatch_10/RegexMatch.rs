/*
https://leetcode.cn/problems/regular-expression-matching/description/
给你一个字符串 s 和一个字符规律 p，请你来实现一个支持 '.' 和 '*' 的正则表达式匹配。

'.' 匹配任意单个字符
'*' 匹配零个或多个前面的那一个元素
所谓匹配，是要涵盖 整个 字符串 s的，而不是部分字符串。
【解】
这种情况下，还是应该进行分类讨论，核心解体方法是动态规划
假设字符串是s，模式串是p。
分类1：
f[i][j] 表示串p中到第j个字符能否匹配到串s中的第i个字符。
那么，如果s[i] != p[j], 就只能返回false。
情况2：
这个时候开始考虑*的情况
如果模式串p[j]是*，那么就说明p[j-1]可以匹配任意自然数次（包括0次）
1）如果只匹配0次
那么，说明p[j]的*和p[j-1]的字符被浪费掉了，不用考虑。如此，f[i][j]退化为
f[i][j] = f[i][j-2]
只用参考第p[0..j-2]的字符串就行
2）如果说匹配大于1次
那么如果匹配k次，那么说明串s末尾的k个字符和字符p[j-1]是一样的。
我们只需要分析，截短这些匹配串之后的情况，即
相当于f[i][j] = f[i-k][j-2]。
这个感觉算数复杂度比较高，而且有一种莫名的递归过程在里面，需要每次从0到length枚举这个k
还是只考虑串s当前最末尾的s[i]的匹配情况
f[i][j] = f[i-1][j] or f[i][j-2] if s[i] = p[j-1]，即s最后一个字母能匹配能用模式串匹配上
f[i][j] = f[i][j-2] if s[i] != p[j-1]， 即退化为（1）中，匹配0次的情况

代码编写
需要一个二维数组用来存储
然后，动态规划，需要一个二维的分布
上述动态规划对f[i][j]的情况经过了分类讨论。因此，不同的结果之间是或的关系

你会发现如果脱离了空串，匹配 s:a, p:c*a* 死活匹配不出来，因为无法做s: \epsilon, p: c*的匹配
那么如果遇到f[i][j] = f[i][j-2]的情况，这个时候必定会有上述说的模式串匹配空串的情况
此外，如果字符串的最后一个字符被模式串匹配到，并且s[i] = p[j-1]
并不只有模式串末尾的字母+'*'匹配字符串s[i]的情况，可能本身这个星号匹配0次，是s[0..j]被p[0..j-2]匹配的情况。
所以0行和0列都要用于作为空串的匹配结果
*/

pub fn is_match(s: String, p: String) -> bool {
    // 声明一个二维数组
    //
    let mut dp:Vec<Vec<bool>> = Vec::new();
    for row in 0..s.len()+1 {
        dp.push(vec![false; p.len()+1]);
    }
    let s_bytes = s.as_bytes();
    let p_bytes = p.as_bytes();
    let matches = |i: usize, j: usize| -> bool {
        if i == 0 { return false; } // 空字符和所有字符都不匹配
        if p_bytes[j - 1] == '.' as u8 { return true; }
        let ret = s_bytes[i - 1] == p_bytes[j - 1];
        return ret;
    };

    dp[0][0] = true;

    for i in 0..s.len() + 1 {
        for j in 1..p.len() + 1 {
            /*
             这个实际上是这样的:
             1. dp[0][0]要根据s[0]和p[0]进行直接匹配
             2.
             */
            if p_bytes[j - 1] == ('*' as u8) {
                /* '*'匹配0次  即使matches(i, j - 1)为true
                 * 也有一种情况，目前的i不是p[j-1..j]匹配上的
                 * 而是之前的某个串已经匹配好的。比如说aa，ab*a*
                 * 字符串的末尾大于等于1次，但已经不是模式串的末尾的字母+*所匹配到。
                 */
                dp[i][j] |= dp[i][j - 2];
                // 2,5 -> 1,5 -> 0,5 -> false
                if matches(i, j - 1) {
                    dp[i][j] |= dp[i - 1][j]; // 最末尾的字母+*匹配字符串的末尾大于等于1次
                }
            } else {
                if matches(i,j) {
                    dp[i][j] |= dp[i - 1][j - 1];
                }
            }
        }
    }
    return dp[s.len()][p.len()];
}
pub fn main_rs() {
    let s1 = String::from("hhah");
    let p1 = String::from(".*");
    // println!("{} match {} : {}", s1.clone(), p1.clone(), is_match(s1.clone(), p1.clone()));
    let s2 = String::from("aa");
    let p2 = String::from("ab*a*");
    println!("{} match {} : {}", s2.clone(), p2.clone(), is_match(s2.clone(), p2.clone()));
}