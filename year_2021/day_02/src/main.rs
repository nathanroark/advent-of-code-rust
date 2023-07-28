fn part_1() {
    let (f, d) = include_str!("../input.txt")
        .lines()
        .map(|l| l.split_once(" ").unwrap())
        .fold((0, 0), |(f, d), (k, v)| {
            match (k, v.parse::<i32>().unwrap()) {
                ("forward", v) => (f + v, d),
                ("down", v) => (f, d + v),
                ("up", v) => (f, d - v),
                _ => unreachable!(),
            }
        });

    println!("Day-02 Part-01 Result: {}", f * d);
}

fn part_2() {
    let (f, d, _) = include_str!("../input.txt")
        .lines()
        .map(|l| l.split_once(" ").unwrap())
        .fold((0, 0, 0), |(f, d, a), (k, v)| {
            match (k, v.parse::<i32>().unwrap()) {
                ("forward", v) => (f + v, d + a * v, a),
                ("down", v) => (f, d, a + v),
                ("up", v) => (f, d, a - v),
                _ => unreachable!(),
            }
        });

    println!("Day-02 Part-02 Result: {}", f * d);
}

fn main() {
    part_1();
    part_2();
}
