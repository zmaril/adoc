use std::num::ParseIntError;
use std::str::FromStr;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct Line {
    start: Point,
    end: Point,
}

const SIZE: usize = 1000;

#[derive(Clone, PartialEq, Eq)]
pub struct Grid {
    values: Vec<Vec<usize>>
}

impl Grid {
    fn new() -> Grid {
        let values = vec![vec![0; SIZE]; SIZE];
        return Grid { values };
    }
    
    fn mark(self: &mut Self, p: Point) -> &mut Grid {
        self.values[p.x as usize][p.y as usize] += 1; 
        return self;
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = "\n".to_owned();
        let mut i = 0;
        for line in &self.values {
            for number in line {
                if *number != (0 as usize) {
                    s += &format!("{}", number).to_owned();
                }
                else {
                s += &format!(".").to_owned();
                }
            }
            i += 1;
            if i != SIZE {
                s += "\n";
            }
        }
        return write!(f, "{}", s);
    }
}

/// taken from docs https://doc.rust-lang.org/std/str/trait.FromStr.html
impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.trim().split(',').collect();

        let x = coords[0].parse::<i32>()?;
        let y = coords[1].parse::<i32>()?;

        Ok(Point { x, y })
    }
}

impl FromStr for Line {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.trim().split("->").collect();

        let start = Point::from_str(coords[0]).unwrap();
        let end = Point::from_str(coords[1]).unwrap();

        Ok(Line { start, end })
    }
}

impl Line {
    fn is_not_diagonal(self: &Self) -> bool {
        self.start.x == self.end.x  || self.start.y == self.end.y
    }

    fn all_points(self: &Self) -> Vec<Point> {
        //dbg!(self);
        let x_diff = self.end.x - self.start.x;
        let y_diff = self.end.y - self.start.y;
        //dbg!(x_diff, y_diff);
        let steps = std::cmp::max(x_diff.abs(),y_diff.abs());
        //dbg!(steps);
        let points: Vec<Point> = (0..steps+1).map(|i| {
            Point{
                x: self.start.x + i * (x_diff/steps) ,
                y: self.start.y + i * (y_diff/steps) 
            }
        }).collect();
        //dbg!(points.clone());
        return points;
    }
}

pub fn get_lines() -> Vec<Line> {
    let data = include_str!("../inputs/day5sample.txt")
        .lines()
        .map(|line| Line::from_str(line).unwrap())
        .collect::<Vec<Line>>();
    return data;
}

pub fn compute_overlap_grid(lines: Vec<Line>) -> Grid {
    let mut grid = Grid::new();
    //let to_consider = lines.iter().filter(|l| {l.is_not_diagonal()});
    let to_consider = lines;
    for line in to_consider {
        for p in line.all_points() {
            grid.mark(p);
        } 
    }
    return grid;
}

pub fn main() {
    let lines = get_lines();
    let overlap_grid = compute_overlap_grid(lines);
    //dbg!(overlap_grid.clone());

    let mut score = 0;
    for x in 0..SIZE{
        for y in 0..SIZE {
            if overlap_grid.values[x][y] > 1 {
                score += 1;
            }
        }
    }
    dbg!(score);
}
