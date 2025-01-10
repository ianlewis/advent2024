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

struct Region {
    plant_type: char,
    area: usize,
    perimeter: usize,
    sides: usize,
    locations: collections::HashSet<(usize, usize)>,
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

const UP: (isize, isize) = (0, -1);
const DOWN: (isize, isize) = (0, 1);
const LEFT: (isize, isize) = (-1, 0);
const RIGHT: (isize, isize) = (1, 0);

fn find_regions(map: &[Vec<char>]) -> Vec<Region> {
    let mut regions = Vec::new();
    let mut visited = collections::HashSet::new();
    let dirs: [(isize, isize); 4] = [UP, DOWN, LEFT, RIGHT];

    for (y, col) in map.iter().enumerate() {
        for (x, plant_type) in col.iter().enumerate() {
            if visited.contains(&(x, y)) {
                continue;
            }

            // Process the current region.
            let mut stack: collections::VecDeque<(usize, usize)> = collections::VecDeque::new();
            stack.push_back((x, y));

            let mut region = Region {
                plant_type: *plant_type,
                area: 0,
                perimeter: 0,
                sides: 0,
                locations: collections::HashSet::new(),
            };

            while !stack.is_empty() {
                let (cur_x, cur_y) = stack.pop_back().unwrap();
                if visited.contains(&(cur_x, cur_y)) {
                    continue;
                }

                region.area += 1;

                for (dx, dy) in dirs {
                    let next_x_o = cur_x.checked_add_signed(dx);
                    let next_y_o = cur_y.checked_add_signed(dy);

                    if next_y_o.is_none() {
                        region.perimeter += 1;
                        continue;
                    }
                    let next_y = next_y_o.unwrap();
                    if next_y >= map.len() {
                        region.perimeter += 1;
                        continue;
                    }

                    if next_x_o.is_none() {
                        region.perimeter += 1;
                        continue;
                    }
                    let next_x = next_x_o.unwrap();
                    if next_x >= map[next_y].len() {
                        region.perimeter += 1;
                        continue;
                    }

                    if map[next_y][next_x] == region.plant_type {
                        stack.push_back((next_x, next_y));
                    } else {
                        region.perimeter += 1;
                    }
                }

                region.locations.insert((cur_x, cur_y));
                visited.insert((cur_x, cur_y));
            }

            regions.push(region);
        }
    }

    for region in &mut regions {
        for (dx, dy) in dirs {
            // Find locations in the current region with an adjacent location in the
            // current direction that is not in the region.
            let mut edge_locations = collections::HashSet::new();
            for (x, y) in &region.locations {
                let next_x_o = x.checked_add_signed(dx);
                let next_y_o = y.checked_add_signed(dy);

                let is_edge = (|| -> bool {
                    if next_y_o.is_none() {
                        return true;
                    }
                    let next_y = next_y_o.unwrap();
                    if next_y >= map.len() {
                        return true;
                    }

                    if next_x_o.is_none() {
                        return true;
                    }
                    let next_x = next_x_o.unwrap();
                    if next_x >= map[next_y].len() {
                        return true;
                    }

                    !region.locations.contains(&(next_x, next_y))
                })();

                if is_edge {
                    edge_locations.insert((x, y));
                }
            }

            // Search along the edge for adjacent locations.
            let mut adjacent_loc = collections::HashSet::new();
            for (x, y) in &edge_locations {
                // Check in the perpendicular direction for adjacent locations (switch dx and dy).
                let (mut cur_x, mut cur_y) = (**x, **y);
                loop {
                    let next_x_o = cur_x.checked_add_signed(dy);
                    let next_y_o = cur_y.checked_add_signed(dx);

                    if next_y_o.is_none() {
                        break;
                    }
                    let next_y = next_y_o.unwrap();
                    if next_y >= map.len() {
                        break;
                    }

                    if next_x_o.is_none() {
                        break;
                    }
                    let next_x = next_x_o.unwrap();
                    if next_x >= map[next_y].len() {
                        break;
                    }

                    if edge_locations.contains(&(&next_x, &next_y)) {
                        adjacent_loc.insert((next_x, next_y));
                    } else {
                        break;
                    }

                    cur_x = next_x;
                    cur_y = next_y;
                }
            }

            region.sides += edge_locations.len() - adjacent_loc.len();
        }
    }

    regions
}

fn run(r: impl BufRead) -> Result<(usize, usize), Box<dyn error::Error>> {
    let map = read_map(r)?;
    let regions = find_regions(&map);

    let p_cost = regions
        .iter()
        .fold(0, |acc, r| acc + (r.area * r.perimeter));

    let s_cost = regions.iter().fold(0, |acc, r| acc + (r.area * r.sides));

    Ok((p_cost, s_cost))
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
            "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
",
        );

        let (n, n2) = run(input.reader())?;
        assert_eq!(n, 1930);
        assert_eq!(n2, 1206);
        Ok(())
    }
}
