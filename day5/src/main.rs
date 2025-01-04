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

// Program day5 prints the sum of the middle page number from correctly-ordered updates and the sum
// of middle page numbers of incorrectly-ordered updates after they have been corrected.

use std::error;
use std::io::{self, BufRead};
use std::process;

type Rules = Vec<(i64, i64)>;
type Updates = Vec<Vec<i64>>;

fn read_rules_and_updates(r: impl BufRead) -> Result<(Rules, Updates), Box<dyn error::Error>> {
    let mut rules: Vec<(i64, i64)> = Vec::new();
    let mut updates: Vec<Vec<i64>> = Vec::new();

    let mut reading_rules = true;
    for line_r in r.lines() {
        let line = line_r?;
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
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

fn is_valid(rules: &[(i64, i64)], update: &[i64]) -> bool {
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

    true
}

fn correct_update(rules: &[(i64, i64)], update: &[i64]) -> Result<Vec<i64>, String> {
    let mut corrected: Vec<i64> = Vec::new();

    // Build up a new list of pages where the numbers are inserted at the correct position.
    for n in update.iter() {
        let mut after: Vec<i64> = Vec::new();
        let mut before: Vec<i64> = Vec::new();
        for (x, y) in rules.iter() {
            if n == x {
                before.push(*y);
            }
            if n == y {
                after.push(*x);
            }
        }

        let mut inserted = false;
        for (i, c) in corrected.clone().iter().enumerate() {
            if !inserted && before.contains(c) {
                corrected.insert(i, *n);
                inserted = true;
            }

            // NOTE: We add a consistency check for good measure.
            if inserted && after.contains(c) {
                return Err("inconsistent rules".to_string());
            }
        }
        if !inserted {
            corrected.push(*n);
        }
    }

    Ok(corrected)
}

fn run(r: impl BufRead) -> Result<(i64, i64), Box<dyn error::Error>> {
    let (rules, updates) = read_rules_and_updates(r)?;
    // Filter the valid updates and sum the middle page numbers.
    let valid_update_sum = updates
        .iter()
        .filter(|u| is_valid(&rules, u))
        .fold(0, |acc, u| acc + u[u.len() / 2]);

    // Filter the invalid updates, correct them, and sum the middle page numbers.
    let invalid_updates = updates.iter().filter(|u| !is_valid(&rules, u));
    let mut invalid_update_sum = 0;
    for u in invalid_updates {
        let corrected = correct_update(&rules, u)?;
        invalid_update_sum += corrected[corrected.len() / 2];
    }

    Ok((valid_update_sum, invalid_update_sum))
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

        let (n, n2) = run(input.reader())?;
        assert_eq!(n, 143);
        assert_eq!(n2, 123);
        Ok(())
    }

    #[test]
    fn test_inconsistent_rules() -> Result<(), Box<dyn error::Error>> {
        let input = Bytes::from(
            "1|2
             2|3
             3|1

1,2,3
",
        );

        match run(input.reader()) {
            Ok((n, n2)) => {
                println!("{} {}", n, n2);
                Err("Expected error".to_string().into())
            }
            Err(_) => Ok(()),
        }
    }
}
