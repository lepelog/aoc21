use std::collections::HashMap;

pub fn main() {
    part1();
    part2();
}

struct Puzzle {
    start: String,
    replacements: HashMap<(u8, u8), u8>,
}

fn parse_puzzle() -> Puzzle {
    // let s = include_str!("14example.txt");
    let s = include_str!("14.txt");
    let mut lines = s.trim().split("\n");
    let start = lines.next().unwrap().into();
    lines.next();
    let replacements = lines
        .map(|line| {
            let a1 = line.bytes().nth(0).unwrap();
            let a2 = line.bytes().nth(1).unwrap();
            let b = line.bytes().nth(6).unwrap();
            ((a1, a2), b)
        })
        .collect();
    Puzzle {
        start,
        replacements,
    }
}

fn get_counts<T>(v: &Vec<T>) -> HashMap<T, usize>
where
    T: Eq + std::hash::Hash + Clone,
{
    let mut result = HashMap::new();
    for e in v.iter() {
        *result.entry(e.clone()).or_insert(0) += 1;
    }
    result
}

fn part1() {
    let puzzle = parse_puzzle();
    let mut current: Vec<_> = puzzle.start.bytes().collect();
    for _ in 0..10 {
        let mut next = Vec::with_capacity(2 * current.len());
        next.push(current[0]);
        for (a1, a2) in current.iter().zip(current.iter().skip(1)) {
            if let Some(insert) = puzzle.replacements.get(&(*a1, *a2)) {
                next.extend([*insert, *a2]);
            } else {
                next.extend([*a2]);
            };
        }
        current = next;
    }
    let counts = get_counts(&current);
    let max_count = counts.values().max().unwrap();
    let min_count = counts.values().min().unwrap();
    println!("{}", max_count - min_count);
}

fn part2() {
    let puzzle = parse_puzzle();
    let mut current: Vec<_> = puzzle.start.bytes().collect();
    let mut pair_counts = HashMap::new();
    for (a1, a2) in current.iter().zip(current.iter().skip(1)) {
        *pair_counts.entry((*a1, *a2)).or_insert(0) += 1;
    }
    for _ in 0..40 {
        let mut next_pair_counts: HashMap<(u8, u8), usize> = HashMap::new();
        for ((a1, a2), count) in pair_counts {
            if let Some(insert) = puzzle.replacements.get(&(a1, a2)) {
                *next_pair_counts.entry((a1, *insert)).or_insert(0) += count;
                *next_pair_counts.entry((*insert, a2)).or_insert(0) += count;
            } else {
                *next_pair_counts.entry((a1, a2)).or_insert(0) += count;
            }
        }
        pair_counts = next_pair_counts;
    }
    let mut final_countmap = HashMap::new();
    for ((a1, a2), count) in pair_counts.iter() {
        *final_countmap.entry(*a1).or_insert(0) += count;
        *final_countmap.entry(*a2).or_insert(0) += count;
    }
    // we now counted every letter twice (except start and finish)
    // so make sure start and end are doubled too
    *final_countmap.get_mut(current.first().unwrap()).unwrap() += 1;
    *final_countmap.get_mut(current.last().unwrap()).unwrap() += 1;
    let max_count = final_countmap.values().max().unwrap() / 2;
    let min_count = final_countmap.values().min().unwrap() / 2;
    println!("{}", max_count - min_count);
}
