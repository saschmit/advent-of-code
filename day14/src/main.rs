struct RecipeBoard {
    scores : Vec<usize>,
    current : [usize; 2],
    round : usize,
}

impl RecipeBoard {
    pub fn new() -> Self {
        let mut board = Self {
            scores : Vec::new(),
            current : [ 0, 1 ],
            round : 0,
        };
        board.scores.push(3);
        board.scores.push(7);
        board
    }

    pub fn add_round(&mut self) {
        let sum = self.scores[self.current[0]] + self.scores[self.current[1]];
        let digit1 = sum / 10;
        let digit2 = sum % 10;
        if digit1 != 0 {
            self.scores.push(digit1);
        }
        self.scores.push(digit2);
        for i in 0..=1 {
            self.current[i] = (self.current[i] + self.scores[self.current[i]] + 1) % self.scores.len();
        }
        self.round += 1
    }

    pub fn get_10_after(&mut self, n : usize) -> usize {
        let mut out = 0;
        while self.scores.len() < n + 10 {
            self.add_round();
        }
        for i in n..n+10 {
            out = out * 10 + self.scores[i];
        }
        out
    }

    pub fn get_n_before(&mut self, seq : &[usize]) -> usize {
        let mut n = 0;
        'outer: loop {
            while self.scores.len() < n + seq.len() {
                self.add_round();
            }
            for i in 0..seq.len() {
                if self.scores[n + i] != seq[i] {
                    n += 1;
                    continue 'outer;
                }
            }
            break 'outer n;
        }
    }
}

impl std::fmt::Display for RecipeBoard {
    fn fmt(&self, f : &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut out = String::new();
        for (i, score) in self.scores.iter().enumerate() {
            if i == self.current[0] && i == self.current[1] {
                out += "\x1b[1m";
            } else if i == self.current[0] {
                out += "\x1b[31;1m";
            } else if i == self.current[1] {
                out += "\x1b[32;1m";
            }
            out += &format!(" {}", score);
            if i == self.current[0] || i == self.current[1] {
                out += "\x1b[0m";
            }
        }
        write!(f, "{}:{}", self.round, out)
    }
}

fn main() {
    let mut board = RecipeBoard::new();
    println!("{}", board);
    for _ in 0..15 {
        board.add_round();
        println!("{}", board);
    }

    assert_eq!(5158916779, board.get_10_after(9));
    assert_eq!(0124515891, board.get_10_after(5));
    assert_eq!(9251071085, board.get_10_after(18));
    assert_eq!(5941429882, board.get_10_after(2018));

    println!("Part 1: {:010}", board.get_10_after(409551));

    assert_eq!(9, board.get_n_before(&[5,1,5,8,9]));
    assert_eq!(5, board.get_n_before(&[0,1,2,4,5]));
    assert_eq!(18, board.get_n_before(&[9,2,5,1,0]));
    assert_eq!(2018, board.get_n_before(&[5,9,4,1,4]));

    println!("Part 2: {}", board.get_n_before(&[4,0,9,5,5,1]));
}
