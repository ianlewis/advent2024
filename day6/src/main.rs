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

// Program day6 prints the number of positions visited by the guard in their route.

use std::error;
use std::io::{self, BufRead};
use std::process;

struct Map {
    map: Vec<Vec<char>>,
    guard_x: usize,
    guard_y: usize,

    positions_num: i64,
}

impl Map {
    pub fn new(map: Vec<Vec<char>>) -> Self {
        let mut guard_x = 0;
        let mut guard_y = 0;
        for (y, col) in map.iter().enumerate() {
            for (x, c) in col.iter().enumerate() {
                if *c == '^' || *c == '>' || *c == 'v' || *c == '<' {
                    guard_x = x;
                    guard_y = y;
                }
            }
        }

        Map {
            map,
            guard_x,
            guard_y,
            positions_num: 1,
        }
    }

    pub fn next(&mut self) -> Option<(usize, usize)> {
        let (guard_dx, guard_dy) = match self.map[self.guard_y][self.guard_x] {
            '^' => (0, -1),
            '<' => (-1, 0),
            '>' => (1, 0),
            'v' => (0, 1),
            _ => panic!("map data not valid"),
        };

        let new_guard_x_r = self.guard_x.checked_add_signed(guard_dx);
        let new_guard_y_r = self.guard_y.checked_add_signed(guard_dy);

        if new_guard_x_r.is_none() || new_guard_y_r.is_none() {
            return None;
        }
        let new_guard_x = new_guard_x_r.unwrap();
        let new_guard_y = new_guard_y_r.unwrap();
        if new_guard_y >= self.map.len() || new_guard_x >= self.map[new_guard_y].len() {
            return None;
        }

        // Check for obstructions
        if self.map[new_guard_y][new_guard_x] == '#' {
            // - If there is something directly in front of you, turn right 90 degrees.
            self.map[self.guard_y][self.guard_x] = match self.map[self.guard_y][self.guard_x] {
                '^' => '>',
                '<' => '^',
                '>' => 'v',
                'v' => '<',
                _ => panic!("map data not valid"),
            };
        } else {
            // - Otherwise, take a step forward.

            // Check if we have been to this position.
            if self.map[new_guard_y][new_guard_x] != 'X' {
                self.positions_num += 1;
            }

            self.map[new_guard_y][new_guard_x] = self.map[self.guard_y][self.guard_x];
            self.map[self.guard_y][self.guard_x] = 'X'; // Mark the postions we have been to.

            self.guard_x = new_guard_x;
            self.guard_y = new_guard_y;
        }

        Some((self.guard_y, self.guard_x))
    }
}

fn read_map(r: impl BufRead) -> Result<Map, Box<dyn error::Error>> {
    let mut map_vec: Vec<Vec<char>> = Vec::new();
    for line in r.lines() {
        let mut col: Vec<char> = Vec::new();
        for c in line?.chars() {
            col.push(c);
        }
        map_vec.push(col);
    }

    Ok(Map::new(map_vec))
}

fn run(r: impl BufRead) -> Result<i64, Box<dyn error::Error>> {
    let mut map = read_map(r)?;

    // Advance the guard until they leave the map.
    while let Some((_x, _y)) = map.next() {}

    Ok(map.positions_num)
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
            "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
",
        );

        let n = run(input.reader())?;
        assert_eq!(n, 41);
        Ok(())
    }
}
