/**
 * https://codeforces.com/contest/800/problem/A
 * Suppose the stop time is t
 * Then, we have the following constraint
 * t * p = sum(a_i * t) - sum (b_i)
 * But how to determine the value of t, since there are some situation that cannot have any valid solution of t.
 * if t * p > sum(a_i * t) - sum (b_i), it shows that the charger provide more electricity the devices need.
 * It should decrease t.
 */
use std::io;
struct Solution {
    pub _numberDevices: i32,
    pub _chargePower:   i32,
    pub _powerUsage:    Vec<i32>,
    pub _powerStorage:  Vec<i32>,
    pub _size:          i32,
    pub _M:             Vec<Vec<u32>>
}

impl Solution {
    fn new(matrix_size: &i32) -> Self  {
        Self{
            _numberDevices: 0, _chargePower: 0,
            _powerUsage: Vec::new(),
            _powerStorage: Vec::new(),
            _size: 0,
            _M: vec![vec![0; *matrix_size as usize]; *matrix_size as usize]
        }
    }
    /*
     * 适用于一行有多个数字的情况
     */
    fn InputLineToMultiNums()
    {

    }
    fn Input(&mut self) {
        let mut input=String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut s = input.split_whitespace();
        let a: i32 = s.next().unwrap().parse().unwrap();
        let b: i32 = s.next().unwrap().parse().unwrap();

        for row in 0 .. self._size {
            let mut matrixRowStr = String::new();
            std::io::stdin().read_line(&mut matrixRowStr).expect("read_line error!");
            let matrixRowElemsStr:Vec<&str> = matrixRowStr.split(" ").collect();
            assert_eq!(matrixRowElemsStr.len(), self._size as usize);
            let mut matrixRowElems=Vec::new();
            for col in 0 .. self._size {
                matrixRowElems.push(matrixRowElemsStr[col as usize].trim().parse::<u32>().unwrap());
            }
            self._M.push(matrixRowElems);
        }
    }
}