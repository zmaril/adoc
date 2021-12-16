use std::collections::HashMap;
const PUZZLE: &str = "HBCHSNFFVOBNOFHFOBNO

HF -> O
KF -> F
NK -> F
BN -> O
OH -> H
VC -> F
PK -> B
SO -> B
PP -> H
KO -> F
VN -> S
OS -> B
NP -> C
OV -> C
CS -> P
BH -> P
SS -> P
BB -> H
PH -> V
HN -> F
KV -> H
HC -> B
BC -> P
CK -> P
PS -> O
SH -> N
FH -> N
NN -> P
HS -> O
CB -> F
HH -> F
SB -> P
NB -> F
BO -> V
PN -> H
VP -> B
SC -> C
HB -> H
FP -> O
FC -> H
KP -> B
FB -> B
VK -> F
CV -> P
VF -> V
SP -> K
CC -> K
HV -> P
NC -> N
VH -> K
PF -> P
PB -> S
BF -> K
FF -> C
FV -> V
KS -> H
VB -> F
SV -> F
HO -> B
FN -> C
SN -> F
OB -> N
KN -> P
BV -> H
ON -> N
NF -> S
OF -> P
NV -> S
VS -> C
OO -> C
BP -> H
BK -> N
CP -> N
PC -> K
CN -> H
KB -> B
BS -> P
KK -> P
SF -> V
CO -> V
CH -> P
FO -> B
FS -> F
VO -> H
NS -> F
KC -> H
VV -> K
NO -> P
OK -> F
PO -> V
FK -> H
OP -> H
PV -> N
CF -> P
NH -> K
SK -> O
KH -> P
HP -> V
OC -> V
HK -> F";

const SAMPLE: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

type Transforms = HashMap<String,String>;
type Counts = HashMap<String,i64>;

pub fn parse_data(s: &str) -> (Transforms, Counts, String, String){
    let mut transform = HashMap::new();
    let mut counts = HashMap::new();

    let (template, formulas) = s.split_once("\n\n").unwrap();
    for i in 0..(template.len()-1) {
        let pair = template[i..=i+1].to_string();
        *counts.entry(pair).or_insert(0) += 1;
    }
    for l in formulas.lines() {
        let (front, back) = l.split_once(" -> ").unwrap();
        transform.insert(front.to_string(), back.to_string());
    }
    let front = template.chars().nth(0).unwrap().to_string();
    let back = template.chars().nth(template.len()-1).unwrap().to_string();
    return (transform, counts, front, back);
} 

pub fn step(counts: &Counts, transforms: &Transforms) -> Counts {
    let mut next = Counts::new();
    for (pair,value) in counts.iter() {
        let middle = &transforms[pair];
        let new_left = format!("{}{}", pair.chars().nth(0).unwrap(), middle);
        let new_right = format!("{}{}", middle, pair.chars().nth(1).unwrap());
        *next.entry(new_left).or_insert(0) += value;
        *next.entry(new_right).or_insert(0) += value;
    }
    return next;
}


pub fn sum(front: String, back: String, counts: &Counts) -> Counts {
    let mut sums = Counts::new();
    for (pair, value) in counts.iter() {
        *sums.entry(pair.chars().nth(0).unwrap().to_string()).or_insert(0) += value;
        *sums.entry(pair.chars().nth(1).unwrap().to_string()).or_insert(0) += value;
    }
    let mut lol = Counts::new();
    for (k,v) in sums.iter() {
        lol.insert(k.clone(), *v/2);
    }
    lol.entry(front).and_modify(|e| {*e += 1});
    lol.entry(back).and_modify(|e| {*e += 1});
    return lol;
}

pub fn get_scores(counts: &Counts) -> (i64, i64, i64) {
    let mut max = 0;
    let mut min = 100000000000000000;
    for (_, v) in counts.iter() {
        max = std::cmp::max(max,*v);
        min = std::cmp::min(min,*v);
    }
    dbg!(max,min);
    dbg!(max-min);
    return (max, min, max-min);
}
pub fn take_steps(n: i32, counts: &Counts, transforms: &Transforms) -> Counts {
    let mut next = counts.clone();
    for _ in 1..=n {
        next = step(&next, transforms);
    }
    return next.clone();
}
pub fn main() {
    let (transform, counts, front, back) = parse_data(SAMPLE);
    let next = take_steps(10, &counts, &transform);
    let sums = sum(front, back, &next);
    dbg!(&sums);
    let (max, min, diff) = get_scores(&sums);
    assert_eq!(*sums.get("B").unwrap(), 1749);
    assert_eq!(*sums.get("C").unwrap(), 298);
    assert_eq!(*sums.get("H").unwrap(), 161);
    assert_eq!(*sums.get("N").unwrap(), 865);
    assert_eq!(max, 1749);
    assert_eq!(min, 161);
    assert_eq!(diff, 1588);

    let (transform, counts, front, back) = parse_data(PUZZLE);
    let next = take_steps(10, &counts, &transform);
    let sums = sum(front, back, &next);
    dbg!(&sums);
    let (_,_, diff) = get_scores(&sums);
    assert_eq!(diff, 3408);


    let (transform, counts, front, back) = parse_data(SAMPLE);
    let next = take_steps(40, &counts, &transform);
    let sums = sum(front, back, &next);
    dbg!(&sums);
    let (max, min, diff) = get_scores(&sums);
    assert_eq!(*sums.get("B").unwrap(), 2192039569602);
    assert_eq!(*sums.get("H").unwrap(), 3849876073);
    assert_eq!(max, 2192039569602);
    assert_eq!(min, 3849876073);
    assert_eq!(diff, 2188189693529);

    let (transform, counts, front, back) = parse_data(PUZZLE);
    let next = take_steps(40, &counts, &transform);
    let sums = sum(front, back, &next);
    dbg!(&sums);
    let (max, min, diff) = get_scores(&sums);
    assert_eq!(*sums.get("B").unwrap(), 2192039569602);
    assert_eq!(*sums.get("H").unwrap(), 3849876073);
    assert_eq!(max, 2192039569602);
    assert_eq!(min, 3849876073);
    assert_eq!(diff, 2188189693529);
}