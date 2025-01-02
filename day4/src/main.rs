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

// Program day4 prints the number of times XMAS appears in the input grid.

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

// xmas returns if the current position spells XMAS in the given direction.
fn xmas(grid: &Vec<Vec<char>>, x: usize, y: usize, dx: isize, dy: isize) -> bool {
    let dx1 = x.checked_add_signed(dx);
    let dy1 = y.checked_add_signed(dy);
    let dx2 = x.checked_add_signed(dx * 2);
    let dy2 = y.checked_add_signed(dy * 2);
    let dx3 = x.checked_add_signed(dx * 3);
    let dy3 = y.checked_add_signed(dy * 3);

    if dx3.is_none() || dx3.unwrap() >= grid[y].len() {
        return false;
    }
    if dy3.is_none() || dy3.unwrap() >= grid.len() {
        return false;
    }

    grid[y][x] == 'X'
        && grid[dy1.unwrap()][dx1.unwrap()] == 'M'
        && grid[dy2.unwrap()][dx2.unwrap()] == 'A'
        && grid[dy3.unwrap()][dx3.unwrap()] == 'S'
}

fn find_xmas(grid: &Vec<Vec<char>>) -> i64 {
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
                if xmas(&grid, x, y, dx, dy) {
                    total += 1;
                }
            }
        }
    }

    total
}

fn run(r: impl BufRead) -> Result<i64, Box<dyn error::Error>> {
    // Read the full grid.
    let grid = read_grid(r)?;
    Ok(find_xmas(&grid))
}

fn main() -> process::ExitCode {
    let stdin = io::stdin();
    let n = match run(stdin.lock()) {
        Ok(n) => n,
        Err(e) => {
            println!("error running: {e:?}");
            return process::ExitCode::from(1);
        }
    };

    println!("{}", n);

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

        let n = run(input.reader())?;
        assert_eq!(n, 18);
        Ok(())
    }
}
