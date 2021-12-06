pub fn main() {
    part2();
}

pub fn part1() {
    let s = include_str!("06.txt");
    // let s = "3,4,3,1,2";
    let mut fishies: Vec<u8> = s
        .split(",")
        .filter_map(|s| s.trim().parse::<u8>().ok())
        .collect();
    for i in 0..80 {
        let mut new_fish_count: usize = 0;
        for fish in fishies.iter_mut() {
            if *fish == 0 {
                *fish = 6;
                new_fish_count += 1;
            } else {
                *fish -= 1;
            }
        }
        for _ in 0..new_fish_count {
            fishies.push(8);
        }
        println!("done loop {}", i);
    }
    println!("{}", fishies.len());
}

pub fn part2() {
    let s = include_str!("06.txt");
    let mut population: [usize; 9] = [0; 9];
    // let s = "3,4,3,1,2";
    for val in s.split(",").filter_map(|s| s.trim().parse::<u8>().ok()) {
        population[val as usize] += 1;
    }
    for _ in 0..256 {
        let prev0 = population[0];
        for i in 0..8 {
            population[i] = population[i + 1];
        }
        population[6] += prev0;
        population[8] = prev0;
    }
    let total_count: usize = population.iter().sum();
    println!("{}", total_count);
}
