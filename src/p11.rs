use colored::*;
use std::fmt;

#[derive(Clone)]
pub struct Grid {
    values: Vec<Vec<i32>>,
}
const SIZE: i32 = 10;

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = "\n".to_owned();
        let mut i = 0;
        for line in &self.values {
            for number in line {
                if *number == 9 {
                    s += &format!("{}", number.to_string().green()).to_owned();
                } else if *number == 8 {
                    s += &format!("{}", number.to_string().yellow()).to_owned();
                } else if *number == 0 {
                    s += &format!("{}", number.to_string().bold()).to_owned();
                } else {
                    s += &format!("{}", number.to_string()).to_owned();
                }
            s += " ";
            }
            i += 1;
            if i != SIZE {
                s += "\n";
            }
        }
        return write!(f, "{}", s);
    }
}

pub fn get_data() -> Grid {
    let values = include_str!("../inputs/day11puzzle.txt")
        .lines()
        .map(|x| {
            x.split("")
                .filter(|x| *x != "")
                .map(|y| y.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    return Grid { values };
}

pub fn take_step(mut g: Grid) -> Grid {
    for (x, line) in g.values.clone().iter().enumerate() {
        for (y, _) in line.iter().enumerate() {
            g.values[x][y] += 1;
        }
    }
    while g.values.iter().any(|vs| vs.iter().any(|v| {v >=&10})) {
        for (x, line) in g.values.clone().iter().enumerate() {
            for (y, _) in line.iter().enumerate() {
                if g.values[x][y] >= 10 {
                    g.values[x][y] = 0;
                    let directions: Vec<(i32,i32)> = vec![(-1,1), (0,1), (1,1), (-1,0), (1,0), (-1,-1), (0, -1), (1,-1)];
                    //let directions: Vec<(i32,i32)> = vec![(0, -1), (0,1)];
                    for (dx,dy) in directions {
                        let px: i32 = (x as i32) + dx;
                        let py: i32 = (y as i32) + dy;
                        if  0 <= px && px < SIZE && 0 <= py && py < SIZE && g.values[px as usize][py as usize] != 0 {
                            g.values[px as usize][py as usize] += 1;
                        }
                    }
                }
            }
        }
    }
    return g;
}
pub fn main() {
    let mut grid = get_data();
    dbg!(grid.clone());
    //let mut flashes = 0;
    for i in 1..450 {
        grid = take_step(grid);
        let mut all_flashes = true; 
        for vs in grid.clone().values{
            for v in vs {
                if v != 0 {
                    all_flashes = false;
                }
            }
        }
        if all_flashes {
            dbg!(i);
            dbg!(grid.clone());
        }

    }
    dbg!(grid.clone());
    //dbg!(flashes);
}
