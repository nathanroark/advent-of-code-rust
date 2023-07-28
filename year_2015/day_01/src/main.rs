// =====================================================================
// Advent of Code 2015 - Day 01 - Not Quite Lisp 
// http://adventofcode.com/day/1
// Author: Nathan Roark
// =====================================================================

fn part_1() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let mut floor = 0;
    input.chars().for_each(|c| match c {
        '(' => floor += 1,
        ')' => floor -= 1,
        _ => (),
    });
    println!("Part_1 Solution: {}", floor);
} // 74

fn part_2() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let mut floor = 0;
    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }
        if floor == -1 {
            println!("Part_2 Solution: {}", i + 1);
            break;
        }
    }
} // 1795

fn main() {
    part_1();
    part_2();
}
