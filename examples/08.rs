use std::collections::HashSet;

pub fn main() {
    //     let s = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
    // edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
    // fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
    // fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
    // aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
    // fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
    // dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
    // bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
    // egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
    // gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
    let s = include_str!("08.txt");
    part1(s);
    part2(s);
}

pub fn process_line_part1(line: &str) -> usize {
    let lights = line.split_once("|").unwrap().1;
    lights
        .split(" ")
        .map(|s| s.len())
        .filter(|s| *s == 2 || *s == 4 || *s == 3 || *s == 7)
        .count()
}

pub fn process_line_part2(line: &str) -> usize {
    let mut known_patterns = vec![HashSet::new(); 10];
    let (front, back) = line.split_once("|").unwrap();
    let splitted: Vec<HashSet<_>> = front
        .trim()
        .split(" ")
        .map(|s| s.chars().collect::<HashSet<_>>())
        .collect();
    let mut patt5 = Vec::with_capacity(3);
    let mut patt6 = Vec::with_capacity(3);
    for pat in splitted.into_iter() {
        match pat.len() {
            2 => known_patterns[1] = pat,
            3 => known_patterns[7] = pat,
            4 => known_patterns[4] = pat,
            7 => known_patterns[8] = pat,
            5 => patt5.push(pat),
            6 => patt6.push(pat),
            _ => unreachable!(),
        };
    }
    for pat in patt5 {
        // 2, 3, 5
        if pat.is_superset(&known_patterns[1]) {
            known_patterns[3] = pat;
        } else if pat.intersection(&known_patterns[4]).count() == 3 {
            known_patterns[5] = pat;
        } else {
            known_patterns[2] = pat;
        }
    }
    for pat in patt6 {
        // 0, 6, 9
        if pat.is_superset(&known_patterns[3]) {
            known_patterns[9] = pat;
        } else if pat.is_superset(&known_patterns[5]) {
            known_patterns[6] = pat;
        } else {
            known_patterns[0] = pat;
        }
    }
    for (i, pat) in known_patterns.iter().enumerate() {
        if pat.len() == 0 {
            panic!("didn't find pattern: {}!", i);
        }
    }
    back.split(" ")
        .filter_map(|s| {
            let set: HashSet<_> = s.chars().collect();
            if set.len() > 0 {
                Some(set)
            } else {
                None
            }
        })
        .map(|set| {
            for (i, other) in known_patterns.iter().enumerate() {
                if set == *other {
                    return i;
                }
            }
            unreachable!();
        })
        .fold(0, |acc, val| acc * 10 + val)
}

pub fn part1(s: &str) {
    let sum: usize = s.trim().split("\n").map(process_line_part1).sum();
    println!("part1: {}", sum);
}

pub fn part2(s: &str) {
    let sum: usize = s.trim().split("\n").map(process_line_part2).sum();
    println!("part2: {}", sum);
}
