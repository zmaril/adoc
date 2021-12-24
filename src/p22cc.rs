use colored::Colorize;
const EXAMPLE: &str = "on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10";

pub fn pdd(s: &str) -> (i64, i64) {
    let (l, r) = s[2..].split_once("..").unwrap();
    (l.parse::<i64>().unwrap(), r.parse::<i64>().unwrap())
}

#[derive(Debug, Clone, Copy)]
pub struct Region {
    x: (i64, i64),
    y: (i64, i64),
    z: (i64, i64),
}

#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    on: bool,
    region: Region,
}

#[derive(Debug, Clone)]
pub struct Compression {
    x: Vec<i64>,
    y: Vec<i64>,
    z: Vec<i64>,
}

pub fn parse_data(s: &str) -> (Vec<Instruction>, Compression) {
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
            Instruction {
                on,
                region: Region { x, y, z },
            }
        })
        .collect::<Vec<Instruction>>();
    let mut x_list = data
        .iter()
        .flat_map(|int| {
            let (a, b) = int.region.x;
            return vec![a, b, a+1, b+1];
        })
        .collect::<Vec<i64>>();
    x_list.sort();
    x_list.dedup();

    let mut y_list = data
        .iter()
        .flat_map(|int| {
            let (a, b) = int.region.y;
            return vec![a, b, a+1, b+1];
        })
        .collect::<Vec<i64>>();
    y_list.sort();
    y_list.dedup();

    let mut z_list = data
        .iter()
        .flat_map(|int| {
            let (a, b) = int.region.z;
            return vec![a, b, a+1, b+1];
        })
        .collect::<Vec<i64>>();
    z_list.sort();
    z_list.dedup();
    // for d in &data {
    //     println!("{:?}", d);
    // }
    let mut instructions: Vec<Instruction> = vec![];
    for old_int in data {
        let mut int = old_int.clone();

        int.region.x.0 = x_list.iter().position(|v| *v == int.region.x.0).unwrap() as i64;
        int.region.x.1 = x_list.iter().position(|v| *v == int.region.x.1).unwrap() as i64;

        int.region.y.0 = y_list.iter().position(|v| *v == int.region.y.0).unwrap() as i64;
        int.region.y.1 = y_list.iter().position(|v| *v == int.region.y.1).unwrap() as i64;

        int.region.z.0 = z_list.iter().position(|v| *v == int.region.z.0).unwrap() as i64;
        int.region.z.1 = z_list.iter().position(|v| *v == int.region.z.1).unwrap() as i64;

        instructions.push(int);
    }
    (
        instructions,
        Compression {
            x: x_list,
            y: y_list,
            z: z_list,
        },
    )
}

pub fn clamp(l: i64, n: i64, r: i64) -> i64 {
    if n < l {
        return l;
    } else if r < n {
        return r;
    } else {
        return n;
    }
}

pub fn count_cubes(ints: Vec<Instruction>, compression: Compression) -> i64 {
    //println!("{:?}", &compression);
    let mut cubes =
        vec![vec![vec![false; compression.z.len()]; compression.y.len()]; compression.x.len()];

    for int in ints {
        //println!("Doing {:?}", int);
        for x in int.region.x.0..=int.region.x.1 {
            for y in int.region.y.0..=int.region.y.1 {
                for z in int.region.z.0..=int.region.z.1 {
                    cubes[x as usize][y as usize][z as usize] = int.on;
                }
            }
        }
    }

    let mut total = 0;
    let mut xd_total = 0;
    for (xi, yz) in cubes.iter().enumerate().take(cubes.iter().len() - 1) {
        let x1 = clamp(-50, compression.x[xi + 1], 50);
        let x2 = clamp(-50, compression.x[xi], 50);

        let x1 = compression.x[xi + 1];
        let x2 = compression.x[xi];

        let xd = x1 - x2;
        if yz.iter().any(|z| z.iter().any(|on| *on)) {
            //println!("x@{}: ({}, {}) = {}", xi, x2, x1, xd);
            xd_total += xd;
        }
        let mut yd_total = 0;
        for (yi, z) in yz.iter().enumerate().take(yz.iter().len() - 1) {
            let y1 = clamp(-50, compression.y[yi + 1], 50);
            let y2 = clamp(-50, compression.y[yi], 50);

            let y1 = compression.y[yi + 1];
            let y2 = compression.y[yi];
            let yd = y1 - y2;
            if z.iter().any(|on| *on) {
                //println!(" . y@{}: ({}, {}) = {}", yi, y2, y1, yd);
                yd_total += yd;
            }
            let mut zd_total = 0;
            for (zi, on) in z.iter().enumerate().take(z.iter().len() - 1) {
                let z1 = clamp(-50, compression.z[zi + 1], 50);
                let z2 = clamp(-50, compression.z[zi], 50);

                let z1 = compression.z[zi + 1];
                let z2 = compression.z[zi];

                let zd = z1 - z2;
                if *on {
                    zd_total += zd;
                    let product = xd * yd * zd;
                    //println!(" .   z@{}: ({}, {}) = {}", zi, z2, z1, zd);
                    //println!(" .      {}", product);
                    total += product;
                }
            }
            if zd_total != 0 {
                //println!(" .   {} {}, {}", "z@@ =".green(),zd_total, total);
            }
        }
        if yd_total != 0 {
            //println!(" . {} {}, {}", "y@@ =".green(),yd_total, total);
        }
    }
    if xd_total != 0 {
        //println!("{} {}, {}", "x@@ =".green(), xd_total, total);
    }
    return total;
}

pub fn main() {
    // let (instructions, compressions) = parse_data(EXAMPLE);
    // let sizes = vec![27, 46, 38, 39];
    // for i in 0..sizes.len() {
    //     assert_eq!(
    //         sizes[i],
    //         count_cubes(instructions[0..i + 1].to_vec(), compressions.clone())
    //     );
    // }

    // let sizes = vec![
    //     139590, 210918, 225476, 328328, 387734, 420416, 436132, 478727, 494759, 494804, 492164,
    //     534936, 534936, 567192, 567150, 592167, 588567, 592902, 590029, 590784, 590784,
    // ];
    // for i in 11..sizes.len() {
    //         dbg!(i);
    //         let (instructions, compressions) =
    //             parse_data(include_str!("../inputs/day22sample.txt"));
    //         dbg!(
    //             count_cubes(instructions[0..i + 1].to_vec(), compressions.clone()),
    //             sizes[i]
    //         );
    //         //println!("\n")
    // }

    //let (instructions, compressions) = parse_data(include_str!("../inputs/day22puzzle.txt"));
    //assert_eq!(543306, count_cubes(instructions, compressions));

    //let (instructions, compressions) =parse_data(include_str!("../inputs/day22bsample.txt"));
    //assert_eq!(2758514936282235, count_cubes(instructions, compressions));

    let (instructions, compressions) = parse_data(include_str!("../inputs/day22puzzle.txt"));
    assert_eq!(543306, count_cubes(instructions, compressions));
}
