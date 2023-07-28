// =====================================================================
// Advent of Code 2015 - Day 03 - Perfectly Spherical Houses in a Vacuum
// http://adventofcode.com/day/3
// Author: Nathan Roark
// =====================================================================

fn part_1() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let mut houses = std::collections::HashSet::new();
    let mut x = 0;
    let mut y = 0;
    houses.insert((x, y));
    input.chars().for_each(|c| match c {
        '^' => {
            y += 1;
            houses.insert((x, y));
        },
        'v' => {
            y -= 1;
            houses.insert((x, y));
        },
        '>' => {
            x += 1;
            houses.insert((x, y));
        },
        '<' => {
            x -= 1;
            houses.insert((x, y));
        },
        _ => (),
    });
    println!("Part_1 Solution: {}", houses.len());
}

fn part_2() {
}

fn main() {
    part_1();
    part_2();
}
