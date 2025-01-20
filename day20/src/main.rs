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

type Map = (usize, usize, usize, usize, Vec<Vec<char>>);

fn read_map(r: impl BufRead) -> Result<Map, Box<dyn error::Error>> {
    let mut map = Vec::new();
    let mut start_x = 0;
    let mut start_y = 0;
    let mut end_x = 0;
    let mut end_y = 0;
    for (y, line) in r.lines().enumerate() {
        let mut col = Vec::new();
        for (x, c) in line?.chars().enumerate() {
            if c == 'S' {
                start_x = x;
                start_y = y;
            }
            if c == 'E' {
                end_x = x;
                end_y = y;
            }
            col.push(c);
        }

        map.push(col);
    }

    Ok((start_x, start_y, end_x, end_y, map))
}

const UP: (isize, isize) = (0, -1);
const DOWN: (isize, isize) = (0, 1);
const LEFT: (isize, isize) = (-1, 0);
const RIGHT: (isize, isize) = (1, 0);
const DIRS: [(isize, isize); 4] = [DOWN, RIGHT, UP, LEFT];

fn maze_path((start_x, start_y, end_x, end_y, map): &Map) -> Vec<(usize, usize)> {
    let mut stack = collections::VecDeque::new();
    let mut visited = collections::HashSet::new();

    stack.push_back((*start_x, *start_y, Vec::new()));

    while !stack.is_empty() {
        let (x, y, mut path) = stack.pop_back().unwrap();

        if x == *end_x && y == *end_y {
            path.push((x, y));
            return path;
        }

        if map[y][x] == '#' {
            continue;
        }

        if visited.contains(&(x, y)) {
            continue;
        }

        path.push((x, y));

        for (dx, dy) in DIRS {
            let new_x = ((x as isize) + dx) as usize;
            let new_y = ((y as isize) + dy) as usize;
            stack.push_back((new_x, new_y, path.clone()));
        }

        visited.insert((x, y));
    }

    Vec::new()
}

fn find_cheats(path: Vec<(usize, usize)>, max_cheat_length: usize, min_saved: usize) -> usize {
    let mut cheats = collections::HashSet::new();

    // For each tile in the path to the exit, find another tile later in the path that is within
    // the max_cheat_length number of tiles and at least max_saved tiles further down the path.

    for (start_pos, (start_x, start_y)) in path.iter().enumerate() {
        for (end_pos, (end_x, end_y)) in path.iter().enumerate().skip(start_pos + min_saved) {
            let cheat_length = ((*end_x as isize) - (*start_x as isize)).abs()
                + ((*end_y as isize) - (*start_y as isize)).abs();
            let picos_saved = (end_pos as isize) - (start_pos as isize) - cheat_length;
            if cheat_length <= (max_cheat_length as isize) && picos_saved >= (min_saved as isize) {
                cheats.insert(((start_x, start_y), (end_x, end_y)));
            }
        }
    }

    cheats.len()
}

fn run(
    r: impl BufRead,
    max_cheat_length: usize,
    min_save: usize,
    max_cheat_length2: usize,
    min_save2: usize,
) -> Result<(usize, usize), Box<dyn error::Error>> {
    let (start_x, start_y, end_x, end_y, map) = read_map(r)?;

    let path = maze_path(&(start_x, start_y, end_x, end_y, map.clone()));

    Ok((
        find_cheats(path.clone(), max_cheat_length, min_save),
        find_cheats(path.clone(), max_cheat_length2, min_save2),
    ))
}

fn main() -> process::ExitCode {
    let stdin = io::stdin();
    let (n, n2) = match run(stdin.lock(), 2, 100, 20, 100) {
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
            "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
",
        );

        let (n, n2) = run(input.reader(), 2, 1, 20, 50)?;
        assert_eq!(n, 44);
        assert_eq!(n2, 285);
        Ok(())
    }
}
