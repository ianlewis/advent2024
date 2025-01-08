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

use std::error;
use std::io::{self, BufRead};
use std::process;

// read_disk_map_to_blocks reads the disk map and returns a vector of blocks containing their file
// ID. The block is None if empty.
fn read_disk_map_to_blocks(mut r: impl BufRead) -> Result<Vec<Option<i64>>, Box<dyn error::Error>> {
    let mut blocks = Vec::new();
    let mut is_file = true;
    let mut file_id = 0;
    loop {
        let mut buf = [0; 1];
        match r.read_exact(&mut buf) {
            Ok(_) => {
                let c = buf[0] as char;
                // The end of file may have a newline.
                if c == '\n' {
                    break;
                }
                let n = String::from(c).parse::<i64>()?;

                if is_file {
                    for _i in 0..n {
                        blocks.push(Some(file_id));
                    }
                    file_id += 1;
                } else {
                    for _i in 0..n {
                        blocks.push(None);
                    }
                }

                is_file = !is_file;
            }
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                break; // Handle EOF
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }

    Ok(blocks)
}

fn compact(blocks: &mut [Option<i64>]) {
    let mut i = 0;
    let mut j = blocks.len() - 1;
    while i < j {
        if blocks[i].is_some() {
            i += 1;
            continue;
        }
        if blocks[j].is_none() {
            j -= 1;
            continue;
        }
        blocks[i] = Some(blocks[j].unwrap());
        blocks[j] = None;
    }
}

fn find_space(blocks: &[Option<i64>], size: usize) -> Option<usize> {
    let mut i = 0;
    'outer: while i < blocks.len() {
        // Find a free space.
        while i < blocks.len() && blocks[i].is_some() {
            i += 1;
            continue;
        }

        // Check if the free space is the appropriate length.
        if i + size > blocks.len() {
            break;
        }
        for (j, b) in blocks.iter().skip(i).take(size).enumerate() {
            if b.is_some() {
                i += j;
                continue 'outer;
            }
        }
        return Some(i);
    }

    None
}

fn defrag(blocks: &mut [Option<i64>]) {
    // i marks the beginning of where to look for open space.
    let mut i = 0;

    // j marks the end of where to look for open space and the beginning of file data.
    let mut j = blocks.len() - 1;

    loop {
        while i < j && blocks[i].is_some() {
            i += 1;
        }
        while i < j && blocks[j].is_none() {
            j -= 1;
        }
        if i >= j {
            break;
        }

        // Find the file start (j) and end (k) locations.
        let file_id = blocks[j].unwrap();
        let k = j;
        while let Some(b) = blocks[j - 1] {
            if b == file_id {
                j -= 1;
            } else {
                break;
            }
        }
        if i >= j {
            break;
        }

        // The file is now at blocks[j..=k]
        let file_len = k - j + 1;
        let s = find_space(&blocks[i..j], file_len);
        if s.is_some() {
            // We found some open space. Move the file.
            let s_index = i + s.unwrap();
            for index in 0..file_len {
                blocks[s_index + index] = blocks[j + index];
                blocks[j + index] = None;
            }
        }

        j -= 1;
    }
}

fn calc_checksum(blocks: &[Option<i64>]) -> i64 {
    let mut checksum = 0;
    for (i, n) in blocks.iter().enumerate() {
        if n.is_some() {
            checksum += i as i64 * n.unwrap();
        }
    }

    checksum
}

fn run(r: impl BufRead) -> Result<(i64, i64), Box<dyn error::Error>> {
    let mut blocks = read_disk_map_to_blocks(r)?;
    let mut blocks2 = blocks.clone();
    compact(&mut blocks);
    defrag(&mut blocks2);
    Ok((calc_checksum(&blocks), calc_checksum(&blocks2)))
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
        let input = Bytes::from("12345\n");

        let (n, n2) = run(input.reader())?;
        assert_eq!(n, 60);
        assert_eq!(n2, 132);
        Ok(())
    }

    #[test]
    fn test_example() -> Result<(), Box<dyn error::Error>> {
        let input = Bytes::from("2333133121414131402\n");

        let (n, n2) = run(input.reader())?;
        assert_eq!(n, 1928);
        assert_eq!(n2, 2858);
        Ok(())
    }
}
