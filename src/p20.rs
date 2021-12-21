use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Grid {
    values: HashMap<(i32, i32), bool>,
}

pub type ImageEnhanceAlgo = Vec<bool>;

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s: String = "".to_string();
        let mut min_x = 0;
        let mut min_y = 0;
        let mut max_x = 1;
        let mut max_y = 1;

        for ((x, y), b) in self.values.iter() {
            min_x = std::cmp::min(min_x, *x);
            min_y = std::cmp::min(min_y, *y);
            max_x = std::cmp::max(max_x, *x);
            max_y = std::cmp::max(max_y, *y);
        }

        for y in (min_y)..=(max_y) {
            for x in (min_x)..=(max_x) {
                if self.values.contains_key(&(x, y)) {
                    if self.values[&(x, y)] {
                        s += "#";
                    } else {
                        s += ".";
                    }
                } else {
                    s += "_"
                }
            }
            if y != max_y {
                s += "\n";
            }
        }
        write!(f, "{}", s)
    }
}

pub fn parse_grid(g: &str) -> Grid {
    let values: HashMap<(i32, i32), bool> = g
        .lines()
        .enumerate()
        .map(|(x, l)| {
            let xt = x;
            l.chars()
                .enumerate()
                .map(move |(y, c)| (y, xt, c == '#'))
                .collect::<Vec<(usize, usize, bool)>>()
        })
        .flatten()
        .map(|(x, y, b)| ((x as i32, y as i32), b))
        .collect::<HashMap<(i32, i32), bool>>();
    Grid { values }
}
pub fn parse_data(s: &str) -> (Grid, ImageEnhanceAlgo) {
    let (i, g) = s.split_once("\n\n").unwrap();
    let algo: ImageEnhanceAlgo = i.chars().map(|x| x == '#').collect::<ImageEnhanceAlgo>();
    return (parse_grid(g), algo);
}

pub fn get_number(step: i32, (x, y): (i32, i32), g: &Grid) -> usize {
    let mut n = 0;
    let mut t = 1;
    for dy in vec![1, 0, -1] {
        for dx in vec![1, 0, -1] {
            let contains = g.values.contains_key(&(x + dx, y + dy));
            if contains {
                let b = g.values[&(x + dx, y + dy)];
                if b {
                    n += t;
                }

            } else {
                if step % 2 == 0 {
                    n += t;
                }
            }
            t = t *2;
        }
    }
    return n;
}
pub fn step(step: i32, grid: Grid, algo: &ImageEnhanceAlgo) -> Grid {
    let mut to_examine:HashSet<(i32,i32)> = HashSet::with_capacity(0);

    for ((x, y), _) in &grid.values {
        for dx in vec![-1, 0, 1] {
            for dy in vec![-1, 0, 1] {
                to_examine.insert((x + dx, y + dy));
            }
        }
    }

    let mut new_grid = Grid {
        values: HashMap::with_capacity(0),
    };

    for xy in to_examine {
        let number = get_number(step, xy, &grid);
        new_grid.values.insert(xy, algo[number]);
    }
    return new_grid;
}

pub fn main() {
    let (sample_grid, sample_algo) = parse_data(include_str!("../inputs/day20sample.txt"));
    let (puzzle_grid, puzzle_algo) = parse_data(include_str!("../inputs/day20puzzle.txt"));

    let mini = "...\n#..\n.#.";
    assert_eq!(34, get_number(1, (1, 1), &parse_grid(mini)));

    let sample1 = parse_grid(include_str!("../inputs/day20sample1.txt"));
    let sample2 = parse_grid(include_str!("../inputs/day20sample2.txt"));
    let sample3 = parse_grid(include_str!("../inputs/day20sample3.txt"));

    assert_eq!(147, get_number(1, (5, 6), &sample1.clone()));

    let mut stepped1 = step(1, sample1.clone(), &sample_algo);
    stepped1.values = stepped1
        .values
        .iter()
        .filter(|((x, y), _)| *x >= 0 && *y >= 0 && *x <= 14 && *y <= 14)
        .map(|((x, y), b)| ((*x, *y), *b))
        .collect::<HashMap<(i32, i32), bool>>();

    assert_eq!(stepped1, sample2);

    let mut stepped2 = step(1, sample2.clone(), &sample_algo);
    stepped2.values = stepped2
        .values
        .iter()
        .filter(|((x, y), _)| *x >= 0 && *y >= 0 && *x <= 14 && *y <= 14)
        .map(|((x, y), b)| ((*x, *y), *b))
        .collect::<HashMap<(i32, i32), bool>>();

    for (xy, b) in &stepped2.values {
        if sample3.values.contains_key(&xy) {
            if sample3.values[&xy] != *b {
                println!("Different values: {:?} {}", xy, b);
            }
        } else {
            println!("No key {:?}", xy);
        }
    }
    assert_eq!(stepped2, sample3);

    let mut g = sample_grid.clone();
    for i in 1..=51 {
        println!(
            "Step {} with {} lit",
            i,
            g.values
                .iter()
                .filter(|(_, b)| { **b })
                .collect::<Vec<_>>()
                .len()
        );
        //println!("{}\n", g);
        g = step(1, g, &sample_algo);
    }

    // let mut g = sample_grid.clone();
    // let mut algo = sample_algo.clone();
    // algo[0] = true;
    // for i in 1..4 {
    //     println!("Step {} with {} lit", i, g.values.len());
    //     println!("{}\n", g);
    //     g = step(i, g, &sample_algo);
    // }

    let mut g = puzzle_grid;
    for i in 1..=51 {
        println!(
            "Step {} with {} lit",
            i,
            g.values
                .iter()
                .filter(|(_, b)| { **b })
                .collect::<Vec<_>>()
                .len()
        );
        //println!("{}\n", g);
        g = step(i, g, &puzzle_algo);
    }
}
