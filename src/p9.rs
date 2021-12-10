use std::collections::HashSet;
type Heatmap = Vec<Vec<i32>>;
use colored::*;

pub fn get_data() -> Heatmap {
    let heat = include_str!("../inputs/day9puzzle.txt")
        .lines()
        .map(|line| {
            return line
                .chars()
                .map(|x| x.to_string().parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
        })
        .collect::<Heatmap>();
    return heat;
}

pub fn get_mins() -> Vec<(usize, usize)> {
    let data = get_data();
    dbg!(data.clone());
    let max_x = data.len() - 1;
    let max_y = data[0].len() - 1;
    let mut mins: Vec<(usize, usize)> = vec![];
    for (x, line) in data.iter().enumerate() {
        for (y, value) in line.iter().enumerate() {
            let mut local_min = true;
            if x > 0 && data[x - 1][y] <= *value {
                local_min = false;
            }
            if x < max_x && data[x + 1][y] <= *value {
                local_min = false;
            }
            if y > 0 && data[x][y - 1] <= *value {
                local_min = false;
            }
            if y < max_y && data[x][y + 1] <= *value {
                local_min = false;
            }
            if local_min {
                //dbg!(x,y,value);
                //dbg!(local_min);
                mins.push((x, y));
                print!("{}", value.to_string().red());
            } else {
                print!("{}", value);
            }
        }
        println!("");
    }
    return mins;
    //dbg!(mins.clone());
    //let risk = mins.iter().fold(0, |acc, x| acc + x + 1);
    //dbg!(risk);
}

pub fn calculate_size(loc: (usize, usize), data: Heatmap) -> usize {
    let mut basin: HashSet<(usize, usize)> = HashSet::new();
    let mut neighbors: HashSet<(usize, usize)> = HashSet::new();

    let max_x = data.len() - 1;
    let max_y = data[0].len() - 1;

    neighbors.insert(loc);
    while neighbors.len() != basin.len() {
        dbg!(neighbors.clone());
        dbg!(basin.clone());
        basin = basin.union(&mut neighbors).map(|x| *x).collect();
        for (x, y) in basin.clone() {
            if x > 0 && data[x - 1][y] != 9 {
                neighbors.insert((x-1,y));
            }
            if x < max_x && data[x + 1][y] !=9 {
                neighbors.insert((x+1,y));
            }
            if y > 0 && data[x][y - 1] != 9 {
                neighbors.insert((x,y-1));
            }
            if y < max_y && data[x][y + 1] != 9 {
                neighbors.insert((x,y+1));
            }
        }
    }
    return basin.len();
}

pub fn main() {
    let data = get_data();
    let mins = get_mins();
    let max_x = data.len() - 1;
    let max_y = data[0].len() - 1;
    let mut basin_sizes: Vec<usize> = vec![];
    for min in mins {
        let r = calculate_size(min, data.clone());
        basin_sizes.push(r);
        dbg!(r);
    }
    basin_sizes.sort();
    dbg!(basin_sizes);
}
