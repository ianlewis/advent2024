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

fn cartesian_product<R: Copy>(vec: &[R]) -> Vec<(R, R)> {
    let mut product: Vec<(R, R)> = Vec::new();

    for i in 0..vec.len() {
        for j in (i + 1)..vec.len() {
            product.push((vec[i], vec[j]));
        }
    }

    product
}

fn calc_first_antinodes(
    map: &[Vec<char>],
    l: (usize, usize),
    r: (usize, usize),
) -> collections::HashSet<(usize, usize)> {
    let node1x_r = l.0.checked_add_signed(l.0 as isize - r.0 as isize);
    let node1y_r = l.1.checked_add_signed(l.1 as isize - r.1 as isize);
    let node2x_r = r.0.checked_add_signed(r.0 as isize - l.0 as isize);
    let node2y_r = r.1.checked_add_signed(r.1 as isize - l.1 as isize);

    let mut antinodes = collections::HashSet::new();
    if let (Some(x), Some(y)) = (node1x_r, node1y_r) {
        if y < map.len() && x < map[y].len() {
            antinodes.insert((x, y));
        }
    }
    if let (Some(x), Some(y)) = (node2x_r, node2y_r) {
        if y < map.len() && x < map[y].len() {
            antinodes.insert((x, y));
        }
    }

    antinodes
}

fn calc_antinodes(
    map: &[Vec<char>],
    l: (usize, usize),
    r: (usize, usize),
) -> collections::HashSet<(usize, usize)> {
    let mut antinodes = collections::HashSet::new();

    // Calculate left antinodes
    let mut m = 1;
    loop {
        let x_r = l.0.checked_add_signed((l.0 as isize - r.0 as isize) * m);
        let y_r = l.1.checked_add_signed((l.1 as isize - r.1 as isize) * m);
        if x_r.is_none() || y_r.is_none() {
            break;
        }
        let x = x_r.unwrap();
        let y = y_r.unwrap();
        if y >= map.len() || x >= map[y].len() {
            break;
        }

        antinodes.insert((x, y));
        m += 1;
    }

    // Calculate right antinodes
    m = 1;
    loop {
        let x_r = r.0.checked_add_signed((r.0 as isize - l.0 as isize) * m);
        let y_r = r.1.checked_add_signed((r.1 as isize - l.1 as isize) * m);
        if x_r.is_none() || y_r.is_none() {
            break;
        }

        let x = x_r.unwrap();
        let y = y_r.unwrap();
        if y >= map.len() || x >= map[y].len() {
            break;
        }

        antinodes.insert((x, y));
        m += 1;
    }

    antinodes
}

fn run(r: impl BufRead) -> Result<(usize, usize), Box<dyn error::Error>> {
    let map = Map::new(read_map(r)?);

    let mut first_antinodes = collections::HashSet::new();
    let mut antinodes = collections::HashSet::new();

    for (_c, antennas) in map.antennas {
        for (l, r) in cartesian_product(&antennas) {
            first_antinodes.extend(calc_first_antinodes(&map.map, l, r));

            antinodes.extend(calc_antinodes(&map.map, l, r));
        }

        // Add all antennas
        antinodes.extend(antennas);
    }

    Ok((first_antinodes.len(), antinodes.len()))
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

        let (n, n2) = run(input.reader())?;
        assert_eq!(n, 14);
        assert_eq!(n2, 34);
        Ok(())
    }
}
