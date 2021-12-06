use std::collections::HashMap;

pub fn main() {
    part1();
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn parse(s: &str) -> Self {
        let (x, y) = s.split_once(",").unwrap();
        return Point {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        };
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Line {
    a: Point,
    b: Point,
}

impl Line {
    fn add_all(&self, set: &mut HashMap<Point, usize>) {
        if self.a.x == self.b.x {
            let from = self.a.y.min(self.b.y);
            let to = self.a.y.max(self.b.y);
            for y in from..=to {
                *set.entry(Point { x: self.a.x, y: y }).or_insert(0) += 1;
            }
        } else if self.a.y == self.b.y {
            let from = self.a.x.min(self.b.x);
            let to = self.a.x.max(self.b.x);
            for x in from..=to {
                *set.entry(Point { x: x, y: self.a.y }).or_insert(0) += 1;
            }
        }
        // comment out for part1
        else if self.a.x - self.b.x == self.a.y - self.b.y {
            let from = (self.a.x.min(self.b.x), self.a.y.min(self.b.y));
            let to = (self.a.x.max(self.b.x), self.a.y.max(self.b.y));
            for (x, y) in (from.0..=to.0).zip(from.1..=to.1) {
                *set.entry(Point { x: x, y: y }).or_insert(0) += 1;
            }
        } else if self.a.x - self.b.x == -(self.a.y - self.b.y) {
            let (from, to) = if self.a.x < self.b.x {
                (&self.a, &self.b)
            } else {
                (&self.b, &self.a)
            };
            for (x, y) in (from.x..=to.x).zip((to.y..=from.y).rev()) {
                *set.entry(Point { x: x, y: y }).or_insert(0) += 1;
            }
        }
    }
}

fn parse_lines() -> Vec<Line> {
    let s = include_str!("05.txt");
    s.lines()
        .filter(|s| *s != "")
        .map(|l| {
            let mut split = l.split(" ");
            let start = Point::parse(split.next().unwrap());
            split.next();
            let end = Point::parse(split.next().unwrap());
            Line { a: start, b: end }
        })
        .collect()
}

pub fn part1() {
    let lines = parse_lines();
    let mut map = HashMap::new();
    for line in lines {
        line.add_all(&mut map);
    }
    let doublehits = map.values().filter(|v| **v >= 2).count();
    println!("{}", doublehits);
}
