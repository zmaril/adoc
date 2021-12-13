use std::collections::HashSet;
use std::iter::FromIterator;

const sample: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

pub fn parse_data(s: &str) -> (Vec<(i32, i32)>, Vec<(String, i32)>) {
    let (ns, fs) = s.split_once("\n\n").unwrap();
    let numbers = ns
        .lines()
        .map(|a| {
            let (x, y) = a.split_once(",").unwrap();
            return (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap());
        })
        .collect::<Vec<(i32, i32)>>();
    let folds = fs
        .lines()
        .map(|l| {
            let rest = l.to_string().split_off(11);
            let (p, i) = rest.split_once("=").unwrap();
            return (p.to_string(), i.parse::<i32>().unwrap());
        })
        .collect::<Vec<(String, i32)>>();
    return (numbers, folds);
}

pub fn main() {
    let c = include_str!("../inputs/day13puzzle.txt");
    let (ns, fs) = parse_data(c);
    let mut points: HashSet<(i32, i32)> = HashSet::from_iter(ns);
    for (p, mark) in fs {
        let mut newer_points: HashSet<(i32, i32)> = HashSet::new();
        for (x,y) in &points {
            if p == "x" && x >= &mark {
                let pd = (2 * mark - x, *y);
                newer_points.insert(pd);
            } else if p == "y" && y >= &mark {
                let pd = (*x, 2* mark - y);
                newer_points.insert(pd);
            } else {
                newer_points.insert((*x,*y));
            }
        }
        points = newer_points.clone();
    }
    // y = 7
    //let mut new_points: HashSet<(i32,i32)> = HashSet::new();
    // for p in points {
    //     if p.1 >= 7 {
    //         let pd = (p.0, 2*7-p.1);
    //         new_points.insert(pd);
    //     }
    //     else {
    //         new_points.insert(p);
    //     }
    // }

    dbg!(points.clone());
    dbg!(points.len());
    for y in 0..100 {
        for x in 0..100 {
            if points.contains(&(x,y)) {
                print!("#");
            }
            else {
                print!(" ")
            }
        }
        println!("");
    }
}
