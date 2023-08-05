// =====================================================================
// Advent of Code 2015 - Day 06 - Probably a Fire Hazard
// http://adventofcode.com/2015/day/6
// Author: Nathan Roark
// =====================================================================

fn part_1(input: &str) -> usize {
    let mut grid = [[false; 1000]; 1000];
    for line in input.lines() {
        let mut words = line.split_whitespace();
        let mut command = words.next().unwrap();
        match command {
            "turn" => command = words.next().unwrap(),
            "toggle" => (),
            _ => panic!("Invalid command"),
        }
        let start = words
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        words.next(); // skip the next word "through"
        let end = words
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        match command {
            "on" => {
                for x in start[0]..=end[0] {
                    for y in start[1]..=end[1] {
                        grid[x][y] = true;
                    }
                }
            }
            "off" => {
                for x in start[0]..=end[0] {
                    for y in start[1]..=end[1] {
                        grid[x][y] = false;
                    }
                }
            }
            "toggle" => {
                for x in start[0]..=end[0] {
                    for y in start[1]..=end[1] {
                        grid[x][y] = !grid[x][y];
                    }
                }
            }
            _ => panic!("Invalid command"),
        }
    }
    grid.iter().flatten().filter(|x| **x).count()
}

fn part_2(input: &str) -> usize {
    let mut grid = [[0; 1000]; 1000];
    for line in input.lines() {
        let mut words = line.split_whitespace();
        let mut command = words.next().unwrap();
        match command {
            "turn" => command = words.next().unwrap(),
            "toggle" => (),
            _ => panic!("Invalid command"),
        }
        let start = words
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        words.next(); // skip the next word "through"
        let end = words
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        match command {
            "on" => {
                for x in start[0]..=end[0] {
                    for y in start[1]..=end[1] {
                        grid[x][y] += 1;
                    }
                }
            }
            "off" => {
                for x in start[0]..=end[0] {
                    for y in start[1]..=end[1] {
                        if grid[x][y] > 0 {
                            grid[x][y] -= 1;
                        }
                    }
                }
            }
            "toggle" => {
                for x in start[0]..=end[0] {
                    for y in start[1]..=end[1] {
                        grid[x][y] += 2;
                    }
                }
            }
            _ => panic!("Invalid command"),
        }
    }
    grid.iter().flatten().sum()
}

fn main() {
    let input = std::fs::read_to_string("./input.txt")
        .unwrap()
        .trim()
        .to_string();
    println!("Part 1 Solution: {}", part_1(&input));
    println!("Part 2 Solution: {}", part_2(&input));
}
