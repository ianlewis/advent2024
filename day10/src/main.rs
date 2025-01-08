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

use std::collections;
use std::error;
use std::io::{self, BufRead};
use std::process;

fn read_map(r: impl BufRead) -> Result<Vec<Vec<Option<i64>>>, Box<dyn error::Error>> {
    let mut map = Vec::new();
    for line in r.lines() {
        let mut col = Vec::new();
        for c in line?.chars() {
            if c.is_numeric() {
                let n = String::from(c).parse::<i64>()?;
                col.push(Some(n));
            } else {
                col.push(None);
            }
        }

        map.push(col);
    }

    Ok(map)
}

fn find_trails(map: &[Vec<Option<i64>>], x: usize, y: usize) -> (usize, usize) {
    let mut distinct_trails = 0;
    let mut unique_trailends = collections::HashSet::new();

    let dirs: [(isize, isize); 4] = [
        (0, -1), // up
        (0, 1),  // down
        (-1, 0), // left
        (1, 0),  // right
    ];

    let mut stack: collections::VecDeque<(usize, usize, i64)> = collections::VecDeque::new();
    stack.push_back((x, y, 0));

    while !stack.is_empty() {
        let (cur_x, cur_y, expected_num) = stack.pop_back().unwrap();
        if map[cur_y][cur_x] != Some(expected_num) {
            continue;
        }
        if expected_num == 9 {
            distinct_trails += 1;
            unique_trailends.insert((cur_x, cur_y));
            continue;
        }

        for (dx, dy) in dirs {
            let next_x_o = cur_x.checked_add_signed(dx);
            let next_y_o = cur_y.checked_add_signed(dy);

            if next_y_o.is_none() {
                continue;
            }
            let next_y = next_y_o.unwrap();
            if next_y >= map.len() {
                continue;
            }

            if next_x_o.is_none() {
                continue;
            }
            let next_x = next_x_o.unwrap();
            if next_x >= map[next_y].len() {
                continue;
            }

            stack.push_back((next_x, next_y, expected_num + 1));
        }
    }

    (unique_trailends.len(), distinct_trails)
}

fn run(r: impl BufRead) -> Result<(usize, usize), Box<dyn error::Error>> {
    let map = read_map(r)?;

    let mut total_score = 0;
    let mut total_score2 = 0;

    for (y, col) in map.iter().enumerate() {
        for (x, _loc) in col.iter().enumerate() {
            let (score, score2) = find_trails(&map, x, y);
            total_score += score;
            total_score2 += score2;
        }
    }

    Ok((total_score, total_score2))
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
            "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
",
        );

        let (n, n2) = run(input.reader())?;
        assert_eq!(n, 36);
        assert_eq!(n2, 81);
        Ok(())
    }
}
