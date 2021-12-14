use std::collections::HashSet;

pub fn main() {
    part1();
    part2();
}

type Point = (isize, isize);

#[derive(Debug)]
enum Fold {
    X(isize),
    Y(isize),
}

#[derive(Debug)]
struct Puzzle {
    points: HashSet<Point>,
    folds: Vec<Fold>,
}

fn parse_puzzle() -> Puzzle {
    // let s = include_str!("13example.txt");
    let s = include_str!("13.txt");
    let mut lines_iter = s.split('\n');
    let mut points = HashSet::new();
    let mut folds = Vec::new();
    // parse points
    while let Some(line) = lines_iter.next() {
        if line == "" {
            break;
        }
        let (x, y) = line.split_once(",").unwrap();
        points.insert((x.parse().unwrap(), y.parse().unwrap()));
    }
    // parse fold instructions
    while let Some(line) = lines_iter.next() {
        if line == "" {
            continue;
        }
        let end = &line["fold along ".len()..];
        folds.push(match &end[0..1] {
            "x" => Fold::X(end[2..].parse().unwrap()),
            "y" => Fold::Y(end[2..].parse().unwrap()),
            _ => unreachable!(),
        });
    }
    Puzzle { points, folds }
}

fn do_fold(points: &HashSet<Point>, fold: &Fold) -> HashSet<Point> {
    points
        .iter()
        .filter_map(|p| match fold {
            Fold::X(fold_x) => {
                let (x, y) = p;
                if x == fold_x {
                    None
                } else if x < fold_x {
                    Some((*x, *y))
                } else {
                    Some((2 * fold_x - x, *y))
                }
            }
            Fold::Y(fold_y) => {
                let (x, y) = p;
                if y == fold_y {
                    None
                } else if y < fold_y {
                    Some((*x, *y))
                } else {
                    Some((*x, 2 * fold_y - y))
                }
            }
        })
        .collect()
}

fn part1() {
    let puzzle = parse_puzzle();
    // println!("{:?}", puzzle);
    let fold = &puzzle.folds[0];
    let result = do_fold(&puzzle.points, fold);
    println!("part1: {}", result.len());
}

fn part2() {
    let mut puzzle = parse_puzzle();
    for fold in puzzle.folds {
        puzzle.points = do_fold(&puzzle.points, &fold);
    }
    let max_x = *puzzle.points.iter().map(|(x, _y)| x).max().unwrap();
    let max_y = *puzzle.points.iter().map(|(_x, y)| y).max().unwrap();
    println!("part2:");
    for y in 0..=max_y {
        for x in 0..=max_x {
            if puzzle.points.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
