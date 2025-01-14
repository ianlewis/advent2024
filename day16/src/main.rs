// Copyright 2025 Ian Lewi
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

type Map = (usize, usize, Vec<Vec<char>>);

fn read_map(r: impl BufRead) -> Result<Map, Box<dyn error::Error>> {
    let mut map = Vec::new();
    let mut start_x = 0;
    let mut start_y = 0;
    for (y, line) in r.lines().enumerate() {
        let mut col = Vec::new();
        for (x, c) in line?.chars().enumerate() {
            if c == 'S' {
                start_x = x;
                start_y = y;
            }
            col.push(c);
        }

        map.push(col);
    }

    Ok((start_x, start_y, map))
}

#[derive(Clone, Hash, Eq, PartialEq)]
enum Action {
    Forward,
    Left,
    Right,
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
enum Dir {
    North,
    South,
    East,
    West,
}

impl Dir {
    fn clockwise(&self) -> Dir {
        match *self {
            Dir::North => Dir::East,
            Dir::South => Dir::West,
            Dir::West => Dir::North,
            Dir::East => Dir::South,
        }
    }

    fn counter_clockwise(&self) -> Dir {
        match *self {
            Dir::North => Dir::West,
            Dir::South => Dir::East,
            Dir::West => Dir::South,
            Dir::East => Dir::North,
        }
    }

    fn value(&self) -> (isize, isize) {
        match *self {
            Dir::North => (0, -1),
            Dir::South => (0, 1),
            Dir::West => (-1, 0),
            Dir::East => (1, 0),
        }
    }
}

#[derive(Clone)]
struct Visit {
    x: usize,
    y: usize,
    dir: Dir,
    score: usize,
}

fn run(r: impl BufRead) -> Result<(usize, usize), Box<dyn error::Error>> {
    let (start_x, start_y, map) = read_map(r)?;
    let mut actions = collections::VecDeque::new();
    let mut paths = Vec::new();

    let start = vec![Visit {
        x: start_x,
        y: start_y,
        dir: Dir::East,
        score: 0,
    }];
    actions.push_back((start.clone(), Action::Forward));
    actions.push_back((start.clone(), Action::Left));
    actions.push_back((start.clone(), Action::Right));

    // visited is a global visited cache that is used as an optimization to
    // avoid going down low score paths.
    let mut visited = collections::HashMap::new();
    while !actions.is_empty() {
        let (mut path, action) = actions.pop_back().unwrap();

        let last_visit = path.last().unwrap();
        let cur_visit;
        let next_actions;
        match action {
            Action::Forward => {
                let (dx, dy) = last_visit.dir.value();
                cur_visit = Visit {
                    x: ((last_visit.x as isize) + dx) as usize,
                    y: ((last_visit.y as isize) + dy) as usize,
                    dir: last_visit.dir,
                    score: last_visit.score + 1,
                };
                next_actions = vec![Action::Forward, Action::Left, Action::Right];
            }
            Action::Left => {
                cur_visit = Visit {
                    x: last_visit.x,
                    y: last_visit.y,
                    dir: last_visit.dir.counter_clockwise(),
                    score: last_visit.score + 1000,
                };
                next_actions = vec![Action::Forward];
            }
            Action::Right => {
                cur_visit = Visit {
                    x: last_visit.x,
                    y: last_visit.y,
                    dir: last_visit.dir.clockwise(),
                    score: last_visit.score + 1000,
                };
                next_actions = vec![Action::Forward];
            }
        }

        if map[cur_visit.y][cur_visit.x] == '#' {
            // We are in a wall. Oops!
            continue;
        }

        if let Some(s) = visited.get(&(cur_visit.x, cur_visit.y, cur_visit.dir)) {
            if *s < cur_visit.score {
                continue;
            }
        }

        let cur_xy = (cur_visit.x, cur_visit.y);
        path.push(cur_visit.clone());
        visited.insert((cur_visit.x, cur_visit.y, cur_visit.dir), cur_visit.score);
        if map[cur_xy.1][cur_xy.0] == 'E' {
            // We found the end!
            paths.push(path);
            continue;
        }

        for next_action in &next_actions {
            // Optimization: process forward actions first by pushing them to the back of the
            // stack. Forward actions tend to result in a lower score so we don't have to
            // backtrack over visited locations as much.
            if action == Action::Forward {
                actions.push_back((path.clone(), next_action.clone()));
            } else {
                actions.push_front((path.clone(), next_action.clone()));
            }
        }
    }

    let score_min = paths
        .iter()
        .map(|path| path.last().unwrap().score)
        .min()
        .unwrap_or(0);

    let mut unique_tiles = collections::HashSet::new();
    for path in paths
        .iter()
        .filter(|p| p.last().unwrap().score == score_min)
    {
        for visit in path {
            unique_tiles.insert((visit.x, visit.y));
        }
    }

    Ok((score_min, unique_tiles.len()))
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
            "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############",
        );

        let (n, n2) = run(input.reader())?;
        assert_eq!(n, 7036);
        assert_eq!(n2, 45);
        Ok(())
    }

    #[test]
    fn test_run2() -> Result<(), Box<dyn error::Error>> {
        let input = Bytes::from(
            "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################",
        );

        let (n, n2) = run(input.reader())?;
        assert_eq!(n, 11048);
        assert_eq!(n2, 64);
        Ok(())
    }
}
