use cached::proc_macro::cached;
use std::cmp::Ordering;

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

pub fn advance(mut g: Game, is_player_ones_turn: bool, n: u64) -> Game {
    if is_player_ones_turn {
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
        g = advance(g, true, n);
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
        g = advance(g, false, n);
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
pub fn count_num_wins(g: Game, is_player_ones_turn: bool) -> (u64, u64) {
    if g.p1_score >= 21 {
        println!(
            "({:2} {:2}) ({:2} {:2}) {} {}",
            g.p1_score, g.p1_place, g.p2_score, g.p2_place, 1, 0
        );
        return (1, 0);
    }
    if g.p2_score >= 21 {
        println!(
            "({:2} {:2}) ({:2} {:2}) {} {}",
            g.p1_score, g.p1_place, g.p2_score, g.p2_place, 0, 1
        );
        return (0, 1);
    }
    let mut p1_wins = 0;
    let mut p2_wins = 0;
    for i in 1..=3 {
        for j in 1..=3 {
            for k in 1..=3 {
                let next = advance(g, is_player_ones_turn, i + j + k);
                let (n1, n2) = count_num_wins(next, !is_player_ones_turn);
                p1_wins += n1;
                p2_wins += n2;
            }
        }
    }

    println!(
        "({:2} {:2}) ({:2} {:2}) {} {}",
        g.p1_score, g.p1_place, g.p2_score, g.p2_place, p1_wins, p2_wins
    );
    return (p1_wins, p2_wins);
}

pub fn part2(a: u64, b: u64) -> (u64, u64) {
    let g = Game {
        p1_place: a,
        p1_score: 0,
        p2_place: b,
        p2_score: 0,
    };
    return count_num_wins(g, true);
}

pub fn main() {
    //assert_eq!(part1(4, 8), 739785);
    //assert_eq!(part1(5, 10), 711480);
    let a: u64 = 444356092776315;
    let b: u64 = 341960390180808;
    assert_eq!(part2(4, 8), (a, b));
    let a: u64 = 0;
    let b: u64 = 0;
    assert_eq!(part2(5, 10), (a, b));
}

// number of times seen
// add on the multiple
//3 4 5 6 7 8 9
//1 3 6 7 6 3 1
