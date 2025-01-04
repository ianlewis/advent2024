// Copyright 2025 Ian Lewis
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// Program day4 prints the number of times XMAS appears in the input grid and
// the number of times an X-MAS shapes are found in the grid.

use std::error;
use std::io::{self, BufRead};
use std::process;

fn read_grid(mut r: impl BufRead) -> Result<Vec<Vec<char>>, Box<dyn error::Error>> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut x = 0;
    let mut y = 0;
    loop {
        let mut buf = [0; 1];
        match r.read_exact(&mut buf) {
            Ok(_) => {
                let c = buf[0] as char;
                if c == '\n' {
                    y += 1;
                    x = 0;
                } else {
                    if x == 0 {
                        grid.push(Vec::new());
                    }
                    grid[y].push(c);
                    x += 1;
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                break; // Handle EOF
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }

    Ok(grid)
}

fn find_xmas(grid: &[Vec<char>]) -> i64 {
    let directions: [(isize, isize); 8] = [
        (0, -1),  // up
        (0, 1),   // down
        (-1, 0),  // left
        (1, 0),   // right
        (-1, -1), // diagonal up left
        (1, -1),  // diagonal up right
        (-1, 1),  // diagonal down left
        (1, 1),   // diagonal down right
    ];

    let mut total = 0;
    for (y, col) in grid.iter().enumerate() {
        for (x, _) in col.iter().enumerate() {
            for (dx, dy) in directions {
                let dx1 = x.checked_add_signed(dx);
                let dy1 = y.checked_add_signed(dy);
                let dx2 = x.checked_add_signed(dx * 2);
                let dy2 = y.checked_add_signed(dy * 2);
                let dx3 = x.checked_add_signed(dx * 3);
                let dy3 = y.checked_add_signed(dy * 3);

                if dx3.is_none() || dx3.unwrap() >= grid[y].len() {
                    continue;
                }
                if dy3.is_none() || dy3.unwrap() >= grid.len() {
                    continue;
                }

                if grid[y][x] == 'X'
                    && grid[dy1.unwrap()][dx1.unwrap()] == 'M'
                    && grid[dy2.unwrap()][dx2.unwrap()] == 'A'
                    && grid[dy3.unwrap()][dx3.unwrap()] == 'S'
                {
                    total += 1;
                }
            }
        }
    }

    total
}

fn find_x_mas(grid: &[Vec<char>]) -> i64 {
    // TODO: use a map?
    // TODO: don't bother with tracking, just divide the total by 2?
    let mut seen: Vec<(usize, usize, usize, usize)> = Vec::new();

    let directions: [(isize, isize); 4] = [
        (-1, -1), // diagonal up left
        (1, -1),  // diagonal up right
        (-1, 1),  // diagonal down left
        (1, 1),   // diagonal down right
    ];

    let mut total = 0;
    for (y, col) in grid.iter().enumerate() {
        for (x, _) in col.iter().enumerate() {
            // We must track where Ms in the X-MAS are located to avoid
            // double counting.

            'outer: for (dx, dy) in directions {
                let dx1 = x.checked_add_signed(dx);
                let dy1 = y.checked_add_signed(dy);
                let dx2 = x.checked_add_signed(dx * 2);
                let dy2 = y.checked_add_signed(dy * 2);

                if dx2.is_none() || dx2.unwrap() >= grid[y].len() {
                    continue;
                }
                if dy2.is_none() || dy2.unwrap() >= grid.len() {
                    continue;
                }

                let dx1_u = dx1.unwrap();
                let dy1_u = dy1.unwrap();
                let dx2_u = dx2.unwrap();
                let dy2_u = dy2.unwrap();

                // The center must be an 'A'
                if grid[y][x] == 'M' && grid[dy1_u][dx1_u] == 'A' && grid[dy2_u][dx2_u] == 'S' {
                    // Find the MAS in the other direction.
                    if grid[y][dx2_u] == 'M' && grid[dy2_u][x] == 'S' {
                        for (sx, sy, sx2, sy2) in seen.iter() {
                            if x == *sx && y == *sy && dx2_u == *sx2 && y == *sy2 {
                                continue 'outer;
                            }
                        }

                        total += 1;
                        seen.push((dx2_u, y, x, y));
                    } else if grid[dy2_u][x] == 'M' && grid[y][dx2_u] == 'S' {
                        for (sx, sy, sx2, sy2) in seen.iter() {
                            if x == *sx && y == *sy && x == *sx2 && dy2_u == *sy2 {
                                continue 'outer;
                            }
                        }

                        seen.push((x, dy2_u, x, y));
                        total += 1;
                    }
                }
            }
        }
    }

    total
}

fn run(r: impl BufRead) -> Result<(i64, i64), Box<dyn error::Error>> {
    // Read the full grid.
    let grid = read_grid(r)?;
    Ok((find_xmas(&grid), find_x_mas(&grid)))
}

fn main() -> process::ExitCode {
    let stdin = io::stdin();
    let (n, n2) = match run(stdin.lock()) {
        Ok(n) => n,
        Err(e) => {
            println!("error running: {e:?}");
            return process::ExitCode::from(1);
        }
    };

    println!("{}", n);
    println!("{}", n2);

    process::ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::{Buf, Bytes};

    #[test]
    fn test_run() -> Result<(), Box<dyn error::Error>> {
        let input = Bytes::from(
            "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
",
        );

        let (n, n2) = run(input.reader())?;
        assert_eq!(n, 18);
        assert_eq!(n2, 9);
        Ok(())
    }
}
