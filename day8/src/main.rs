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

struct Map {
    map: Vec<Vec<char>>,

    antennas: collections::HashMap<char, Vec<(usize, usize)>>,
}

impl Map {
    pub fn new(map: Vec<Vec<char>>) -> Self {
        let mut antennas: collections::HashMap<char, Vec<(usize, usize)>> =
            collections::HashMap::new();
        for (y, col) in map.iter().enumerate() {
            for (x, c) in col.iter().enumerate() {
                if c.is_alphanumeric() {
                    antennas
                        .entry(*c)
                        .and_modify(|vec| vec.push((x, y)))
                        .or_insert(vec![(x, y)]);
                }
            }
        }

        Map { map, antennas }
    }
}

fn read_map(r: impl BufRead) -> Result<Vec<Vec<char>>, Box<dyn error::Error>> {
    let mut map = Vec::new();
    for line in r.lines() {
        let mut col = Vec::new();
        for c in line?.chars() {
            col.push(c);
        }

        map.push(col);
    }

    Ok(map)
}

fn cartesian_product<R: Copy>(vec: Vec<R>) -> Vec<(R, R)> {
    let mut product: Vec<(R, R)> = Vec::new();

    for i in 0..vec.len() {
        for j in (i + 1)..vec.len() {
            product.push((vec[i], vec[j]));
        }
    }

    product
}

type NodeOption = (Option<(usize, usize)>, Option<(usize, usize)>);

fn calc_nodes(l: (usize, usize), r: (usize, usize)) -> NodeOption {
    let node1x = l.0.checked_add_signed(l.0 as isize - r.0 as isize);
    let node1y = l.1.checked_add_signed(l.1 as isize - r.1 as isize);
    let node2x = r.0.checked_add_signed(r.0 as isize - l.0 as isize);
    let node2y = r.1.checked_add_signed(r.1 as isize - l.1 as isize);

    let node1 = match (node1x, node1y) {
        (Some(x), Some(y)) => Some((x, y)),
        _ => None,
    };

    let node2 = match (node2x, node2y) {
        (Some(x), Some(y)) => Some((x, y)),
        _ => None,
    };

    (node1, node2)
}

fn run(r: impl BufRead) -> Result<usize, Box<dyn error::Error>> {
    let map = Map::new(read_map(r)?);

    let mut nodes = collections::HashSet::new();
    for (_c, antennas) in map.antennas {
        for (l, r) in cartesian_product(antennas) {
            let (node1_o, node2_o) = calc_nodes(l, r);
            if node1_o.is_some() {
                let node1 = node1_o.unwrap();
                if node1.1 < map.map.len() && node1.0 < map.map[node1.1].len() {
                    nodes.insert((node1.0, node1.1));
                }
            }

            if node2_o.is_some() {
                let node2 = node2_o.unwrap();
                if node2.1 < map.map.len() && node2.0 < map.map[node2.1].len() {
                    nodes.insert((node2.0, node2.1));
                }
            }
        }
    }

    Ok(nodes.len())
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
    use std::fs;

    #[test]
    fn test_run() -> Result<(), Box<dyn error::Error>> {
        let input = Bytes::from(
            "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
",
        );

        let n = run(input.reader())?;
        assert_eq!(n, 14);
        Ok(())
    }

    #[test]
    fn test_full_input() -> Result<(), Box<dyn error::Error>> {
        let input_file = fs::File::open("input.in.txt")?;

        let n = run(io::BufReader::new(input_file))?;
        assert_eq!(n, 371);
        Ok(())
    }
}
