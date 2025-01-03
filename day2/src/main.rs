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

// Program day2 reads the input on stdin and prints the total number of safe reports and the number
// of semi-safe (reports where omitting one level results in a safe report)..

use std::io;
use std::io::BufRead;
use std::process::ExitCode;
use std::str;

// is_safe returns the index at which an unsafe value was found or zero if the report is safe (zero
// cannot be the index where a report is found to be unsafe).
fn is_safe(list: &[i64], increasing: bool, skip: usize) -> usize {
    let mut prev = 0;
    for (i, n) in list.iter().enumerate() {
        if i == skip && skip != 0 {
            continue;
        }
        // Check for a diff 0 < x < 3
        if i > 0 {
            let diff = (*n - prev).abs();
            // NOTE: for some reason this is prefferable to 'diff < 1 || diff > 3'
            if !(1..=3).contains(&diff) {
                return i;
            }
        }
        if i > 1 {
            // Check increasing/descreasing
            if (increasing && *n < prev) || (!increasing && *n > prev) {
                return i;
            }
        }

        prev = *n;
    }

    0
}

fn run(r: impl BufRead) -> Result<(i64, i64), String> {
    // Read in both lists.
    let mut safe_num = 0;
    let mut semi_safe_num = 0;
    for line in r.lines() {
        if let Err(e) = line {
            return Err(e.to_string());
        }

        let line_str = line.unwrap();
        let iter = str::split_whitespace(&line_str);

        let mut list = Vec::new();
        for v in iter {
            let n = v.parse::<i64>().map_err(|err| err.to_string())?;
            list.push(n);
        }

        if list.len() < 2 {
            safe_num += 1;
            continue;
        }

        let increasing = list[0] < list[1];
        let index = is_safe(&list, increasing, 0);
        if index == 0 {
            safe_num += 1;
            semi_safe_num += 1;
            continue;
        }

        if index < 3 {
            // Check if either the first or second value can be removed.
            if list.len() >= 3 && is_safe(&list[1..], list[1] < list[2], 0) == 0
                || is_safe(&list, list[0] < list[2], 1) == 0
            {
                semi_safe_num += 1;
                continue;
            }
        }
        if index > 1 {
            // Try skipping the level at index.
            let index2 = is_safe(&list[index - 1..], increasing, 1);
            if index2 == 0 {
                semi_safe_num += 1;
            }
        }
    }

    Ok((safe_num, semi_safe_num))
}

fn main() -> ExitCode {
    let stdin = io::stdin();
    let (safe_num, semi_safe_num) = match run(stdin.lock()) {
        Ok((d, s)) => (d, s),
        Err(e) => {
            println!("error running: {e:?}");
            return ExitCode::from(1);
        }
    };

    println!("{}", safe_num);
    println!("{}", semi_safe_num);

    ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::{Buf, Bytes};

    #[test]
    fn test_run() -> Result<(), String> {
        let input = Bytes::from(
            "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
",
        );

        let (safe, semi_safe) = run(input.reader())?;
        assert_eq!(safe, 2);
        assert_eq!(semi_safe, 4);
        Ok(())
    }

    #[test]
    fn test_unsafe() -> Result<(), String> {
        let input = Bytes::from(
            "1 2 7 8 9
9 7 6 2 1
",
        );

        let (safe, semi_safe) = run(input.reader())?;
        assert_eq!(safe, 0);
        assert_eq!(semi_safe, 0);
        Ok(())
    }

    #[test]
    fn test_semi_safe_first_val() -> Result<(), String> {
        let input = Bytes::from(
            "5 0 1 2 3
",
        );

        let (safe, semi_safe) = run(input.reader())?;
        assert_eq!(safe, 0);
        assert_eq!(semi_safe, 1);
        Ok(())
    }

    #[test]
    fn test_semi_safe_last_val() -> Result<(), String> {
        let input = Bytes::from(
            "1 2 3 4 0
",
        );

        let (safe, semi_safe) = run(input.reader())?;
        assert_eq!(safe, 0);
        assert_eq!(semi_safe, 1);
        Ok(())
    }

    #[test]
    fn test_semi_safe_direction_first_val() -> Result<(), String> {
        let input = Bytes::from(
            "1 0 1 2 3
",
        );

        let (safe, semi_safe) = run(input.reader())?;
        assert_eq!(safe, 0);
        assert_eq!(semi_safe, 1);
        Ok(())
    }

    #[test]
    fn test_semi_safe_direction_last_val() -> Result<(), String> {
        let input = Bytes::from(
            "0 1 2 3 2
",
        );

        let (safe, semi_safe) = run(input.reader())?;
        assert_eq!(safe, 0);
        assert_eq!(semi_safe, 1);
        Ok(())
    }
}
