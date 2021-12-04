use std::fs;
use core::cmp::max;

static reading_len: usize = 12; 

pub fn day_three_input() -> Vec<Vec<u32>> {
    let contents = fs::read_to_string("inputs/day3puzzle1.txt")
    //let contents = fs::read_to_string("inputs/day3sample.txt")
    .expect("Something went wrong reading the file");
    let lines = contents.lines();
    let ints = lines.map(|line| {
        return line.chars().map(|v| {
            if v == '1' {
                return 1;
            }
            else {
                return 0;
            }
        }).collect();
    }).collect();
    return ints;
}

pub fn get_greeks(ints: Vec<Vec<u32>>) -> (Vec<u32>, Vec<u32>) {
    let mut sums : Vec<u32> = vec![];
    for i in 0..reading_len {
        let mut index_sum = 0;
        for b in ints.clone() {
            index_sum += b[i]
        }
        sums.push(index_sum)
    }
    println!("Ints: {:?}", ints);
    println!("Sums: {:?}",sums);
    println!("Len: {}", ints.len());
    let halfway: f32 = ((ints.len() as f32)/2.0) as f32;

    let mut gamma: Vec<u32> = vec![];
    let mut epsilon: Vec<u32> = vec![];
    for s in sums {
        let sf = s as f32;
        println!("{} {}", sf, halfway);
        if sf >= halfway  {
            gamma.push(1);
            epsilon.push(0);
        }
        else  {
            gamma.push(0);
            epsilon.push(1);
        }
    }
    return (gamma, epsilon);
}

pub fn binarray_to_int(arr: Vec<u32>) -> isize {
    let mut s = "".to_owned(); 
    for a in arr {
        s.push_str(&a.to_string());
    }
    let i = isize::from_str_radix(&s,2).unwrap();
    return i;
}

pub fn d3p1() {
    let ints = day_three_input().clone();
    let (gamma, epsilon) = get_greeks(ints);
    //println!("{:?}", ints.clone());
    println!("Gamma: {:?}", gamma.clone());
    println!("Epsilon: {:?}", epsilon.clone());
    let gi = binarray_to_int(gamma);
    let ei = binarray_to_int(epsilon);
    println!("Gamma: {}", gi);
    println!("Epsilon: {}", ei);
    println!("Together: {}", ei*gi);
}

pub fn d3p2() {
    let ints = day_three_input().clone();
    //  
    let mut oxys = ints.clone();
    for i in 0..reading_len {
        let gamma = get_greeks(oxys.clone()).0; 
        let mut new_oxys = vec![];
        println!("Gamma: {:?}", gamma);
        println!("Should be {} at {}", gamma[i], i);
        for oxy in oxys.clone() {
            if gamma[i] == oxy[i] {
                new_oxys.push(oxy)
            }
        }
        if new_oxys.len() == 0 {
            break;
        }
        oxys = new_oxys;
        println!("{:?}", oxys);
        println!("");
        if oxys.len() == 1 {
            break;
        }
    }

    println!("Oxys: {:?}", oxys);
    let first_oxys = &oxys[0];
    println!("First oxys: {:?}\n", first_oxys);

    let mut scrubbers = ints.clone();
    for i in 0..reading_len {
        let epsilon = get_greeks(scrubbers.clone()).1; 
        let mut new_scrubbers = vec![];
        println!("Epi: {:?}", epsilon);
        println!("Should be {} at {}", epsilon[i], i);
        for scrubber in scrubbers.clone() {
            if epsilon[i] == scrubber[i] {
                new_scrubbers.push(scrubber)
            }
        }
        if new_scrubbers.len() == 0 {
            break;
        }
        scrubbers = new_scrubbers;
        println!("{:?}", scrubbers);
        println!("");
        if scrubbers.len() == 1 {
            break;
        }
    }

    println!("{:?}", oxys);
    let first_oxy = &oxys[0];
    println!("First oxys: {:?}\n", first_oxy);
    let oi = binarray_to_int(first_oxy.to_vec());
    println!("First oxys: {:?}\n", oi);

    println!("{:?}", scrubbers);
    let first_scruber = &scrubbers[0];
    println!("First scrubber: {:?}\n", first_scruber);
    let si = binarray_to_int(first_scruber.to_vec());
    println!("First scrubber: {:?}\n", si);

    println!("Together: {}", oi*si);
}
