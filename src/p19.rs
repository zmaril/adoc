use std::collections::HashSet;
use std::ops::{Mul, Add, Sub};
use std::fmt;

#[derive(Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct Coord {
    x: i32,
    y: i32,
    z: i32,
}

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
    let x = a.x - b.x;
    let y = a.y - b.y;
    let z = a.z - b.z;
    return x.pow(2) + y.pow(2) + z.pow(2);
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
    dbg!(&a);
    for sx in vec![-1, 1] {
        for sy in vec![-1, 1] {
            for sz in vec![-1, 1] {
                //TODO: maybe there are more rotations idk
                for rotations in vec![0, 1, 2, 3, 4, 5] {
                    let mut new_b = Coord {
                        x: sx * f.1.x,
                        y: sy * f.1.y,
                        z: sz * f.1.z,
                    };

                    new_b = rotate_coord(new_b, rotations);
                    dbg!(&new_b);

                    let mut d = Coord {
                        x: a.x - new_b.x,
                        y: a.y - new_b.y,
                        z: a.z - new_b.z,
                    };

                    let s = Coord {
                        x: sx,
                        y: sy,
                        z: sz,
                    };

                    let shifted = matched
                        .iter()
                        .map(|(ia, ib)| {
                            (
                                ia.clone(),
                                Coord {
                                    x: d.x + sx * ib.x,
                                    y: d.y + sy * ib.y,
                                    z: d.z + sz * ib.z,
                                },
                            )
                        })
                        .collect::<Vec<(Coord, Coord)>>();

                    let rotated = shifted
                        .iter()
                        .map(|(ia, ib)| (ia.clone(), rotate_coord(ib.clone(), rotations)))
                        .collect::<Vec<(Coord, Coord)>>();

                    let distances = rotated
                        .iter()
                        .map(|(ia, ib)| Coord {
                            x: ia.x - ib.x,
                            y: ia.y - ib.y,
                            z: ia.z - ib.z,
                        })
                        .collect::<Vec<Coord>>();

                    dbg!(&d, &s, rotations);
                    for (i, distance) in distances.iter().enumerate() {
                        println!("A: {:?}", matched[i].0);
                        println!("B: {:?}", matched[i].1);
                        println!("B: {:?} shifted", shifted[i].1);
                        println!("B: {:?} rotated", rotated[i].1);
                        println!("Distance: {:?}\n", distance);
                    }
                    if distances.iter().all(|c| c.x == 0 && c.y == 0 && c.z == 0) {
                        println!("Found orientation!");
                        dbg!(&d, &s, rotations);
                        return Some((d, s, rotations));
                    }
                    println!("\n")
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
            let mut new_c = Coord {
                x: s.x * c.x,
                y: s.y * c.y,
                z: s.z * c.z,
            };
            new_c = rotate_coord(new_c, rotations);
            return Coord {
                x: d.x + new_c.x,
                y: d.y + new_c.y,
                z: d.z + new_c.z,
            };
        })
        .collect();
    return sc;
}

pub fn test0() {
    let repeated = parse_data(include_str!(
        "../inputs/day19shouldhaveusedtheseearlier.txt"
    ));
    //dbg!(&repeated);
    let first = repeated[0].clone();
    for sc in &repeated[1..] {
        let matched = get_matching_overlap(first.clone(), sc.clone());
        let (d, s, rotations) = find_orientation(matched).unwrap();
        let reoriented = reorient(sc.clone(), d, s, rotations);
        assert_eq!(reoriented.signals, first.signals);
    }
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

pub fn test2() {
    let sample2_scanners = parse_data(include_str!("../inputs/day19sample2.txt"));
    let s0 = sample2_scanners[0].clone();
    let s1 = sample2_scanners[1].clone();
    let s4 = sample2_scanners[4].clone();

    println!("Doing s0 and s1");
    let matched = get_matching_overlap(s0.clone(), s1.clone());
    let (d, s, r) = find_orientation(matched).unwrap();
    let reoriented_s1 = reorient(s1.clone(), d, s, r);

    println!("Doing rs1 and s4");
    let mut matched = get_matching_overlap(s1.clone(), s4.clone());
    let (d, s, r) = find_orientation(matched).unwrap();
    let reoriented_s4 = reorient(s4, d, s, r);
}

pub fn find_num_beacons(scanners: Vec<Scanner>) -> usize {
    let mut oriented: Vec<Scanner> = vec![];
    let mut newly_oriented: Vec<Scanner> = vec![];
    newly_oriented.push(scanners[0].clone());
    let mut unoriented: Vec<Scanner> = vec![];
    unoriented.append(&mut scanners[1..].to_vec());

    while let Some(next) = newly_oriented.pop() {
        println!("Orienting off of scanner {}", next.id);
        let mut leftovers: Vec<Scanner> = vec![];
        for sc in unoriented.clone() {
            let matched = get_matching_overlap(next.clone(), sc.clone());
            if matched.len() >= 12 {
                match find_orientation(matched) {
                    Some((d, s, r)) => {
                        println!("Found a good match {}", sc.id);
                        let reoriented_sc = reorient(sc, d, s, r);
                        newly_oriented.push(reoriented_sc);
                    }
                    None => {
                        println!("Found a unorientable match {}", sc.id);
                        leftovers.push(sc);
                    }
                }
            } else {
                println!("Pushing to leftovers {}", sc.id);
                leftovers.push(sc);
            }
        }
        unoriented = leftovers;
        oriented.push(next);
        println!("");
    }

    let mut beacons: HashSet<Coord> = HashSet::new();
    for sc in oriented {
        //println!("making beacon list: {}", sc.id);
        for s in sc.signals {
            //println!("orinted: {}, {}, {}", s.x, s.y, s.z);
            beacons.insert(s.clone());
        }
    }
    let mut r = beacons.into_iter().collect::<Vec<Coord>>();
    r.sort();
    for poop in r.clone() {
        //println!("{}, {}, {}", poop.x, poop.y, poop.z);
    }
    dbg!(r.len())
}
pub fn main() {
    let _sample1_scanners = parse_data(include_str!("../inputs/day19sample1.txt"));
    let sample2_scanners = parse_data(include_str!("../inputs/day19sample2.txt"));
    let _puzzle_scanners = parse_data(include_str!("../inputs/day19puzzle.txt"));

    test0();
    test1();
    //test2();
    //find_num_beacons(sample2_scanners);
}
