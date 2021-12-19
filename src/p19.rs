use std::collections::HashSet;
use std::fmt;
use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct Coord {
    x: i32,
    y: i32,
    z: i32,
}

const ZERO: Coord = Coord { x: 0, y: 0, z: 0 };

impl fmt::Debug for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

impl<'a, 'b> Add<&'b Coord> for &'a Coord {
    type Output = Coord;

    fn add(self, other: &'b Coord) -> Coord {
        Coord {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<'a, 'b> Sub<&'b Coord> for &'a Coord {
    type Output = Coord;

    fn sub(self, other: &'b Coord) -> Coord {
        Coord {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<'a, 'b> Mul<&'b Coord> for &'a Coord {
    type Output = Coord;

    fn mul(self, other: &'b Coord) -> Coord {
        Coord {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

type DistanceMap = Vec<HashSet<i32>>;

#[derive(Debug, Clone, PartialEq)]
pub struct Scanner {
    id: usize,
    signals: Vec<Coord>,
    distances: DistanceMap,
}

pub fn distance_between(a: Coord, b: Coord) -> i32 {
    let d = &a - &b;
    let dd = &d * &d;
    return dd.x + dd.y + dd.z;
}

pub fn distance_map(signals: Vec<Coord>) -> DistanceMap {
    let mut dm: DistanceMap = Vec::new();
    for sig in signals.clone() {
        let distances = signals
            .clone()
            .iter()
            .map(|x| distance_between(sig.clone(), x.clone()))
            .collect::<HashSet<i32>>();
        dm.push(distances);
    }
    return dm;
}

pub fn parse_data(s: &str) -> Vec<Scanner> {
    let scanners: Vec<Scanner> = s
        .split("\n\n")
        .enumerate()
        .map(|(id, x)| {
            let signals = x.lines().collect::<Vec<&str>>()[1..]
                .iter()
                .map(|c| {
                    let s: Vec<&str> = c.split(",").collect();
                    let x = s[0].parse::<i32>().unwrap();
                    let y = s[1].parse::<i32>().unwrap();
                    let z = s[2].parse::<i32>().unwrap();
                    Coord { x, y, z }
                })
                .collect::<Vec<Coord>>();
            let distances = distance_map(signals.clone());
            Scanner {
                id,
                signals,
                distances,
            }
        })
        .collect();
    return scanners;
}

pub fn get_matching_overlap(s0: Scanner, s1: Scanner) -> Vec<(Coord, Coord)> {
    let mut matched: Vec<(Coord, Coord)> = vec![];
    for (i0, sig1) in s0.signals.iter().enumerate() {
        for (i1, sig2) in s1.signals.iter().enumerate() {
            let overlap = s0.distances[i0].intersection(&s1.distances[i1]);
            let count = overlap.clone().count();
            if count > 4 {
                matched.push((sig1.clone(), sig2.clone()));
            }
        }
    }
    return matched;
}

pub fn rotate_coord(mut c: Coord, rotations: i32) -> Coord {
    match rotations {
        0 => {}
        1 => {
            //x stays the same
            //flip y and z
            let tmp = c.y;
            c.y = c.z;
            c.z = tmp;
        }
        2 => {
            // y stays the same
            //flip x and z
            let tmp = c.x;
            c.x = c.z;
            c.z = tmp;
        }
        3 => {
            //z stays the same
            // flip x and y
            let tmp = c.x;
            c.x = c.y;
            c.y = tmp;
        }
        4 => {
            // all move one left
            let tmp = c.x;
            c.x = c.y;
            c.y = c.z;
            c.z = tmp;
        }
        5 => {
            // all move one left
            let tmp = c.x;
            c.x = c.z;
            c.z = c.y;
            c.y = tmp;
        }
        _ => unreachable!(),
    }
    return c;
}

pub fn find_orientation(matched: Vec<(Coord, Coord)>) -> Option<(Coord, Coord, i32)> {
    let f = matched[0].clone();
    let a = f.0;

    for sx in vec![-1, 1] {
        for sy in vec![1, -1] {
            for sz in vec![-1, 1] {
                let s = Coord {
                    x: sx,
                    y: sy,
                    z: sz,
                };

                // I have a
                // I have b
                // Both are in different coord systems.
                // I rotate b into the right one.
                // Then I found the distance between first two matches.
                // a = d + r(s(b));
                for rotations in vec![0, 1, 2, 3, 4, 5] {
                    let mut new_b = f.1;
                    new_b = &s * &new_b;
                    new_b = rotate_coord(new_b, rotations); //these should be communitative,

                    let d = &a - &new_b;

                    let flipped = matched
                        .iter()
                        .map(|(ia, ib)| (ia.clone(), &s * &ib))
                        .collect::<Vec<(Coord, Coord)>>();

                    let rotated = flipped
                        .iter()
                        .map(|(ia, ib)| (ia.clone(), rotate_coord(ib.clone(), rotations)))
                        .collect::<Vec<(Coord, Coord)>>();

                    let shifted = rotated
                        .iter()
                        .map(|(ia, ib)| (ia.clone(), &d + &ib))
                        .collect::<Vec<(Coord, Coord)>>();

                    let distances = shifted
                        .iter()
                        .map(|(ia, ib)| ia - ib)
                        .collect::<Vec<Coord>>();

                    // dbg!(&d, &s, rotations);
                    // for (i, distance) in distances.iter().enumerate() {
                    //     println!("A: {:?}, B': {:?}, Distance: {:?}", matched[i].0, rotated[i].1, distance);
                    // }

                    if distances.iter().all(|c| c == &ZERO) {
                        //    println!("Found orientation!\n");
                        return Some((d, s, rotations));
                    }
                    //println!("\n")
                }
            }
        }
    }
    return None;
}

pub fn reorient(mut sc: Scanner, d: Coord, s: Coord, rotations: i32) -> Scanner {
    sc.signals = sc
        .signals
        .iter()
        .map(|c| {
            let mut new_c = &s * &c;
            new_c = rotate_coord(new_c, rotations);
            return &d + &new_c;
        })
        .collect();
    return sc;
}

pub fn test0() {
    let repeated = parse_data(include_str!(
        "../inputs/day19shouldhaveusedtheseearlier.txt"
    ));

    //orient yourself with rotations and s
    //then swim
    let first = repeated[0].clone();
    for sc in &repeated[1..] {
        let matched = get_matching_overlap(first.clone(), sc.clone());
        let (d, s, rotations) = find_orientation(matched).unwrap();
        let reoriented = reorient(sc.clone(), d, s, rotations);
        assert_eq!(reoriented.signals, first.signals);
    }
}

pub fn testa() {
    let repeated = parse_data(include_str!("../inputs/day19syn.txt"));
    let first = repeated[0].clone();
    // first step is 1,1,-1 switched
    // next step is rotation 2, xz switched
    // step after that is shift down -1, -1, -1

    let matched = get_matching_overlap(first.clone(), repeated[1].clone());
    let (d, s, rotations) = find_orientation(matched).unwrap();
    assert_eq!(rotations, 0);
    assert_eq!(s, Coord { x: 1, y: 1, z: -1 });
    assert_eq!(d, Coord { x: 0, y: 0, z: 0 });

    let matched = get_matching_overlap(first.clone(), repeated[2].clone());
    let (d, s, rotations) = find_orientation(matched).unwrap();
    assert_eq!(rotations, 2);
    assert_eq!(s, Coord { x: -1, y: 1, z: 1 });
    assert_eq!(d, Coord { x: 0, y: 0, z: 0 });

    let matched = get_matching_overlap(first.clone(), repeated[3].clone());
    let (d, s, rotations) = find_orientation(matched).unwrap();
    assert_eq!(rotations, 2);
    assert_eq!(s, Coord { x: -1, y: 1, z: 1 });
    assert_eq!(d, Coord { x: 1, y: 1, z: -1 });
}

pub fn test1() {
    let sample2_scanners = parse_data(include_str!("../inputs/day19sample2.txt"));
    let s0 = sample2_scanners[0].clone();
    let s1 = sample2_scanners[1].clone();

    let matched = get_matching_overlap(s0.clone(), s1.clone());
    let (d, s, rotations) = find_orientation(matched).unwrap();
    assert_eq!(
        d,
        Coord {
            x: 68,
            y: -1246,
            z: -43,
        }
    );
    assert_eq!(s, Coord { x: -1, y: 1, z: -1 });
    assert_eq!(rotations, 0);
    let reoriented_s1 = reorient(s1, d, s, rotations);

    let rm = get_matching_overlap(s0, reoriented_s1);
    let (rd, rs, _) = find_orientation(rm).unwrap();
    assert_eq!(rd, Coord { x: 0, y: 0, z: 0 });
    assert_eq!(rs, Coord { x: 1, y: 1, z: 1 });
}

// make sure that they are oriented together or else it will braek
pub fn combine_scanner(mut l: Scanner, r: Scanner, d: Coord, s: Coord, rotations: i32) -> Scanner {
    let mut reoriented = reorient(r.clone(), d, s, rotations);
    l.signals.append(&mut reoriented.signals);
    l.signals.sort();
    l.signals.dedup();
    l.distances = distance_map(l.clone().signals);
    return l;
}

pub fn test2() {
    let sample2_scanners = parse_data(include_str!("../inputs/day19sample2.txt"));
    let mut s0 = sample2_scanners[0].clone();
    let s1 = sample2_scanners[1].clone();
    let s2 = sample2_scanners[2].clone();
    let s3 = sample2_scanners[3].clone();
    let s4 = sample2_scanners[4].clone();

    //s1
    let matched = get_matching_overlap(s0.clone(), s1.clone());
    let (d, s, r) = find_orientation(matched).unwrap();

    assert_eq!(r, 0);
    assert_eq!(s, Coord { x: -1, y: 1, z: -1 });
    assert_eq!(
        d,
        Coord {
            x: 68,
            y: -1246,
            z: -43
        }
    );
    s0 = combine_scanner(s0, s1, d, s, r);

    //s4
    let matched = get_matching_overlap(s0.clone(), s4.clone());
    let (d, s, r) = find_orientation(matched).unwrap();

    assert_eq!(r, 4);
    assert_eq!(s, Coord { x: 1, y: -1, z: -1 });
    assert_eq!(
        d,
        Coord {
            x: -20,
            y: -1133,
            z: 1061
        }
    );
    s0 = combine_scanner(s0, s4, d, s, r);

    //s2
    let matched = get_matching_overlap(s0.clone(), s2.clone());
    let (d, s, r) = find_orientation(matched).unwrap();

    assert_eq!(r, 1);
    assert_eq!(s, Coord { x: -1, y: 1, z: 1 });
    assert_eq!(
        d,
        Coord {
            x: 1105,
            y: -1205,
            z: 1229
        }
    );
    s0 = combine_scanner(s0, s2, d, s, r);

    //s3
    let matched = get_matching_overlap(s0.clone(), s3.clone());
    let (d, s, r) = find_orientation(matched).unwrap();

    assert_eq!(r, 0);
    assert_eq!(s, Coord { x: -1, y: 1, z: -1 });
    assert_eq!(
        d,
        Coord {
            x: -92,
            y: -2380,
            z: -20
        }
    );
    s0 = combine_scanner(s0, s3, d, s, r);
    assert_eq!(s0.signals.len(), 79);
}

pub fn find_num_beacons(scanners: Vec<Scanner>) -> (usize, Vec<Coord>) {
    let mut spots: Vec<Coord> = vec![];
    let mut mega_scanner = scanners[0].clone();
    let mut unoriented: Vec<Scanner> = scanners[1..].to_vec();

    while unoriented.len() != 0 {
        let mut leftovers: Vec<Scanner> = vec![];
        for sc in unoriented.clone() {
            let matched = get_matching_overlap(mega_scanner.clone(), sc.clone());
            if matched.len() >= 12 {
                match find_orientation(matched) {
                    Some((d, s, r)) => {
                        println!("Found a good match {}", sc.id);
                        spots.push(d);
                        mega_scanner = combine_scanner(mega_scanner, sc, d, s, r);
                    }
                    None => {
                        println!("Found a unorientable match {}", sc.id);
                        leftovers.push(sc);
                    }
                }
            } else {
                leftovers.push(sc);
            }
        }
        unoriented = leftovers;
    }

    return (mega_scanner.signals.len(), spots);
}

pub fn main() {
    let _sample1_scanners = parse_data(include_str!("../inputs/day19sample1.txt"));
    let sample2_scanners = parse_data(include_str!("../inputs/day19sample2.txt"));
    let puzzle_scanners = parse_data(include_str!("../inputs/day19puzzle.txt"));

    test0();
    test1();
    testa();
    test2();
    let (num, spots) = find_num_beacons(sample2_scanners);
    let mut max_distance = 0;
    for s in &spots {
        for t in &spots {
            let d = (s.x - t.x).abs() + (s.y - t.y).abs() + (s.z - t.z).abs();
            max_distance = std::cmp::max(d, max_distance);
        }
    }
    assert_eq!(79, num);
    assert_eq!(3621, max_distance);

    let (num, spots) = find_num_beacons(puzzle_scanners);
    let mut max_distance = 0;
    for s in &spots {
        for t in &spots {
            let d = (s.x - t.x).abs() + (s.y - t.y).abs() + (s.z - t.z).abs();
            max_distance = std::cmp::max(d, max_distance);
        }
    }
    assert_eq!(451, num);
    assert_eq!(13184, max_distance);
}
