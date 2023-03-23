fn dfs(grid: &mut Vec<Vec<char>>, r: i32, c: i32) {
    let nr = grid.len() as i32;
    let nc = grid[0].len() as i32;

    if r < 0 || c < 0 || r >= nr || c >= nc || grid[r as usize][c as usize] == '0' {
        return;
    }

    grid[r as usize][c as usize] = '0';
    dfs(grid, r - 1, c);
    dfs(grid, r + 1, c);
    dfs(grid, r, c - 1);
    dfs(grid, r, c + 1);
}

pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    if grid.is_empty() {
        return 0;
    }

    let nr = grid.len();
    let nc = grid[0].len();
    let mut num_islands = 0;
    let mut grid = grid;

    for r in 0..nr {
        for c in 0..nc {
            if grid[r][c] == '1' {
                num_islands += 1;
                dfs(&mut grid, r as i32, c as i32);
            }
        }
    }

    num_islands
}

fn main() {
    let grid = vec![
        vec!['1', '1', '1', '1', '0'],
        vec!['1', '1', '0', '1', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '0', '0', '0'],
    ];
    println!("{}", num_islands(grid));
}