pub fn main() {
    part2();
}

pub struct BingoCard {
    pub nums: [u8; 25],
    pub crossed_out: [bool; 25],
}

impl BingoCard {
    pub fn reset(&mut self) {
        self.crossed_out = [false; 25];
    }

    pub fn cross_out(&mut self, crossed: u8) {
        for (i, num) in self.nums.iter().enumerate() {
            if *num == crossed {
                self.crossed_out[i] = true;
            }
        }
    }

    pub fn has_bingo(&self) -> bool {
        // test rows
        for row in 0..5 {
            if self.crossed_out[row*5..(row+1)*5].iter().all(|s| *s) {
                return true;
            }
        }
        // test columns
        for column in 0..5 {
            if self.crossed_out[column] &&
            self.crossed_out[column+5] &&
            self.crossed_out[column+10] &&
            self.crossed_out[column+15] &&
            self.crossed_out[column+20] {
                return true;
            }
        }
        return false;
    }

    pub fn unmarked_sum(&self) -> usize {
        self.nums.iter().zip(self.crossed_out.iter()).filter_map(|(num, cro)| {
            if *cro {
                None
            } else {
                Some(*num as usize)
            }
        }).sum()
    }
}

pub fn read_puzzle() -> (Vec<u8>, Vec<BingoCard>) {
    let s = include_str!("04.txt");
    let guesses: Vec<u8> = s.lines().nth(0).unwrap().split(',').map(|s| s.parse::<u8>().unwrap()).collect();
    let mut line_itr = s.lines().skip(2).peekable();
    let mut bingo_cards: Vec<BingoCard> = Vec::with_capacity(1000);
    loop {
        let mut current_card = BingoCard { nums: [0; 25], crossed_out: [false; 25] };
        let row1 = line_itr.next().unwrap().split(' ').filter_map(|s| s.trim().parse::<u8>().ok());
        let row2 = line_itr.next().unwrap().split(' ').filter_map(|s| s.trim().parse::<u8>().ok());
        let row3 = line_itr.next().unwrap().split(' ').filter_map(|s| s.trim().parse::<u8>().ok());
        let row4 = line_itr.next().unwrap().split(' ').filter_map(|s| s.trim().parse::<u8>().ok());
        let row5 = line_itr.next().unwrap().split(' ').filter_map(|s| s.trim().parse::<u8>().ok());
        for (i, val) in row1.chain(row2).chain(row3).chain(row4).chain(row5).enumerate() {
            current_card.nums[i] = val;
        }
        bingo_cards.push(current_card);
        line_itr.next();
        if line_itr.peek() == None {
            return (guesses, bingo_cards);
        }
    }
}

pub fn part1() {
    let (guesses, mut bingo_cards) = read_puzzle();
    for guess in guesses.iter() {
        for card in bingo_cards.iter_mut() {
            card.cross_out(*guess);
            if card.has_bingo() {
                let unmarked_sum = card.unmarked_sum();
                println!("{}, {}, {}", guess, unmarked_sum, *guess as usize * unmarked_sum);
                return;
            }
        }
    }
}

pub fn part2() {
    let (guesses, mut bingo_cards) = read_puzzle();
    for guess in guesses.iter() {
        let mut i = 0;
        while i < bingo_cards.len() {
            bingo_cards[i].cross_out(*guess);
            if bingo_cards[i].has_bingo() {
                if bingo_cards.len() == 1 {
                    let unmarked_sum = bingo_cards[i].unmarked_sum();
                    println!("{}, {}, {}", guess, unmarked_sum, *guess as usize * unmarked_sum);
                    return;
                }
                bingo_cards.swap_remove(i);
            } else {
                i+=1;
            }
        }
    }
}
