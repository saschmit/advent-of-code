use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Debug)]
enum Side {
    ImmuneSystem,
    Infection,
}

#[derive(Debug)]
struct Unit {
    hp : usize,
    ap : usize,
    attack_type : String,
    initiative : usize,
    weaknesses : HashSet<String>,
    immunities : HashSet<String>,
}

#[derive(Debug)]
struct Group {
    n_units : usize,
    unit : Unit,
}

impl Group {
    pub fn get_effective_power(&self) -> usize {
        self.n_units * self.unit.ap
    }

    pub fn calc_hit(&self, enemy : &Group) -> usize {
        self.get_effective_power() * if enemy.unit.weaknesses.contains(&self.unit.attack_type) {
            2
        } else if enemy.unit.immunities.contains(&self.unit.attack_type) {
            0
        } else {
            1
        }
    }
}

impl Group {
    pub fn new(desc: &str) -> Self {
        let mut grp = Self {
            n_units: 0,
            unit: Unit {
                hp: 0,
                ap: 0,
                attack_type: String::new(),
                initiative: 0,
                weaknesses : HashSet::new(),
                immunities : HashSet::new(),
            }
        };
        let words : Vec<&str> = desc.split_whitespace().collect();
        grp.n_units = words[0].parse().unwrap();
        grp.unit.hp = words[4].parse().unwrap();
        let mut offset = 7;
        loop {
            if words[offset].starts_with("(weak") || words[offset] == "weak" {
                offset += 2;
                loop {
                    if words[offset].ends_with(',') {
                        grp.unit.weaknesses.insert(words[offset].trim_end_matches(',').to_string());
                    } else if words[offset].ends_with(';') {
                        grp.unit.weaknesses.insert(words[offset].trim_end_matches(';').to_string());
                        break;
                    } else if words[offset].ends_with(')') {
                        grp.unit.weaknesses.insert(words[offset].trim_end_matches(')').to_string());
                        break;
                    }
                    offset += 1;
                }

            } else if words[offset].starts_with("(immune") || words[offset] == "immune" {
                offset += 2;
                loop {
                    if words[offset].ends_with(',') {
                        grp.unit.immunities.insert(words[offset].trim_end_matches(',').to_string());
                    } else if words[offset].ends_with(';') {
                        grp.unit.immunities.insert(words[offset].trim_end_matches(';').to_string());
                        break;
                    } else if words[offset].ends_with(')') {
                        grp.unit.immunities.insert(words[offset].trim_end_matches(')').to_string());
                        break;
                    }
                    offset += 1;
                }
            } else if words[offset] == "with" {
                break;
            } else {
                assert!(false);
            }
            offset += 1;
        }
        grp.unit.ap = words[offset + 5].parse().unwrap();
        grp.unit.attack_type = words[offset + 6].to_string();
        grp.unit.initiative = words[offset + 10].parse().unwrap();
        grp
    }
}

#[derive(Debug)]
struct Army {
    side : Side,
    groups : Vec<Group>,
}

impl Army {
    pub fn select_targets(&self, enemy : &Army) -> HashMap<usize, usize> {
        let mut order = vec![];
        for (g, group) in self.groups.iter().enumerate() {
            if group.n_units == 0 {
                continue;
            }
            order.push((group.get_effective_power(), group.unit.initiative, g));
        }
        // sort such that the first to pick are at the end, then reverse
        order.sort_unstable();
        order.reverse();
        let mut taken = HashSet::new();
        let mut mapping = HashMap::new();
        for (_, _, g) in order {
            let mut damages = Vec::new();
            for (e, enemy) in enemy.groups.iter().enumerate() {
                // Skip targets that other groups have already picked or that are already dead
                if taken.contains(&e) || enemy.n_units == 0 {
                    continue;
                }

                let damage = self.groups[g].calc_hit(enemy);
                eprintln!("{:?} group {} would deal defending group {} {} damage", self.side, g+1, e+1, damage);

                if damage != 0 {
                    damages.push((damage, enemy.get_effective_power(), enemy.unit.initiative, e));
                }
            }
            // sort such that the unit to attack is last
            damages.sort_unstable();
            if let Some((damage, _, _, e)) = damages.pop() {
                assert_ne!(damage, 0);
                mapping.insert(g, e);
                taken.insert(e);
            }
        }

        mapping
    }
}

