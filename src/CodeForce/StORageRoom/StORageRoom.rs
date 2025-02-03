/*
 * Problem: B. StORage room
 * Contest: Codeforces - Codeforces Round 912 (Div. 2)
 * URL: https://codeforces.com/contest/1903/problem/B
 * Memory Limit: 256 MB
 * Time Limit: 2000 ms
 * Solution:
 * M_{i,j}= a_i|a_j. Since `|` operator has a specific meaning,
 * At first we set all a_i is 111111, then we are interested in finding those
 * bits should be flipped. If M_{i,j,k} is 0, then we know both a_{i,k} and a_{j,k}
 * must be 0. If M_{i,j,k} is 1, then either a_{i,k} or a_{j,k} should be 1.
 * On other words, a_{i,k} and a_{j,k} do not need to be flipped.
 * If we make such operation for each elemment in M, we could get an {a_i}.
 * We should check whether such {a_i} can be used to generate the target M.
 * In some cases, there would be some contradictive elements in target M, and
 * {a_i} cannot be figured out.
 */
struct Solution {
    pub M:Vec<Vec<u32>>,
    pub _size:u32
}

impl Solution{
    fn new(matrix_size: &i32) -> Self  {
        Self{
            M:Vec::<Vec<u32>>::new(),
            _size:*matrix_size as u32
        }
    }

    fn Input(&mut self) {
        for row in 0 .. self._size {
            let mut matrixRowStr = String::new();
            std::io::stdin().read_line(&mut matrixRowStr).expect("read_line error!");
            let matrixRowElemsStr:Vec<&str> = matrixRowStr.split(" ").collect(); 
            assert_eq!(matrixRowElemsStr.len(), self._size as usize);
            let mut matrixRowElems=Vec::new();
            for col in 0 .. self._size {
                matrixRowElems.push(matrixRowElemsStr[col as usize].trim().parse::<u32>().unwrap());
            }
            self.M.push(matrixRowElems);
        }
    }

    fn Solve(&self)-> Vec<u32> {
        let mut ret = Vec::with_capacity(self._size as usize);
        if self._size == 1 {
            return [0].to_vec();
        }
        for i in 0 .. self._size {
            ret.push(0x3FFFFFFF);
        }
        for row in 0 .. self._size as usize {
            for col in 0 .. self._size as usize {
                if row != col {
                    ret[row] &= self.M[row][col];
                    ret[col] &= self.M[row][col];
                    // println!("ret[{}]:{}, ret[{}]:{}", row, ret[row], col, ret[col]);
                }

            }
        }
        return ret;
    }

    fn Check(&self, ans: &Vec<u32>)-> bool {
        if self._size == 1 {
            return true;
        }
        for row in 0 .. self._size as usize {
            for col in 0 .. self._size as usize {
                if (row != col) && (self.M[row][col] != (ans[row]|ans[col])) {
                    return false;
                }
            }
        }
        true
    }
}

pub fn main_rs() {
    let mut caseNumStr = String::new();
    std::io::stdin().read_line(&mut caseNumStr).expect("read_line error!");
    let caseNum = caseNumStr.trim().parse::<i32>().unwrap();

    for i in 0..caseNum {
        let mut thisCaseMSize = String::new();
        std::io::stdin().read_line(&mut thisCaseMSize).expect("read_line error!");
        let thisMSize = thisCaseMSize.trim().parse::<i32>().unwrap();
        let mut currSolution = Solution::new(&thisMSize);
        currSolution.Input();
        let ans = currSolution.Solve();
        let isSolvable = currSolution.Check(&ans);
        if (isSolvable) {
            println!("YES");
            for j in 0 .. ans.len() {
                print!("{}", ans[j]);
                if j != ans.len()-1 {
                    print!(" ");
                } else {
                    println!("");
                }
            }
        } else {
            println!("NO");
        }
    }
}