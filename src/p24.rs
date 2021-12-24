use cached::proc_macro::cached;
use itertools::Itertools;
use num_format::{Locale, ToFormattedString};
use rayon::prelude::*;
use std::time::{Duration, Instant};

#[derive(Hash, Clone, Debug, Eq, PartialEq, Copy)]
pub enum Op {
    Inp,
    Add,
    Mul,
    Div,
    Mod,
    Eql,
}

#[derive(Hash, Clone, Debug, Eq, PartialEq, Copy)]
pub enum Register {
    X,
    Y,
    W,
    Z,
}

#[derive(Hash, Clone, Debug, Eq, PartialEq, Copy)]
pub enum Arg {
    Register(Register),
    Value(i64),
}

#[derive(Hash, Clone, Debug, Eq, PartialEq)]
pub struct Instruction {
    op: Op,
    args: Vec<Arg>,
}

#[derive(Hash, Clone, Debug, Eq, PartialEq, Copy)]
pub struct State {
    w: i64,
    x: i64,
    y: i64,
    z: i64,
}

pub fn parse_op(s: &str) -> Op {
    match s {
        "inp" => Op::Inp,
        "add" => Op::Add,
        "mul" => Op::Mul,
        "div" => Op::Div,
        "mod" => Op::Mod,
        "eql" => Op::Eql,
        _ => unreachable!(),
    }
}

pub fn parse_arg(s: &str) -> Arg {
    match s {
        "w" => Arg::Register(Register::W),
        "x" => Arg::Register(Register::X),
        "y" => Arg::Register(Register::Y),
        "z" => Arg::Register(Register::Z),
        _ => Arg::Value(s.parse::<i64>().unwrap()),
    }
}

pub fn parse_instruction(s: &str) -> Instruction {
    let (ops, rest) = s.trim().split_once(" ").unwrap();
    let op = parse_op(ops);
    let args = rest.split(" ").map(parse_arg).collect();
    Instruction { op, args }
}

pub fn parse_data(s: &str) -> Vec<Instruction> {
    s.lines().map(parse_instruction).collect()
}

pub fn r_to_u(v: Arg) -> usize {
    match v {
        Arg::Register(Register::W) => 0,
        Arg::Register(Register::X) => 1,
        Arg::Register(Register::Y) => 2,
        Arg::Register(Register::Z) => 3,
        _ => unreachable!(),
    }
}

pub fn execute(state: State, ints: Vec<Instruction>, mut inputs: Vec<i64>) -> State {
    let mut state = [state.w, state.x, state.y, state.z];
    inputs.reverse();
    //println!("{:?}",state);
    for int in ints {
        //println!("{:?}", int);
        match int {
            Instruction { op: Op::Inp, args } => {
                state[r_to_u(args[0])] = inputs.pop().unwrap();
            }
            Instruction { op: Op::Add, args } => {
                let left = args[0];
                let right = if let Arg::Value(r) = args[1] {
                    r
                } else {
                    state[r_to_u(args[1])]
                };
                state[r_to_u(left)] += right;
            }
            Instruction { op: Op::Mul, args } => {
                let left = args[0];
                let right = if let Arg::Value(r) = args[1] {
                    r
                } else {
                    state[r_to_u(args[1])]
                };
                state[r_to_u(left)] *= right;
            }
            Instruction { op: Op::Div, args } => {
                let left = args[0];
                let right = if let Arg::Value(r) = args[1] {
                    r
                } else {
                    state[r_to_u(args[1])]
                };
                state[r_to_u(left)] = state[r_to_u(left)] / right;
            }
            Instruction { op: Op::Mod, args } => {
                let left = args[0];
                let right = if let Arg::Value(r) = args[1] {
                    r
                } else {
                    state[r_to_u(args[1])]
                };
                state[r_to_u(left)] = state[r_to_u(left)] % right;
            }
            Instruction { op: Op::Eql, args } => {
                let left = args[0];
                let right = if let Arg::Value(r) = args[1] {
                    r
                } else {
                    state[r_to_u(args[1])]
                };
                state[r_to_u(left)] = if state[r_to_u(left)] == right { 1 } else { 0 }
            }
            _ => unimplemented!(),
        }
        //println!("{:?}", state);
    }
    State {
        w: state[0],
        x: state[1],
        y: state[2],
        z: state[3],
    }
}

pub fn testing() {
    // let sample1 = "inp x
    // mul x -1";
    // let ints = parse_data(sample1);
    // assert_eq!(
    //     State {
    //         x: -1,
    //         y: 0,
    //         z: 0,
    //         w: 0
    //     },
    //     execute(ints.clone(), vec![1])
    // );
    // assert_eq!(
    //     State {
    //         x: 1,
    //         y: 0,
    //         z: 0,
    //         w: 0
    //     },
    //     execute(ints, vec![-1])
    // );

    // let sample2 = "inp z
    // inp x
    // mul z 3
    // eql z x";
    // let ints = parse_data(sample2);

    // assert_eq!(
    //     State {
    //         x: 2,
    //         y: 0,
    //         z: 0,
    //         w: 0
    //     },
    //     execute(ints.clone(), vec![1, 2])
    // );

    // assert_eq!(
    //     State {
    //         x: 9,
    //         y: 0,
    //         z: 1,
    //         w: 0
    //     },
    //     execute(ints.clone(), vec![3, 9])
    // );
    println!("Doing binsample");
    let binsample = "inp w
    add z w
    mod z 2
    div w 2
    add y w
    mod y 2
    div w 2
    add x w
    mod x 2
    div w 2
    mod w 2";
    let ints = parse_data(binsample);
    assert_eq!(
        State {
            x: 0,
            y: 0,
            z: 1,
            w: 1
        },
        execute(
            State {
                x: 0,
                y: 0,
                z: 0,
                w: 0
            },
            ints.clone(),
            vec![9]
        )
    );
}

