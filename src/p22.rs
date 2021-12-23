use std::collections::HashSet;

const EXAMPLE: &str = "on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10";

pub fn pdd(s: &str) -> (i32, i32) {
    let (l, r) = s[2..].split_once("..").unwrap();
    (l.parse::<i32>().unwrap(), r.parse::<i32>().unwrap())
}

#[derive(Debug, Clone, Copy)]
pub struct Region {
    x: (i32, i32),
    y: (i32, i32),
    z: (i32, i32),
}

#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    on: bool,
    region: Region
}

pub fn parse_data(s: &str) -> Vec<Instruction> {
    let data = s
        .lines()
        .map(|l| {
            let (toggle, coords) = l.split_once(" ").unwrap();
            let on = toggle == "on";
            let (xs, yzs) = coords.split_once(",").unwrap();
            let (ys, zs) = yzs.split_once(",").unwrap();
            let x = pdd(xs);
            let y = pdd(ys);
            let z = pdd(zs);
            Instruction { on, region: Region{x, y, z} }
        })
        .collect::<Vec<Instruction>>();
    data
}

pub fn int_contains(int: Instruction, x: i32, y: i32, z: i32) -> bool {
    int.region.x.0 <= x && x <= int.region.x.1 && 
    int.region.y.0 <= y && y <= int.region.y.1 && 
    int.region.z.0 <= z && z <= int.region.z.1 
}

pub fn range_overlap(l: (i32, i32), r: (i32,i32)) -> Option<(i32,i32)> {
    let lt = (l.0..=l.1).collect::<HashSet<i32>>();
    let rt = (r.0..=r.1).collect::<HashSet<i32>>();
    let ot = lt.intersection(&rt).map(|x| *x).collect::<HashSet<i32>>();
    if ot.len() == 0 {
        return None;
    }
    else {
        Some((*ot.iter().min().unwrap(), *ot.iter().max().unwrap()))
    }
}

pub fn region_overlap(l: Region, r: Region) -> Option<Region> {
    let xo = range_overlap(l.x, r.x);
    let yo = range_overlap(l.y, r.y);
    let zo = range_overlap(l.z, r.z);
    match (xo,yo,zo) {
        (Some(x), Some(y), Some(z)) => Some(Region{x,y,z}),
        _ => None
    }
}

pub fn region_contains(l: Region, r: Region) -> bool {
    l.x.0 <= r.x.0 && 
    l.x.1 >= r.x.1 && 
    l.y.0 <= r.y.0 && 
    l.y.1 >= r.y.1 && 
    l.z.0 <= r.z.0 && 
    l.z.1 >= r.z.1 
}

type Cubes = HashSet<(i32, i32, i32)>;
pub fn get_points(int: Instruction) -> Cubes {
    let mut s = HashSet::new();
    let lx = std::cmp::max(int.region.x.0, -50);
    let rx = std::cmp::min(int.region.x.1, 50);
    for x in lx..=rx {
        let ly = std::cmp::max(int.region.y.0, -50);
        let ry = std::cmp::min(int.region.y.1, 50);
        for y in ly..=ry {
            let lz = std::cmp::max(int.region.z.0, -50);
            let rz = std::cmp::min(int.region.z.1, 50);
            for z in lz..=rz {
                s.insert((x, y, z));
            }
        }
    }
    return s;
}

pub fn count_cubes(ints: Vec<Instruction>) -> usize {
    let mut cubes: Cubes = HashSet::new();
    for int in ints {
        let points = get_points(int);
        if int.on {
            cubes = cubes.union(&points).map(|x| *x).collect::<Cubes>();
        } else {
            cubes = cubes.difference(&points).map(|x| *x).collect::<Cubes>();
        }
    }
    return cubes.iter().count();
}

//Store all overlapping on's, and then only the part of off's that apply to them 
// create new on vec for each
// remove ons from existing on's
// count up on - count offs, off's shouldn't be overlapping at all
pub fn count_region(r : Region) -> i32 {
    (r.x.1 - r.x.0 ) *
    (r.y.1 - r.y.0 ) *
    (r.z.1 - r.z.0 )
}

pub fn count_ball(b: RegionBall) -> i32 {
    let mut on = count_region(b.on);
    for off in b.offs {
        on -= count_region(off);
    }
    return on;
}

pub struct RegionBall {
    on: Region,
    offs: Vec<Region>
}

// pub fn count_cubes2(ints: Vec<Instruction>) -> i32 {
//     let mut balls: Vec<RegionBall> = vec![];

//     for int in ints {
//         if int.on {
//             let mut leftovers = RegionBall{on: int.region, offs: vec![]};
//             for b in &balls {
//                 if let Some(overlap) = region_overlap(b.on, leftovers.on)  {
//                     leftovers.offs.push(overlap);
//                     b.offs = b.offs.iter().filter(|x| { 
//                         !region_contains(leftovers.on, x)
//                     }).collect::<Vec<RegionBall>>();
//                     for off in &b.offs {


//                     }

//                 }
//             }
//             // ball is contained by any current on, check if it turns off any off of that ball.
//             // modify existing on's based on this on. 
//             // remove that overlap from the ball.
//             // create new ball based on the leftover on region
//         }
//         else {
//             //if off overlaps with on of any existing ball 
//             //add new off to that ball 
//             //make sure none of the offs overlap, remove whatever is existing from it.
//         }
//     }

//     let mut total = 0;
//     for b in balls {
//         total += count_ball(b);
//     }
//     return total;
// }

// pub fn find_components(ints: Vec<Instruction>) -> Vec<Vec<usize>> {
//     let mut overlaps: Vec<(usize,usize)> = vec![];
//     for (i, iint) in ints.iter().enumerate() {
//          for (j, jint)  in ints.iter().enumerate() {
//             if let Some(_) = region_overlap(jint.region, iint.region) {
//                 overlaps.push((i,j))
//             }
//         }
//     }
//     let components: Vec<Vec<usize>> = vec![];
//     for (i,j) in overlaps {
//         // can just sort these 
//         let existing: = components.iter().filter(|x| x.contains(&i) || x.contains(&j)).collect();
//     }
//     return components;
// }

pub fn main() {
    let instructions = parse_data(EXAMPLE);
    //assert_eq!(39, count_cubes(instructions));
    for i in 0..instructions.clone().len(){
        println!("step {}, size {}", i, count_cubes(instructions[0..i+1].to_vec()));
    }
    let sizes = vec![27, 46, 38, 39];
    for i in 0..sizes.len() {
        assert_eq!(sizes[i], count_cubes(instructions[0..i+1].to_vec()));
    }

    let instructions = parse_data(include_str!("../inputs/day22sample.txt"));
    for i in 0..instructions.clone().len(){
        println!("step {}, size {}", i, count_cubes(instructions[0..i].to_vec()));
    }

    // assert_eq!(590784, count_cubes(instructions));
    // let instructions = parse_data(include_str!("../inputs/day22puzzle.txt"));
    // assert_eq!(543306, count_cubes(instructions));

    // let instructions=parse_data(include_str!("../inputs/day22bsample.txt"));
    // assert_eq!(2758514936282235, count_cubes(instructions));
}
