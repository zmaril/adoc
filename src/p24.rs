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
    Value(i32),
}

#[derive(Hash, Clone, Debug, Eq, PartialEq)]
pub struct Instruction {
    op: Op,
    args: Vec<Arg>,
}

#[derive(Hash, Clone, Debug, Eq, PartialEq, Copy)]
pub struct State {
    w: i32,
    x: i32,
    y: i32,
    z: i32,
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
        _ => Arg::Value(s.parse::<i32>().unwrap()),
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

pub fn execute(ints: Vec<Instruction>, mut inputs: Vec<i32>) -> State {
    let mut state = [0; 4];
    inputs.reverse();
    for int in ints {
        match int {
            Instruction { op: Op::Inp, args } => {
                state[r_to_u(args[0])] = inputs.pop().unwrap();
            },
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
    let binsample ="inp w
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
            w:1 
        },
        execute(ints.clone(), vec![9])
    );
}

pub fn main() {
    testing();
}
