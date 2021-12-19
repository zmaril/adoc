use std::collections::HashSet;
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
            if count > 11 {
                matched.push((sig1.clone(), sig2.clone()));
            }
        }
    }
    return matched;
}

pub fn find_orientation(matched: Vec<(Coord, Coord)>) -> Option<(Coord, Coord)> {
    for sx in vec![1, -1] {
        for sy in vec![1, -1] {
            for sz in vec![1, -1] {
                let f = matched[0].clone();
                let a = f.0;
                let b = f.1;
                let dx = a.x - (sx * b.x);
                let dy = a.y - (sy * b.y);
                let dz = a.z - (sz * b.z);
                let d = Coord {
                    x: dx,
                    y: dy,
                    z: dz,
                };
                let s = Coord {
                    x: sx,
                    y: sy,
                    z: sz,
                };
                dbg!(&d, &s);
                let distances = matched
                    .iter()
                    .map(|(a, b)| Coord {
                        x: a.x - (dx + sx * b.x),
                        y: a.y - (dy + sy * b.y),
                        z: a.z - (dz + sz * b.z),
                    })
                    .collect::<Vec<Coord>>();
                for (i, distance) in distances.iter().enumerate() {
                    println!(
                        "match diff: {:#?} - {:#?}, {:#?}",
                        matched[i].0, matched[i].1, distance
                    );
                }
                if distances.iter().all(|c| c.x == 0 && c.y == 0 && c.z == 0) {
                    println!("Found orientation!");
                    return Some((d, s));
                }
            }
        }
    }
    return None;
}

pub fn reorient(mut sc: Scanner, d: Coord, s: Coord) -> Scanner {
    sc.signals = sc
        .signals
        .iter()
        .map(|c| Coord {
            x: d.x + s.x * c.x,
            y: d.y + s.y * c.y,
            z: d.z + s.z * c.z,
        })
        .collect();
    return sc;
}

pub fn test1() {
    let sample2_scanners = parse_data(include_str!("../inputs/day19sample2.txt"));
    let s0 = sample2_scanners[0].clone();
    let s1 = sample2_scanners[1].clone();

    let matched = get_matching_overlap(s0.clone(), s1.clone());
    let (d,s) = find_orientation(matched).unwrap();
    assert_eq!(d, Coord {
        x: 68,
        y: -1246,
        z: -43,
    });
    assert_eq!(s, Coord {
        x: -1,
        y: 1,
        z: -1
    });
    let reoriented_s1 = reorient(s1, d, s);

    let rm = get_matching_overlap(s0, reoriented_s1);
    let (rd, rs)= find_orientation(rm).unwrap();
    assert_eq!(rd, Coord {
        x: 0,
        y: 0,
        z: 0,
    });
    assert_eq!(rs, Coord {
        x: 1,
        y: 1,
        z: 1
    });
}

pub fn test2() {
    let sample2_scanners = parse_data(include_str!("../inputs/day19sample2.txt"));
    let s1 = sample2_scanners[1].clone();
    let s4 = sample2_scanners[4].clone();

    let matched = get_matching_overlap(s1.clone(), s4.clone());
    let result = find_orientation(matched.clone());
    dbg!(matched);
    dbg!(result);
}

pub fn main() {
    let _sample1_scanners = parse_data(include_str!("../inputs/day19sample1.txt"));
    let sample2_scanners = parse_data(include_str!("../inputs/day19sample2.txt"));
    let _puzzle_scanners = parse_data(include_str!("../inputs/day19puzzle.txt"));

    //test1();
    test2();
    return ();
    //reorient all that are found and then do the rest of them.
    let mut oriented: Vec<Scanner> = vec![];
    let mut newly_oriented: Vec<Scanner> = vec![];
    newly_oriented.push(sample2_scanners[0].clone());
    let mut unoriented: Vec<Scanner> = vec![];
    unoriented.append(&mut sample2_scanners[1..].to_vec());

    while let Some(next) = newly_oriented.pop() {
        println!("Orienting off of scanner {}", next.id);
        let mut leftovers: Vec<Scanner> = vec![];
        for sc in unoriented.clone() {
            let matched = get_matching_overlap(next.clone(), sc.clone());
            if matched.len() >= 12 {
                match find_orientation(matched) {
                    Some((d, s)) => {
                        println!("Found a good match {}", sc.id);
                        let reoriented_sc = reorient(sc, d, s);
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
    dbg!(r.len());
}
