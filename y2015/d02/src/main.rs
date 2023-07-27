// =====================================================================
// Advent of Code 2015 - Day 02 - Perfectly Spherical Houses in a Vacuum
// http://adventofcode.com/day/3
// Author: Nathan Roark
// =====================================================================

fn part_1() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let mut total = 0;
    for line in input.lines() {
        let mut dims: Vec<u32> = line.split('x').map(|x| x.parse().unwrap()).collect();
        dims.sort();
        let l = dims[0];
        let w = dims[1];
        let h = dims[2];
        total += 2 * l * w + 2 * w * h + 2 * h * l + l * w;
    }
    println!("Part_1 Solution: {}", total);
}

fn part_2() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let mut total = 0;
    for line in input.lines() {
        let mut dims: Vec<u32> = line.split('x').map(|x| x.parse().unwrap()).collect();
        dims.sort();
        let l = dims[0];
        let w = dims[1];
        let h = dims[2];
        total += 2 * l + 2 * w + l * w * h;
    }
    println!("Part_2 Solution: {}", total);
}

fn main() {
    part_1();
    part_2();
}
