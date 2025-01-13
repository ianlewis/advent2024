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

struct Robot {
    map: Vec<Vec<char>>,
    moves: collections::VecDeque<char>,
    x: usize,
    y: usize,
}

impl Robot {
    pub fn advance(&mut self) -> bool {
        let dirs: collections::HashMap<char, (isize, isize)> = collections::HashMap::from([
            ('^', (0, -1)),
            ('v', (0, 1)),
            ('<', (-1, 0)),
            ('>', (1, 0)),
        ]);

        let move_o = self.moves.pop_front();
        if move_o.is_none() {
            return false;
        }

        let mv = &move_o.unwrap();
        let (dx, dy) = dirs[mv];

        let mut i = 1;
        while self.map[((self.y as isize) + dy * i) as usize][((self.x as isize) + dx * i) as usize]
            == 'O'
        {
            i += 1;
        }

        if self.map[((self.y as isize) + dy * i) as usize][((self.x as isize) + dx * i) as usize]
            == '.'
        {
            while i >= 0 {
                self.map[((self.y as isize) + dy * i) as usize]
                    [((self.x as isize) + dx * i) as usize] = self.map
                    [((self.y as isize) + dy * (i - 1)) as usize]
                    [((self.x as isize) + dx * (i - 1)) as usize];
                i -= 1;
            }
            self.map[self.y][self.x] = '.';
            self.x = ((self.x as isize) + dx) as usize;
            self.y = ((self.y as isize) + dy) as usize;
        }

        true
    }
}

struct Robot2 {
    map: Vec<Vec<char>>,
    moves: collections::VecDeque<char>,
    x: usize,
    y: usize,
}

impl Robot2 {
    pub fn advance(&mut self) -> bool {
        let vert_dirs: collections::HashMap<char, (isize, isize)> =
            collections::HashMap::from([('^', (0, -1)), ('v', (0, 1))]);

        let horiz_dirs: collections::HashMap<char, (isize, isize)> =
            collections::HashMap::from([('<', (-1, 0)), ('>', (1, 0))]);

        let move_o = self.moves.pop_front();
        if move_o.is_none() {
            return false;
        }

        let mv = &move_o.unwrap();

        if horiz_dirs.contains_key(mv) {
            self._advance_horiz(horiz_dirs[mv]);
            return true;
        }

        self._advance_vert(vert_dirs[mv]);
        true
    }

    pub fn _advance_vert(&mut self, dir: (isize, isize)) {
        let (dx, dy) = dir;

        // Create a vector for each row we will push and keep a set of indexes
        // that will get pushed.
        let mut h = self.y;

        let mut push_stack: collections::VecDeque<(usize, collections::HashSet<usize>)> =
            collections::VecDeque::new();
        push_stack.push_back((h, collections::HashSet::from([self.x])));

        loop {
            // Temporarily get the previous row's push set.
            let (prev_h, prev_push_set) = push_stack.pop_back().unwrap();

            h = ((h as isize) + dy) as usize;
            let mut push_set = collections::HashSet::new();

            for i in &prev_push_set {
                if self.map[h][*i] == '#' {
                    // We are blocked.
                    return;
                }
                if self.map[h][*i] == ']' {
                    push_set.insert(*i);
                    push_set.insert(i - 1);
                }
                if self.map[h][*i] == '[' {
                    push_set.insert(*i);
                    push_set.insert(i + 1);
                }
            }

            // Put back the previous push set.
            push_stack.push_back((prev_h, prev_push_set));

            // Add the current push set or stop.
            if push_set.is_empty() {
                break;
            }
            push_stack.push_back((h, push_set));
        }

        // push up each row.
        while !push_stack.is_empty() {
            let (y, push_set) = push_stack.pop_back().unwrap();

            for x in push_set {
                self.map[((y as isize) + dy) as usize][x] = self.map[y][x];
                self.map[y][x] = '.';
            }
        }

        self.x = ((self.x as isize) + dx) as usize;
        self.y = ((self.y as isize) + dy) as usize;
    }

    pub fn _advance_horiz(&mut self, dir: (isize, isize)) {
        let (dx, dy) = dir;

        let mut i = 1;
        loop {
            let next_loc = self.map[((self.y as isize) + dy * i) as usize]
                [((self.x as isize) + dx * i) as usize];
            if next_loc == '[' || next_loc == ']' {
                i += 1;
            } else {
                break;
            }
        }

        if self.map[((self.y as isize) + dy * i) as usize][((self.x as isize) + dx * i) as usize]
            == '.'
        {
            while i >= 0 {
                self.map[((self.y as isize) + dy * i) as usize]
                    [((self.x as isize) + dx * i) as usize] = self.map
                    [((self.y as isize) + dy * (i - 1)) as usize]
                    [((self.x as isize) + dx * (i - 1)) as usize];
                i -= 1;
            }
            self.map[self.y][self.x] = '.';
            self.x = ((self.x as isize) + dx) as usize;
            self.y = ((self.y as isize) + dy) as usize;
        }
    }
}

fn read_input(r: impl BufRead) -> Result<(Robot, Robot2), Box<dyn error::Error>> {
    let mut map_done = false;
    let mut robot = Robot {
        map: Vec::new(),
        moves: collections::VecDeque::new(),
        x: 0,
        y: 0,
    };
    let mut robot2 = Robot2 {
        map: Vec::new(),
        moves: collections::VecDeque::new(),
        x: 0,
        y: 0,
    };

    for (y, line_r) in r.lines().enumerate() {
        let line = line_r?;
        if line.is_empty() {
            map_done = true;
            continue;
        }

        if !map_done {
            let mut col = Vec::new();
            let mut col2 = Vec::new();
            for (x, c) in line.chars().enumerate() {
                if c == '@' {
                    robot.x = x;
                    robot.y = y;
                }
                col.push(c);

                match c {
                    '@' => {
                        col2.push('@');
                        col2.push('.');
                        robot2.x = x * 2;
                        robot2.y = y;
                    }
                    '#' => {
                        col2.push('#');
                        col2.push('#');
                    }

                    'O' => {
                        col2.push('[');
                        col2.push(']');
                    }
                    _ => {
                        col2.push('.');
                        col2.push('.');
                    }
                }
            }

            robot.map.push(col);
            robot2.map.push(col2);
        } else {
            for c in line.chars() {
                if c == '^' || c == 'v' || c == '<' || c == '>' {
                    robot.moves.push_back(c);
                    robot2.moves.push_back(c);
                }
            }
        }
    }

    Ok((robot, robot2))
}

fn gps_sum(map: &[Vec<char>]) -> usize {
    let mut total = 0;
    for (y, col) in map.iter().enumerate() {
        for (x, c) in col.iter().enumerate() {
            if *c == 'O' || *c == '[' {
                total += 100 * y + x;
            }
        }
    }

    total
}

fn run(r: impl BufRead) -> Result<(usize, usize), Box<dyn error::Error>> {
    let (mut robot, mut robot2) = read_input(r)?;

    while robot.advance() {}

    while robot2.advance() {}

    Ok((gps_sum(&robot.map), gps_sum(&robot2.map)))
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
            "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<",
        );

        let (n, n2) = run(input.reader())?;
        assert_eq!(n, 2028);
        assert_eq!(n2, 1751);
        Ok(())
    }

    #[test]
    fn test_run_large() -> Result<(), Box<dyn error::Error>> {
        let input = Bytes::from(
            "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
    ",
        );

        let (n, n2) = run(input.reader())?;
        assert_eq!(n, 10092);
        assert_eq!(n2, 9021);
        Ok(())
    }
}
