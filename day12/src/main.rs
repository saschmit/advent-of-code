#[derive(Debug)]
struct Pots {
    state : Vec<bool>,
    transforms : [bool; 32],
    zero_offset : usize,
    gen : usize,
}

impl Pots {
    pub fn new(buff : &str) -> Self {
        let mut obj = Self {
            state : Vec::new(),
            transforms : [
                false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false,
            ],
            zero_offset : 0,
            gen : 0,
        };
        let mut tokens = buff.split_whitespace();
        tokens.next(); // skip "initial"
        tokens.next(); // skip "state:"
        for ch in tokens.next().unwrap().chars() {
            obj.state.push(match ch {
                '.' => false,
                '#' => true,
                _ => unreachable!(),
            });
        }

        loop {
            let from = match tokens.next() {
                None => break,
                Some(x) => match x {
                    "....." => 0b00000,
                    "....#" => 0b00001,
                    "...#." => 0b00010,
                    "...##" => 0b00011,
                    "..#.." => 0b00100,
                    "..#.#" => 0b00101,
                    "..##." => 0b00110,
                    "..###" => 0b00111,
                    ".#..." => 0b01000,
                    ".#..#" => 0b01001,
                    ".#.#." => 0b01010,
                    ".#.##" => 0b01011,
                    ".##.." => 0b01100,
                    ".##.#" => 0b01101,
                    ".###." => 0b01110,
                    ".####" => 0b01111,
                    "#...." => 0b10000,
                    "#...#" => 0b10001,
                    "#..#." => 0b10010,
                    "#..##" => 0b10011,
                    "#.#.." => 0b10100,
                    "#.#.#" => 0b10101,
                    "#.##." => 0b10110,
                    "#.###" => 0b10111,
                    "##..." => 0b11000,
                    "##..#" => 0b11001,
                    "##.#." => 0b11010,
                    "##.##" => 0b11011,
                    "###.." => 0b11100,
                    "###.#" => 0b11101,
                    "####." => 0b11110,
                    "#####" => 0b11111,
                    _ => unreachable!(),
                },
            };
            tokens.next();
            let to = match tokens.next() {
                None => break,
                Some(x) => match x {
                    "." => false,
                    "#" => true,
                    _ => unreachable!(),
                },
            };
            obj.transforms[from] = to;
        }
        assert_eq!(obj.transforms[0], false);
        obj
    }

    pub fn generate(&mut self) {
        let mut new_state = Vec::with_capacity(self.state.len()+4);
        let mut zero_shift = 0;
        for center in -2 .. self.state.len() as isize + 2 {
            let mut bitmap : usize = 0;
            for offset in -2..=2 {
                let i = center + offset;
                let bit = if i < 0 || i >= self.state.len() as isize { 0 }
                          else if self.state[i as usize] { 1 } else { 0 };
                bitmap = bitmap << 1 | bit;
            }
            let next = self.transforms[bitmap];
            if center < 0 && zero_shift == 0 {
                if next {
                    zero_shift = (-center) as usize;
                    new_state.push(next);
                }
            } else {
                new_state.push(next);
            }
        }
        // Trim off the end
        while ! new_state[new_state.len() - 1] {
            new_state.pop();
        }
        self.state.clear();
        self.state.append(&mut new_state);
        self.zero_offset += zero_shift as usize;
        self.gen += 1;
    }

    pub fn sum_live_pots(&self) -> isize {
        let mut sum : isize = 0;
        for i in 0..self.state.len() {
            if self.state[i] {
                sum += i as isize - self.zero_offset as isize;
            }
        }
        sum
    }
}

impl std::fmt::Display for Pots {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut pots = String::new();
        for i in 0..self.state.len() {
            if i == self.zero_offset {
                pots += "\x1b[31;1m";
            }
            pots.push(if self.state[i] { '#' } else { '.' });
            if i == self.zero_offset {
                pots += "\x1b[0m";
            }
        }
        write!(f, "{:2}: {}", self.gen, pots)
    }
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let buff = std::fs::read(filename).unwrap();
    let buff = String::from_utf8(buff).unwrap();
    let mut pots = Pots::new(&buff);
    println!("{}", pots);
    for _ in 0..20 {
        pots.generate();
        println!("{}", pots);
    }
    println!("{}", pots.sum_live_pots());
}
