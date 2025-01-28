struct Solution;
impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut ret = String::new();
        let pairs = vec![
           (1000, "M"), 
           (900, "CM"),
           (500, "D"),
           (400, "CD"),
           (100, "C"),
           (90, "XC"),
           (50, "L"),
           (40, "XL"),
           (10, "X"),
           (9, "IX"),
           (5, "V"),
           (4, "IV"),
           (1, "I")
        ];
        let mut n = num;
        while n > 0 {
            for (val, roman) in pairs.iter() {
                if n >= *val {
                    ret.push_str(roman);
                    n -= val;
                    break;
                }
            }
        }
        return ret;
    }
}

pub fn main_rs() {
    println!("{}", Solution::int_to_roman(3749));
    println!("{}", Solution::int_to_roman(58));
}