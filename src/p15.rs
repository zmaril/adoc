use std::fmt;
use std::collections::{HashSet, BinaryHeap};

#[derive(Clone, PartialEq)]
pub struct Grid {
    values: Vec<Vec<i32>>,
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = "\n".to_owned();
        for (_,line) in self.values.iter().enumerate() {
            for number in line {
                s += &format!("{}", number.to_string());
            }
            s += "\n";
        }
        return write!(f, "{}", s);
    }
}

pub fn parse_data(s: &str) -> Grid {
    let values = s
        .lines()
        .map(|x| {
            x.chars()
                .map(|y| y.to_string().parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    return Grid { values };
}

const SAMPLE: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

pub fn dijkstra(g: &Grid) -> Grid {
    let mut visited: HashSet<(usize,usize)> = HashSet::new();

    let mut node_costs: Grid = Grid{values: vec![vec![1000000000;g.values[0].len()]; g.values.len()]}; 
    node_costs.values[0][0] = 0;

    let mut pq: BinaryHeap<(i32,(usize,usize))> = BinaryHeap::new();
    pq.push((0,(0,0)));

    while let Some((cost,coord)) = pq.pop() {
        //println!("Checking {:?} @ {}", coord, cost);
        visited.insert(coord);
        for diff in vec![(-1,0), (1,0), (0,1), (0,-1)] {
            let px = coord.0 as i32 + diff.0; 
            let py = coord.1 as i32 + diff.1;
            if 0 <= px && px < g.values[0].len() as i32 && 0 <= py && py < g.values.len() as i32 {
                let neighbor = (px as usize,py as usize);
                let neighbor_cost = g.values[px as usize][py as usize];
                if !visited.contains(&neighbor) {
                    let new_cost = -cost + neighbor_cost;
                    if new_cost < node_costs.values[px as usize][py as usize] {
                        node_costs.values[px as usize][py as usize] = new_cost;
                        pq.push((-new_cost, neighbor));
                        //println!("Pushing {:?} @ {}", neighbor, new_cost);
                    }
                }
            }
        }
        //println!("");
    }
    return node_costs;
}

pub fn tiled(g: &Grid) -> Grid {
    let len_x = g.values[0].len();
    let len_y = g.values.len();

    let mut tile: Grid = Grid{values: vec![vec![1000000000;len_x * 5]; len_y * 5]}; 
    for tx in 0..5 {
        for ty in 0..5 {
            for x in 0..len_x {
                for y in 0..len_y {
                    let px = x + len_x * tx;
                    let py = y + len_y * ty;
                    let mut new_value = g.values[x][y] + tx as i32 +ty as i32;
                    if new_value >= 10 {
                        new_value = new_value % 10 + 1;
                    }
                    tile.values[px][py] = new_value;
                }
            }
        }
    }
    return tile;
}

pub fn main() {
    let puzzle_grid = parse_data(include_str!("../inputs/day15puzzle.txt"));
    let sample_grid = parse_data(SAMPLE);

    let sample_costs =dijkstra(&sample_grid);
    assert_eq!(40, *sample_costs.values.last().unwrap().last().unwrap());


    let puzzle_costs =dijkstra(&puzzle_grid);
    assert_eq!(508, *puzzle_costs.values.last().unwrap().last().unwrap());


    let test = Grid{values: vec![vec![8]]};

    let tiled_sample_grid = parse_data(include_str!("../inputs/day15sampletile.txt"));
    assert_eq!(tiled_sample_grid, tiled(&sample_grid));

    let tiled_sample_costs = dijkstra(&tiled(&sample_grid));
    assert_eq!(315, *tiled_sample_costs.values.last().unwrap().last().unwrap());

    let tiled_puzzle_costs = dijkstra(&tiled(&puzzle_grid));
    assert_eq!(315, *tiled_puzzle_costs.values.last().unwrap().last().unwrap());
}
