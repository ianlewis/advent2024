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

#[derive(Clone)]
struct Robot {
    p_x: i64,
    p_y: i64,
    v_x: i64,
    v_y: i64,

    w: i64,
    h: i64,
}

impl Robot {
    pub fn advance(&mut self, t: i64) {
        self.p_x += self.v_x * t;
        if self.p_x < 0 {
            self.p_x = self.w - (self.p_x.abs() % self.w);
        }
        self.p_x %= self.w;
        self.p_y += self.v_y * t;
        if self.p_y < 0 {
            self.p_y = self.h - (self.p_y.abs() % self.h);
        }
        self.p_y %= self.h;
    }
}

fn read_robots(r: impl BufRead, w: i64, h: i64) -> Result<Vec<Robot>, Box<dyn error::Error>> {
    let mut robots = Vec::new();

    for line_r in r.lines() {
        let line = line_r?;
        let var_parts: Vec<&str> = line.split(" ").collect();

        // Parse out the position.
        let pos_parts: Vec<&str> = var_parts[0].split("=").collect();
        let pos_vars: Vec<&str> = pos_parts[1].split(",").collect();
        let p_x = pos_vars[0].parse::<i64>()?;
        let p_y = pos_vars[1].parse::<i64>()?;

        let vel_parts: Vec<&str> = var_parts[1].split("=").collect();
        let vel_vars: Vec<&str> = vel_parts[1].split(",").collect();
        let v_x = vel_vars[0].parse::<i64>()?;
        let v_y = vel_vars[1].parse::<i64>()?;

        robots.push(Robot {
            p_x,
            p_y,
            v_x,
            v_y,
            w,
            h,
        });
    }

    Ok(robots)
}

fn run(r: impl BufRead, w: i64, h: i64, t: i64) -> Result<(i64, i64), Box<dyn error::Error>> {
    let mut robots = read_robots(r, w, h)?;

    let mut robots2 = robots.clone();

    // Advance the robots.
    for robot in &mut robots {
        robot.advance(t);
    }

    // Calculate the safety factor.
    let mut quad_robots: [i64; 4] = [0; 4];
    let mid_x_left = w / 2;
    let mut mid_x_right = mid_x_left;
    if w % 2 != 0 {
        mid_x_right += 1;
    }
    let mid_y_top = h / 2;
    let mut mid_y_bottom = mid_y_top;
    if h % 2 != 0 {
        mid_y_bottom += 1;
    }

    for robot in &robots {
        if robot.p_x < mid_x_left && robot.p_y < mid_y_top {
            // top left
            quad_robots[0] += 1;
        } else if robot.p_x >= mid_x_right && robot.p_y < mid_y_top {
            // top right
            quad_robots[1] += 1;
        } else if robot.p_x < mid_x_left && robot.p_y >= mid_y_bottom {
            // bottom left
            quad_robots[2] += 1;
        } else if robot.p_x >= mid_x_right && robot.p_y >= mid_y_bottom {
            quad_robots[3] += 1;
        }
    }

    let mut safety_factor = 1;
    for num_robots in quad_robots {
        safety_factor *= num_robots;
    }

    // The robots arrange themselves in a pattern where many of them are next to each other.
    // We just look for this pattern...
    let mut xmas_t = -1;
    for t in 0..10000 {
        let mut positions = collections::HashSet::new();
        for robot in &mut robots2 {
            positions.insert((robot.p_x, robot.p_y));
        }

        if robots2
            .iter()
            .filter(|r| positions.contains(&(r.p_x + 1, r.p_y)))
            .count()
            > 200
        {
            xmas_t = t;
            // Print the map to make sure we got the right arrangement.
            // print_map(w, h, &robots2);
            break;
        }

        for robot in &mut robots2 {
            robot.advance(1);
        }
    }

    Ok((safety_factor, xmas_t))
}

/*
fn print_map(w: i64, h: i64, robots: &[Robot]) {
    for y in 0..h {
        for x in 0..w {
            let num_robots = robots.iter().filter(|r| r.p_x == x && r.p_y == y).count();
            if num_robots == 0 {
                print!(".");
            } else {
                print!("{}", num_robots);
            }
        }
        println!();
    }
}
*/

fn main() -> process::ExitCode {
    let stdin = io::stdin();
    let (n, n2) = match run(stdin.lock(), 101, 103, 100) {
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
            "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
",
        );

        let (n, n2) = run(input.reader(), 11, 7, 100)?;
        assert_eq!(n, 12);
        assert_eq!(n2, -1);
        Ok(())
    }
}
