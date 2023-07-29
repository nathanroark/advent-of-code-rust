// =====================================================================
// Advent of Code 2015 - Day 02 - I Was Told There Would Be No Math
// http://adventofcode.com/day/2
// Author: Nathan Roark
// =====================================================================

fn part_1(input: &str) -> u32 {
    let mut total = 0;
    for line in input.lines() {
        let mut dims: Vec<u32> = line.split('x').map(|x| x.parse().unwrap()).collect();
        dims.sort();
        let l = dims[0];
        let w = dims[1];
        let h = dims[2];
        total += 2 * l * w + 2 * w * h + 2 * h * l + l * w;
    }
    return total;
}

fn part_2(input: &str) -> u32 {
    let mut total = 0;
    for line in input.lines() {
        let mut dims: Vec<u32> = line.split('x').map(|x| x.parse().unwrap()).collect();
        dims.sort();
        let l = dims[0];
        let w = dims[1];
        let h = dims[2];
        total += 2 * l + 2 * w + l * w * h;
    }
    return total;
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    println!("Part 1 Solution: {}", part_1(&input)); // 74
    println!("Part 2 Solution: {}", part_2(&input)); // 74
}
