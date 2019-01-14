use std::collections::VecDeque;

#[derive(Copy,Clone,Debug,Eq,PartialEq)]
#[repr(u8)]
enum Square {
    Sand = b'.',
    Clay = b'#',
    Spring = b'+',
    Water = b'~',
    Flow = b'|',
}

const SPRING_X : usize = 500;

#[derive(Debug)]
struct Ground {
    scan : Vec<VecDeque<Square>>,
    spring_offset : usize,
    min_y : usize,
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl Ground {
    pub fn new(buff : &[u8]) -> Self {
        // Initialize Ground
        let mut out = Self {
            scan : vec![VecDeque::new()],
            spring_offset : 0,
            min_y : std::usize::MAX,
        };
        out.scan[0].push_back(Square::Spring);

        // Parse what to do from buff
        enum Clay {
            Horiz(usize, usize, usize),
            Vert(usize, usize, usize),
        }
        let mut offset = 0;
        let mut clay = Vec::new();
        while offset < buff.len() {
            match buff[offset] {
                b'x' => {
                    // Vertical
                    offset += 2;
                    let x = &buff[offset..].split(|c| *c == b',').next().unwrap();
                    offset += x.len() + 4;
                    let x = usize::from_str_radix(&String::from_utf8_lossy(x), 10).unwrap();

                    let y1 = &buff[offset..].split(|c| *c == b'.').next().unwrap();
                    offset += y1.len() + 2;
                    let y1 = usize::from_str_radix(&String::from_utf8_lossy(y1), 10).unwrap();

                    let y2 = &buff[offset..].split(|c| *c == b'\n').next().unwrap();
                    offset += y2.len() + 1;
                    let y2 = usize::from_str_radix(&String::from_utf8_lossy(y2), 10).unwrap();
                    
                    assert!(y1 < y2);
                    clay.push(Clay::Vert(x, y1, y2));
                },
                b'y' => {
                    // Vertical
                    offset += 2;
                    let y = &buff[offset..].split(|c| *c == b',').next().unwrap();
                    offset += y.len() + 4;
                    let y = usize::from_str_radix(&String::from_utf8_lossy(y), 10).unwrap();

                    let x1 = &buff[offset..].split(|c| *c == b'.').next().unwrap();
                    offset += x1.len() + 2;
                    let x1 = usize::from_str_radix(&String::from_utf8_lossy(x1), 10).unwrap();

                    let x2 = &buff[offset..].split(|c| *c == b'\n').next().unwrap();
                    offset += x2.len() + 1;
                    let x2 = usize::from_str_radix(&String::from_utf8_lossy(x2), 10).unwrap();

                    assert!(x1 < x2);
                    clay.push(Clay::Horiz(y, x1, x2));
                },
                _ => unreachable!(),
            }
            assert_eq!(buff[offset-1], b'\n');
        }

        // Figure out bounds of ground
        let mut min_x = std::usize::MAX;
        let mut max_x = 0;
        let mut min_y = std::usize::MAX;
        let mut max_y = 0;
        for line in &clay {
            match line {
                Clay::Horiz(y, x1, x2) => {
                    min_y = std::cmp::min(min_y, *y);
                    max_y = std::cmp::max(max_y, *y);
                    min_x = std::cmp::min(min_x, *x1);
                    max_x = std::cmp::max(max_x, *x2);
                },
                Clay::Vert(x, y1, y2) => {
                    min_x = std::cmp::min(min_x, *x);
                    max_x = std::cmp::max(max_x, *x);
                    min_y = std::cmp::min(min_y, *y1);
                    max_y = std::cmp::max(max_y, *y2);
                },
            }
        }

        // Save the minimum y -- we'll need it later
        out.min_y = min_y;

        // Grow ground to match bounds
        for y in 1..=max_y {
            out.scan.push(VecDeque::new());
            for _ in 0..out.scan[0].len() {
                out.scan[y].push_back(Square::Sand);
            }
        }
        if min_x < SPRING_X {
            out.grow(SPRING_X - min_x, Direction::Left);
        }
        if max_x > SPRING_X {
            out.grow(max_x - SPRING_X, Direction::Right);
        }

        // Draw clay into ground
        for line in clay {
            match line {
                Clay::Horiz(y, x1, x2) => {
                    for x in out.x(x1)..=out.x(x2) {
                        out.scan[y][x] = Square::Clay;
                    }
                },
                Clay::Vert(x, y1, y2) => {
                    let x = out.x(x);
                    for y in y1..=y2 {
                        out.scan[y][x] = Square::Clay;
                    }
                },
            }
        }

        out
    }

    fn x(&self, x : usize) -> usize {
        if x < SPRING_X {
            self.spring_offset - (SPRING_X - x)
        } else {
            self.spring_offset + (x - SPRING_X)
        }
    }

    fn grow(&mut self, count : usize, direction : Direction) {
        for y in 0..self.scan.len() {
            for _ in 0..count {
                match direction {
                    Direction::Left  => self.scan[y].push_front(Square::Sand),
                    Direction::Right => self.scan[y].push_back(Square::Sand),
                }
            }
        }
        match direction {
            Direction::Left => self.spring_offset += count,
            _ => (),
        }
    }

    fn find_bounds(&self, pos : (usize, usize)) -> (Option<usize>, Option<usize>, bool) {
        let mut left : Option<usize> = None;
        let mut right : Option<usize> = None;
        let mut stable = true;
        let (x_center, y) = pos;

        // If there's nothing underneath, we're done
        if y + 1 >= self.scan.len() {
           return (None, None, false);
        }

        // Look left...
        for x in (0..x_center).rev() {
            // If there's nothing underneath us, stop
            match self.scan[y+1][x] {
                Square::Flow | Square::Sand => {
                    left = Some(x-1);
                    stable = false;
                    break;
                },
                Square::Clay | Square::Water => (),
                Square::Spring => unreachable!(),
            }

            // If we hit a wall, stop
            match self.scan[y][x] {
                Square::Clay => {
                    left = Some(x);
                    break;
                },
                _ => (),
            }
        }

        // Look right...
        for x in x_center + 1 .. self.scan[y].len() {
            // If there's nothing underneath us, stop
            match self.scan[y+1][x] {
                Square::Flow | Square::Sand => {
                    right = Some(x+1);
                    stable = false;
                    break;
                },
                Square::Clay | Square::Water => (),
                Square::Spring => unreachable!(),
            }

            // If we hit a wall, stop
            match self.scan[y][x] {
                Square::Clay => {
                    right = Some(x);
                    break;
                },
                _ => (),
            }
        }
        (left, right, stable)
    }

    pub fn flow(&mut self) {
        let mut q = std::collections::VecDeque::new();
        q.push_back(0);
        while q.len() != 0 {
            let y = q.pop_front().unwrap();
            if y >= self.scan.len() {
                eprintln!("Skipping \"row\" {}, which doesn't exist", y);
                continue;
            }
            let mut row_scan = 0..self.scan[y].len();
            while let Some(x) = row_scan.next() {
                match self.scan[y][x] {
                    Square::Spring | Square::Flow => {
                        if y+1 >= self.scan.len() {
                            continue;
                        }
                        match self.scan[y+1][x] {
                            Square::Sand => {
                                self.scan[y+1][x] = Square::Flow;

                                // Schedule a scan of the next row
                                eprintln!("Water falling, adding row {}", y+1);
                                q.push_back(y+1);
                            },
                            Square::Clay | Square::Water => {
                                let (left, right, stable) = self.find_bounds((x, y));
                                let fill = if stable {
                                    // Schedule a rescan of the previous row
                                    eprintln!("Water filling, adding row {}", y-1);
                                    q.push_back(y-1);
                                    Square::Water
                                } else {
                                    Square::Flow
                                };

                                let (left, rt_off) = match left {
                                    Some(x1) => {
                                        (x1 + 1, 0)
                                    },
                                    None => {
                                        assert!(!stable);
                                        // Fell off the left, so grow left
                                        self.grow(1, Direction::Left);

                                        // Resize the iteration range, starting with the next
                                        // element (x+1, adjusted for the shift right)
                                        eprintln!("Grew left, adjusting scan range to {}..{}", x, self.scan[y].len());
                                        row_scan = x..self.scan[y].len();

                                        (0, 1)
                                    }
                                };

                                let right = match right {
                                    Some(x2) => x2,
                                    None => {
                                        assert!(!stable);
                                        // Fell off the right, so grow right
                                        self.grow(1, Direction::Right);

                                        // Resize the iteration range
                                        row_scan = x + rt_off..self.scan[y].len();

                                        self.scan[y].len()
                                    },
                                };

                                eprintln!("Filling {}..{}, {} with {}", left, right, y, fill as u8 as char);
                                for x in left..right {
                                    self.scan[y][x] = fill;
                                }

                                if y + 1 < self.scan.len() {
                                    let mut changed = false;
                                    for x in left..right {
                                        match self.scan[y+1][x] {
                                            Square::Sand => {
                                                changed = true;
                                                self.scan[y+1][x] = Square::Flow;
                                            },
                                            _ => (),
                                        }
                                    }
                                    if changed {
                                        eprintln!("Water falling, adding row {}", y+1);
                                        q.push_back(y+1);
                                    }
                                }
                            },
                            Square::Flow => (),
                            Square::Spring => unreachable!(),
                        }
                    },
                    _ => (),
                }
            }
            eprintln!("After row {} processing:\n{}", y, self);
        }
    }

    pub fn count_water(&self) -> usize {
        let mut count = 0;
        for y in self.min_y..self.scan.len() {
            for x in 0..self.scan[y].len() {
                count += match self.scan[y][x] {
                    Square::Water | Square::Flow => 1,
                    _ => 0,
                };
            }
        }
        count
    }
}

impl std::fmt::Display for Ground {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut out = String::new();
        for y in 0..self.scan.len() {
            for x in 0..self.scan[y].len() {
                out += match self.scan[y][x] {
                    Square::Sand => "\x1b[33m.",
                    Square::Clay => "\x1b[37m#",
                    Square::Spring => "\x1b[90m+",
                    Square::Water => "\x1b[94m~",
                    Square::Flow => "\x1b[36m|",
                };
            }
            out += "\x1b[0m\n";
        }
        //write!(f, "{}", &out[..out.len()-1])
        write!(f, "{}", out)
    }
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    let filename = &args[1];
    let buff = std::fs::read(filename).unwrap();
    let mut ground = Ground::new(&buff);
    println!("Initial State:\n{}", ground);
    ground.flow();
    println!("After water flow:\n{}", ground);
    println!("Part 1: {} m²", ground.count_water());
}
