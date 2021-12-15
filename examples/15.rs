use std::collections::HashMap;

pub fn main() {
    // let s = include_str!("15example.txt");
    let s = include_str!("15.txt");
    let puzzle = parse_grid(s);
    part1(&puzzle);
    part2(&puzzle);
}

fn part1(puzzle: &Grid<u8>) {
    // find_path_try2(&puzzle);
    find_path_try3(&puzzle);
}

#[inline(always)]
fn weird_mod(val: u8) -> u8 {
    ((val - 1) % 9) + 1
}

fn part2(puzzle: &Grid<u8>) {
    let mut new_puzzle = Grid::new(puzzle.width * 5, puzzle.height * 5, 0);
    for x in 0..puzzle.width {
        for y in 0..puzzle.height {
            let val = *puzzle.get(x, y).unwrap();
            for ix in 0..5u8 {
                for iy in 0..5u8 {
                    *new_puzzle.get_mut(x + puzzle.width * ix as isize, y + puzzle.height * iy as isize).unwrap() = weird_mod(val + ix + iy);
                }
            }
        }
    }
    find_path_try3(&new_puzzle);
}

pub struct Grid<T> {
    data: Vec<T>,
    width: isize,
    height: isize,
}

fn parse_grid(s: &str) -> Grid<u8> {
    let mut data_buf = Vec::with_capacity(s.len());
    let mut current_width = 0;
    let mut known_width = None;
    let mut height = 0;
    for b in s.bytes() {
        if b == '\n' as u8 {
            match known_width {
                None => known_width = Some(current_width),
                Some(width) => assert_eq!(width, current_width),
            };
            current_width = 0;
            height += 1;
        } else {
            data_buf.push(b - '0' as u8);
            current_width += 1;
        }
    }
    if current_width != 0 {
        assert_eq!(Some(current_width), known_width);
        height += 1;
    }
    Grid {data: data_buf, width: known_width.unwrap(), height}
}

fn calc_grid_path_sum(grid: &Grid<u8>, turns: &Vec<TakenTurn>) -> usize {
    let mut result: usize = 0;
    let mut pos = (0,0);
    for turn in turns.iter() {
        pos = turn.forward_pos(&pos);
        result += *grid.get(pos.0, pos.1).unwrap() as usize;
    }
    result
}

impl <T: Copy> Grid<T> {
    fn new(width: isize, height: isize, startval: T) -> Self{
        Grid {
            data: vec![startval; (width * height) as usize],
            width,
            height,
        }
    }

    fn is_inbounds_turn_pos(&self, turn: TakenTurn, pos: &(isize, isize)) -> bool {
        self.is_inbounds_pos(&turn.forward_pos(pos))
    }

    #[inline(always)]
    fn is_inbounds_pos(&self, pos: &(isize, isize)) -> bool {
        self.is_inbounds(pos.0, pos.1)
    }

    #[inline(always)]
    fn is_inbounds(&self, x: isize, y: isize) -> bool {
        !(x < 0 || y < 0 || x >= self.width || y >= self.height)
    }

    fn get(&self, x: isize, y: isize) -> Option<&T> {
        if self.is_inbounds(x, y) {
            return self.data.get((y * self.width + x) as usize);
        } else {
            return None;
        }
    }

    fn get_mut(&mut self, x: isize, y: isize) -> Option<&mut T> {
        if self.is_inbounds(x, y) {
            return self.data.get_mut((y * self.width + x) as usize);
        } else {
            return None;
        }
    }

    fn get_inbound_neighbors<'a>(&'a self, pos: &'a (isize, isize)) -> impl Iterator<Item = (isize, isize)> + 'a{
        [(0,-1), (0,1), (-1, 0), (1, 0)].into_iter().map(|mov| (mov.0 + pos.0, mov.1 + pos.1)).filter(|p| self.is_inbounds_pos(p))
    }
}

// pls tell me this is enough
#[derive(Copy, Clone, Debug)]
enum TakenTurn {
    Right,
    Down,
}

impl TakenTurn {
    fn forward_pos(&self, pos: &(isize, isize)) -> (isize, isize) {
        self.forward(pos.0, pos.1)
    }
    fn forward(&self, x: isize, y: isize) -> (isize, isize) {
        match self {
            TakenTurn::Right => (x + 1, y),
            TakenTurn::Down => (x, y + 1),
        }
    }
    fn backward_pos(&self, pos: &(isize, isize)) -> (isize, isize) {
        self.backward(pos.0, pos.1)
    }
    fn backward(&self, x: isize, y: isize) -> (isize, isize) {
        match self {
            TakenTurn::Right => (x - 1, y),
            TakenTurn::Down => (x, y - 1),
        }
    }
}

