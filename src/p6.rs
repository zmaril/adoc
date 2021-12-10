const SAMPLE: &str = "3,4,3,1,2";
const PUZZLE: &str = "5,1,5,3,2,2,3,1,1,4,2,4,1,2,1,4,1,1,5,3,5,1,5,3,1,2,4,4,1,1,3,1,1,3,1,1,5,1,5,4,5,4,5,1,3,2,4,3,5,3,5,4,3,1,4,3,1,1,1,4,5,1,1,1,2,1,2,1,1,4,1,4,1,1,3,3,2,2,4,2,1,1,5,3,1,3,1,1,4,3,3,3,1,5,2,3,1,3,1,5,2,2,1,2,1,1,1,3,4,1,1,1,5,4,1,1,1,4,4,2,1,5,4,3,1,2,5,1,1,1,1,2,1,5,5,1,1,1,1,3,1,4,1,3,1,5,1,1,1,5,5,1,4,5,4,5,4,3,3,1,3,1,1,5,5,5,5,1,2,5,4,1,1,1,2,2,1,3,1,1,2,4,2,2,2,1,1,2,2,1,5,2,1,1,2,1,3,1,3,2,2,4,3,1,2,4,5,2,1,4,5,4,2,1,1,1,5,4,1,1,4,1,4,3,1,2,5,2,4,1,1,5,1,5,4,1,1,4,1,1,5,5,1,5,4,2,5,2,5,4,1,1,4,1,2,4,1,2,2,2,1,1,1,5,5,1,2,5,1,3,4,1,1,1,1,5,3,4,1,1,2,1,1,3,5,5,2,3,5,1,1,1,5,4,3,4,2,2,1,3";

const DAYS: usize = 9;

type Fishtank = [usize; DAYS];

pub fn parse_data(data: &str) -> Fishtank {
    let mut counter = [0; DAYS];
    let counts = data.split(",").map(|x| {x.parse::<usize>().unwrap()});
    for count in counts {
        counter[count] += 1;
    }
    return counter;
}

pub fn step(fish: Fishtank) -> Fishtank {
    let mut next = [0; DAYS];
    for i in 1..DAYS {
        next[i-1] = fish[i]
    }
    next[6] += fish[0];
    next[8] += fish[0];
    return next;
}

pub fn main () {
    let mut data = parse_data(PUZZLE);
    for _ in 0..256{ 
        println!("{:?}", data);
        data = step(data);
    }
    println!("{:?}", data);
    println!("{}", data.iter().sum::<usize>());
}