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
use std::error;
use std::io::{self, BufRead};
use std::process;

type NetworkData = (
    collections::HashSet<String>,
    collections::HashMap<String, Vec<String>>,
);

fn read_network(r: impl BufRead) -> Result<NetworkData, Box<dyn error::Error>> {
    let mut computers = collections::HashSet::new();
    let mut network = collections::HashMap::new();
    for line_o in r.lines() {
        let line = line_o?;
        let parts = line.split("-").collect::<Vec<_>>();
        computers.insert(parts[0].to_string());
        network
            .entry(parts[0].to_string())
            .and_modify(|c: &mut Vec<String>| c.push(parts[1].to_string()))
            .or_insert(vec![parts[1].to_string()]);

        computers.insert(parts[1].to_string());
        network
            .entry(parts[1].to_string())
            .and_modify(|c: &mut Vec<String>| c.push(parts[0].to_string()))
            .or_insert(vec![parts[0].to_string()]);
    }

    Ok((computers, network))
}

fn get_three_groups(
    computers: collections::HashSet<String>,
    network: collections::HashMap<String, Vec<String>>,
) -> Vec<Vec<String>> {
    let mut stack = collections::VecDeque::new();

    for name in &computers {
        if name.starts_with('t') {
            let group = vec![name.to_string()];
            stack.push_back(group);
        }
    }

    let mut groups = Vec::new();
    let mut seen = Vec::new();
    while !stack.is_empty() {
        let group = stack.pop_back().unwrap();

        if group.len() == 3 {
            groups.push(group);
            continue;
        }

        for name in &computers {
            if group.contains(name) {
                continue;
            }

            let tmp = Vec::new();
            let connected = network.get(name).unwrap_or(&tmp);
            let all_connected = 'blk: {
                for g in &group {
                    if !connected.contains(g) {
                        break 'blk false;
                    }
                }
                true
            };

            if all_connected {
                let mut new_group = group.clone();
                new_group.push(name.to_string());
                new_group.sort();
                if !seen.contains(&new_group) {
                    stack.push_back(new_group.clone());
                    seen.push(new_group);
                }
            }
        }
    }

    groups
}

fn get_lan_party(
    computers: collections::HashSet<String>,
    network: collections::HashMap<String, Vec<String>>,
) -> Vec<String> {
    let mut stack = collections::VecDeque::new();

    stack.push_back((Vec::new(), computers));

    let mut groups = Vec::new();
    let mut seen = collections::HashSet::new();
    while !stack.is_empty() {
        let (group, rest) = stack.pop_back().unwrap();

        if seen.contains(&group) {
            continue;
        }

        let mut connected = collections::HashSet::new();
        for name in rest.clone() {
            let all_connected = 'blk: {
                for name_g in &group {
                    if !network.get(name_g).unwrap().contains(&name) {
                        break 'blk false;
                    }
                }
                true
            };

            if all_connected {
                connected.insert(name);
            }
        }

        seen.insert(group.clone());

        if connected.is_empty() {
            if !groups.contains(&group) {
                groups.push(group);
            }
            continue;
        }

        for name in &connected {
            let mut new_group = group.clone();
            new_group.push(name.to_string());
            new_group.sort();

            let mut new_rest = connected.clone();
            new_rest.remove(name);

            stack.push_back((new_group, new_rest));
        }
    }

    groups
        .into_iter()
        .max_by(|a, b| a.len().cmp(&b.len()))
        .unwrap_or(Vec::new())
}

fn run(r: impl BufRead) -> Result<(usize, String), Box<dyn error::Error>> {
    let (computers, network) = read_network(r)?;

    Ok((
        get_three_groups(computers.clone(), network.clone()).len(),
        get_lan_party(computers, network).join(","),
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
    fn test_run() -> Result<(), Box<dyn error::Error>> {
        let input = Bytes::from(
            "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
",
        );

        let (n, n2) = run(input.reader())?;
        assert_eq!(n, 7);
        assert_eq!(n2, "co,de,ka,ta");
        Ok(())
    }
}
