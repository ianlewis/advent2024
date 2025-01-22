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

fn read_input(r: impl BufRead) -> Result<Vec<Vec<char>>, Box<dyn error::Error>> {
    let mut codes = Vec::new();
    for line in r.lines() {
        let mut code = Vec::new();
        for c in line?.chars() {
            code.push(c);
        }
        codes.push(code);
    }

    Ok(codes)
}

type Pos = (usize, usize);
type Dir = (char, (isize, isize));
const UP: Dir = ('^', (0, 1));
const DOWN: Dir = ('v', (0, -1));
const LEFT: Dir = ('<', (-1, 0));
const RIGHT: Dir = ('>', (1, 0));
const DIRS: [Dir; 4] = [DOWN, RIGHT, UP, LEFT];

#[derive(Clone)]
struct Keypad {
    // NOTE: We hold the map in a BTreeMap because it is hashable.
    //       We use this property to use the map as part of a cache key later.
    _map: collections::BTreeMap<char, Pos>,
    _pos_map: collections::HashMap<Pos, char>,
    _paths_cache: collections::HashMap<(char, char), Vec<Vec<char>>>,
}

impl Keypad {
    pub fn new(map: collections::HashMap<char, Pos>) -> Self {
        Keypad {
            _pos_map: map.iter().map(|(k, v)| (*v, *k)).collect(),
            _map: map.into_iter().collect(),
            _paths_cache: collections::HashMap::new(),
        }
    }

    pub fn new_numpad() -> Self {
        let mut map = collections::HashMap::new();
        map.insert('0', (1, 0));
        map.insert('A', (2, 0));
        map.insert('1', (0, 1));
        map.insert('2', (1, 1));
        map.insert('3', (2, 1));
        map.insert('4', (0, 2));
        map.insert('5', (1, 2));
        map.insert('6', (2, 2));
        map.insert('7', (0, 3));
        map.insert('8', (1, 3));
        map.insert('9', (2, 3));

        Keypad::new(map)
    }

    pub fn new_dirpad() -> Self {
        let mut map = collections::HashMap::new();
        map.insert('<', (0, 0));
        map.insert('v', (1, 0));
        map.insert('>', (2, 0));
        map.insert('^', (1, 1));
        map.insert('A', (2, 1));

        Keypad::new(map)
    }

    // get_min_paths returns the paths of minimum length to get from button a to button b.
    fn get_min_paths(&mut self, a: char, b: char) -> Vec<Vec<char>> {
        if let Some(paths) = self._paths_cache.get(&(a, b)) {
            return paths.to_vec();
        }

        let mut stack = collections::VecDeque::new();
        stack.push_back((a, collections::HashSet::new(), Vec::new()));

        let mut paths = Vec::new();
        while !stack.is_empty() {
            let (button, mut visited, mut path) = stack.pop_back().unwrap();

            if let Some((x, y)) = self._map.get(&button) {
                if button == b {
                    path.push('A');
                    paths.push(path);
                    continue;
                }

                if visited.contains(&button) {
                    continue;
                }
                visited.insert(button);

                for (dir_char, (dx, dy)) in DIRS {
                    let next_x = (*x as isize + dx) as usize;
                    let next_y = (*y as isize + dy) as usize;

                    if let Some(next_button) = self._pos_map.get(&(next_x, next_y)) {
                        let mut new_path = path.clone();
                        new_path.push(dir_char);
                        stack.push_back((*next_button, visited.clone(), new_path.clone()));
                    }
                }
            }
        }

        // Get minimum length of the found paths.
        let min_len = paths.iter().map(|p| p.len()).min().unwrap_or(0);
        // Get the paths of minimum length.
        let filtered: Vec<_> = paths.into_iter().filter(|p| p.len() <= min_len).collect();
        // Insert into the cache.
        self._paths_cache.insert((a, b), filtered.clone());

        // Return paths of minimum length.
        filtered
    }
}

// RobotChain represents a chain of directional keypads that robots type into to a keypad.
struct RobotChain {
    _keypad: Keypad,
    _dir_keypad: Keypad,
    _chain_len: usize,
    _cost_cache: collections::HashMap<(String, usize, collections::BTreeMap<char, Pos>), usize>,
}

impl RobotChain {
    // new creates a new RobotChain with the given number of directional keypads (including the
    // human).
    pub fn new(keypad: Keypad, num_dir_keypads: usize) -> Self {
        RobotChain {
            _keypad: keypad,
            _chain_len: num_dir_keypads,
            _dir_keypad: Keypad::new_dirpad(),
            _cost_cache: collections::HashMap::new(),
        }
    }

    // calc_cost returns the total number of button presses required to enter the given code on the
    // given keypad at the human's position in the chain.
    pub fn calc_cost(&mut self, code: &[char]) -> usize {
        self._calc_cost(code, self._chain_len, self._keypad.clone())
    }

    fn _calc_cost(&mut self, code: &[char], chain_len: usize, mut keypad: Keypad) -> usize {
        // Check the cost_cache in case we have processed this code before.
        // This is necessary to allow this to run in a reasonable amount of time.
        if let Some(cost) =
            self._cost_cache
                .get(&(String::from_iter(code), chain_len, keypad._map.clone()))
        {
            return *cost;
        }

        let mut total_cost = 0;

        if chain_len == 0 {
            return code.len();
        }

        // Get all button start and end values in a chain starting from A.
        // e.g. ABCDE -> AB,BC,CD,DE
        let button_paths = [&'A'].into_iter().chain(code).zip(code);

        for (a, b) in button_paths {
            let mut costs = Vec::new();

            // Find the all combinations of presses of minimum length needed to enter the code in a
            // directional keypad for the start and end buttons.
            for path in keypad.get_min_paths(*a, *b) {
                // Calculate the cost of the entering this key combination in further down the chain.
                let path_cost = self._calc_cost(&path, chain_len - 1, self._dir_keypad.clone());
                costs.push(path_cost)
            }

            // Add the minimum cost associated with this button path.
            total_cost += costs.iter().min().unwrap_or(&0);
        }

        self._cost_cache.insert(
            (String::from_iter(code), chain_len, keypad._map),
            total_cost,
        );

        total_cost
    }
}

