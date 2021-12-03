use std::fs;
use core::cmp::max;

static reading_len: usize = 5; 

fn day_three_input() -> Vec<Vec<u32>> {
    //let contents = fs::read_to_string("inputs/day3puzzle1.txt")
    let contents = fs::read_to_string("inputs/day3sample.txt")
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

fn get_greeks(ints: Vec<Vec<u32>>) -> (Vec<u32>, Vec<u32>) {
    let mut sums : Vec<u32> = vec![];
    for i in 0..reading_len {
        let mut index_sum = 0;
        for b in ints.clone() {
            index_sum += b[i]
        }
        sums.push(index_sum)
    }
    println!("Sums: {:?}",sums);
    println!("Len: {}", ints.len());
    let halfway = ints.len()/2;

    let mut gamma: Vec<u32> = vec![];
    let mut epsilon: Vec<u32> = vec![];
    for s in sums {
        if s > halfway.try_into().unwrap()  {
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

fn binarray_to_int(arr: Vec<u32>) -> isize {
    let mut s = "".to_owned(); 
    for a in arr {
        s.push_str(&a.to_string());
    }
    let i = isize::from_str_radix(&s,2).unwrap();
    return i;
}

fn d3p1() {
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

fn count_left_overlap(left: Vec<u32>, right: Vec<u32>) -> u32 {
    let mut count = 0; 
    for i in 0..reading_len {
        if left[i] == right[i] {
            count += 1;
        }
        else {
            break;
        }
    }
    return count; 
}

fn d3p2() {
    let ints = day_three_input().clone();
    let (gamma, epsilon) = get_greeks(ints.clone());
    println!("{:?}", ints.clone());
    println!("Gamma: {:?}", gamma.clone());
    println!("Epsilon: {:?}", epsilon.clone());

    //  
    let mut oxy_count = 0; 
    for arr in ints.clone() {
        oxy_count = max(oxy_count, count_left_overlap(gamma.clone(), arr))
    }
    println!("oxy: {:?}", oxy_count);
    let mut oxys: Vec<Vec<u32>> = vec![];
    for arr in ints.clone() {
        if count_left_overlap(gamma.clone(), arr.clone()) == oxy_count {
            oxys.push(arr.clone())
        }
    }
    println!("{:?}", oxys);
    let first_oxys = &oxys[0];
    println!("First oxys: {:?}\n", first_oxys);
    //  
    let mut scrubber_count = 0; 
    for arr in ints.clone() {
        scrubber_count = max(scrubber_count, count_left_overlap(epsilon.clone(), arr))
    }
    println!("scrubber: {:?}", scrubber_count);
    let mut scrubbers: Vec<Vec<u32>> = vec![];
    for arr in ints.clone() {
        if count_left_overlap(epsilon.clone(), arr.clone()) == scrubber_count {
            scrubbers.push(arr.clone())
        }
    }
    println!("{:?}", scrubbers);
    let first_scrubber = &scrubbers[0];
    println!("First scrubber: {:?}\n", first_scrubber);

    let oi = binarray_to_int(first_oxys.to_vec());
    let si = binarray_to_int(first_scrubber.to_vec());
    println!("Oxy: {}", oi);
    println!("Scrubber: {}", si);
    println!("Together: {}", oi*si);
}

fn main() {
    d3p2();
}
