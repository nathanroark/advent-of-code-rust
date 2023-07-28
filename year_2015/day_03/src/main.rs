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
        }
        'v' => {
            y -= 1;
            houses.insert((x, y));
        }
        '>' => {
            x += 1;
            houses.insert((x, y));
        }
        '<' => {
            x -= 1;
            houses.insert((x, y));
        }
        _ => (),
    });
    println!("Part_1 Solution: {}", houses.len());
}

fn part_2() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let mut houses = std::collections::HashSet::new();
    let mut santa_x = 0;
    let mut santa_y = 0;
    let mut robo_x = 0;
    let mut robo_y = 0;
    houses.insert((santa_x, santa_y));
    input.chars().enumerate().for_each(|(i, c)| match c {
        '^' => {
            if i % 2 == 0 {
                santa_y += 1;
                houses.insert((santa_x, santa_y));
            } else {
                robo_y += 1;
                houses.insert((robo_x, robo_y));
            }
        }
        'v' => {
            if i % 2 == 0 {
                santa_y -= 1;
                houses.insert((santa_x, santa_y));
            } else {
                robo_y -= 1;
                houses.insert((robo_x, robo_y));
            }
        }
        '>' => {
            if i % 2 == 0 {
                santa_x += 1;
                houses.insert((santa_x, santa_y));
            } else {
                robo_x += 1;
                houses.insert((robo_x, robo_y));
            }
        }
        '<' => {
            if i % 2 == 0 {
                santa_x -= 1;
                houses.insert((santa_x, santa_y));
            } else {
                robo_x -= 1;
                houses.insert((robo_x, robo_y));
            }
        }
        _ => (),
    });
    println!("Part_2 Solution: {}", houses.len());
}

fn main() {
    part_1();
    part_2();
}
