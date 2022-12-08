use std::collections::HashMap;

#[derive(Debug)]
enum Action {
    TurnLeft,
    Forward,
    TurnRight,
}

const CART_ACTION_SEQ : [Action; 3] = [
    Action::TurnLeft, Action::Forward, Action::TurnRight
];

#[derive(Debug)]
#[repr(u8)]
enum Dir {
    Up = b'^',
    Down = b'v',
    Left = b'<',
    Right = b'>',
}

#[derive(Debug,PartialEq,Eq,Hash)]
struct Pos {
    x : usize,
    y : usize,
}

impl std::fmt::Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[derive(Debug,PartialEq)]
#[repr(u8)]
enum Track {
    Curve1 = b'/',
    Curve2 = b'\\',
    Horiz = b'-',
    Vert = b'|',
    Cross = b'+',
    OffTrack = b'!',
}

#[derive(Debug)]
struct Cart {
    next_action : usize,
    direction : Dir,
    underneath : Track,
    last_moved_on_tick : usize,
}

impl Cart {
    pub fn draw(&self) -> u8 {
        match self.direction {
            Dir::Up => b'^',
            Dir::Down => b'v',
            Dir::Left => b'<',
            Dir::Right => b'>',
        }
    }
}

#[derive(Debug)]
struct Mine {
    map : Vec<u8>,
    height : usize,
    width : usize,

    carts : HashMap<Pos, Cart>,
    tick : usize,
}

enum PlaceResult {
    Safe,
    Crash(Pos),
}

enum Event {
    Crash,
    LastCart,
}

impl Mine {
    pub fn new(buff : Vec<u8>) -> Mine {
        let mut mine = Mine {
            map : buff,
            carts : HashMap::new(),
            tick : 0,
            height : 0,
            width : 0,
        };
        for (i, val) in mine.map.iter().enumerate() {
            let pos = Pos {
                x : if mine.width == 0 { i } else { i % (mine.width + 1) },
                y : mine.height,
            };
            match val {
                b'\n' => {
                    mine.height += 1;
                    if mine.width == 0 {
                        mine.width = i;
                    } else {
                        assert_eq!(mine.height - 1, i % (mine.width));
                        assert_eq!(0, (i + 1) % (mine.width + 1));
                    }
                },
                b' ' => (),
                b'/' => (),
                b'\\' => (),
                b'-' => (),
                b'|' => (),
                b'+' => (),
                b'^' => {
                    mine.carts.insert(pos, Cart {
                        next_action : 0,
                        direction : Dir::Up,
                        underneath : Track::Vert,
                        last_moved_on_tick : mine.tick,
                    });
                },
                b'v' => {
                    mine.carts.insert(pos, Cart {
                        next_action : 0,
                        direction : Dir::Down,
                        underneath : Track::Vert,
                        last_moved_on_tick : mine.tick,
                    });
                },
                b'<' => {
                    mine.carts.insert(pos, Cart {
                        next_action : 0,
                        direction : Dir::Left,
                        underneath : Track::Horiz,
                        last_moved_on_tick : mine.tick,
                    });
                },
                b'>' => {
                    mine.carts.insert(pos, Cart {
                        next_action : 0,
                        direction : Dir::Right,
                        underneath : Track::Horiz,
                        last_moved_on_tick : mine.tick,
                    });
                },
                _ => unreachable!(),
            }
        }
        mine
    }

    fn at(&self, pos : &Pos) -> usize {
        pos.y * (self.width+1) + pos.x
    }

    fn lift(&mut self, pos : &Pos) -> Cart {
        let mut cart = self.carts.remove(&pos).unwrap();
        let offset = self.at(&pos);
        self.map[offset] = cart.underneath as u8;
        cart.underneath = Track::OffTrack;
        cart
    }