pub fn get_sections(mints: Vec<Instruction>) -> Vec<Vec<Instruction>> {
    let mut sections: Vec<Vec<Instruction>> = vec![];
    let mut current: Vec<Instruction> = vec![mints[0].clone()];
    for i in 1..mints.len() {
        let mint = mints[i].clone();
        if mint
            == (Instruction {
                op: Op::Inp,
                args: vec![Arg::Register(Register::W)],
            })
        {
            sections.push(current);
            current = vec![mint];
        } else {
            current.push(mint);
        }
    }
    sections.push(current);
    sections
}

pub fn get_variables(sections: Vec<Vec<Instruction>>) -> Vec<(bool, i64, i64)> {
    let vs = sections
        .iter()
        .map(
            |section| match (section[4].args[1], section[5].args[1], section[15].args[1]) {
                (Arg::Value(x), Arg::Value(y), Arg::Value(z)) => (x == 26, y, z),
                _ => unreachable!(),
            },
        )
        .collect::<Vec<(bool, i64, i64)>>();
    vs
}

// what digits possible for previous z
// z needs to be less than 26
pub fn func(input: i64, previous_z: i64, variables: (bool, i64, i64)) -> i64 {
    let mut z = previous_z;

    if variables.0 {
        z = z / 26; //only way to become zero
    }

    if (previous_z % 26) + variables.1 != input {
        z *= 26; //not gonna become zero
        z += input + variables.2; // not gonna become zero either
    }

    z
}

pub fn test_same(sections: Vec<Vec<Instruction>>, vs: Vec<(bool, i64, i64)>) {
    for i in 0..sections.len() {
        for z in 1..1000 {
            for digit in 1..10 {
                let state = State {
                    x: 0,
                    y: 0,
                    w: 0,
                    z: z,
                };
                let execed = execute(state, sections[i].clone(), vec![digit]);
                let funced = func(digit, z, vs[i as usize].clone());
                assert_eq!(execed.z, funced);
            }
        }
    }
}

#[derive(Hash, Clone, Debug, Eq, PartialEq, Copy)]
struct Compute {
    digits: [u8; 14],
    z: i64,
}

pub fn main() {
    testing();

    let monad = include_str!("../inputs/day24puzzle.txt");
    let mints: Vec<Instruction> = parse_data(monad);

    let sections = get_sections(mints);
    let vs = get_variables(sections.clone());
    test_same(sections, vs.clone());

    for v in &vs {
        println!("{:?}", v);
    }

    let mut numbers = vec![Compute {
        z: 0,
        digits: [0; 14],
    }];
    let limit = (26 as i64).pow(4); //4?
    let limits = [7, 7, 7, 7, 7, 6, 6, 5, 5, 4, 3, 3, 2, 1];

    dbg!(limit);
    for i in 0..14 {
        let start = Instant::now();
        let new_limit = (26 as i64).pow(limits[i as usize]);

        println!(
            "step {}, testing {} of {}",
            i,
            numbers.len().to_formatted_string(&Locale::en),
            (9 as i64).pow(i).to_formatted_string(&Locale::en)
        );
        let mut new_numbers = vec![];
        for number in &numbers {
            let z_previous = if i == 0 { 0 } else { number.z };
            for digit in 1..=9 {
                let computed_z = func(digit, z_previous, vs[i as usize].clone());
                let mut new_number = number.clone();
                new_number.digits[i as usize] = digit as u8;
                new_number.z = computed_z;
                if computed_z <= new_limit {
                    new_numbers.push(new_number)
                }
            }
        }
        numbers = new_numbers;
        println!("Time elapsed in this step is: {:?}\n", start.elapsed());
    }
    for number in numbers.iter().rev() {
        if number.z == 0 {
            println!("{:?}", number.digits);
        }
    }

    // for d1 in 1..9 {
    //     let mut v = func(d1, 0, vs[0]);
    //     dbg!(v);
    //     for d2 in 1..9 {
    //         dbg!(v);
    //         v = func(d2, v, vs[1]);
    //         for d3 in 1..9 {
    //             dbg!(v);
    //             v = func(d3, v, vs[2]);
    //             for d4 in 1..9 {
    //                 v = func(d4, v, vs[3]);
    //                 for d5 in 1..9 {
    //                     v = func(d5, v, vs[4]);
    //                     for d6 in 1..9 {
    //                         v = func(d6, v, vs[5]);
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
}
