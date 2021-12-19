type SnailNum = Vec<(u32, u32)>;

pub fn parse_data(s: &str) -> SnailNum {
    let mut data: SnailNum = vec![];
    let mut depth = 0;
    let mut placeholder: String = "".to_string();
    for c in s.trim().chars() {
        let mut flagged = false;
        let current_depth = depth;
        match c {
            '[' => {
                depth += 1;
                flagged = true;
            }
            ']' => {
                depth -= 1;
                flagged = true;
            }
            ',' => {
                flagged = true;
            }
            _ => {
                placeholder += &c.to_string();
            }
        }
        if flagged && placeholder.len() != 0 {
            let t = placeholder.parse::<u32>().unwrap();
            data.push((t, current_depth));
            placeholder = "".to_string();
        }
    }
    return data;
}

pub fn add(mut l: SnailNum, mut r: SnailNum) -> SnailNum {
    l.append(&mut r);
    let sum: SnailNum = l
        .iter()
        .map(|x| {
            return (x.0, x.1 + 1);
        })
        .collect();
    return sum;
}

pub fn explode(mut s: SnailNum) -> SnailNum {
    for i in 0..(s.len() - 1) {
        let l = s[i];
        let r = s[i + 1];
        if l.1 == r.1 && l.1 > 4 {
            if i > 0 {
                s[i - 1].0 += l.0
            }
            if i + 2 < s.len() {
                s[i + 2].0 += r.0
            }
            s.remove(i);
            s.remove(i);
            s.insert(i, (0, l.1 - 1));
            return s;
        }
    }
    return s;
}

pub fn split(mut s: SnailNum) -> SnailNum {
    for i in 0..s.len() {
        let n = s[i];
        if n.0 >= 10 {
            let l = n.0 - n.0 / 2;
            let r = n.0 / 2;
            assert_eq!(n.0, l + r);
            s.remove(i);
            s.insert(i, (l, n.1 + 1));
            s.insert(i, (r, n.1 + 1));
            return s;
        }
    }
    return s;
}

pub fn n_s(s: &str, n: i32) -> String {
    let mut r: String = "".to_string();
    for _ in 0..n {
        r += s;
    }
    return r;
}

pub fn dis(sn: SnailNum) -> String {
    let mut s: String = "".to_string();
    let mut depth = 0;
    let mut in_pair = false;
    for v in sn.clone() {
        let diff: i32 = v.1 as i32 - depth as i32;
        if diff > 0 {
            s += &n_s("[", diff);
            s += format!("{},", v.0).as_str();
            depth = v.1;
            in_pair = true;
        } else if diff < 0 {
            s += format!("{}", v.0).as_str();
            s += &n_s("]", -diff);
            s += ",";
            depth = v.1;
            in_pair = true;
        } else {
            if in_pair {
                s += format!("{}]", v.0).as_str();
                in_pair = false;
            } else {
                s += format!("[{}", v.0).as_str();
                in_pair = true;
            }
            s += ",";
        }
    }
    // remove last ,
    s = s[0..s.len() - 1].to_string();

    let diff = sn.last().unwrap().1 - 1;
    dbg!(depth, diff);
    s += &n_s("]", diff as i32);
    return s;
}
pub fn reduce(s: SnailNum) -> SnailNum {
    let mut new_num = s;
    //println!("reducing:  {:?}", new_num.clone());
    loop {
        let e = explode(new_num.clone());
        if e != new_num {
            //println!("exploding: {:?}", e.clone());
            new_num = e;
            continue;
        }
        let e = split(new_num.clone());
        if e != new_num {
            //println!("splitting: {:?}", e.clone());
            new_num = e;
            continue;
        }
        break;
    }
    return new_num;
}

pub fn sum(l: SnailNum, r: SnailNum) -> SnailNum {
    return reduce(add(l, r));
}

