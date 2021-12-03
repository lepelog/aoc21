pub fn main() {
    part1();
    part2();
}

pub fn part1() {
    let s = include_str!("01.txt");
    let mut prev = None;
    let mut increases = 0;
    for num in s.split("\n").filter_map(|r| r.parse::<usize>().ok()) {
        match prev {
            None => {}
            Some(p) => {
                if num > p {
                    increases += 1;
                }
            }
        };
        prev = Some(num);
    }
    println!("part1: {}", increases); // 1390
}

pub fn part2() {
    // let s = "607\n618\n618\n617\n647\n716\n769\n792";
    let s = include_str!("01.txt");
    let mut prev = None;
    let mut increases = 0;
    let nums: Vec<_> = s
        .split("\n")
        .filter_map(|r| r.parse::<usize>().ok())
        .collect();
    for i in 0..(nums.len() - 2) {
        let num = nums[i] + nums[i + 1] + nums[i + 2];
        match prev {
            None => {}
            Some(p) => {
                if num > p {
                    increases += 1;
                }
            }
        };
        prev = Some(num);
    }
    println!("part2: {}", increases); // 1457
}
