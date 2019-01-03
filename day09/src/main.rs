use std::collections::VecDeque;

struct Game {
    circle : VecDeque<usize>,
    current : usize,
    players : Vec<Vec<usize>>,
    last_marble : usize,
}

impl Game {
    pub fn new(num_players : usize) -> Game {
        let mut game = Game {
            circle : VecDeque::with_capacity(100),
            current : 0,
            players : Vec::with_capacity(num_players),
            last_marble : 0,
        };
        game.circle.push_back(0);
        for _ in 0..num_players {
            game.players.push(Vec::new());
        }
        game
    }

    fn cw(&self, n : usize) -> usize {
        (self.current + n) % self.circle.len()
    }

    fn ccw(&self, n : usize) -> usize {
        (self.circle.len() + self.current - n) % self.circle.len()
    }

    fn get_current_player(&self) -> usize {
        (self.last_marble - 1) % self.players.len() + 1
    }

    pub fn add_marble(&mut self) {
        self.last_marble += 1;
        if self.last_marble % 23 != 0 {
            let pos = self.cw(2);
            //eprintln!("{} {}", self.last_marble, pos);
            if pos == 0 || pos > self.circle.len() {
                self.circle.push_back(self.last_marble);
                self.current = self.circle.len() - 1;
            } else {
                self.circle.insert(pos, self.last_marble);
                self.current = pos;
            }
        } else {
            let rm_pos = self.ccw(7);
            let current_player = self.get_current_player();
            let current_player = &mut self.players[current_player - 1];
            current_player.push(self.last_marble);
            current_player.push(self.circle.remove(rm_pos).unwrap());
            if rm_pos < self.current {
                self.current -= 1; // Adjust for removed element
            }
            self.current = self.ccw(6);
        }
    }

    pub fn get_high_score(&self) -> usize {
        let mut high_score = 0;
        for player in &self.players {
            let sum = player.iter().fold(0, |acc, x| acc + x);
            high_score = std::cmp::max(high_score, sum);
        }
        high_score
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut out = String::new();
        let width = 2+self.last_marble.to_string().len();
        out.push('[');
        if self.last_marble == 0 {
            out.push('-');
        } else {
            out += &self.get_current_player().to_string();
        }
        out.push(']');
        out.push(' ');
        for (i, marble) in self.circle.iter().enumerate() {
            if i == self.current {
                out += "\x1B[1m";
            }
            out += &format!("{:^width$}", if i == self.current {
                    format!("({})", marble)
                } else {
                    marble.to_string()
                }, width=width);
            if i == self.current {
                out += "\x1B[0m";
            }
        }
        write!(f, "{}", out)
    }
}

fn main() {
    let num_players = usize::from_str_radix(&std::env::args().nth(1).unwrap(), 10).unwrap();
    let last_marble = usize::from_str_radix(&std::env::args().nth(2).unwrap(), 10).unwrap();

    let mut game = Game::new(num_players);

    //eprintln!("{}", game);
    for _ in 0..last_marble {
        game.add_marble();
        //eprintln!("{}", game);
    }
    let high_score = game.get_high_score();
    println!("{} players; last marble is worth {} points: high score is {}",
        num_players, last_marble, high_score);
}
