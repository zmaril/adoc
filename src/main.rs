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


fn day_two_input() -> Vec<(String, i32)> {
    let contents = fs::read_to_string("inputs/day2puzzle1.txt")
    .expect("Something went wrong reading the file");
    let lines = contents.lines();
    let directions = lines.map(|x| {
        let mut split = x.split(" ");
        let direction = split.next().unwrap().to_string();
        let distance = split.next().unwrap().parse::<i32>().unwrap();
        return (direction, distance)
    }).collect();
    return directions;
}

fn d2p1 () {
    let directions = day_two_input();
    println!("{:?}",directions);

    let mut x = 0;
    let mut y = 0; 
    for (direction, distance) in directions {
        println!("{} {}", direction, distance);
        match direction.as_str() {
            "forward" => x+= distance, 
            "down" => y+= distance, 
            "up" => y-= distance, 
            _ => unreachable!()
        };
        println!("{} {}\n", x, y);
    }
    println!("{}",x*y);
}


fn d2p2 () {
    let directions = day_two_input();
    println!("{:?}",directions);

    let mut x = 0;
    let mut y = 0; 
    let mut aim = 0;
    for (direction, distance) in directions {
        println!("{} {}", direction, distance);
        match direction.as_str() {
            "forward" => {
                x += distance;
                y += aim*distance;
            }, 
            "down" => {
                aim += distance
            }, 
            "up" => {
                aim -= distance
            }, 
            _ => unreachable!()
        };
        println!("{} {} {}\n", x, y, aim);
    }
    println!("{}",x*y);
}
fn main() {
    d2p2();
}
