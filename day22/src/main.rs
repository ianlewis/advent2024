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
use std::collections::vec_deque;
use std::error;
use std::io::{self, BufRead};
use std::process;

struct BoundedVecDeque<T> {
    _deque: collections::VecDeque<T>,
    _limit: usize,
}

impl<T> BoundedVecDeque<T> {
    pub fn new(limit: usize) -> Self {
        BoundedVecDeque {
            _deque: collections::VecDeque::new(),
            _limit: limit,
        }
    }

    pub fn push_back(&mut self, elem: T) {
        self._deque.push_back(elem);
        while self._deque.len() > self._limit {
            self._deque.pop_front();
        }
    }

    pub fn iter(&self) -> vec_deque::Iter<'_, T> {
        self._deque.iter()
    }

    pub fn len(&self) -> usize {
        self._deque.len()
    }
}

fn read_input(r: impl BufRead) -> Result<Vec<usize>, Box<dyn error::Error>> {
    let mut secret_numbers = Vec::new();
    for line in r.lines() {
        secret_numbers.push(line?.parse::<usize>()?);
    }
    Ok(secret_numbers)
}

fn evolve(secret_num: usize) -> usize {
    // Calculate the result of multiplying the secret number by 64. Then, mix this result into the
    // secret number. Finally, prune the secret number.
    //
    // NOTE: 64 = 2^6
    //       This shifts the secret number 6 bits to the left, XORs the value
    //       against the original secret number, and mods by 2^24.
    //       This does not change the least significant digit.
    let mut new_num = prune(mix(secret_num, secret_num * 64));

    // Calculate the result of dividing the secret number by 32. Round the result down to the
    // nearest integer. Then, mix this result into the secret number. Finally, prune the secret
    // number.
    //
    // NOTE: 32 = 2^5
    //       This shifts the secret number 5 bits to the right, XORs the value
    //       against the original secret number, and mods by 2^24.
    //
    new_num = prune(mix(new_num, new_num / 32));

    // Calculate the result of multiplying the secret number by 2048. Then, mix this result into
    // the secret number. Finally, prune the secret number.
    //
    // NOTE: 2048 = 2^11
    prune(mix(new_num, new_num * 2048))
}

fn mix(secret_num: usize, value: usize) -> usize {
    // To mix a value into the secret number, calculate the bitwise XOR of the
    // given value and the secret number. Then, the secret number becomes the
    // result of that operation. (If the secret number is 42 and you were to mix 15
    // into the secret number, the secret number would become 37.)
    secret_num ^ value
}

fn prune(secret_num: usize) -> usize {
    // To prune the secret number, calculate the value of the secret number modulo
    // 16777216. Then, the secret number becomes the result of that operation. (If the
    // secret number is 100000000 and you were to prune the secret number, the secret
    // number would become 16113920.)
    //
    // NOTE: 16777216 = 4096^2 = 2^24
    // 	     prune effectively strips the bits higher than the 24th bit.
    secret_num % 16777216
}

fn price_change(secret_num: usize) -> (usize, isize) {
    let next_num = evolve(secret_num);
    (
        next_num,
        (next_num as isize % 10) - (secret_num as isize % 10),
    )
}

fn run(r: impl BufRead) -> Result<(usize, usize), Box<dyn error::Error>> {
    let secret_numbers = read_input(r)?;

    let mut final_numbers = Vec::new();
    for num in &secret_numbers {
        let mut secret_num = *num;
        for _i in 0..2000 {
            secret_num = evolve(secret_num);
        }
        final_numbers.push(secret_num);
    }

    let mut sequences = collections::HashMap::new();
    for num in &secret_numbers {
        let mut seen = collections::HashSet::new();
        let mut changes = BoundedVecDeque::new(4);

        let mut secret_num = *num;
        let mut diff;
        for _i in 0..2000 {
            (secret_num, diff) = price_change(secret_num);
            changes.push_back(diff);
            if changes.len() == 4 {
                let c = changes.iter().cloned().collect::<Vec<_>>();
                if !seen.contains(&c) {
                    seen.insert(c.clone());
                    sequences.insert(c.clone(), sequences.get(&c).unwrap_or(&0) + secret_num % 10);
                }
            }
        }
    }

    Ok((
        final_numbers.iter().sum(),
        *sequences
            .iter()
            .max_by(|(_, a), (_, b)| a.cmp(b))
            .unwrap_or((&Vec::new(), &0))
            .1,
    ))
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
    fn test_mix() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(mix(42, 15), 37);
        Ok(())
    }

    #[test]
    fn test_prune() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(prune(100000000), 16113920);
        Ok(())
    }

    #[test]
    fn test_evolve() -> Result<(), Box<dyn error::Error>> {
        let secret_numbers = [
            123, 15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484,
            7753432, 5908254,
        ];
        let price_changes: Vec<_> = secret_numbers[1..]
            .iter()
            .cloned()
            .zip([-3, 6, -1, -1, 0, 2, -2, 0, -2, 2])
            .collect();

        for (i, num) in secret_numbers[..secret_numbers.len() - 1]
            .iter()
            .enumerate()
        {
            assert_eq!(evolve(*num), secret_numbers[i + 1]);
            assert_eq!(price_change(*num), price_changes[i]);
        }

        Ok(())
    }

    #[test]
    fn test_evolve_2000() -> Result<(), Box<dyn error::Error>> {
        let secret_numbers = [
            (1, 8685429),
            (10, 4700978),
            (100, 15273692),
            (2024, 8667524),
        ];
        for (mut num, expected) in secret_numbers {
            for _i in 0..2000 {
                num = evolve(num);
            }
            assert_eq!(num, expected);
        }

        Ok(())
    }

    #[test]
    fn test_run() -> Result<(), Box<dyn error::Error>> {
        let input = Bytes::from(
            "1
10
100
2024
",
        );

        let (n, n2) = run(input.reader())?;
        assert_eq!(n, 37327623);
        assert_eq!(n2, 24);
        Ok(())
    }

    #[test]
    fn test_run2() -> Result<(), Box<dyn error::Error>> {
        let input = Bytes::from(
            "1
2
3
2024
",
        );

        let (n, n2) = run(input.reader())?;
        assert_eq!(n, 37990510);
        assert_eq!(n2, 23);
        Ok(())
    }
}
