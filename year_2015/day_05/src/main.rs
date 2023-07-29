// =====================================================================
// Advent of Code 2015 - Day 05 - Doesn't He Have Intern-Elves For This?
// http://adventofcode.com/2015/day/5
// Author: Nathan Roark
// =====================================================================

fn part_1(input: &str) -> usize {
    let mut nice_strings = 0;
    for line in input.lines() {
        let mut vowel_count = 0;
        let mut double_letter = false;
        let mut naughty = false;
        let mut last_char = ' ';
        for c in line.chars() {
            if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
                vowel_count += 1;
            }
            if c == last_char {
                double_letter = true;
            }
            if (last_char == 'a' && c == 'b')
                || (last_char == 'c' && c == 'd')
                || (last_char == 'p' && c == 'q')
                || (last_char == 'x' && c == 'y')
            {
                naughty = true;
            }
            last_char = c;
        }
        if vowel_count >= 3 && double_letter && !naughty {
            nice_strings += 1;
        }
    }
    nice_strings
}

fn part_2(input: &str) -> usize {
    let mut nice_strings = 0;
    for line in input.lines() {
        let mut pair = false;
        let mut repeat = false;
        let mut last_char = ' ';
        let mut last_last_char = ' ';
        for (i, c) in line.chars().enumerate() {
            if i > 0 {
                if !pair {
                    let pair_str = format!("{}{}", last_char, c);
                    if line.matches(&pair_str).count() > 1 {
                        pair = true;
                    }
                }
                if !repeat {
                    if last_last_char == c {
                        repeat = true;
                    }
                }
            }
            last_last_char = last_char;
            last_char = c;
        }
        if pair && repeat {
            nice_strings += 1;
        }
    }
    nice_strings
}

pub fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    println!("Part 1 Solution: {}", part_1(&input)); // 258
    println!("Part 2 Solution: {}", part_2(&input)); // 53
}
