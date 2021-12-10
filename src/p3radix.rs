//Experimenting with using a radix sort
//To solve Day 3 Part B of Advent of Code 2021 
//In very few lookups

const DIGITS: usize = 5;
const LENGTH: usize = 12;

pub fn get_data() -> [u8; LENGTH] {
    // based off of 
    // https://github.com/timvisee/advent-of-code-2021/blob/master/day03b/src/main.rs 
    let mut readings : [u8; LENGTH] = [0; LENGTH];
    include_str!("../inputs/day3sample.txt")
        .lines()
        .map(|l| u8::from_str_radix(l, 2).unwrap())
        .enumerate()
        .for_each(|(i,x)| {
            //dbg!(i,x);
            readings[i] = x;
        });
    return readings;
}

#[derive(Debug)]
struct Tree {
    nodes: Vec<Node>,
    total: usize
}

#[derive(Debug)]
struct Node {
    idx: usize,
    one: Option<usize>,
    zero: Option<usize>,
    total: i32,
    ones_count: i32
}

impl Node {
    pub fn new(idx: usize ) -> Node {
        return Node{idx: idx, total: 0, ones_count: 0, one: None, zero: None};
    }
}

impl Tree {
    pub fn new() -> Tree {
        let root = Node {idx: 0, one: None, zero: None, total: 0, ones_count: 0};
        Tree {
            nodes: vec![root],
            total: 1
        }
    }
    
    pub fn insert(&mut self, new_value: u8) {
        let mut leaf_idx = 0;
        for digit in (0..DIGITS).rev() {
            let nth_digit_of_val = (new_value & 1 << digit) >> digit;
            //dbg!(nth_digit_of_val);
            match (nth_digit_of_val, &self.nodes[leaf_idx]) {
                (1, Node{one: None, ..}) => {
                    let new_idx = self.total; 
                    let new_leaf = Node::new(new_idx);
                    self.nodes.push(new_leaf);
                    self.total += 1;
                    let mut leaf = &mut self.nodes[leaf_idx];
                    leaf.one = Some(new_idx);
                    leaf.total += 1; 
                    leaf.ones_count += 1;
                    leaf_idx = new_idx;
                },
                (0, Node{zero: None, ..}) => {
                    let new_idx = self.total; 
                    let new_leaf = Node::new(new_idx); 
                    self.nodes.push(new_leaf);
                    self.total += 1;
                    let mut leaf = &mut self.nodes[leaf_idx];
                    leaf.zero = Some(new_idx);
                    leaf.total += 1; 
                    leaf_idx = new_idx;
                },
                (1, _) => {
                    let mut leaf = &mut self.nodes[leaf_idx];
                    leaf.total += 1; 
                    leaf.ones_count += 1;
                    leaf_idx = leaf.one.unwrap();
                },
                (0, _) => {
                    let mut leaf = &mut self.nodes[leaf_idx];
                    leaf.total += 1; 
                    leaf_idx = leaf.zero.unwrap();
                },
                (_, _) => unreachable!()
            }
        }
    }
}

pub fn radix() {
    let mut tree = Tree::new();
    let readings = get_data();
    for reading in &readings {
        tree.insert(*reading)
    }
    println!("{:?}", tree);
    dbg!(tree);
}
