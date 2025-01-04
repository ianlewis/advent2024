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
// of safe dampened reports.

use std::io;
use std::io::BufRead;
use std::process::ExitCode;
use std::str;

fn is_safe_increasing(list: &[i64]) -> bool {
    for (a, b) in list.iter().zip(list[1..].iter()) {
        if a - b < 1 || a - b > 3 {
            return false;
        }
    }
    true
}

fn evaluate_report(report: &[i64]) -> bool {
    // Check that the report is safe in forward or reverse order.
    let report_rev: Vec<i64> = report.iter().copied().rev().collect();
    is_safe_increasing(report) || is_safe_increasing(&report_rev)
}

fn evaluate_report_dampened(report: &[i64]) -> bool {
    for (i, _v) in report.iter().enumerate() {
        // Remove a number and see if it is still safe.
        let mut report_dampened = vec![0; report.len()];
        report_dampened.clone_from_slice(report);
        report_dampened.remove(i);
        if is_safe_increasing(&report_dampened) {
            return true;
        }

        // Do the same thing in revese order.
        let report_dampened_rev: Vec<i64> = report_dampened.iter().copied().rev().collect();
        if is_safe_increasing(&report_dampened_rev) {
            return true;
        }
    }

    false
}

fn run(r: impl BufRead) -> Result<(i64, i64), String> {
    // Read in both lists.
    let mut safe_num = 0;
    let mut safe_num_dampened = 0;
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

        if evaluate_report(&list) {
            safe_num += 1;
        }

        if evaluate_report_dampened(&list) {
            safe_num_dampened += 1;
        }
    }

    Ok((safe_num, safe_num_dampened))
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
