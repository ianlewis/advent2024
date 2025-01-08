// Copyright 2024 Ian Lewis
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

// Program day1 reads the input on stdin and prints the total distance from part one on the first
// line of stdout and the similarity score from part two on the second line.

use std::io;
use std::io::BufRead;
use std::process::ExitCode;
use std::str;

fn run(r: impl BufRead) -> Result<(i64, i64), String> {
    let mut first = Vec::new();
    let mut second = Vec::new();

    // Read in both lists.
    for line in r.lines() {
        if let Err(e) = line {
            return Err(e.to_string());
        }

        let line_str = line.unwrap();
        let mut iter = str::split_whitespace(&line_str);

        first.push(match iter.next() {
            Some(v) => v.parse::<i64>().map_err(|err| err.to_string()),
            None => Err("no left value".to_string()),
        }?);

        second.push(match iter.next() {
            Some(v) => v.parse::<i64>().map_err(|err| err.to_string()),
            None => Err("no right value".to_string()),
        }?);
    }

    // Sort both lists.
    first.sort();
    second.sort();

    // Accumulate the sum of the distances.
    let dist = first
        .iter()
        .zip(second.iter())
        .fold(0, |acc, (l, r)| acc + (l - r).abs());

    // Accumulate the similarity score.
    let similarity = first.iter().fold(0, |acc, n| {
        acc + (n * second.iter().filter(|n2| n == *n2).count() as i64)
    });

    Ok((dist, similarity))
}

fn main() -> ExitCode {
    let stdin = io::stdin();
    let (dist, sim) = match run(stdin.lock()) {
        Ok((d, s)) => (d, s),
        Err(e) => {
            println!("error running: {e:?}");
            return ExitCode::from(1);
        }
    };

    println!("{}", dist);
    println!("{}", sim);

    ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::{Buf, Bytes};

    #[test]
    fn test_run() -> Result<(), String> {
        let input = Bytes::from(
            "3   4
4   3
2   5
1   3
3   9
3   3
",
        );

        let (dist, sim) = run(input.reader())?;
        assert_eq!(dist, 11);
        assert_eq!(sim, 31);
        Ok(())
    }

    #[test]
    fn test_empty() -> Result<(), String> {
        let input = Bytes::from(
            "3   4
4   3

1   3
3   9
3   3
",
        );

        match run(input.reader()) {
            Ok((_d, _s)) => Err("expected error".to_string()),
            Err(e) => {
                assert_eq!(e.to_string(), "no left value");
                Ok(())
            }
        }?;

        Ok(())
    }

    #[test]
    fn test_no_right_value() -> Result<(), String> {
        let input = Bytes::from(
            "3   4
4   3
3
1   3
3   9
3   3
",
        );

        match run(input.reader()) {
            Ok((_d, _s)) => Err("expected error".to_string()),
            Err(e) => {
                assert_eq!(e.to_string(), "no right value");
                Ok(())
            }
        }?;

        Ok(())
    }

    #[test]
    fn test_nan() -> Result<(), String> {
        let input = Bytes::from(
            "3   4
4   3
a   b
3   9
3   3
",
        );

        match run(input.reader()) {
            Ok((_d, _s)) => Err("expected error".to_string()),
            Err(e) => {
                assert_eq!(e.to_string(), "invalid digit found in string");
                Ok(())
            }
        }?;

        Ok(())
    }
}
