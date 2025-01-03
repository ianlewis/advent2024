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

// Program day5 prints the sum of the middle page number from correctly-ordered updates.

use std::error;
use std::io::{self, BufRead};
use std::process;

fn read_rules_and_updates(
    r: impl BufRead,
) -> Result<(Vec<(i64, i64)>, Vec<Vec<i64>>), Box<dyn error::Error>> {
    let mut rules: Vec<(i64, i64)> = Vec::new();
    let mut updates: Vec<Vec<i64>> = Vec::new();

    let mut reading_rules = true;
    for line_r in r.lines() {
        let line = line_r?;
        let trimmed_line = line.trim();
        if trimmed_line == "" {
            reading_rules = false;
            continue;
        }

        // We are either reading rules or updates.
        if reading_rules {
            let parts: Vec<&str> = trimmed_line.split("|").collect();

            let left = parts[0].parse::<i64>()?;
            let right = parts[1].parse::<i64>()?;

            rules.push((left, right));
        } else {
            let mut pages: Vec<i64> = Vec::new();
            let trimmed_line = line.trim();
            for page_str in trimmed_line.split(",") {
                pages.push(page_str.parse::<i64>()?);
            }

            updates.push(pages);
        }
    }

    Ok((rules, updates))
}

fn is_valid(rules: &Vec<(i64, i64)>, update: &Vec<i64>) -> bool {
    // Keep a list of pages that cannot come after pages we have seen.
    let mut invalid_pages: Vec<i64> = Vec::new();
    for page in update.iter() {
        if invalid_pages.contains(page) {
            return false;
        }

        // Add pages that are invalid moving forward.
        for (x, y) in rules.iter() {
            if y == page {
                invalid_pages.push(*x);
            }
        }
    }

    return true;
}

fn run(r: impl BufRead) -> Result<i64, Box<dyn error::Error>> {
    let (rules, updates) = read_rules_and_updates(r)?;
    // Filter the valid updates and sum the middle page numbers.
    Ok(updates
        .iter()
        .filter(|u| is_valid(&rules, *u))
        .fold(0, |acc, u| acc + u[u.len() / 2]))
}

fn main() -> process::ExitCode {
    let stdin = io::stdin();
    let n = match run(stdin.lock()) {
        Ok(n) => n,
        Err(e) => {
            println!("error running: {e:?}");
            return process::ExitCode::from(1);
        }
    };

    println!("{}", n);

    process::ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::{Buf, Bytes};

    #[test]
    fn test_run() -> Result<(), Box<dyn error::Error>> {
        let input = Bytes::from(
            "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
",
        );

        let n = run(input.reader())?;
        assert_eq!(n, 143);
        Ok(())
    }
}