pub fn find_path_try1(puzzle: &Grid<u8>) {
    // does probably work in theory but 2^198 loops lol
    let mut path_taken: Vec<TakenTurn> = Vec::with_capacity((puzzle.width + puzzle.height) as usize);
    let mut min_path = None;
    let mut min_cost = usize::MAX;
    let start = (0,0);
    let end = (puzzle.width - 1, puzzle.height - 1);
    let mut pos = start.clone();
    let mut loop_count = 0;
    'out: loop {
        loop_count += 1;
        if (loop_count % 0x100000) == 0 {
            println!("{}, {}, {:?}", loop_count, min_cost, min_path);
        }
        // first go to the end
        // always try to go right, if that doesn't work go down
        // if that also doesn't work I messed up
        while pos != end {
            let potential_next = TakenTurn::Right.forward_pos(&pos);
            if puzzle.is_inbounds_pos(&potential_next) {
                path_taken.push(TakenTurn::Right);
                pos = potential_next;
                continue;
            }
            let potential_next = TakenTurn::Down.forward_pos(&pos);
            if puzzle.is_inbounds_pos(&potential_next) {
                path_taken.push(TakenTurn::Down);
                pos = potential_next;
                continue;
            }
            panic!("uhm? {:?}", pos);
        }
        let current_sum = calc_grid_path_sum(&puzzle, &path_taken);
        if current_sum < min_cost {
            min_cost = current_sum;
            min_path = Some(path_taken.clone());
        }
        // now rewind the path until we can make a decision again
        // if we went right, try down now
        loop {
            match path_taken.pop() {
                Some(TakenTurn::Right) => {
                    pos = TakenTurn::Right.backward_pos(&pos);
                    if puzzle.is_inbounds_turn_pos(TakenTurn::Down, &pos) {
                        path_taken.push(TakenTurn::Down);
                        pos = TakenTurn::Down.forward_pos(&pos);
                        continue 'out; // time to rebuild the path again
                    }
                },
                Some(TakenTurn::Down) => {
                    pos = TakenTurn::Down.backward_pos(&pos);
                },
                None => {
                    break 'out;
                }
            };
        }
    }
    println!("puzzle 1: {}", min_cost);
}

pub fn find_path_try2(puzzle: &Grid<u8>) {
    let mut node_costs = HashMap::with_capacity((puzzle.height * puzzle.width) as usize);
    let mut unvisited_nodes = Vec::with_capacity((puzzle.height * puzzle.width) as usize);
    for x in 0..puzzle.width {
        for y in 0..puzzle.height {
            node_costs.insert((x,y), usize::MAX);
            unvisited_nodes.push((x,y));
        }
    }
    node_costs.insert((0,0), 0);
    while unvisited_nodes.len() > 0 {
        // find unvisited node with min cost
        let (idx, cost) = unvisited_nodes.iter().map(|node| *node_costs.get(node).unwrap()).enumerate().min_by_key(|(_, cost)| *cost).unwrap();
        // mark node as visited
        let min_node = unvisited_nodes.swap_remove(idx);
        // update neighbors
        for neighbor in puzzle.get_inbound_neighbors(&min_node) {
            let cur_cost = node_costs.get_mut(&neighbor).unwrap();
            // when this cost plus cost to get there is less, overwrite
            let maybe_less = (*cur_cost).min(cost + *puzzle.get(neighbor.0, neighbor.1).unwrap() as usize);
            if maybe_less < *cur_cost {
                *cur_cost = maybe_less;
            }
        }
    }
    println!("{:?}", node_costs.get_mut(&(puzzle.width - 1, puzzle.height - 1)));
}

fn find_path_try3(puzzle: &Grid<u8>) {
    let mut cost_grid = Grid::new(puzzle.width, puzzle.height, usize::MAX);
    *cost_grid.get_mut(0, 0).unwrap() = 0;
    for x in 0..puzzle.width {
        for y in 0..puzzle.height {
            if x == 0 && y == 0 {
                continue;
            }
            let left_cost = if puzzle.is_inbounds(x - 1, y) {
                *cost_grid.get(x - 1, y).unwrap() + *puzzle.get(x, y).unwrap() as usize
            } else {
                usize::MAX
            };
            let top_cost = if puzzle.is_inbounds(x, y - 1) {
                *cost_grid.get(x, y - 1).unwrap() + *puzzle.get(x, y).unwrap() as usize
            } else {
                usize::MAX
            };
            *cost_grid.get_mut(x, y).unwrap() = left_cost.min(top_cost);
        }
    }
    optimize_cost_solution(puzzle, &mut cost_grid);
    // 1: 456
    // 2: 2831
    println!("{:?}", cost_grid.get_mut(puzzle.width - 1, puzzle.height - 1));
}

fn optimize_cost_solution(puzzle: &Grid<u8>, cost_grid: &mut Grid<usize>) {
    let mut queue = Vec::new();
    for x in 0..puzzle.width {
        for y in 0..puzzle.height {
            for neighbor in puzzle.get_inbound_neighbors(&(x,y)) {
                let neighbor_cost = *cost_grid.get(neighbor.0, neighbor.1).unwrap();
                let cur_cost = cost_grid.get_mut(x, y).unwrap();
                let this_cost = *puzzle.get(x, y).unwrap();
                if *cur_cost > neighbor_cost + this_cost as usize {
                    *cur_cost = neighbor_cost + this_cost as usize;
                    queue.push((x,y));
                }
            }
        }
    }
    while let Some((x,y)) = queue.pop() {
        for neighbor in puzzle.get_inbound_neighbors(&(x,y)) {
            let cur_cost = *cost_grid.get(x, y).unwrap();
            let neighbor_cost = cost_grid.get_mut(neighbor.0, neighbor.1).unwrap();
            let neightbor_add = *puzzle.get(neighbor.0, neighbor.1).unwrap();
            if *neighbor_cost > cur_cost + neightbor_add as usize {
                *neighbor_cost = cur_cost + neightbor_add as usize;
                queue.push((neighbor.0, neighbor.1));
            }
        }
    }
}
