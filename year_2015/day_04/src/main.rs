// =====================================================================
// Advent of Code 2015 - Day 04 - The Ideal Stocking Stuffer
// http://adventofcode.com/2015/day/4
// Author: Nathan Roark
// =====================================================================

use md5;

fn part_1(input: &str) -> usize {
    let mut solution = 0;
    for i in 0.. {
        let hash = format!("{:x}", md5::compute(format!("{}{}", input, i)));
        if hash.starts_with("00000") {
            solution = i;
            break;
        }
    }
    return solution;
}

fn part_2(input: &str) -> usize {
    let mut solution = 0;
    for i in 0.. {
        let hash = format!("{:x}", md5::compute(format!("{}{}", input, i)));
        if hash.starts_with("000000") {
            solution = i;
            break;
        }
    }
    return solution;
}
pub fn main() {
    let mut input = std::fs::read_to_string("./input.txt").unwrap();
    if input.ends_with('\n') {
        input = input.trim_end_matches('\n').to_string();
    }
    println!("Part 1 Solution: {}", part_1(&input));
    println!("Part 2 Solution: {}", part_2(&input));
}