    fn place(&mut self, pos : Pos, mut cart : Cart) -> PlaceResult {
        let offset = self.at(&pos);
        assert_eq!(cart.underneath, Track::OffTrack);
        match self.map[offset] {
                b' ' | b'\n' => panic!("Cart has run off the track @ {} / {}!", pos, offset),
                b'/' => {
                    cart.direction = match cart.direction {
                        Dir::Up => Dir::Right,
                        Dir::Down => Dir::Left,
                        Dir::Left => Dir::Down,
                        Dir::Right => Dir::Up,
                    };
                    cart.underneath = Track::Curve1;
                    self.map[offset] = cart.draw();
                    self.carts.insert(pos, cart);
                    PlaceResult::Safe
                },
                b'\\' => {
                    cart.direction = match cart.direction {
                        Dir::Up => Dir::Left,
                        Dir::Down => Dir::Right,
                        Dir::Left => Dir::Up,
                        Dir::Right => Dir::Down,
                    };
                    cart.underneath = Track::Curve2;
                    self.map[offset] = cart.draw();
                    self.carts.insert(pos, cart);
                    PlaceResult::Safe
                },
                b'-' => {
                    cart.underneath = Track::Horiz;
                    self.map[offset] = cart.draw();
                    self.carts.insert(pos, cart);
                    PlaceResult::Safe
                },
                b'|' => {
                    cart.underneath = Track::Vert;
                    self.map[offset] = cart.draw();
                    self.carts.insert(pos, cart);
                    PlaceResult::Safe
                },
                b'+' => {
                    cart.direction = match CART_ACTION_SEQ[cart.next_action] {
                        Action::TurnLeft => match cart.direction {
                            Dir::Up => Dir::Left,
                            Dir::Down => Dir::Right,
                            Dir::Left => Dir::Down,
                            Dir::Right => Dir::Up,
                        },
                        Action::Forward => cart.direction,
                        Action::TurnRight => match cart.direction {
                            Dir::Up => Dir::Right,
                            Dir::Down => Dir::Left,
                            Dir::Left => Dir::Up,
                            Dir::Right => Dir::Down,
                        },
                    };
                    cart.next_action = (cart.next_action + 1) % CART_ACTION_SEQ.len();
                    cart.underneath = Track::Cross;
                    self.map[offset] = cart.draw();
                    self.carts.insert(pos, cart);
                    PlaceResult::Safe
                },
                b'^' | b'v' | b'<' | b'>' => {
                    self.map[offset] = b'X';
                    PlaceResult::Crash(pos)
                },
                _ => unreachable!(),
        }
    }

    pub fn run_until(&mut self, event : Event) -> Pos {
        'frame: loop {
            self.tick += 1;
            for y in 0..self.height {
                for x in 0..self.width {
                    let pos = Pos{x,y};
                    match self.carts.get(&pos) {
                        None => (),
                        Some(cart) => {
                            if cart.last_moved_on_tick + 1 == self.tick {
                                let mut cart = self.lift(&pos);
                                cart.last_moved_on_tick = self.tick;
                                let pos = match cart.direction {
                                    Dir::Up => Pos { x: pos.x, y: pos.y - 1 },
                                    Dir::Down => Pos { x: pos.x, y: pos.y + 1 },
                                    Dir::Left => Pos { x: pos.x - 1, y: pos.y },
                                    Dir::Right => Pos { x: pos.x + 1, y: pos.y },
                                };
                                match self.place(pos, cart) {
                                    PlaceResult::Safe => (),
                                    PlaceResult::Crash(pos) => {
                                        match event {
                                            Event::Crash => break 'frame pos,
                                            Event::LastCart => {
                                                self.lift(&pos);
                                            },
                                        };
                                    },
                                }
                            } else {
                                assert_eq!(self.tick, cart.last_moved_on_tick);
                            }
                        },
                    }
                }
            }
            if let Event::Crash = event {
                println!("{}", self);
            }
            if self.carts.len() == 1 {
                for (pos, _) in &self.carts {
                    break 'frame Pos {
                        x: pos.x,
                        y: pos.y,
                    };
                }
            }
            else if self.carts.is_empty() {
                panic!("No carts remain");
            }
        }
    }
}

impl std::fmt::Display for Mine {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut out = String::new();
        out += &format!("tick = {}\n\n", self.tick);
        out += &String::from_utf8_lossy(&self.map);
        out += "\n";
        write!(f, "{}", out)
    }
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let part = match &std::env::args().nth(2).unwrap()[..] {
        "1" => 1,
        "2" => 2,
        _ => panic!("Invalid argument"),
    };
    let buff = std::fs::read(filename).unwrap();
    let mut mine = Mine::new(buff);
    if part == 1 {
        println!("{}", mine);
        let crash_pos = mine.run_until(Event::Crash);
        println!("{}", mine);
        println!("Crash position = {}", crash_pos);
    } else {
        let last_cart_pos = mine.run_until(Event::LastCart);
        println!("{}", mine);
        println!("Last cart position = {}", last_cart_pos);
    }
}
