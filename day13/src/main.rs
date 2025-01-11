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

use std::error;
use std::io::{self, BufRead};
use std::process;

struct Prize {
    a_x: f64,
    a_y: f64,
    b_x: f64,
    b_y: f64,
    x: f64,
    y: f64,
}

fn parse_button_xy(s: &str) -> Result<(f64, f64), Box<dyn error::Error>> {
    let parts: Vec<_> = s.split(" ").collect();
    if parts.len() != 4 {
        return Err("unexpected line".into());
    }
    let x = parts[2]
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<String>()
        .parse::<f64>()?;
    let y = parts[3]
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<String>()
        .parse::<f64>()?;

    Ok((x, y))
}

fn parse_prize_xy(s: &str) -> Result<(f64, f64), Box<dyn error::Error>> {
    let parts: Vec<_> = s.split(" ").collect();
    if parts.len() != 3 {
        return Err("unexpected line".into());
    }
    let x = parts[1]
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<String>()
        .parse::<f64>()?;
    let y = parts[2]
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<String>()
        .parse::<f64>()?;

    Ok((x, y))
}

fn read_prizes(mut r: impl BufRead) -> Result<Vec<Prize>, Box<dyn error::Error>> {
    let mut prizes = Vec::new();
    loop {
        let mut a_buf = String::new();
        if r.read_line(&mut a_buf)? == 0 {
            // EOF: No more input.
            break;
        }
        let (a_x, a_y) = parse_button_xy(&a_buf)?;

        let mut b_buf = String::new();
        if r.read_line(&mut b_buf)? == 0 {
            return Err("unexpected EOF".into());
        }
        let (b_x, b_y) = parse_button_xy(&b_buf)?;

        let mut p_buf = String::new();
        if r.read_line(&mut p_buf)? == 0 {
            return Err("unexpected EOF".into());
        }
        let (p_x, p_y) = parse_prize_xy(&p_buf)?;

        prizes.push(Prize {
            a_x,
            a_y,
            b_x,
            b_y,
            x: p_x,
            y: p_y,
        });

        let mut empty = String::new();
        if r.read_line(&mut empty)? == 0 {
            // last empty line may be omitted at EOF.
            break;
        }
    }

    Ok(prizes)
}

fn run(r: impl BufRead) -> Result<(u64, u64), Box<dyn error::Error>> {
    let prizes = read_prizes(r)?;

    // The problem can be described as two linear equations the intersection of which is the
    // solution to the problem.
    // a_x*a + b_x*b - x = 0
    // a_y*a + b_y*b - y = 0

    // Standard form for the linear equations is as follows:
    // a1*x + b1*y + c1 = 0
    // a2*x + b2*y + c2 = 0

    // ... and the variables map as follows:
    // a = x
    // b = y
    // a1 = a_x
    // b1 = b_x
    // c1 = -x
    // a2 = a_y
    // b2 = b_y
    // c2 = -y

    // The cross multiplication method allows the solution to be solved via the equations:
    // https://www.cuemath.com/algebra/linear-equations-in-two-variables/
    // x = (b1*c2 - b2*c1) / (a1*b2 - a2*b1)
    // y = (a2*c1 - a1*c2) / (a1*b2 - a2*b1)

    let mut tokens = 0;
    let mut tokens_part2 = 0;
    for p in prizes {
        let a = ((p.b_x * (-p.y)) - (p.b_y * (-p.x))) / ((p.a_x * p.b_y) - (p.a_y * p.b_x));
        let b = ((p.a_y * (-p.x)) - (p.a_x * (-p.y))) / ((p.a_x * p.b_y) - (p.a_y * p.b_x));

        // If the resulting numbers are real numbers then we found a good solution.
        let a32 = a as u64;
        let b32 = b as u64;
        if a == a32 as f64 && b == b32 as f64 {
            tokens += a32 * 3;
            tokens += b32;
        };

        // Part 2 is just the same but with 10000000000000 addedd to x and y.
        let a_part2 = ((p.b_x * (-(p.y + 10000000000000.0)))
            - (p.b_y * (-(p.x + 10000000000000.0))))
            / ((p.a_x * p.b_y) - (p.a_y * p.b_x));
        let b_part2 = ((p.a_y * (-(p.x + 10000000000000.0)))
            - (p.a_x * (-(p.y + 10000000000000.0))))
            / ((p.a_x * p.b_y) - (p.a_y * p.b_x));

        let a32_part2 = a_part2 as u64;
        let b32_part2 = b_part2 as u64;
        if a_part2 == a32_part2 as f64 && b_part2 == b32_part2 as f64 {
            tokens_part2 += a32_part2 * 3;
            tokens_part2 += b32_part2;
        };
    }

    Ok((tokens, tokens_part2))
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
            "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
",
        );

        let (n, n2) = run(input.reader())?;
        assert_eq!(n, 480);
        assert_eq!(n2, 875318608908);
        Ok(())
    }
}
