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

fn read_stones(r: impl BufRead) -> Result<collections::HashMap<i64, i64>, Box<dyn error::Error>> {
    let mut stones: collections::HashMap<i64, i64> = collections::HashMap::new();
    for line in r.lines() {
        for n_str in line?.split(" ") {
            let n = n_str.parse::<i64>()?;
            stones.entry(n).and_modify(|a| *a += 1).or_insert(1);
        }
    }
    Ok(stones)
}

fn blink(stones: &mut collections::HashMap<i64, i64>) -> collections::HashMap<i64, i64> {
    let mut new_stones: collections::HashMap<i64, i64> = collections::HashMap::new();
    for (stone, n) in stones {
        // If the stone is engraved with the number 0, it is replaced by a stone engraved with
        // the number 1.
        if *stone == 0 {
            new_stones.entry(1).and_modify(|a| *a += *n).or_insert(*n);
            continue;
        }

        // If the stone is engraved with a number that has an even number of digits, it is
        // replaced by two stones. The left half of the digits are engraved on the new left stone,
        // and the right half of the digits are engraved on the new right stone. (The new numbers
        // don't keep extra leading zeroes: 1000 would become stones 10 and 0.)
        let num_digits = (*stone as f64).log(10.0) as u32 + 1;
        if num_digits % 2 == 0 {
            let exp = 10_i64.pow(num_digits / 2);
            let left = stone / exp; // NOTE: integer drops lower order digits.
            let right = stone - (left * exp);

            new_stones
                .entry(left)
                .and_modify(|a| *a += *n)
                .or_insert(*n);
            new_stones
                .entry(right)
                .and_modify(|a| *a += *n)
                .or_insert(*n);

            continue;
        }

        // If none of the other rules apply, the stone is replaced by a new stone; the old
        // stone's number multiplied by 2024 is engraved on the new stone.
        new_stones
            .entry(stone * 2024)
            .and_modify(|a| *a += *n)
            .or_insert(*n);
    }

    new_stones
}

fn run(r: impl BufRead) -> Result<(i64, i64), Box<dyn error::Error>> {
    let mut stones = read_stones(r)?;

    for _i in 0..25 {
        stones = blink(&mut stones);
    }
    let len_25 = stones.values().sum();

    for _i in 25..75 {
        stones = blink(&mut stones);
    }

    Ok((len_25, stones.values().sum()))
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
    fn test_blink() -> Result<(), Box<dyn error::Error>> {
        let input = Bytes::from("0 1 10 99 999");
        let stones = blink(&mut read_stones(input.reader())?);
        assert_eq!(stones.values().sum::<i64>(), 7);
        Ok(())
    }

    #[test]
    fn test_run() -> Result<(), Box<dyn error::Error>> {
        let input = Bytes::from("125 17");

        let (n, n2) = run(input.reader())?;
        assert_eq!(n, 55312);
        assert_eq!(n2, 65601038650482);
        Ok(())
    }
}
