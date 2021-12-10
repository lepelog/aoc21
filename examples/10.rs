pub fn main() {
    parts(include_str!("10.txt"));
    // parts(include_str!("10example.txt"));
}

pub fn parts(s: &str) {
    let mut error_score = 0;
    let mut complete_error_scores = Vec::new();
    'outer: for line in s.split("\n") {
        let mut brace_stack = Vec::new();
        for chr in line.bytes() {
            match chr as char {
                '(' | '[' | '{' | '<' => brace_stack.push(chr),
                ')' => {
                    if brace_stack.pop() != Some('(' as u8) {
                        error_score += 3;
                        continue 'outer;
                    }
                }
                ']' => {
                    if brace_stack.pop() != Some('[' as u8) {
                        error_score += 57;
                        continue 'outer;
                    }
                }
                '}' => {
                    if brace_stack.pop() != Some('{' as u8) {
                        error_score += 1197;
                        continue 'outer;
                    }
                }
                '>' => {
                    if brace_stack.pop() != Some('<' as u8) {
                        error_score += 25137;
                        continue 'outer;
                    }
                }
                _ => unreachable!(),
            };
        }
        complete_error_scores.push(
            brace_stack
                .iter()
                .rev()
                .map(|b| match *b as char {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => unreachable!(),
                })
                .fold(0 as usize, |acc, val| acc * 5 + val as usize),
        );
    }
    println!("part1: {}", error_score);
    let middle = complete_error_scores.len() / 2;
    let (_, result, _) = complete_error_scores.select_nth_unstable(middle);
    println!("part2: {}", result);
}