fn run(r: impl BufRead) -> Result<(usize, usize), Box<dyn error::Error>> {
    let codes = read_input(r)?;

    let mut total3 = 0;
    let mut total26 = 0;
    let mut robot3 = RobotChain::new(Keypad::new_numpad(), 3);
    let mut robot26 = RobotChain::new(Keypad::new_numpad(), 26);

    for code in codes {
        let cost3 = robot3.calc_cost(&code);
        let cost26 = robot26.calc_cost(&code);

        let num = String::from_iter(code.clone().into_iter().filter(|c| c.is_numeric()))
            .parse::<usize>()?;
        total3 += cost3 * num;
        total26 += cost26 * num;
    }

    Ok((total3, total26))
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
    fn test_keypad_get_min_paths() -> Result<(), Box<dyn error::Error>> {
        let mut keypad = Keypad::new_dirpad();
        let mut paths = keypad.get_min_paths('A', '<');

        paths.sort();

        assert_eq!(
            paths,
            vec![vec!['<', 'v', '<', 'A'], vec!['v', '<', '<', 'A']]
        );

        Ok(())
    }

    #[test]
    fn test_numpad_get_min_paths() -> Result<(), Box<dyn error::Error>> {
        let mut keypad = Keypad::new_numpad();
        let mut paths = keypad.get_min_paths('1', '6');

        paths.sort();

        assert_eq!(
            paths,
            vec![
                vec!['>', '>', '^', 'A'],
                vec!['>', '^', '>', 'A'],
                vec!['^', '>', '>', 'A'],
            ],
        );

        Ok(())
    }

    #[test]
    fn test_robotchain_calc_cost_shallow_12() -> Result<(), Box<dyn error::Error>> {
        let mut chain = RobotChain::new(Keypad::new_numpad(), 1);
        let cost = chain.calc_cost(&"12".chars().collect::<Vec<char>>());

        // <^<A: A -> 1
        // >A: 1 -> 2

        assert_eq!(cost, 6);
        Ok(())
    }

    #[test]
    fn test_robotchain_calc_cost_shallow_539() -> Result<(), Box<dyn error::Error>> {
        let mut chain = RobotChain::new(Keypad::new_numpad(), 1);
        let cost2 = chain.calc_cost(&"593".chars().collect::<Vec<char>>());

        // <^^A: A -> 5
        // ^>A: 5 -> 9
        // vvA: 9 -> 3

        assert_eq!(cost2, 10);
        Ok(())
    }

    #[test]
    fn test_robotchain_calc_cost_three_gen() -> Result<(), Box<dyn error::Error>> {
        // Numerical keypad
        let code = "029A";

        let mut chain0 = RobotChain::new(Keypad::new_numpad(), 0);
        let cost0 = chain0.calc_cost(&code.chars().collect::<Vec<char>>());
        assert_eq!(cost0, 4);

        // First directional keypad
        // <A^A>^^AvvvA
        //
        // <A: 0
        // ^A: 2
        // >^^A: 9
        // vvvA: A

        let mut chain1 = RobotChain::new(Keypad::new_numpad(), 1);
        let cost1 = chain1.calc_cost(&code.chars().collect::<Vec<char>>());
        assert_eq!(cost1, 12);

        // Second directional keypad
        // v<<A>>^A<A>AvA<^AA>A<vAAA>^A
        //
        // v<<A: <
        // >>^A: A
        //
        // <A: ^
        // >A: A
        //
        // vA: >
        // <^AA: ^^
        // >A: A
        //
        // <vAAA: vvv
        // >^A: A

        let mut chain2 = RobotChain::new(Keypad::new_numpad(), 2);
        let cost2 = chain2.calc_cost(&code.chars().collect::<Vec<char>>());
        assert_eq!(cost2, 28);

        // Third directional keypad
        // <vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A
        //
        // <vA: v
        // <AA: <<
        // >>^A: A
        //
        // vAA: >>
        // <^A: ^
        // >A: A
        //
        // <v<A: <
        // >>^A: A
        //
        // vA: >
        // ^A: A
        //
        // <vA: v
        // >^A: A
        //
        // <v<A: <
        // >^A: ^
        // >AA: AA
        //
        // vA: >
        // ^A: A
        //
        // <v<A: <
        // >A: v
        // >^AAA: AAA
        //
        // vA: >
        // <^A: ^
        // >A: A

        let mut chain3 = RobotChain::new(Keypad::new_numpad(), 3);
        let cost3 = chain3.calc_cost(&code.chars().collect::<Vec<char>>());
        assert_eq!(cost3, 68);

        Ok(())
    }

    #[test]
    fn test_run() -> Result<(), Box<dyn error::Error>> {
        let input = Bytes::from(
            "029A
980A
179A
456A
379A
",
        );

        let (n, n2) = run(input.reader())?;
        assert_eq!(n, 126384);
        assert_eq!(n2, 154115708116294);
        Ok(())
    }
}
