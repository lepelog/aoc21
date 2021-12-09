use std::collections::{BinaryHeap, HashSet};

pub fn main() {
    part1();
    part2();
}

pub fn parse_puzzle() -> Vec<Vec<u8>> {
    let s = include_str!("09.txt");
    // let s = include_str!("09example.txt");
    let mut rows = Vec::new();
    let mut current_row = Vec::new();
    for b in s.bytes() {
        if b == '\n' as u8 {
            rows.push(current_row);
            current_row = Vec::new();
        } else {
            current_row.push(b - '0' as u8);
        }
    }
    assert_eq!(current_row.len(), 0);
    rows
}

pub fn part1() {
    let rows = parse_puzzle();
    let mut total_hazard: usize = 0;
    for i in 0..rows.len() {
        let cur_row = &rows[i];
        for j in 0..cur_row.len() {
            let cur_val = cur_row[j];
            if j > 0 && cur_val >= cur_row[j - 1] {
                continue;
            }
            if i > 0 && cur_val >= rows[i - 1][j] {
                continue;
            }
            if j < cur_row.len() - 1 && cur_val >= cur_row[j + 1] {
                continue;
            }
            if i < rows.len() - 1 && cur_val >= rows[i + 1][j] {
                continue;
            }
            // println!("{}, {}, {}", i, j, cur_val);
            total_hazard += cur_val as usize + 1;
        }
    }
    println!("part1: {}", total_hazard);
}

pub fn get_basin(start: (usize, usize), rows: &Vec<Vec<u8>>) -> HashSet<(usize, usize)> {
    let mut to_check = Vec::new();
    to_check.push(start);
    let mut checked = HashSet::new();
    while let Some((x, y)) = to_check.pop() {
        if checked.contains(&(x, y)) {
            continue;
        }
        // we can grow as much as we want, 9 or the border are the only barriers
        if y > 0 && rows[x][y - 1] != 9 {
            let potential_pos = (x, y - 1);
            if !checked.contains(&potential_pos) {
                to_check.push(potential_pos);
            }
        }
        if x > 0 && rows[x - 1][y] != 9 {
            let potential_pos = (x - 1, y);
            if !checked.contains(&potential_pos) {
                to_check.push(potential_pos);
            }
        }
        if y < rows[x].len() - 1 && rows[x][y + 1] != 9 {
            let potential_pos = (x, y + 1);
            if !checked.contains(&potential_pos) {
                to_check.push(potential_pos);
            }
        }
        if x < rows.len() - 1 && rows[x + 1][y] != 9 {
            let potential_pos = (x + 1, y);
            if !checked.contains(&potential_pos) {
                to_check.push(potential_pos);
            }
        }
        checked.insert((x, y));
    }
    checked
}

pub fn part2() {
    let rows = parse_puzzle();
    let mut max_basin_sizes = BinaryHeap::new();
    let mut already_visited = HashSet::new();
    for i in 0..rows.len() {
        let cur_row = &rows[i];
        for j in 0..cur_row.len() {
            if !already_visited.contains(&(i, j)) && cur_row[j] != 9 {
                let basin = get_basin((i, j), &rows);
                max_basin_sizes.push(basin.len());
                already_visited.extend(basin.into_iter());
            }
        }
    }
    let mut result: usize = 1;
    for _ in 0..3 {
        result *= max_basin_sizes.pop().unwrap_or(0);
    }
    println!("part2: {}", result);
}
