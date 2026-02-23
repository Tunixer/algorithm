struct Solution;
impl Solution {
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let num_rows = grid.len();
        let num_cols = grid[0].len();
        for row in 0..num_rows {
            for col in 0..num_cols {
                let current_ans = Self::dfs(&mut grid, row as i32, col as i32);
                ans = std::cmp::max(current_ans, ans);
            }
        }
        return ans;
    }

    fn dfs(grid: &mut Vec<Vec<i32>>, row: i32, col: i32) -> i32 {
        let num_rows = grid.len() as i32;
        let num_cols = grid[0].len() as i32;

        if row < 0
            || row >= num_rows
            || col < 0
            || col >= num_cols
            || grid[row as usize][col as usize] == 0
        {
            return 0;
        }
        grid[row as usize][col as usize] = 0;
        let mut ans = 1;
        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        for (dir_row, dir_col) in directions {
            ans += Self::dfs(grid, row + dir_row, col + dir_col);
        }
        return ans;
    }
}
