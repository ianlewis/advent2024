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

use std::cmp;
use std::collections;
use std::error;
use std::io::{self, BufRead};
use std::process;

fn read_input(r: impl BufRead) -> Result<(Vec<String>, Vec<String>), Box<dyn error::Error>> {
    let lines: Vec<String> = r.lines().collect::<Result<Vec<_>, _>>()?;

    // Read and sort in reverse order by length so we test longer keys first. This reduces the
    // number of comparisons and backtracking we need to do.
    let mut towel_patterns: Vec<_> = lines[0].split(",").map(|s| s.trim().to_string()).collect();
    towel_patterns.sort_by_key(|p| cmp::Reverse(p.len()));

    let designs = lines[2..]
        .iter()
        .map(|s| s.trim().to_string())
        .collect::<Vec<_>>();

    Ok((towel_patterns, designs.to_vec()))
}

fn valid_design_count(design: String, patterns: &[String]) -> u64 {
    _valid_design_count(design, patterns, &mut collections::HashMap::new())
}

fn _valid_design_count(
    design: String,
    patterns: &[String],
    visited: &mut collections::HashMap<String, u64>,
) -> u64 {
    if design.is_empty() {
        return 1;
    }

    // We keep a cache of visited values and the count of successful pattern
    // combinations.
    if let Some(c) = visited.get(&design) {
        return *c;
    }

    let mut total = 0;
    for p in patterns {
        if design.len() >= p.len() && design[..p.len()] == *p {
            total += _valid_design_count(design[p.len()..].to_string(), patterns, visited);
        }
    }

    visited.insert(design, total);

    total
}

fn run(r: impl BufRead) -> Result<(usize, u64), Box<dyn error::Error>> {
    let (patterns, designs) = read_input(r)?;

    let valid_designs: Vec<_> = designs
        .iter()
        .map(|d| valid_design_count(d.to_string(), &patterns))
        .filter(|c| *c > 0)
        .collect();

    Ok((valid_designs.len(), valid_designs.iter().sum()))
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
            "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
",
        );

        let (n, n2) = run(input.reader())?;
        assert_eq!(n, 6);
        assert_eq!(n2, 16);
        Ok(())
    }
}
