use std::fmt;
use std::collections::HashMap;
use std::cmp::min;

#[derive(Clone, Copy)]
pub struct Bingo {
    values: [[usize; 5]; 5]
}

impl fmt::Debug for Bingo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = "\n".to_owned();
        let mut i = 0;
        for line in &self.values {
            for number in line {
                s += &format!("{} ", number).to_owned();
            }
            i+= 1;
            if i != 5 {
                s += "\n";
            }
        }
        return write!(f,"{}", s);
    }
}

pub fn get_data() -> (Vec<usize>, Vec<Bingo>) {
    let chunks: Vec<&str> = include_str!("../inputs/day4puzzle.txt").split("\n\n").collect();
    let numbers: Vec<usize> = chunks[0].split(",").map(|x| {x.parse::<usize>().unwrap()}).collect();
    let mut bingos: Vec<Bingo> = vec![];

    for number in &chunks[1..] {
        let mut values = [[0; 5]; 5];
        for (i, line) in number.lines().enumerate() {
            for (j, number) in line.split_ascii_whitespace().enumerate(){
                values[i][j] = number.parse::<usize>().unwrap();
            }
        }
        bingos.push(Bingo{values})
    }
    return (numbers, bingos);
}

pub fn make_number_to_round(numbers: Vec<usize>) -> HashMap<usize,usize> {
    let mut converter: HashMap<usize,usize> = HashMap::new();
    for (i, number) in numbers.iter().enumerate() {
        converter.insert(*number, i);
    }
    return converter;
}

const INFINITE_ROUND: usize = 666;

pub fn make_round_board(converter: HashMap<usize,usize>, bingo: Bingo) -> Bingo {
    let mut round_board = [[INFINITE_ROUND; 5] ; 5];
    for x in 0..5 {
        for y in 0..5 {
            round_board[x][y] = converter[&bingo.values[x][y]]
        }
    }
    return Bingo{values: round_board};
}

pub fn get_winning_round(round_board: Bingo) -> usize {
    let mut winning_round = INFINITE_ROUND;
    for x in 0..5 {
        let mut rows: Vec<usize> = vec![];
        let mut columns: Vec<usize> = vec![];
        for y in 0..5 {
            rows.push(round_board.values[x][y]);
            columns.push(round_board.values[y][x])
        }
        winning_round = min(winning_round, *rows.iter().max().unwrap());
        winning_round = min(winning_round, *columns.iter().max().unwrap());
    }
    return winning_round;

}

pub fn calculate_score(round: usize, numbers: Vec<usize>, round_board: Bingo, board: Bingo) -> usize {
    let mut leftovers:  usize = 0; 
    for x in 0..5 {
        for y in 0..5 {
            if round_board.values[x][y] > round {
                leftovers += board.values[x][y];
            } 
        }
    }
    dbg!(leftovers);
    dbg!(leftovers * numbers[round]);
    return leftovers * numbers[round]; 
}

pub fn main() {
    let (numbers, bingos) = get_data();
    let converter = make_number_to_round(numbers.clone());

    let mut first_board = Bingo{values: [[INFINITE_ROUND; 5]; 5]};
    let mut first_round_board = Bingo{values: [[INFINITE_ROUND; 5]; 5]};
    let mut first_round = INFINITE_ROUND;
    for bingo in bingos.clone() {
        let round_board = make_round_board(converter.clone(),bingo);
        let round = get_winning_round(round_board);
        if round < first_round {
            first_board = bingo; 
            first_round_board = round_board;
            first_round = round;
        }
    }

    dbg!(calculate_score(first_round, numbers.clone(), first_round_board, first_board));

    let mut last_board = Bingo{values: [[INFINITE_ROUND; 5]; 5]};
    let mut last_round_board = Bingo{values: [[INFINITE_ROUND; 5]; 5]};
    let mut last_round = 1;
    for bingo in bingos {
        let round_board = make_round_board(converter.clone(),bingo);
        let round = get_winning_round(round_board);
        if round > last_round {
            last_board = bingo; 
            last_round_board = round_board;
            last_round = round;
        }
    }

    dbg!(calculate_score(last_round, numbers.clone(), last_round_board, last_board));
}