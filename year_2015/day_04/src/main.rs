// =====================================================================
// Advent of Code 2015 - Day 04 - The Ideal Stocking Stuffer
// http://adventofcode.com/2015/day/4
// Author: Nathan Roark
// =====================================================================

use md5;

fn part_1() {
    let mut input = std::fs::read_to_string("./input.txt").unwrap();
    if input.ends_with('\n') {
        input = input.trim_end_matches('\n').to_string();
    }
    for i in 0.. {
        let hash = format!("{:x}", md5::compute(format!("{}{}", input, i)));
        if hash.starts_with("00000") {
            println!("Part_1 Solution: {}", i);
            break;
        }
    }
}

fn part_2() {
    let mut input = std::fs::read_to_string("./input.txt").unwrap();
    if input.ends_with('\n') {
        input = input.trim_end_matches('\n').to_string();
    }
    for i in 0.. {
        let hash = format!("{:x}", md5::compute(format!("{}{}", input, i)));
        if hash.starts_with("000000") {
            println!("Part_2 Solution: {}", i);
            break;
        }
    }
}
pub fn main() {
    part_1();
    part_2();
}
