use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use cached::proc_macro::cached;

#[derive(Hash, Clone, Debug, PartialEq, Eq, Copy)]
pub struct Game {
    p1_place: u64,
    p1_score: u64,
    p2_place: u64,
    p2_score: u64,
}

impl Ord for Game {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .p1_score
            .cmp(&self.p1_score)
            .then_with(|| other.p2_score.cmp(&self.p2_score))
    }
}

impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn advance(mut g: Game, p: u8, n: u64) -> Game {
    if p == 1 {
        g.p1_place += n;
        g.p1_place = (g.p1_place - 1) % 10 + 1;
        g.p1_score += g.p1_place;
    } else {
        g.p2_place += n;
        g.p2_place = (g.p2_place - 1) % 10 + 1;
        g.p2_score += g.p2_place;
    }
    g
}

pub fn part1(a: u64, b: u64) -> u64 {
    let mut g = Game {
        p1_place: a,
        p1_score: 0,
        p2_place: b,
        p2_score: 0,
    };
    let mut dice = 1;
    let mut rolls = 0;

    loop {
        let mut n = 0;
        for _ in 0..3 {
            n += dice;
            dice += 1;
            dice = (dice - 1) % 100 + 1;
        }
        rolls += 3;
        g = advance(g, 1, n);
        if g.p1_score >= 1000 {
            break;
        }

        let mut n = 0;
        for _ in 0..3 {
            n += dice;
            dice += 1;
            dice = (dice - 1) % 100 + 1;
        }
        rolls += 3;
        g = advance(g, 2, n);
        if g.p2_score >= 1000 {
            break;
        }
    }
    dbg!(&g);
    dbg!(dice);
    dbg!(rolls);
    std::cmp::min(g.p1_score, g.p2_score) * rolls
}

#[cached]
pub fn find_num_wins(g: Game) -> (u64, u64) {
    let mut s1 = 0;
    let mut s2 = 0;
    for n in 3..=9 {
        let next = advance(g, 1, n);
        if g.p1_score >= 21 {
            s1 += 1;
            return (s1, s2);
        }
    }
    (s1, s2)
}
const OUTCOMES: [(u64,u64); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

pub fn part2(a: u64, b: u64) -> u64 {
    let mut g = Game {
        p1_place: a,
        p1_score: 0,
        p2_place: b,
        p2_score: 0,
    };

    let mut counts: HashMap<Game, u64> = HashMap::new();
    // once it is done, remove it?
    counts.insert(g, 1);

    /// filter out binary heap???
    let mut todo = BinaryHeap::new();
    todo.push(g);
    let mut total = 0;

    while let Some(game) = todo.pop() {
        if g.p1_score >= 21 {
            total += counts.get(&game).unwrap();
            continue;
        }
        if g.p2_score >= 21 {
            continue;
        }
        let count = counts.get(&game).unwrap().clone();

        dbg!(game);
        for (roll, ways) in &OUTCOMES {
            let next = advance(game, 1, *roll);
            dbg!(count, ways);
            counts
                .entry(next)
                .and_modify(|e| *e += count * ways)
                .or_insert(count * ways);
            todo.push(next);
        }

        for (roll, ways) in &OUTCOMES {
            let next = advance(game, 2, *roll);
            dbg!(count, ways);
            counts
                .entry(next)
                .and_modify(|e| *e += count * ways)
                .or_insert(count * ways);
            todo.push(next);
        }
    }
    total
}

#[cached]
pub fn count_num_wins(g: Game) -> (i32,i32) {
}

pub fn part2a() {

}

pub fn main() {
    assert_eq!(part1(4, 8), 739785);
    assert_eq!(part1(5, 10), 711480);
    let big: u64 = 444356092776315;
    assert_eq!(part2(4,8),big);
    let bigger: i64 = 0;
    assert_eq!(part2(5,10),0);
}

// number of times seen
// add on the multiple
//3 4 5 6 7 8 9
//1 3 6 7 6 3 1
