pub fn main() {
    part2();
}

pub fn part1() {
    let input = include_str!("02.txt");
    let mut pos = (0, 0);
    for line in input.split("\n") {
        if let Some((cmd, count)) = line.split_once(' ') {
            let count: usize = count.parse().unwrap();
            match cmd {
                "forward" => {
                    pos.0 += count;
                }
                "down" => {
                    pos.1 += count;
                }
                "up" => {
                    pos.1 -= count;
                }
                _ => panic!("uh: {}", cmd),
            }
        }
    }
    println!("{:?}", pos);
    println!("{}", pos.0 * pos.1);
}

pub fn part2() {
    let input = include_str!("02.txt");
    let mut pos = (0, 0);
    let mut aim: isize = 0;
    for line in input.split("\n") {
        if let Some((cmd, count)) = line.split_once(' ') {
            let count: isize = count.parse().unwrap();
            match cmd {
                "forward" => {
                    pos.0 += count;
                    pos.1 += count * aim;
                }
                "down" => {
                    aim += count;
                }
                "up" => {
                    aim -= count;
                }
                _ => panic!("uh: {}", cmd),
            }
        }
    }
    println!("{:?}", pos);
    println!("{}", pos.0 * pos.1);
}
