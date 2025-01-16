// Copyright 2025 Ian Lewis
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

fn read_input(r: impl BufRead) -> Result<Vec<(usize, usize)>, Box<dyn error::Error>> {
    let mut memory = Vec::new();
    for line_r in r.lines() {
        let line = line_r?;
        let parts: Vec<_> = line.split(",").collect();
        let x = parts[0].parse::<usize>()?;
        let y = parts[1].parse::<usize>()?;
        memory.push((x, y));
    }

    Ok(memory)
}

const UP: (isize, isize) = (0, -1);
const DOWN: (isize, isize) = (0, 1);
const LEFT: (isize, isize) = (-1, 0);
const RIGHT: (isize, isize) = (1, 0);
const DIRS: [(isize, isize); 4] = [DOWN, RIGHT, UP, LEFT];

fn run(
    r: impl BufRead,
    w: usize,
    h: usize,
    t: usize,
) -> Result<(usize, String), Box<dyn error::Error>> {
    let memory = read_input(r)?;

    let block = first_block(&memory, w, h)?;
    Ok((
        min_path(&memory, w, h, t),
        format!("{},{}", block.0, block.1),
    ))
}

fn first_block(
    memory: &[(usize, usize)],
    w: usize,
    h: usize,
) -> Result<(usize, usize), Box<dyn error::Error>> {
    // Go in reverse until we find a path exists. This is much faster than
    // starting from the beginning with very sparse memory.
    for (t, _c) in memory.iter().enumerate().rev() {
        // Only check if the path exists. Don't bother checking all paths.
        if path_exists(memory, w, h, t + 1) {
            return Ok(memory[t + 1]);
        }
    }

    Ok((0, 0))
}

fn path_exists(memory: &[(usize, usize)], w: usize, h: usize, t: usize) -> bool {
    let mut stack = collections::VecDeque::new();

    stack.push_back((0, 0));

    let mut visited = collections::HashSet::new();
    while !stack.is_empty() {
        let (x, y) = stack.pop_front().unwrap();

        if x == w - 1 && y == h - 1 {
            // We found the exit.
            return true;
        }

        if visited.contains(&(x, y)) {
            continue;
        }

        visited.insert((x, y));

        for (dx, dy) in DIRS {
            let new_x_r = x.checked_add_signed(dx);
            if new_x_r.is_none() {
                continue;
            }
            let new_x = new_x_r.unwrap();
            if new_x >= w {
                continue;
            }

            let new_y_r = y.checked_add_signed(dy);
            if new_y_r.is_none() {
                continue;
            }
            let new_y = new_y_r.unwrap();
            if new_y >= w {
                continue;
            }

            // println!("{},{}", x, y);
            if memory[..t].contains(&(new_x, new_y)) {
                continue;
            }

            stack.push_back((new_x, new_y));
        }
    }

    false
}

fn min_path(memory: &[(usize, usize)], w: usize, h: usize, t: usize) -> usize {
    let mut paths = Vec::new();
    let mut stack = collections::VecDeque::new();

    stack.push_back((0, 0, 0));

    let mut visited = collections::HashSet::new();
    while !stack.is_empty() {
        let (x, y, mut path_len) = stack.pop_front().unwrap();

        if x == w - 1 && y == h - 1 {
            // We found the exit.
            paths.push(path_len);
            continue;
        }

        path_len += 1;
        if visited.contains(&(x, y)) {
            continue;
        }

        visited.insert((x, y));

        for (dx, dy) in DIRS {
            let new_x_r = x.checked_add_signed(dx);
            if new_x_r.is_none() {
                continue;
            }
            let new_x = new_x_r.unwrap();
            if new_x >= w {
                continue;
            }

            let new_y_r = y.checked_add_signed(dy);
            if new_y_r.is_none() {
                continue;
            }
            let new_y = new_y_r.unwrap();
            if new_y >= w {
                continue;
            }

            // println!("{},{}", x, y);
            if memory[..t].contains(&(new_x, new_y)) {
                continue;
            }

            stack.push_back((new_x, new_y, path_len));
        }
    }

    *paths.iter().min().unwrap_or(&0)
}

fn main() -> process::ExitCode {
    let stdin = io::stdin();
    let (n, n2) = match run(stdin.lock(), 71, 71, 1024) {
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
            "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
",
        );

        let (n, n2) = run(input.reader(), 7, 7, 12)?;
        assert_eq!(n, 22);
        assert_eq!(n2, "6,1");
        Ok(())
    }
}
