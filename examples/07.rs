pub fn main() {
    part1();
}

fn part1_cost_fn(pos: isize, target: isize) -> isize {
    return (pos - target).abs();
}

fn part2_cost_fn(pos: isize, target: isize) -> isize {
    let distance = (pos - target).abs();
    return (distance * (distance + 1)) / 2;
}

fn part1() {
    let s = include_str!("07.txt");
    // let s = "16,1,2,0,4,2,7,1,2,14";
    let nums: Vec<_> = s
        .split(",")
        .filter_map(|s| s.trim().parse::<isize>().ok())
        .collect();
    let (count, total) = nums
        .iter()
        .fold((0, 0), |(count, total), n| (count + 1, total + n));
    let mut minresult: Option<(isize, isize)> = None;
    for middle in 0..1000 {
        let result: isize = nums.iter().map(|s| part2_cost_fn(*s, middle)).sum();
        minresult = match minresult {
            Some((_, val)) => {
                if result < val {
                    Some((middle, result))
                } else {
                    minresult
                }
            }
            None => Some((middle, result)),
        };
    }
    println!(
        "{}, {}, {}, {}",
        count,
        total,
        minresult.unwrap().0,
        minresult.unwrap().1
    );
}
