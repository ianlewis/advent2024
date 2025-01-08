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

// Program day7 prints the sum of possible calibration targets achievable with
// just the addition and multiplication operators, and the sum of possible
// calibration targets achievable with addition, multiplication, and
// concatenation.

use std::error;
use std::io::{self, BufRead};
use std::process;

type Calibrations = Vec<(i64, Vec<i64>)>;

fn read_calibrations(r: impl BufRead) -> Result<Calibrations, Box<dyn error::Error>> {
    let mut numbers = Vec::new();
    for line_r in r.lines() {
        let line = line_r?;
        let parts: Vec<_> = line.split(":").collect();

        let mut numvec = Vec::new();
        let target = parts[0].parse::<i64>()?;
        for n_str in parts[1].trim().split(" ") {
            numvec.push(n_str.parse::<i64>()?);
        }
        numbers.push((target, numvec));
    }

    Ok(numbers)
}

fn test_num(target: i64, numbers: &[i64], opers: &[fn(i64, i64) -> i64]) -> bool {
    if numbers.is_empty() {
        return false;
    }
    if numbers.len() == 1 {
        return target == numbers[0];
    }

    for oper in opers {
        let n = oper(numbers[0], numbers[1]);
        let mut new_numbers = vec![n];
        new_numbers.extend_from_slice(&numbers[2..]);

        if test_num(target, &new_numbers, opers) {
            return true;
        }
    }

    false
}

fn run(r: impl BufRead) -> Result<(i64, i64), Box<dyn error::Error>> {
    let calibrations = read_calibrations(r)?;

    let mut total = 0;
    let mut total2 = 0;
    let opers = [|l, r| l + r, |l, r| l * r];
    let opers2 = [
        |l, r| l + r,
        |l, r| l * r,
        // Concatenates numbers together.
        // NOTE: Number of digits in a number n is log_10(n) + 1
        |l, r| l * 10_i64.pow(((r as f64).log(10.0) as u32) + 1) + r,
    ];
    for (target, numbers) in calibrations {
        if test_num(target, &numbers, &opers) {
            total += target;
            total2 += target;
            continue;
        }
        if test_num(target, &numbers, &opers2) {
            total2 += target;
        }
    }

    Ok((total, total2))
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
            "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
",
        );

        let (n, n2) = run(input.reader())?;
        assert_eq!(n, 3749);
        assert_eq!(n2, 11387);
        Ok(())
    }
}
