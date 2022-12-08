use std::collections::HashMap;

#[cfg(test)]
mod tests;

#[derive(Clone,Copy,Debug,PartialEq)]
enum Team {
    Elf,
    Goblin,
}

impl std::fmt::Display for Team {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", match self {
            Team::Elf => "Elf",
            Team::Goblin => "Goblin",
        })
    }
}

struct Unit {
    team : Team,
    hp : usize,
    ap : usize,
    played : bool,
}

#[derive(Debug,Hash,Eq,PartialEq,Ord,PartialOrd)]
// Defined this way around (y first) so that derive(Ord) DTRT
struct Pos {
    y : usize,
    x : usize,
}

impl std::fmt::Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[repr(u8)]
#[derive(Clone,Eq,PartialEq)]
enum Square {
    Wall = b'#',
    Floor = b'.',
    Elf = b'E',
    Goblin = b'G',
    EOL = b'\n',
}

struct Game {
    map : Vec<Square>,
    height : usize,
    width : usize,
    units : HashMap<Pos, Unit>,
    round : usize,
}

impl Game {
    pub fn new(buf : &[u8], tilt : usize) -> Self {
        let mut game = Game {
            map : Vec::with_capacity(buf.len()),
            height : 0,
            width : 0,
            units : HashMap::new(),
            round : 0,
        };
        for (i, byte) in buf.iter().enumerate() {
            let pos = Pos {
                x : if game.width == 0 { i } else { i % (game.width + 1) },
                y : game.height,
            };
            game.map.push(match byte {
                b'\n' => {
                    if game.height == 0 {
                        game.width = i;
                    }
                    game.height += 1;
                    Square::EOL
                },
                b'G' => {
                    game.units.insert(pos, Unit {
                        team : Team::Goblin,
                        hp : 200,
                        ap : 3,
                        played : false,
                    });
                    Square::Goblin
                },
                b'E' => {
                    game.units.insert(pos, Unit {
                        team : Team::Elf,
                        hp : 200,
                        ap : 3 + tilt,
                        played : false,
                    });
                    Square::Elf
                },
                b'#' => Square::Wall,
                b'.' => Square::Floor,
                _ => unreachable!(),
            });
        }
        game
    }

    fn at(&self, pos : &Pos) -> usize {
        pos.y * (self.width+1) + pos.x
    }

