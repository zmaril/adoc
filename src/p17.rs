use std::fmt;
use std::collections::HashSet;

const SAMPLE: &str = "target area: x=20..30, y=-10..-5";
const PUZZLE: &str = "target area: x=153..199, y=-114..-75";

pub type Rect = ((i32, i32), (i32, i32));
pub fn parse_data(s: &str) -> Rect {
    let (xs, ys) = s[12..].split_once(",").unwrap();

    let xt = xs.trim()[2..].split_once("..").unwrap();
    let x1 = xt.0.parse::<i32>().unwrap();
    let x2 = xt.1.parse::<i32>().unwrap();

    let yt = ys.trim()[2..].split_once("..").unwrap();
    let y1 = yt.0.parse::<i32>().unwrap();
    let y2 = yt.1.parse::<i32>().unwrap();
    return ((x1, x2), (y1, y2));
}

#[derive(Debug, PartialEq)]
pub struct Trajectory {
    p: Point,
    dp: Point,
}

#[derive(PartialEq, Clone)]
pub struct Point {
    x: i32,
    y: i32,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Iterator for Trajectory {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        self.p.x = self.p.x + self.dp.x;
        self.p.y = self.p.y + self.dp.y;

        self.dp.y -= 1;
        if self.dp.x > 0 {
            self.dp.x -= 1
        } else if self.dp.x < 0 {
            self.dp.x += 1
        }
        Some(self.p.clone())
    }
}

pub fn trajectory(p: Point) -> Trajectory {
    Trajectory {
        p: Point { x: 0, y: 0 },
        dp: p,
    }
}

pub fn point_within_rect(p: Point, r: Rect) -> bool {
    return r.0 .0 <= p.x && p.x <= r.0 .1 && r.1 .0 <= p.y && p.y <= r.1 .1;
}

pub fn point_beyond_rect(p: Point, r: Rect) -> bool {
    dbg!(&p, r);

    if r.0 .1 > 0 && p.x > r.0 .1 {
        return true;
    }

    if r.0 .0 < 0 && p.x < r.0 .0 {
        return true;
    }

    if r.1 .1 > 0 && p.x > r.1 .1 {
        return true;
    }

    if r.1 .0 < 0 && p.x < r.1 .0 {
        return true;
    }

    return false;
}

pub fn display_run(t: Point, r: Rect) {
    let mut path = trajectory(t);
    let mut points: Vec<Point> = path.take(100).collect::<Vec<Point>>();
    // loop {
    //     let next = path.next().unwrap();
    //     points.push(next.clone());
    //     if point_beyond_rect(next, r) {
    //         break;
    //     }
    // }
    let mut max_y = 0;
    for y in (-50..47).rev() {
        for x in 0..35 {
            let p = Point { x, y };
            if points.contains(&p) {
                print!("#");
                max_y = std::cmp::max(max_y, y);
            } else if point_within_rect(p, r) {
                print!("T");
            } else if x == 0 && y == 0 {
                print!("S");
            } else {
                print!(".");
            }
        }
        println!("");
    }
    dbg!(max_y);
}

pub fn do_run(t: Point, r: Rect) -> (bool, i32) {
    let mut path = trajectory(t);
    let mut points: Vec<Point> = path.take(1000).collect::<Vec<Point>>();
    let enters = points.iter().any(|p| point_within_rect(p.clone(), r));
    if !enters {
        return (false, -1);
    } else {
        let max_y = points.iter().fold(0, |acc, p| std::cmp::max(acc, p.y));
        return (true, max_y);
    }
}

pub fn check_rect(r: Rect) -> i32 {
    let mut max_y = 0;
    for x in 0..200 {
        for y in 0..1000 {
            let (entered, max) = do_run(Point { x, y }, r);
            if entered {
                max_y = std::cmp::max(max, max_y);
            }
        }
    }
    return max_y;
}

pub fn calc_direct(r: Rect) -> i32 {
    let mut max_height = 0;
    for start in 1..1000 {
        let mut dy = start;
        let mut y = 0;
        let mut local_max = 0;
        while y >= r.1.1 {
            y += dy;
            dy -= 1;
            local_max = std::cmp::max(local_max, y);
            if r.1.0 <= y && y <= r.1.1 {
                max_height = std::cmp::max(local_max, max_height);
            }
        }
    }
    return max_height;
}

pub fn get_xs(r: Rect) -> HashSet<i32> {
    let mut xs: HashSet<i32> = HashSet::new();
    for start in 1..1000 {
        let mut dx = start;
        let mut x = 0;
        while dx != 0 {
            x += dx;
            dx -= 1;
            if r.0.0 <= x && x <= r.0.1 {
                xs.insert(start);
            }
        }
    }
    return xs;
}

pub fn get_ys(r: Rect) -> HashSet<i32> {
    let mut ys: HashSet<i32> = HashSet::new();
    for start in -1000..1000 {
        let mut dy = start;
        let mut y = 0;
        while y > r.1.1 {
            y += dy;
            dy -= 1;
            if r.1.0 <= y && y <= r.1.1 {
                ys.insert(start);
            }
        }
    }
    return ys;
}

pub fn brute_force(r: Rect) -> i32 {
    let mut num_match = 0; 
    for x in get_xs(r) {
        for y in get_ys(r) {
            let (matched, _) = do_run(Point{x,y}, r);
            if matched {
                println!("{} {}", x, y);
                num_match += 1
            }
        }
    }
    return num_match;
}

pub fn main() {
    let sample = parse_data(SAMPLE);
    assert_eq!(((20, 30), (-10, -5)), sample);

    let puzzle = parse_data(PUZZLE);
    assert_eq!(((153, 199), (-114, -75)), puzzle);

    //let run1 = trajectory(Point { x: 6, y: 9 });
    //assert_eq!(run1.take(10).collect::<Vec<Point>>(), vec![]);

    //assert!(point_beyond_rect(Point{x: 2,y: 2}, ((0,1),(0,1))));
    //assert!(!point_beyond_rect(Point{x: 2,y: 2}, ((0,2),(0,2))));
    assert_eq!(do_run(Point { x: 6, y: 9 }, sample), (true, 45));
    assert_eq!(do_run(Point { x: 1, y: 9 }, sample), (false, -1));
    //assert_eq!(check_rect(sample), 45);
    //assert_eq!(check_rect(puzzle), 45);
    assert_eq!(45, calc_direct(sample));
    assert_eq!(6441, calc_direct(puzzle));

    assert_eq!(112, brute_force(sample));
    assert_eq!(3146, brute_force(puzzle));
}
