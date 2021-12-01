use std::fs;

fn day_one_input() -> Vec<i32> {
    let contents = fs::read_to_string("inputs/day1puzzle1.txt")
    .expect("Something went wrong reading the file");
    let lines = contents.lines();
    let numbers : Vec<i32>= lines.map(|x| {x.parse::<i32>().unwrap()}).collect();
    return numbers;
}

fn d1p1() {
    let numbers = day_one_input();
    let mut increases = 0;
    for i in 0..(numbers.len()-1){
        println!("{} {}", numbers[i], numbers[i+1]);
        if numbers[i+1] > numbers[i] {
            increases += 1;
        }
    }
    println!("{:#?}", increases);
}

fn d1p2() {
    let numbers = day_one_input();
    let mut increases = 0;
    for i in 0..(numbers.len()-3){
        println!("{} {} {}", i, numbers[i], numbers[i+3]);
        if numbers[i+3] > numbers[i] {
            increases += 1;
        }
    }
    println!("{:#?}", increases);
}

fn main() {
    d1p1();
    d1p2();
}
