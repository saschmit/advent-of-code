#[derive(Debug,Copy,Clone)]
enum Acre {
    Open,
    Trees,
    Lumberyard,
}

struct Base {
    area : Vec<Vec<Acre>>,
    time : usize,
}

impl Base {
    pub fn new(buff : &[u8]) -> Self {
        let mut out = Self {
            area : vec![Vec::new()],
            time : 0,
        };
        let mut y : usize = 0;
        for byte in buff {
            match byte {
                b'\n' => {
                    y += 1;
                    out.area.push(Vec::new());
                },
                b'.' => {
                    out.area[y].push(Acre::Open);
                },
                b'|' => {
                    out.area[y].push(Acre::Trees);
                },
                b'#' => {
                    out.area[y].push(Acre::Lumberyard);
                },
                _ => unreachable!(),
            }
        }

        out
    }

    fn get_adjacent(&self, y : usize, x : usize) -> Vec<(usize, usize)> {
        let mut out = Vec::new();
        for y_off in -1 ..= 1 {
            for x_off in -1 ..= 1 {
                if y_off == 0 && x_off == 0 {
                    continue;
                }
                let y1 = y as isize + y_off;
                let x1 = x as isize + x_off;
                if y1 >= 0 && x1 >= 0 {
                    let y1 = y1 as usize;
                    let x1 = x1 as usize;
                    if y1 < self.area.len() && x1 < self.area[y1].len() {
                        out.push((y1, x1));
                    }
                }
            }
        }
        out
    }

    fn count_adjacent(&self, y : usize, x : usize) -> (usize, usize, usize) {
        let mut cnt_opens = 0;
        let mut cnt_trees = 0;
        let mut cnt_yards = 0;
        for (y, x) in self.get_adjacent(y, x) {
            match self.area[y][x] {
                Acre::Open => cnt_opens += 1,
                Acre::Trees => cnt_trees += 1,
                Acre::Lumberyard => cnt_yards += 1,
            }
        }
        (cnt_opens, cnt_trees, cnt_yards)
    }

    pub fn tick(&mut self) {
        let mut new = self.area.clone();
        for y in 0..self.area.len() {
            for x in 0..self.area[y].len() {
                let counts = self.count_adjacent(y, x);
                match self.area[y][x] {
                    Acre::Open => if counts.1 >= 3 {
                        new[y][x] = Acre::Trees;
                    },
                    Acre::Trees => if counts.2 >= 3 {
                        new[y][x] = Acre::Lumberyard;
                    },
                    Acre::Lumberyard => if !(counts.1 >= 1 && counts.2 >= 1) {
                        new[y][x] = Acre::Open;
                    },
                }
            }
        }
        self.area = new;
        self.time += 1;
    }

    pub fn get_counts(&self) -> (usize, usize, usize) {
        let mut cnt_opens = 0;
        let mut cnt_trees = 0;
        let mut cnt_yards = 0;
        for row in &self.area {
            for acre in row {
                match acre {
                    Acre::Open => cnt_opens += 1,
                    Acre::Trees => cnt_trees += 1,
                    Acre::Lumberyard => cnt_yards += 1,
                }
            }
        }
        (cnt_opens, cnt_trees, cnt_yards)
    }

    pub fn get_time(&self) -> usize {
        self.time
    }
}

impl std::fmt::Display for Base {
    fn fmt(&self, f : &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut out = String::new();
        for y in 0..self.area.len() {
            for x in 0..self.area[y].len() {
                out += match self.area[y][x] {
                    Acre::Open => "\x1b[32m.",
                    Acre::Trees => "\x1b[92m|",
                    Acre::Lumberyard => "\x1b[33m#",
                }
            }
            out += "\x1b[0m\n";
        }
        write!(f, "{}", out)
    }
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    let filename = &args[1];
    let buff = std::fs::read(filename).unwrap();
    let mut base = Base::new(&buff);
    println!("Initial state:\n{}", base);
    for _ in 0..10 {
        base.tick();
        println!("After {} minutes:\n{}", base.get_time(), base);
    }
    let counts = base.get_counts();
    println!("Part 1: {} ({} x {})", counts.1 * counts.2, counts.1, counts.2);
}
