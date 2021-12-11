pub fn main() {
    parts();
}

struct Cell {
    level: u8,
    flashing: bool,
}

fn parse_input() -> Vec<Vec<Cell>> {
    // let s = include_str!("11example.txt");
    let s = include_str!("11.txt");
    s.trim()
        .split('\n')
        .map(|line| {
            line.bytes()
                .map(|b| Cell {
                    level: b - '0' as u8,
                    flashing: false,
                })
                .collect()
        })
        .collect()
}

fn parts() {
    // (value, bool set to true when the cell is processed)
    let mut puzzle = parse_input();
    let mut flashes_to_process: Vec<(usize, usize)> = Vec::new();
    let mut flashes_count = 0;
    let width = puzzle.len() as isize;
    let height = puzzle[0].len() as isize;
    let mut first_sync_flash = None;
    for i in 1.. {
        // reset flashes
        // increase energy level by 1
        for line in puzzle.iter_mut() {
            for cell in line.iter_mut() {
                cell.flashing = false;
                cell.level += 1;
            }
        }
        // check for flashes
        for x in 0..(width as usize) {
            for y in 0..(height as usize) {
                let mut current_cell = &mut puzzle[x][y];
                if !current_cell.flashing && current_cell.level > 9 {
                    flashes_count += 1;
                    current_cell.flashing = true;
                    current_cell.level = 0;
                    let other_cells = [
                        (-1, -1),
                        (-1, 0),
                        (-1, 1),
                        (0, -1),
                        (0, 1),
                        (1, -1),
                        (1, 0),
                        (1, 1),
                    ]
                    .into_iter()
                    .map(|(x_off, y_off)| (x as isize + x_off, y as isize + y_off))
                    .filter(|(x, y)| *x >= 0 && *y >= 0 && *x < width && *y < height);
                    for (other_x, other_y) in other_cells {
                        let other_x = other_x as usize;
                        let other_y = other_y as usize;
                        let mut other_cell = &mut puzzle[other_x][other_y];
                        if !other_cell.flashing {
                            other_cell.level += 1;
                            if other_cell.level > 9 {
                                flashes_count += 1;
                                other_cell.flashing = true;
                                other_cell.level = 0;
                                flashes_to_process.push((other_x, other_y));
                            }
                        }
                    }
                }
            }
        }
        // check for flashes triggered by flashes
        while let Some((x, y)) = flashes_to_process.pop() {
            // cell should already be 0
            assert_eq!(puzzle[x][y].level, 0);
            let other_cells = [
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ]
            .into_iter()
            .map(|(x_off, y_off)| (x as isize + x_off, y as isize + y_off))
            .filter(|(x, y)| *x >= 0 && *y >= 0 && *x < width && *y < height);
            for (other_x, other_y) in other_cells {
                let other_x = other_x as usize;
                let other_y = other_y as usize;
                let mut other_cell = &mut puzzle[other_x][other_y];
                if !other_cell.flashing {
                    other_cell.level += 1;
                    if other_cell.level > 9 {
                        flashes_count += 1;
                        other_cell.flashing = true;
                        other_cell.level = 0;
                        flashes_to_process.push((other_x, other_y));
                    }
                }
            }
        }
        if first_sync_flash.is_none() {
            if puzzle
                .iter()
                .all(|line| line.iter().all(|cell| cell.flashing))
            {
                first_sync_flash = Some(i);
            }
        }
        if i == 100 {
            println!("part1: {}", flashes_count);
        }
        if i >= 100 && first_sync_flash.is_some() {
            break;
        }
    }
    println!("part2: {:?}", first_sync_flash);
}
