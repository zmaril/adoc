pub fn get_data() -> Vec<&'static str> {
    return include_str!("../inputs/day10puzzle.txt").lines().collect::<Vec<&str>>();
}

#[derive(Debug)]
pub struct ParseResult {
    error: Option<(usize, char)>,
    leftover: Option<Vec<char>>
}

pub fn matched(left: char, right: char) -> bool {
    return matches!((left,right), ('(',')') | ('[',']') | ('{','}') | ('<','>'));
}

pub fn parse_line(line: &str) -> ParseResult {
    let mut stack: Vec<char> = vec![];
    let mut i = 0;
    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => {
                stack.push(c);
                i+=1;
            },
            ')' | ']' | '}' | '>' => {
                let last = stack.pop().unwrap();
                if !matched(last, c){
                    return ParseResult{error: Some((i, c)), leftover: None};
                }
            },
            _ => {
                dbg!(c);
                unreachable!()
            }
        }
    }
    return ParseResult{error: None, leftover: Some(stack)};
}

pub fn main() {
    let lines = get_data();
    let mut score_a = 0;
    let mut score_b: Vec<usize> = vec![];
    for line in lines {
        println!("{}", line.clone());
        let r = parse_line(line); 
        match r {
            ParseResult{error: Some((i, c)), leftover: _} => {
                println!("Error at {}: {}", i, c);
                match c {
                    ')' => score_a += 3,
                    ']' => score_a += 57,
                    '}' => score_a += 1197,
                    '>' => score_a += 25137,
                    _ => unreachable!()
                }
            },
            ParseResult{error: None, leftover: stack} => {
                let mut local_score: usize = 0;
                for s in stack.unwrap().iter().rev() {
                    let v = match s {
                        '(' => 1, 
                        '[' => 2, 
                        '{' => 3,
                        '<' => 4,
                        _ => unreachable!()
                    };
                    local_score = v + (local_score * 5);
                }

                dbg!(local_score);
                score_b.push(local_score);
            }
        }
        println!("");
    }
    dbg!(score_a);
    score_b.sort();
    dbg!(score_b[score_b.len()/2]);
}