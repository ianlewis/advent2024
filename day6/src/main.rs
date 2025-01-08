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

// Program day6 prints the number of positions visited by the guard in their route and the number
// of places an obstruction could be added to cause the gaurd to go into a loop.

use std::collections;
use std::error;
use std::io::{self, BufRead};
use std::process;

struct Map {
    map: Vec<Vec<char>>,
    guard_x: usize,
    guard_y: usize,

    // visited_pos is a HashMap that maps visited positions to the guard's directional character
    // <,^,>, or v.
    visited_pos: collections::HashMap<(usize, usize), Vec<char>>,
}

impl Map {
    pub fn new(map: Vec<Vec<char>>) -> Self {
        let mut guard_x = 0;
        let mut guard_y = 0;
        let mut guard_set = Vec::new();
        for (y, col) in map.iter().enumerate() {
            for (x, c) in col.iter().enumerate() {
                if *c == '^' || *c == '>' || *c == 'v' || *c == '<' {
                    guard_x = x;
                    guard_y = y;
                    guard_set.push(*c);
                }
            }
        }

        let mut visited_pos = collections::HashMap::new();
        visited_pos.insert((guard_x, guard_y), guard_set);

        Map {
            map,
            guard_x,
            guard_y,

            visited_pos,
        }
    }

    pub fn clone(&mut self) -> Self {
        Map {
            map: self.map.clone(),
            guard_x: self.guard_x,
            guard_y: self.guard_y,
            visited_pos: self.visited_pos.clone(),
        }
    }

    pub fn next_pos(&mut self) -> Option<(usize, usize)> {
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

        Some((new_guard_x, new_guard_y))
    }

    pub fn advance(&mut self) -> Result<Option<(usize, usize)>, String> {
        let next_pos = self.next_pos();
        if next_pos.is_none() {
            return Ok(None);
        }

        let (new_guard_x, new_guard_y) = next_pos.unwrap();

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

            // First check if we have entered a loop.
            if let Some(p) = self.visited_pos.get(&(new_guard_x, new_guard_y)) {
                if p.contains(&self.map[self.guard_y][self.guard_x]) {
                    return Err("loop".to_string());
                };
            }

            self.map[new_guard_y][new_guard_x] = self.map[self.guard_y][self.guard_x];
            self.map[self.guard_y][self.guard_x] = '.'; // Mark the postions we have been to.

            self.guard_x = new_guard_x;
            self.guard_y = new_guard_y;

            // Insert our visited position.
            self.visited_pos
                .entry((self.guard_x, self.guard_y))
                .and_modify(|p| p.push(self.map[self.guard_y][self.guard_x]))
                .or_insert(vec![self.map[self.guard_y][self.guard_x]]);
        }

        Ok(Some((self.guard_y, self.guard_x)))
    }
}

fn read_map(r: impl BufRead) -> Result<Vec<Vec<char>>, Box<dyn error::Error>> {
    let mut map_vec: Vec<Vec<char>> = Vec::new();
    for line in r.lines() {
        let mut col: Vec<char> = Vec::new();
        for c in line?.chars() {
            col.push(c);
        }
        map_vec.push(col);
    }

    Ok(map_vec)
}

fn run(r: impl BufRead) -> Result<(usize, usize), Box<dyn error::Error>> {
    let mut map = Map::new(read_map(r)?);

    let mut obstruction_positions = collections::HashSet::new();

    // Advance the guard until they leave the map.
    loop {
        match map.advance() {
            Ok(Some(_p)) => {}
            Ok(None) => break,
            Err(_e) => panic!("we should not have found a loop"),
        }

        // Insert an obstruction in front of the guard and see if it goes into a loop.
        let mut new_map = map.clone();
        let (o_x, o_y) = match new_map.next_pos() {
            Some((x, y)) => {
                // Don't count if there is already an obstruction there, or if we have visited this location already.
                if new_map.map[y][x] == '#' || new_map.visited_pos.contains_key(&(x, y)) {
                    continue;
                }
                new_map.map[y][x] = '#';
                (x, y)
            }
            None => continue,
        };

        if !obstruction_positions.contains(&(o_x, o_y)) {
            loop {
                match new_map.advance() {
                    Ok(Some(_p)) => {}
                    Ok(None) => break,
                    Err(_e) => {
                        obstruction_positions.insert((o_x, o_y));
                        break;
                    }
                }
            }
        }
    }

    Ok((map.visited_pos.len(), obstruction_positions.len()))
}

fn main() -> process::ExitCode {
    let stdin = io::stdin();
    let (n, n2) = match run(stdin.lock()) {
        Ok((n, n2)) => (n, n2),
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

        let (n, n2) = run(input.reader())?;
        assert_eq!(n, 41);
        assert_eq!(n2, 6);
        Ok(())
    }
}
