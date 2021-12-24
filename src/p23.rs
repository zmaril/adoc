use cached::proc_macro::cached;
use maplit::hashmap;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Game {
    hallway: [char; 11],
    rooms: [[char; 2]; 4],
    cost: i32,
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s: String = "".to_string();
        s += &"#############\n".to_string();
        s += &format!("#{}#\n", self.hallway.iter().collect::<String>());
        s += &format!(
            "###{}#{}#{}#{}###\n",
            self.rooms[ti(Letter::A)][0],
            self.rooms[ti(Letter::B)][0],
            self.rooms[ti(Letter::C)][0],
            self.rooms[ti(Letter::D)][0]
        );
        s += &format!(
            "  #{}#{}#{}#{}#  \n",
            self.rooms[ti(Letter::A)][1],
            self.rooms[ti(Letter::B)][1],
            self.rooms[ti(Letter::C)][1],
            self.rooms[ti(Letter::D)][1]
        );
        s += &"  #########\n".to_string();
        s += &format!("Cost: {}", self.cost);
        write!(f, "{}", s)
    }
}

#[derive(Clone, Hash, PartialEq, Eq, Debug, Copy)]
pub enum Letter {
    A,
    B,
    C,
    D,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Places {
    Hallway,
    Room(Letter),
}

#[derive(Clone, Debug, Copy)]
pub struct Location {
    position: usize,
    place: Places,
}

#[derive(Clone, Debug, Copy)]
pub struct Action {
    start: Location,
    stop: Location,
}

pub fn cost_of_letter(c: char) -> i32 {
    match c {
        'A' => 1,
        'B' => 10,
        'C' => 100,
        'D' => 1000,
        _ => unreachable!(),
    }
}

pub fn ti(l: Letter) -> usize {
    match l {
        Letter::A => 0,
        Letter::B => 1,
        Letter::C => 2,
        Letter::D => 3,
    }
}

pub fn location(l: Letter) -> i32 {
    match l {
        Letter::A => 2,
        Letter::B => 4,
        Letter::C => 6,
        Letter::D => 8,
    }
}

pub fn char_to_letter(c: char) -> Letter {
    match c {
        'A' => Letter::A,
        'B' => Letter::B,
        'C' => Letter::C,
        'D' => Letter::D,
        _ => unreachable!(),
    }
}

pub fn move_thing(g: &Game, Action { start, stop }: &Action) -> Game {
    let mut new_game = g.clone();
    let mut old_char = '.';
    match start.clone() {
        Location {
            place: Places::Room(letter),
            position: x,
        } => {
            old_char = g.rooms[ti(letter)][x];
            new_game.rooms[ti(letter)][x] = '.';
        }
        Location {
            place: Places::Hallway,
            position: x,
        } => {
            old_char = g.hallway[x];
            new_game.hallway[x] = '.';
        }
    }

    match stop.clone() {
        Location {
            place: Places::Room(letter),
            position: x,
        } => {
            new_game.rooms[ti(letter)][x] = old_char;
        }
        Location {
            place: Places::Hallway,
            position: x,
        } => {
            new_game.hallway[x] = old_char;
        }
    }

    match (start, stop) {
        (
            Location {
                place: Places::Room(room1),
                position: p1,
            },
            Location {
                place: Places::Room(room2),
                position: p2,
            },
        ) => {
            let distance =
                ((location(*room1) - location(*room2)) as i32).abs() + *p1 as i32 + *p2 as i32 + 2;
            let cost = distance * cost_of_letter(old_char);
            new_game.cost += cost;
        }
        (
            Location {
                place: Places::Room(room1),
                position: p1,
            },
            Location {
                place: Places::Hallway,
                position: p2,
            },
        ) => {
            let distance = (location(*room1) - *p2 as i32).abs() + *p1 as i32 + 1;
            let cost = distance * cost_of_letter(old_char);
            new_game.cost += cost;
        }
        (
            Location {
                place: Places::Hallway,
                position: p1,
            },
            Location {
                place: Places::Room(room2),
                position: p2,
            },
        ) => {
            let distance = (location(*room2) - *p1 as i32).abs() + *p2 as i32 + 1;
            let cost = distance * cost_of_letter(old_char);
            new_game.cost += cost;
        }
        (
            Location {
                place: Places::Hallway,
                position: p1,
            },
            Location {
                place: Places::Hallway,
                position: p2,
            },
        ) => {
            let distance = (*p1 as i32 - *p2 as i32).abs();
            let cost = distance * cost_of_letter(old_char);
            new_game.cost += cost;
        }
    }

    new_game
}

pub fn is_done(g: &Game) -> bool {
    let done = Game {
        hallway: ['.'; 11],
        rooms: [['A', 'A'], ['B', 'B'], ['C', 'C'], ['D', 'D']],
        cost: 0,
    };
    done.hallway == g.hallway && done.rooms == g.rooms
}

pub fn example_test() {
    let mut example = Game {
        hallway: ['.'; 11],
        rooms: [['B', 'A'], ['C', 'D'], ['B', 'C'], ['D', 'A']],
        cost: 0,
    };

    let actions = vec![
        Action {
            start: Location {
                position: 0,
                place: Places::Room(Letter::C),
            },
            stop: Location {
                position: 3,
                place: Places::Hallway,
            },
        },
        Action {
            start: Location {
                position: 0,
                place: Places::Room(Letter::B),
            },
            stop: Location {
                position: 0,
                place: Places::Room(Letter::C),
            },
        },
        Action {
            start: Location {
                position: 1,
                place: Places::Room(Letter::B),
            },
            stop: Location {
                position: 5,
                place: Places::Hallway,
            },
        },
        Action {
            start: Location {
                position: 3,
                place: Places::Hallway,
            },
            stop: Location {
                position: 1,
                place: Places::Room(Letter::B),
            },
        },
        Action {
            start: Location {
                position: 0,
                place: Places::Room(Letter::A),
            },
            stop: Location {
                position: 0,
                place: Places::Room(Letter::B),
            },
        },
        Action {
            start: Location {
                position: 0,
                place: Places::Room(Letter::D),
            },
            stop: Location {
                position: 7,
                place: Places::Hallway,
            },
        },
        Action {
            start: Location {
                position: 1,
                place: Places::Room(Letter::D),
            },
            stop: Location {
                position: 9,
                place: Places::Hallway,
            },
        },
        Action {
            start: Location {
                position: 7,
                place: Places::Hallway,
            },
            stop: Location {
                position: 1,
                place: Places::Room(Letter::D),
            },
        },
        Action {
            start: Location {
                position: 5,
                place: Places::Hallway,
            },
            stop: Location {
                position: 0,
                place: Places::Room(Letter::D),
            },
        },
        Action {
            start: Location {
                position: 9,
                place: Places::Hallway,
            },
            stop: Location {
                position: 0,
                place: Places::Room(Letter::A),
            },
        },
    ];
    for action in actions {
        assert!(path_clear(&example, &action));
        example = move_thing(&example, &action);
    }
    assert_eq!(example.cost, 12521);
    assert!(is_done(&example));
}

pub fn path_clear(g: &Game, act: &Action) -> bool {
    let start = act.start;
    let stop = act.stop;
    //can only move pieces that exist
    match start {
        Location {
            position: x,
            place: Places::Hallway,
        } => {
            if g.hallway[x] == '.' {
                if let (Places::Room(_), Places::Room(_)) = (start.place,stop.place) {
                println!("That doesn't exist");
                }
                return false;
            }
        }
        Location {
            position: x,
            place: Places::Room(letter),
        } => {
            if g.rooms[ti(letter)][x] == '.' {
                if let (Places::Room(_), Places::Room(_)) = (start.place,stop.place) {
                println!("That doesn't exist");
                }
                return false;
            }
        }
    }

    //can't move into places that are closed off
    match stop {
        Location {
            position: x,
            place: Places::Hallway,
        } => {
            if g.hallway[x] != '.' {
                // if let (Places::Room(_), Places::Room(_)) = (start.place,stop.place) {
                // println!("Somebody already there");
                // }
                return false;
            }
        }
        Location {
            position: x,
            place: Places::Room(letter),
        } => {
            if g.rooms[ti(letter)][x] != '.' {
                // if let (Places::Room(_), Places::Room(_)) = (start.place,stop.place) {
                // println!("Somebody already there");
                // }
                return false;
            }
        }
    }

    // can't move across things
    let mut start_hallway = match start {
        Location {
            position: x,
            place: Places::Hallway,
        } => x as i32,
        Location {
            position: _,
            place: Places::Room(letter),
        } => location(letter),
    };
    let mut end_hallway = match stop {
        Location {
            position: x,
            place: Places::Hallway,
        } => x as i32,
        Location {
            position: _,
            place: Places::Room(letter),
        } => location(letter),
    };

    if start_hallway > end_hallway {
        let tmp = start_hallway;
        start_hallway = end_hallway;
        end_hallway = tmp;
    }
    for i in start_hallway + 1..end_hallway {
        if g.hallway[i as usize] != '.' {
            if let (Places::Room(_), Places::Room(_)) = (start.place,stop.place) {
                dbg!(act);
            println!("Somebody in the way");
            }
            return false;
        }
    }

    //must be racist
    //cannot move into another's room
    if let Location {
        position: _,
        place: Places::Room(letter),
    } = stop
    {
        let amigo_type = match start {
            Location {
                position: x,
                place: Places::Hallway,
            } => g.hallway[x],
            Location {
                position: x,
                place: Places::Room(l),
            } => g.rooms[ti(l)][x],
        };
        if char_to_letter(amigo_type) != letter {
            if let (Places::Room(_), Places::Room(_)) = (start.place,stop.place) {

                dbg!(act);
            println!("Different letter already in room");
            }
            return false;
        }
    }
    return true;
}

pub fn generate_all_possible_actions(g: &Game) -> Vec<Action> {
    let mut actions: Vec<Action> = vec![];
    // assuming unobstructed;
    //every top element of room can move out of hallway;
    //
    let rooms = &[Letter::A, Letter::B, Letter::C, Letter::D];
    let hallway_spots = &[0, 1, 3, 5, 7, 9, 10];
    //move top elements into hallway spots;
    for room in rooms {
        for spot in hallway_spots {
            let top = if g.rooms[ti(*room)][0] != '.' {
                0
            } else if g.rooms[ti(*room)][1] != '.' {
                1
            } else {
                continue;
            };
            let act = Action {
                start: Location {
                    position: top,
                    place: Places::Room(*room),
                },
                stop: Location {
                    position: *spot,
                    place: Places::Hallway,
                },
            };
            if path_clear(g, &act) {
                actions.push(act);
            }
        }
    }
    
    // move across rooms
    for room1 in rooms {
        for room2 in rooms {
            let top1 = if g.rooms[ti(*room1)][0] != '.' {
                0
            } else if g.rooms[ti(*room1)][1] != '.' {
                1
            } else {
                continue;
            };

            let top2 = if g.rooms[ti(*room2)][0] != '.' {
                0
            } else if g.rooms[ti(*room2)][1] != '.' {
                1
            } else {
                continue;
            };

            let act = Action {
                start: Location {
                    position: top1,
                    place: Places::Room(*room1),
                },
                stop: Location {
                    position: top2,
                    place: Places::Room(*room2),
                },
            };
            if path_clear(g, &act) {
                println!("room -> room");
                actions.push(act);
            }
        }
    }

    //every element in hallway can move into destination room if it's open for them
    for spot in hallway_spots {
        for room in rooms {
            let top = if g.rooms[ti(*room)][0] != '.' {
                0
            } else if g.rooms[ti(*room)][1] != '.' {
                1
            } else {
                continue;
            };
            let act = Action {
                start: Location {
                    position: *spot,
                    place: Places::Hallway,
                },
                stop: Location {
                    position: top,
                    place: Places::Hallway,
                },
            };
            if path_clear(g, &act) {
                actions.push(act);
            }
        }
    }
    return actions;
}

#[cached]
pub fn find_lowest_actions(g: Game) -> Option<i32> {
    //println!("Starting with: \n{}\n", &g);
    if is_done(&g) {
        println!("Finished @ {}", g.cost);
        return Some(g.cost);
    }
    if g.cost > 13000 {
        //println!("Too expensive");
        return None;
    }

    let possible = generate_all_possible_actions(&g);

    for p in &possible {
        assert!(path_clear(&g, &p));
    }
    if possible.len() == 0 {
        //println!("No moves left");
        return None;
    }

    let mut best = possible
        .iter()
        .map(|a| {
            let moved = move_thing(&g.clone(), a);
            find_lowest_actions(moved)
        })
        .filter(|x| *x != None)
        .map(|x| x.unwrap())
        .collect::<Vec<i32>>();
    best.sort_by(|a, b| a.partial_cmp(b).unwrap());
    if best.len() != 0 {
        return Some(best.first().unwrap().clone());
    } else {
        return None;
    }
}

pub fn testing_things() {
    let mut testing = Game {
        hallway: ['.'; 11],
        rooms: [['B', 'A'], ['C', 'D'], ['B', 'C'], ['D', 'A']], //change to variants someday
        cost: 0,
    };

    testing.hallway[0] = 'B'; 
    testing.rooms[2][0] = '.';

    let right = Action {
        start: Location {
            place: Places::Room(Letter::B),
            position: 0
        },
        stop: Location {
            place: Places::Room(Letter::C),
            position: 0
        }
    };
    assert!(path_clear(&testing, &right));

    let wrong = Action {
        start: Location {
            place: Places::Room(Letter::A),
            position: 0
        },
        stop: Location {
            place: Places::Room(Letter::C),
            position: 0
        }
    };

    assert!(path_clear(&testing, &wrong));
}

pub fn main() {
    testing_things();
    // example_test();
    // let example = Game {
    //     hallway: ['.'; 11],
    //     rooms: [['B', 'A'], ['C', 'D'], ['B', 'C'], ['D', 'A']],
    //     cost: 0,
    // };
    // dbg!(find_lowest_actions(example));
}
