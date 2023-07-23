fn part_1() {
    let seafloor = include_str!("../input.txt")
        .lines()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<u16>>();

    let mut i = 0;

    for items in seafloor.windows(2) {
        if let [a, b] = items {
            if a < b {
                i += 1;
            }
        }
    }

    println!("Day-01 Part-02 Result: {}", i); // 1167
}

fn part_2() {
    let seafloor = include_str!("../input.txt")
        .lines()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<u16>>();

    let mut i = 0;

    for items in seafloor.windows(4) {
        if let [a, _, _, b] = items {
            if a < b {
                i += 1;
            }
        }
    }

    println!("Day-01 Part-02 Result: {}", i); //1130
}

fn main() {
    part_1();
    part_2();
}
