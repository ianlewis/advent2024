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

// https://adventofcode.com/2024/day/2
// --- Day 2: Red-Nosed Reports ---
//
// Fortunately, the first location The Historians want to search isn't a long walk
// from the Chief Historian's office.
//
// While the Red-Nosed Reindeer nuclear fusion/fission plant appears to contain no
// sign of the Chief Historian, the engineers there run up to you as soon as they
// see you. Apparently, they still talk about the time Rudolph was saved through
// molecular synthesis from a single electron.
//
// They're quick to add that - since you're already here - they'd really
// appreciate your help analyzing some unusual data from the Red-Nosed reactor.
// You turn to check if The Historians are waiting for you, but they seem to have
// already divided into groups that are currently searching every corner of the
// facility. You offer to help with the unusual data.
//
// The unusual data (your puzzle input) consists of many reports, one report per
// line. Each report is a list of numbers called levels that are separated by
// spaces. For example:
//
// 7 6 4 2 1
// 1 2 7 8 9
// 9 7 6 2 1
// 1 3 2 4 5
// 8 6 4 4 1
// 1 3 6 7 9
//
// This example data contains six reports each containing five levels.
//
// The engineers are trying to figure out which reports are safe. The Red-Nosed
// reactor safety systems can only tolerate levels that are either gradually
// increasing or gradually decreasing. So, a report only counts as safe if both of
// the following are true:
//
// - The levels are either all increasing or all decreasing.
// - Any two adjacent levels differ by at least one and at most three.
//
// In the example above, the reports can be found safe or unsafe by checking those rules:
//
// 7 6 4 2 1: Safe because the levels are all decreasing by 1 or 2.
// 1 2 7 8 9: Unsafe because 2 7 is an increase of 5.
// 9 7 6 2 1: Unsafe because 6 2 is a decrease of 4.
// 1 3 2 4 5: Unsafe because 1 3 is increasing but 3 2 is decreasing.
// 8 6 4 4 1: Unsafe because 4 4 is neither an increase or a decrease.
// 1 3 6 7 9: Safe because the levels are all increasing by 1, 2, or 3.
//
// So, in this example, 2 reports are safe.
//
// Analyze the unusual data from the engineers. How many reports are safe?
//
// --- Part Two ---
// The engineers are surprised by the low number of safe reports until they realize they forgot to
// tell you about the Problem Dampener.
//
// The Problem Dampener is a reactor-mounted module that lets the reactor safety systems tolerate a
// single bad level in what would otherwise be a safe report. It's like the bad level never
// happened!
//
// Now, the same rules apply as before, except if removing a single level from an unsafe report
// would make it safe, the report instead counts as safe.
//
// More of the above example's reports are now safe:
//
// 7 6 4 2 1: Safe without removing any level.
// 1 2 7 8 9: Unsafe regardless of which level is removed.
// 9 7 6 2 1: Unsafe regardless of which level is removed.
// 1 3 2 4 5: Safe by removing the second level, 3.
// 8 6 4 4 1: Safe by removing the third level, 4.
// 1 3 6 7 9: Safe without removing any level.
//
// Thanks to the Problem Dampener, 4 reports are actually safe!
//
// Update your analysis by handling situations where the Problem Dampener can remove a single level
// from unsafe reports. How many reports are now safe?

use std::io;
use std::io::BufRead;
use std::process::ExitCode;
use std::str;

// Program day2 reads the input on stdin and prints the total number of safe reports and the number
// of semi-safe (reports where omitting one level results in a safe report)..

// is_safe returns the index at which an unsafe value was found or zero if the report is safe (zero
// cannot be the index where a report is found to be unsafe).
fn is_safe(list: &Vec<i64>, increasing: bool, skip: usize) -> usize {
    let mut prev = 0;
    for (i, n) in list.iter().enumerate() {
        if i == skip && skip != 0 {
            continue;
        }
        // Check for a diff 0 < x < 3
        if i > 0 {
            let diff = (*n - prev).abs();
            if diff < 1 || diff > 3 {
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

    return 0;
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
            if list.len() >= 3 {
                if is_safe(&list[1..].to_vec(), list[1] < list[2], 0) == 0 {
                    semi_safe_num += 1;
                    continue;
                } else if is_safe(&list, list[0] < list[2], 1) == 0 {
                    semi_safe_num += 1;
                    continue;
                }
            }
        }
        if index > 1 {
            // Try skipping the level at index.
            let index2 = is_safe(&list[index - 1..].to_vec(), increasing, 1);
            if index2 == 0 {
                semi_safe_num += 1;
            }
        }
    }

    return Ok((safe_num, semi_safe_num));
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
