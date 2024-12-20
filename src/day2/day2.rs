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
// So, in this example, 2 reports are safe.
//
// Analyze the unusual data from the engineers. How many reports are safe?

use std::io;
use std::io::BufRead;
use std::process::ExitCode;
use std::str;

// Program day2 reads the input on stdin and prints the total number of safe reports.

fn run(r: impl BufRead) -> Result<(i64, i64), String> {
    // Read in both lists.
    let mut safe_num = 0;
    for line in r.lines() {
        if let Err(e) = line {
            return Err(e.to_string());
        }

        let line_str = line.unwrap();
        let iter = str::split_whitespace(&line_str);

        let mut increasing = true;
        let mut safe = true;
        let mut prev = 0;
        for (i, v) in iter.enumerate() {
            let n = v.parse::<i64>().map_err(|err| err.to_string())?;
            // Check for a diff 0 < x < 3
            if i > 0 {
                let diff = (n - prev).abs();
                if diff < 1 || diff > 3 {
                    safe = false;
                    break;
                }
            }
            if i == 1 {
                // determine if increasing or decreasing
                increasing = n > prev;
            } else if i > 1 {
                // Check increasing/descreasing
                if (increasing && n < prev) || (!increasing && n > prev) {
                    safe = false;
                    break;
                }
            }

            prev = n;
        }

        if safe {
            safe_num += 1;
        }
    }

    return Ok((safe_num, 0));
}

fn main() -> ExitCode {
    let stdin = io::stdin();
    let (safe_num, _) = match run(stdin.lock()) {
        Ok((d, s)) => (d, s),
        Err(e) => {
            println!("error running: {e:?}");
            return ExitCode::from(1);
        }
    };

    println!("{}", safe_num);

    ExitCode::SUCCESS
}
