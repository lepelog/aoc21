use std::collections::HashMap;

pub fn main() {
    part1();
}

#[derive(Debug)]
struct Node {
    name: String,
    connections: Vec<String>,
}

#[derive(Debug)]
struct Maze {
    nodes: HashMap<String, Node>,
}

struct SearchState {
    visited: Vec<String>,
    count: usize,
    double_allowed: Option<String>,
}

impl SearchState {
    fn new() -> Self {
        Self::with_double_allowed(None)
    }
    fn with_double_allowed(double_allowed: Option<String>) -> Self {
        SearchState {
            visited: Vec::new(),
            count: 0,
            double_allowed,
        }
    }
}

impl Maze {
    fn new() -> Self {
        Maze {
            nodes: HashMap::new(),
        }
    }

    fn add_connection(&mut self, a: &str, b: &str) {
        let node = self.nodes.entry(a.into()).or_insert(Node {
            name: a.into(),
            connections: Vec::new(),
        });
        node.connections.push(b.into());
    }

    fn find_all_paths(&self) -> usize {
        let mut state = SearchState::new();
        self.find_subroutine("start", &mut state);
        state.count
    }

    fn find_all_doubles_allowed(&self) -> usize {
        let mut total_count = 0;
        for small_cave in self.nodes.keys().filter(|cave| {
            // not start, not end, has to be lowercase
            *cave != "start" && *cave != "end" && cave.bytes().nth(0).unwrap() > 90
        }) {
            let mut state = SearchState::with_double_allowed(Some(small_cave.into()));
            self.find_subroutine("start", &mut state);
            total_count += state.count;
        }
        total_count
    }

    fn find_subroutine(&self, node: &str, state: &mut SearchState) {
        // go info all nodes from here, that haven't been visited
        let node = self.nodes.get(node).unwrap();
        for con_str in node.connections.iter() {
            if con_str == "start" {
                continue;
            }
            if con_str == "end" {
                if let Some(dc) = state.double_allowed.as_ref() {
                    // if this was a search with a double allowed,
                    // we can only count this if the cave appeared twice
                    // otherwise this cave has already been counted when no
                    // double caves were allowed
                    let visited_count = state.visited.iter().filter(|v| *v == dc).count();
                    if visited_count == 2 {
                        state.count += 1;
                    } else if visited_count > 2 {
                        panic!("count greater than 2, how did that happen???");
                    }
                } else {
                    state.count += 1;
                };
                continue;
            }
            // check if already visited, or if only visited once when
            let visited_count = state.visited.iter().filter(|v| *v == con_str).count();

            // if it has already been visited once, that's fine when this is the
            // cave we can visit twice this round
            if visited_count == 0
                || (visited_count == 1
                    && state
                        .double_allowed
                        .as_ref()
                        .filter(|dc| *dc == con_str)
                        .is_some())
            {
                // check for upper case
                let can_visit_again = con_str.bytes().nth(0).unwrap() < 90;
                if !can_visit_again {
                    state.visited.push(con_str.into());
                }
                self.find_subroutine(con_str, state);
                if !can_visit_again {
                    state.visited.pop();
                }
            }
        }
    }
}

fn parse_input() -> Maze {
    // let s = include_str!("12example.txt");
    let s = include_str!("12.txt");
    let raw_conns = s
        .trim()
        .split('\n')
        .map(|line| line.split_once('-').unwrap());
    let mut maze = Maze::new();
    for (a, b) in raw_conns {
        maze.add_connection(a, b);
        maze.add_connection(b, a);
    }
    maze
}

pub fn part1() {
    let maze = parse_input();
    // println!("{:?}", maze);
    let count = maze.find_all_paths();
    println!("part1: {}", count);
    let double_allowed_count = maze.find_all_doubles_allowed();
    println!("part2: {}", double_allowed_count + count);
}