fn parse(input : String) -> (Army, Army) {
    let mut lines = input.lines();

    assert_eq!(Some("Immune System:"), lines.next());
    let mut good = Army {
        side: Side::ImmuneSystem,
        groups: vec![],
    };
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        good.groups.push(Group::new(line));
    }

    assert_eq!(Some("Infection:"), lines.next());
    let mut bad = Army {
        side: Side::Infection,
        groups: vec![],
    };
    loop {
        if let Some(line) = lines.next() {
            bad.groups.push(Group::new(line));
        } else {
            break;
        }
    }

    (good, bad)
}

fn fight(good : &mut Army, bad : &mut Army) -> usize {
    loop {
        eprintln!("Immune System:");
        for g in 0..good.groups.len() {
            if good.groups[g].n_units != 0 {
                eprintln!("Group {} contains {} units", g+1, good.groups[g].n_units);
            }
        }
        eprintln!("Infection:");
        for g in 0..bad.groups.len() {
            if bad.groups[g].n_units != 0 {
                eprintln!("Group {} contains {} units", g+1, bad.groups[g].n_units);
            }
        }
        eprintln!("");

        // Target selection
        let bad2good = bad.select_targets(&good);
        let good2bad = good.select_targets(&bad);
        eprintln!("");

        // Attack
        let mut order = vec![];
        for g in 0..good.groups.len() {
            order.push((good.groups[g].unit.initiative, Side::ImmuneSystem, g));
        }
        for g in 0..bad.groups.len() {
            order.push((bad.groups[g].unit.initiative, Side::Infection, g));
        }
        order.sort_unstable_by_key(|k| -(k.0 as isize));

        for (_, side, g) in order {
            let e = match side {
                Side::ImmuneSystem => {
                    if good.groups[g].n_units == 0 || !good2bad.contains_key(&g) {
                        continue;
                    }
                    good2bad[&g]
                },
                Side::Infection => {
                    if bad.groups[g].n_units == 0 || !bad2good.contains_key(&g) {
                        continue;
                    }
                    bad2good[&g]
                },
            };
            let units_lost = match side {
                Side::ImmuneSystem => {
                    let damage = good.groups[g].calc_hit(&bad.groups[e]);
                    let n = bad.groups[e].n_units;
                    bad.groups[e].n_units = bad.groups[e].n_units.saturating_sub(damage / bad.groups[e].unit.hp);
                    std::cmp::min(n, damage / bad.groups[e].unit.hp)
                },
                Side::Infection => {
                    let damage = bad.groups[g].calc_hit(&good.groups[e]);
                    let n = good.groups[e].n_units;
                    good.groups[e].n_units = good.groups[e].n_units.saturating_sub(damage / good.groups[e].unit.hp);
                    std::cmp::min(n, damage / good.groups[e].unit.hp)
                },
            };
            eprintln!("{:?} group {} attacks defending group {}, killing {} units", side, g+1, e+1, units_lost);
        }

        eprintln!("");

        if good.groups.iter().fold(0, |sum, group| sum + group.n_units) == 0 ||
           bad.groups.iter().fold(0, |sum, group| sum + group.n_units) == 0 {
            break;
        }
    }
    eprintln!("Immune System:");
    let mut printed = false;
    for g in 0..good.groups.len() {
        if good.groups[g].n_units != 0 {
            eprintln!("Group {} contains {} units", g+1, good.groups[g].n_units);
            printed = true;
        }
    }
    if ! printed {
        eprintln!("No groups remain.");
    }
    printed = false;
    eprintln!("Infection:");
    for g in 0..bad.groups.len() {
        if bad.groups[g].n_units != 0 {
            eprintln!("Group {} contains {} units", g+1, bad.groups[g].n_units);
            printed = true;
        }
    }
    if ! printed {
        eprintln!("No groups remain.");
    }
    eprintln!("");
    good.groups.iter().fold(0, |sum, group| sum + group.n_units) +
        bad.groups.iter().fold(0, |sum, group| sum + group.n_units)
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    let filename = &args[1];
    let buff = std::fs::read(filename).unwrap();
    let buff = String::from_utf8(buff).unwrap();
    let (mut good, mut bad) = parse(buff);
    let part1 = fight(&mut good, &mut bad);
    println!("Part 1 = {}", part1);
}
