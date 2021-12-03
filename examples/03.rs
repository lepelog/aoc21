pub fn main() {
    part2();
}

pub fn part1() {
    let s = include_str!("03.txt");
    let mut total_count = 0;
    let mut bit_counts = [0; 12];
    for line in s.split("\n") {
        total_count += 1;
        for (b, cnt) in line.bytes().zip(bit_counts.iter_mut()) {
            if b == '1' as u8 {
                *cnt += 1;
            }
        }
    }
    let mut gamma = 0;
    for cnt in bit_counts.iter() {
        gamma <<= 1;
        if *cnt > total_count / 2 {
            gamma += 1;
        }
    }
    println!("{:?}", bit_counts);
    let epsilon = (!gamma) & 0xFFF;
    println!("{}", gamma * epsilon);
}

type PuzzleInput = Vec<[bool; 12]>;

pub fn do_filter(vec: PuzzleInput, pos: usize, most_count: bool) -> PuzzleInput {
    let mut bit_count = 0;
    for val in vec.iter() {
        if val[pos] {
            bit_count += 1;
        }
    }
    let bit_to_keep = if most_count {
        bit_count >= vec.len() - bit_count
    } else {
        bit_count < vec.len() - bit_count
    };
    println!(
        "bits: {}, total: {}, keep: {}",
        bit_count,
        vec.len(),
        bit_to_keep
    );
    vec.into_iter().filter(|x| x[pos] == bit_to_keep).collect()
}

pub fn to_num(inp: &[bool; 12]) -> usize {
    let mut out = 0;
    for num in inp.iter() {
        out <<= 1;
        if *num {
            out += 1;
        }
    }
    out
}

pub fn part2() {
    let s = include_str!("03.txt");
    let lines: PuzzleInput = s
        .split("\n")
        .map(|line| {
            let mut line_b = [false; 12];
            for (b, bit_set) in line.bytes().zip(line_b.iter_mut()) {
                *bit_set = b == '1' as u8;
            }
            line_b
        })
        .collect();
    let mut copied = lines.clone();
    for i in 0..12 {
        if copied.len() == 1 {
            break;
        }
        copied = do_filter(copied, i, true);
    }
    let oxyg = to_num(&copied[0]);
    copied = lines.clone();
    for i in 0..12 {
        if copied.len() == 1 {
            break;
        }
        copied = do_filter(copied, i, false);
    }
    let co2 = to_num(&copied[0]);
    println!("{}, {}, {}", oxyg, co2, oxyg * co2);
}
