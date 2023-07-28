const WIDTH: usize = 12;
const COUNT: usize = 1000;

fn part_1() {
    let gamma = include_str!("../input.txt")
        .lines()
        .map(|l| usize::from_str_radix(l, 2).unwrap())
        .fold(vec![0; WIDTH], |count, bits| {
            count
                .into_iter()
                .enumerate()
                .map(|(i, n)| n + ((bits & 1 << i) >> i))
                .collect()
        })
        .into_iter()
        .enumerate()
        .map(|(i, b)| ((b >= COUNT / 2) as u32) << i)
        .sum::<u32>();

    println!(
        "Day-03 Part-01 Result: {}",
        gamma * (!gamma & ((1 << WIDTH) - 1))
    ); // 2261546
}

fn part_2() {
    fn get_o2(reports: &Vec<u32>) -> u32 {
        let mut filter: Vec<u32> = reports.clone();
        for x in (0..WIDTH).rev() {
            let ones_count = filter.iter().filter(|diag| (*diag & (1 << x) > 0)).count();
            let ones_common = if ones_count * 2 >= filter.len() { 1 } else { 0 };
            filter = filter
                .into_iter()
                .filter(|diag| (*diag & (1 << x)) == (ones_common << x))
                .collect();
            if filter.len() == 1 {
                return *filter.get(0).unwrap();
            }
        }
        return 0;
    }

    fn get_co2(reports: &Vec<u32>) -> u32 {
        let mut filter: Vec<u32> = reports.clone();
        for x in (0..WIDTH).rev() {
            let ones_count = filter.iter().filter(|diag| (*diag & (1 << x) > 0)).count();
            let ones_uncommon = if ones_count * 2 >= filter.len() { 0 } else { 1 };
            filter = filter
                .into_iter()
                .filter(|diag| (*diag & (1 << x)) == (ones_uncommon << x))
                .collect();
            if filter.len() == 1 {
                return *filter.get(0).unwrap();
            }
        }
        return 0;
    }

    let file = include_str!("../input.txt")
        .lines()
        .map(|l| u32::from_str_radix(l, 2).unwrap())
        .collect::<Vec<_>>();

    println!("Day-03 Part-02 Result: {}", get_o2(&file) * get_co2(&file)); // 6775520
}

fn main() {
    part_1();
    part_2();
}
