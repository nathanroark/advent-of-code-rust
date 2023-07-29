// =====================================================================
// Advent of Code 2015 - Day 01 - Not Quite Lisp
// http://adventofcode.com/day/1
// Author: Nathan Roark
// =====================================================================

fn part_1(input: &str) -> i32 {
    let mut floor = 0;
    input.chars().for_each(|c| match c {
        '(' => floor += 1,
        ')' => floor -= 1,
        _ => (),
    });
    return floor;
}

fn part_2(input: &str) -> usize {
    let mut floor = 0;
    let mut answer = 0;
    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }
        if floor == -1 {
            answer = i + 1;
            break;
        }
    }
    return answer;
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    println!("Part 1 Solution: {}", part_1(&input)); // 74
    println!("Part 2 Solution: {}", part_2(&input)); // 1795
}
