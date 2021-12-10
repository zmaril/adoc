use std::collections::HashMap;
use std::collections::HashSet;

pub fn hashchar(s: &str) -> HashSet<char> {
    return s.to_string().chars().collect::<HashSet<char>>();
}

const simple: &str =
    "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
pub fn get_data() -> Vec<(Vec<HashSet<char>>, Vec<HashSet<char>>)> {
    let data = include_str!("../inputs/day8puzzle.txt")
        .lines()
        .map(|line| {
            let (patterns, output) = line.split_once(" | ").unwrap();
            let mut ps: Vec<HashSet<char>> = patterns.split(" ").map(hashchar).collect();
            ps.sort_by(|a, b| a.len().cmp(&b.len()));
            let os = output.split(" ").map(hashchar).collect();
            return (ps, os);
        })
        .collect::<Vec<(Vec<HashSet<char>>, Vec<HashSet<char>>)>>();
    return data;
}
// # count
// 0 5
// 1 2
// 2 5
// 3 5
// 4 4
// 5 5
// 6 6
// 7 3
// 8 7
// 9 6

// 2: 1
// 3: 7
// 4: 4
// 5: 2, 3, 5
// 6: 0, 6, 9
// 7: 8

//8:

pub fn main() {
    let data = get_data();
    //dbg!(data.clone());

    let mut num_uniques = 0;
    for (_, output) in data.clone() {
        for signal in output {
            if [2, 3, 4, 7].contains(&signal.len()) {
                num_uniques += 1;
            }
        }
    }

    // 2: 1
    // 3: 7
    // 4: 4
    // 5: 2, 3, 5
    // 6: 0, 6, 9
    // 7: 8
    let mut sum = 0;
    for (ps, output) in data.clone() {
        let one = &ps[0];
        let seven = &ps[1];
        let four = &ps[2];
        let eight = &ps[9];

        let two_three_five = &ps[3..=5];
        let zero_six_nine = &ps[6..=8];

        let a = seven - one;

        let adg = two_three_five[0]
            .intersection(&two_three_five[1])
            .copied()
            .collect::<HashSet<char>>()
            .intersection(&two_three_five[2])
            .copied()
            .collect::<HashSet<char>>();
        let dg = &adg - &a;

        let zero = zero_six_nine
            .iter()
            .filter(|x| !x.is_superset(&dg.clone()))
            .collect::<Vec<&HashSet<char>>>()[0];

        let six_nine = zero_six_nine
            .iter()
            .filter(|x| x.is_superset(&dg.clone()))
            .collect::<Vec<&HashSet<char>>>();

        let d = eight - zero;

        let b = &(four - one) - &d;

        let five = two_three_five
            .iter()
            .filter(|x| x.is_superset(&b.clone()))
            .collect::<Vec<&HashSet<char>>>()[0];

        let two_three = two_three_five
            .iter()
            .filter(|x| !x.is_superset(&b.clone()))
            .collect::<Vec<&HashSet<char>>>();

        let nine = six_nine
            .iter()
            .filter(|x| x.is_superset(&one.clone()))
            .collect::<Vec<&&HashSet<char>>>()[0];
        let six = six_nine
            .iter()
            .filter(|x| !x.is_superset(&one.clone()))
            .collect::<Vec<&&HashSet<char>>>()[0];

        let three = two_three
            .iter()
            .filter(|x| x.is_superset(&one.clone()))
            .collect::<Vec<&&HashSet<char>>>()[0];
        let two = two_three
            .iter()
            .filter(|x| !x.is_superset(&one.clone()))
            .collect::<Vec<&&HashSet<char>>>()[0];

        dbg!(zero);
        dbg!(one);
        dbg!(two);
        dbg!(three);
        dbg!(four);
        dbg!(five);
        dbg!(six);
        dbg!(seven);
        dbg!(eight);
        dbg!(nine);
        let mut s = "".to_owned();
        for o in output {
            if o == *zero {
                s += "0"
            }
            else if o == *one {
                s += "1"
            }
            else if o == **two {
                s += "2"
            }
            else if o == **three {
                s += "3"
            }
            else if o == *four {
                s += "4"
            }
            else if o == *five {
                s += "5"
            }
            else if o == **six {
                s += "6"
            }
            else if o == *seven {
                s += "7"
            }
            else if o == *eight {
                s += "8"
            }
            else if o == **nine {
                s += "9"
            }
        }
        dbg!(s.clone());
        sum += s.parse::<i32>().unwrap();
    }
    dbg!(sum);
}

// aaaa
//b    c
//b    c
// dddd
//e    f
//e    f
// gggg

// xxxx
//x    c
//x    c
// xxxx
//e    f
//e    f
// gggg
