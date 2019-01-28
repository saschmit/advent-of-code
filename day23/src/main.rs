#[derive(Debug)]
struct Pos {
    x : i64,
    y : i64,
    z : i64,
}
impl Pos {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Self {
            x,
            y,
            z,
        }
    }

    pub fn calc_dist(&self, other : &Pos) -> u64 {
        use std::cmp::{min,max};

        let min_x = min(self.x, other.x);
        let max_x = max(self.x, other.x);
        let min_y = min(self.y, other.y);
        let max_y = max(self.y, other.y);
        let min_z = min(self.z, other.z);
        let max_z = max(self.z, other.z);

        (max_x - min_x) as u64
        + (max_y - min_y) as u64
        + (max_z - min_z) as u64
    }
}

#[derive(Debug)]
struct Nanobot {
    pos : Pos,
    signal_radius : u64,
}
impl Nanobot {
    pub fn new(pos : Pos, signal_radius : u64) -> Self {
        Self {
            pos,
            signal_radius,
        }
    }

    pub fn in_range(&self, other : &Self) -> bool {
        self.pos.calc_dist(&other.pos) <= self.signal_radius
    }
}

fn parse_input(buff : &[u8]) -> Vec<Nanobot> {
    let mut out = Vec::new();

    for line in String::from_utf8_lossy(buff).lines() {
        //let (_, _, x, y, z, _, r) = line.split(|x| match x {
        let tokens : Vec<&str> = line.split(|x| match x {
            '=' | '<' | '>' | ',' => true,
            _ => false,
        }).collect();
        if tokens.len() != 8 {
            eprintln!("End?");
            break;
        }
        let x = tokens[2].parse().unwrap();
        let y = tokens[3].parse().unwrap();
        let z = tokens[4].parse().unwrap();
        let r = tokens[7].parse().unwrap();
        out.push(Nanobot::new(Pos::new(x, y, z), r));
    }
    out
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    let filename = &args[1];
    let buff = std::fs::read(filename).unwrap();
    let nanos = parse_input(&buff);
    let mut max_sig = 0;
    let mut strongest = usize::max_value();
    for (i,nano) in nanos.iter().enumerate() {
        if nano.signal_radius > max_sig {
            strongest = i;
            max_sig = nano.signal_radius;
        }
    }
    let mut in_range = 0;
    let strongest = &nanos[strongest];
    for nano in &nanos {
        if strongest.in_range(&nano) {
            in_range += 1;
        }
    }
    println!("Part 1 = {}", in_range);
}