pub fn add_list(s: &str) -> SnailNum {
    let numbers: Vec<SnailNum> = s.lines().map(|l| parse_data(l)).collect();
    let mut acc: SnailNum = numbers[0].clone();
    for x in 1..numbers.len() {
        //println!("{:?}", acc.clone());
        //println!("+{:?}", numbers[x].clone());
        acc = sum(acc, numbers[x].clone());
        //println!("={:?}\n\n", acc.clone());
    }
    return acc;
}
//3l+2r
pub fn magnitude(mut sn: SnailNum) -> u32 {
    //println!("finding magnitude of: {:?}", sn);
    let max_depth = sn.iter().fold(0, |acc, (_, d)| std::cmp::max(acc, *d));
    //dbg!(max_depth);
    for d in (1..=max_depth).rev() {
        //dbg!(d);
        let indexes = sn
            .iter()
            .enumerate()
            .filter(|(_, (_, e))| *e == d)
            .map(|(i, _)| i)
            .collect::<Vec<usize>>();
        for chunk in indexes.chunks(2).rev() {
            let lv = sn[chunk[0]].0;
            let rv = sn[chunk[1]].0;
            let nv = 3*lv + 2 *rv;
            sn.remove(chunk[0]);
            sn.remove(chunk[0]);
            sn.insert(chunk[0], (nv, d-1));
        }
        //println!("after reducing a step: {:?}", sn);
    }
    //println!("finished: {:?}\n", sn);
    return sn[0].0;
}
pub fn main() {
    let samples = "
    [1,2]
    [[1,2],3]
    [9,[8,7]]
    [[1,9],[8,5]]
    [[[[1,2],[3,4]],[[5,6],[7,8]]],9]
    [[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]
    [[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]";
    let eqs = vec![
        vec![(1, 1), (2, 1)],
        vec![(1, 2), (2, 2), (3, 1)],
        vec![(9, 1), (8, 2), (7, 2)],
        vec![(1, 2), (9, 2), (8, 2), (5, 2)],
        vec![
            (1, 4),
            (2, 4),
            (3, 4),
            (4, 4),
            (5, 4),
            (6, 4),
            (7, 4),
            (8, 4),
            (9, 1),
        ],
        vec![
            (9, 3),
            (3, 4),
            (8, 4),
            (0, 4),
            (9, 4),
            (6, 3),
            (3, 4),
            (7, 4),
            (4, 4),
            (9, 4),
            (3, 2),
        ],
        vec![
            (1, 4),
            (3, 4),
            (5, 4),
            (3, 4),
            (1, 4),
            (3, 4),
            (8, 4),
            (7, 4),
            (4, 4),
            (9, 4),
            (6, 4),
            (9, 4),
            (8, 4),
            (2, 4),
            (7, 4),
            (3, 4),
        ],
    ];
    for (i, line) in samples.trim().lines().enumerate() {
        let l = line.trim();
        let result = parse_data(l);
        assert_eq!(result, eqs[i]);
        //assert_eq!(dis(result), l);
    }

    let l = parse_data("[1,2]");
    let r = parse_data("[[3,4],5]");
    let s = add(l, r);
    assert_eq!(s, vec![(1, 2), (2, 2), (3, 3), (4, 3), (5, 2)]);

    let before = parse_data("[[[[[9,8],1],2],3],4]");
    let after = parse_data("[[[[0,9],2],3],4]");
    assert_eq!(explode(before), after);

    let before = parse_data("[7,[6,[5,[4,[3,2]]]]]");
    let after = parse_data("[7,[6,[5,[7,0]]]]");
    assert_eq!(explode(before), after);

    let before = parse_data("[[6,[5,[4,[3,2]]]],1]");
    let after = parse_data("[[6,[5,[7,0]]],3]");
    assert_eq!(explode(before), after);
    let before = parse_data("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
    let after = parse_data("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
    assert_eq!(explode(before), after);

    let before = parse_data("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
    let after = parse_data("[[3,[2,[8,0]]],[9,[5,[7,0]]]]");
    assert_eq!(explode(before), after);

    // println!("\nnn how it should be!!");
    // println!("reducing: {:?}", parse_data("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]"));
    // println!("after explode: {:?}", parse_data("[[[[0,7],4],[7,[[8,4],9]]],[1,1]]"));
    // println!("after explode: {:?}", parse_data("[[[[0,7],4],[15,[0,13]]],[1,1]]"));
    // println!("after split: {:?}", parse_data("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]"));
    // println!("after split: {:?}", parse_data("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]"));
    // println!("after explode: {:?}", parse_data("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"));

    // println!("\n");
    let l = parse_data("[[[[4,3],4],4],[7,[[8,4],9]]]");
    let r = parse_data("[1,1]");
    let s = parse_data("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
    assert_eq!(sum(l, r), s);
    //println!("{:?}", before.clone());
    //println!("{:?}", reduce(before.clone()));
    //println!("{:?}", after.clone());

    let listed = add_list(
        "[1,1]
    [2,2]
    [3,3]
    [4,4]",
    );
    let sum = parse_data("[[[[1,1],[2,2]],[3,3]],[4,4]]");
    assert_eq!(reduce(listed), sum);

    let listed = add_list(
        "[1,1]
    [2,2]
    [3,3]
    [4,4]
    [5,5]",
    );
    let sum = parse_data("[[[[3,0],[5,3]],[4,4]],[5,5]]");
    assert_eq!(reduce(listed), sum);

    let listed = add_list(
        "[1,1]
    [2,2]
    [3,3]
    [4,4]
    [5,5]
    [6,6]",
    );
    let sum = parse_data("[[[[5,0],[7,4]],[5,5]],[6,6]]");
    assert_eq!(reduce(listed), sum);

    //println!("\n\n\n-------");

    let listed = add_list(
        "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
    [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
    [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
    [[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
    [7,[5,[[3,8],[1,4]]]]
    [[2,[2,2]],[8,[8,1]]]
    [2,9]
    [1,[[[9,3],9],[[9,0],[0,7]]]]
    [[[5,[7,4]],7],1]
    [[[[4,2],2],6],[8,7]]",
    );
    let sum = parse_data("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");
    assert_eq!(reduce(listed), sum);

    let pairs = [
        ("[9,1]", 29),
        ("[1,9]", 21),
        ("[[9,1],[1,9][", 129),
        ("[[1,2],[[3,4],5]]", 143),
        ("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384),
        ("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445),
        ("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791),
        ("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137),
        (
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
            3488,
        ),
    ];
    for (s, v) in pairs.iter() {
        assert_eq!(magnitude(parse_data(s)), *v);
    }

    let listed = add_list("[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
    [[[5,[2,8]],4],[5,[[9,9],0]]]
    [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
    [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
    [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
    [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
    [[[[5,4],[7,7]],8],[[8,3],8]]
    [[9,3],[[9,9],[6,[4,9]]]]
    [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
    [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]");
    let sum = parse_data("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]");
    let result = 4140;
    assert_eq!(listed,sum);
    assert_eq!(magnitude(listed), result);

    let puzzle = include_str!("../inputs/day18puzzle.txt");
    let listed = add_list(puzzle.clone());
    assert_eq!(magnitude(listed), 4235);

    // part two 
    let numbers2 = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
    [[[5,[2,8]],4],[5,[[9,9],0]]]
    [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
    [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
    [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
    [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
    [[[[5,4],[7,7]],8],[[8,3],8]]
    [[9,3],[[9,9],[6,[4,9]]]]
    [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
    [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";


    let numbers: Vec<SnailNum> = puzzle.lines().map(|l| parse_data(l)).collect();
    let mut max_m = 0;
    let mut max_x : SnailNum = vec![];
    let mut max_y : SnailNum = vec![];
    for x in numbers.clone() {
        for y in numbers.clone() {
            if x.clone() != y {
                let m = magnitude(reduce(add(x.clone(),y.clone())));
                //dbg!(m);
                if m > max_m {
                    max_m = std::cmp::max(m, max_m);
                    max_x = x.clone();
                    max_y = y.clone();
                }
            }
        }
    }
    println!("max magnitude {}", max_m); 
    println!("max x:  {:?}", max_x.clone());
    println!("max y:  {:?}", max_y.clone());

    assert_eq!(3993, magnitude(parse_data("[[[[7,8],[6,6]],[[6,0],[7,7]]],[[[7,8],[8,8]],[[7,9],[0,6]]]]")));

}