    fn find_next_unit(&self) -> Option<Pos> {
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = Pos { x, y };
                let offset = self.at(&pos);
                match self.map[offset] {
                    Square::Elf | Square::Goblin => {
                        if ! self.units[&pos].played {
                            return Some(pos);
                        }
                    },
                    _ => (),
                }
            }
        }
        None
    }

    fn find_targets(&self, team : Team) -> Vec<Pos> {
        let mut targets : Vec<Pos> = Vec::new();
        for (pos, unit) in &self.units {
            if unit.team != team {
                targets.push(Pos { .. *pos });
            }
        }
        targets
    }

    fn find_adjacent_spaces<P>(&self, target : &Pos, predicate : P) -> Vec<Pos>
        where P: Fn(&Square, &Pos) -> bool {
        let mut spaces = Vec::new();
        if target.y > 0 {
            let space = Pos { y: target.y - 1, .. *target };
            if predicate(&self.map[self.at(&space)], &space) {
                spaces.push(space);
            }
        }
        if target.x > 0 {
            let space = Pos { x: target.x - 1, .. *target };
            if predicate(&self.map[self.at(&space)], &space) {
                spaces.push(space);
            }
        }
        if target.x < std::usize::MAX {
            let space = Pos { x: target.x + 1, .. *target };
            if predicate(&self.map[self.at(&space)], &space) {
                spaces.push(space);
            }
        }
        if target.y < std::usize::MAX {
            let space = Pos { y: target.y + 1, .. *target };
            if predicate(&self.map[self.at(&space)], &space) {
                spaces.push(space);
            }
        }
        spaces
    }

    fn find_next_step(&self, source : &Pos, targets : &Vec<Pos>) -> Option<Pos> {
        #[derive(Debug)]
        struct Node {
            dist : usize,
            prev : Vec<Pos>,
        }
        let mut graph = HashMap::new();
        let mut todo = Vec::new();

        // Initialize graph & todo list
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = Pos { x, y };
                match self.map[self.at(&pos)] {
                    Square::Floor => {
                        todo.push(Pos { .. pos });
                        graph.insert(pos, Node { dist: std::usize::MAX, prev: Vec::new(), });
                    },
                    _ => (),
                }
            }
        }
        todo.push(Pos { .. *source });
        graph.insert(Pos { .. *source }, Node { dist: 0, prev: Vec::new(), });

        // Go through all the graph nodes and find the optimal paths
        loop {
            // Sort by distance such that the last elements have the minimum
            todo.sort_unstable_by(|a, b| {
                let a = &graph.get(a).unwrap().dist;
                let b = &graph.get(b).unwrap().dist;
                b.cmp(a)
            });

            if let Some(current) = todo.pop() {
                for next in self.find_adjacent_spaces(&current, |_,pos| graph.contains_key(&pos)) {
                    // If our current node is infinitely far from the source, there are no more
                    // paths
                    if graph[&current].dist == std::usize::MAX {
                        todo.clear();
                        break;
                    }

                    let alt = graph[&current].dist + 1;
                    let mut node = graph.get_mut(&next).unwrap();
                    if alt < node.dist {
                        node.dist = alt;
                        node.prev.clear();
                        node.prev.push(Pos { .. current });
                    } else if alt == node.dist {
                        node.prev.push(Pos { .. current });
                    }
                }
            } else {
                break;
            }
        }

        assert_eq!(0, todo.len());

        if let Some(min_dist_tgt_pos) = targets.iter().min_by_key(|target| graph[&target].dist) {
            let min_dist = graph[&min_dist_tgt_pos].dist;
            // If our best path is infinitely long, there is no path
            if min_dist == std::usize::MAX {
                None
            } else {
                let mut locs = Vec::new();
                for target in targets {
                    let node = &graph[&target];
                    if node.dist == min_dist {
                        eprintln!("Found {}-step path to {}", node.dist, target);
                        locs.push(target);
                    }
                }
                locs.sort();
                locs.truncate(1);
                let mut first_steps = Vec::new();

                while !locs.is_empty() {
                    let next = locs.pop().unwrap();
                    let node = &graph[&next];
                    for prev in &node.prev {
                        if prev == source {
                            first_steps.push(next);
                        } else {
                            locs.push(prev);
                        }
                    }
                }

                first_steps.sort();
                Some(Pos { .. *first_steps[0] })
            }
        } else {
            None
        }
    }

    fn select_target(&self, here : &Pos, unit : &Unit) -> Pos {
        let candidates = self.find_adjacent_spaces(&here, |sq,_pos| {
            match sq {
                Square::Goblin => unit.team == Team::Elf,
                Square::Elf => unit.team == Team::Goblin,
                _ => false,
            }
        });
        Pos { .. *candidates.iter().min_by_key(|pos| self.units[*pos].hp).unwrap() }
    }

    pub fn fight(&mut self) -> (usize, usize, Team) {
        'battle: loop {
            println!("{}", self);
            eprintln!("Starting round");
            'turn: while let Some(mut here) = self.find_next_unit() {
                let mut unit = self.units.remove(&here).unwrap();
                eprintln!("{} @ {} takes a turn", unit.team, here);
                let targets = self.find_targets(unit.team);
                if targets.len() == 0 {
                    self.units.insert(here, unit);
                    break 'battle;
                }
                let mut in_range_spaces = Vec::new();
                for target in targets {
                    in_range_spaces.append(
                        &mut self.find_adjacent_spaces(&target,
                            |sq,pos| *sq == Square::Floor || *pos == here));
                }
                if in_range_spaces.len() == 0 {
                    unit.played = true;
                    self.units.insert(here, unit);
                    continue 'turn;
                }
                in_range_spaces.sort();
                let (pos, mut enemy) = if let Ok(_) = in_range_spaces.binary_search(&here) {
                    // Stay
                    let pos = self.select_target(&here, &unit);
                    let enemy = self.units.remove(&pos).unwrap();
                    (pos, enemy)
                } else {
                    // Move, if possible
                    eprintln!("{} @ {} wants to move", unit.team, here);
                    if let Some(there) = self.find_next_step(&here, &in_range_spaces) {
                        eprintln!("Move from {} to {}", here, there);
                        let old_offset = self.at(&here);
                        let new_offset = self.at(&there);
                        self.map.swap(old_offset, new_offset);
                        here = Pos { .. there };

                        // Let's see if we're now in range to attack
                        if let Ok(_) = in_range_spaces.binary_search(&there) {
                            let pos = self.select_target(&there, &unit);
                            let enemy = self.units.remove(&pos).unwrap();
                            (pos, enemy)
                        } else {
                            // Unable to attack
                            eprintln!("Out of range for attack");
                            unit.played = true;
                            self.units.insert(there, unit);
                            continue 'turn;
                        }
                    } else {
                        // Unable to move
                        eprintln!("Nowhere to move");
                        unit.played = true;
                        self.units.insert(here, unit);
                        continue 'turn;
                    }
                };


                // Fight
                if enemy.hp <= unit.ap {
                    eprintln!("{} @ {} kills {} @ {}", unit.team, here, enemy.team, pos);
                    // Destroy this enemy
                    let pos_offset = self.at(&pos);
                    self.map[pos_offset] = Square::Floor;

                    unit.played = true;
                    self.units.insert(here, unit);

                    if enemy.team == Team::Elf && enemy.ap != 3 {
                        eprintln!("An elf died.  Unacceptable!");
                        return (0, 0, Team::Goblin);
                    }
                } else {
                    eprintln!("{} @ {} attacks {} @ {}", unit.team, here, enemy.team, pos);
                    enemy.hp -= unit.ap;

                    unit.played = true;
                    self.units.insert(here, unit);
                    self.units.insert(pos, enemy);
                }
            }

            eprintln!("End of round {}", self.round);

            // Prep the units for the next round
            for (_, mut unit) in &mut self.units {
                unit.played = false;
            }

            // Start the next round
            self.round += 1;

        };

        (self.round,
         self.units.iter().fold(0, |acc, unit| acc + unit.1.hp),
         self.units.iter().nth(0).unwrap().1.team)
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f : &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut out = String::new();
        let mut right = String::new();
        let mut row_units = Vec::new();
        let mut y = 0;
        for square in &self.map {
            out += match square {
                Square::Elf => "🧝",
                Square::Goblin => "🧟",
                Square::Floor => "  ",
                Square::Wall => "██",
                Square::EOL => {
                    right.clear();
                    for (pos, unit) in &self.units {
                        if pos.y == y {
                            row_units.push((pos, unit));
                        }
                    }
                    row_units.sort_unstable_by_key(|pu| pu.0.x);
                    for (pos, unit) in row_units.drain(..) {
                        right += &format!(" {}{}({}){}", unit.team, pos, unit.hp, if unit.played { "*" } else { "" });
                    }
                    right += "\n";
                    y += 1;
                    &right
                },
            };
        }
        write!(f, "Round {}\n{}", self.round, out)
    }
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    let filename = &args[1];
    let buff = std::fs::read(filename).unwrap();
    let mut game = Game::new(&buff, 0);
    let result = game.fight();
    println!("Combat ends after {} full rounds", result.0);
    println!("{} win with {} total hit points left", match result.2 {
            Team::Elf => "Elves", Team::Goblin => "Goblins" }, result.1);
    println!("Part 1: {} ({} x {})", result.0 * result.1, result.0, result.1);

    let mut tilt = 1;
    loop {
        println!("Restarting with Elven AP of {}", 3 + tilt);
        let mut game = Game::new(&buff, tilt);
        let result = game.fight();
        if result.2 == Team::Goblin {
            tilt += 1;
            continue;
        }
        println!("Combat ends after {} full rounds", result.0);
        println!("{} win with {} total hit points left", match result.2 {
                Team::Elf => "Elves", Team::Goblin => "Goblins" }, result.1);
        println!("Part 2: {} ({} x {})", result.0 * result.1, result.0, result.1);
        break;
    }
}
