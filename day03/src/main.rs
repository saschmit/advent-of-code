#[derive(Debug)]
struct Claim {
    n : usize,
    x : usize,
    y : usize,
    w : usize,
    h : usize,
}

impl Claim {
    pub fn parse(line : &str) -> Result<Claim, String> {
        enum State {
            Initial,
            Claim,
            Delim1,
            Coord1,
            Coord2,
            Delim2,
            Size1,
            Size2,
        }

        let mut state : State = State::Initial;
        let mut buffer = String::new();
        let mut n = 0;
        let mut x = 0;
        let mut y = 0;
        let mut w = 0;
        for ch in line.chars() {
            match state {
                State::Initial => {
                    if ch != '#' {
                        return Err("invalid start character".to_string());
                    }
                    state = State::Claim;
                    buffer.clear();
                },
                State::Claim => {
                    match ch {
                        '0'..='9' => {
                            buffer.push(ch);
                        },
                        ' ' => {
                            n = usize::from_str_radix(&buffer, 10)
                                .or(Err("invalid claim number".to_string()))?;
                            buffer.clear();
                            state = State::Delim1;
                        }
                        _ => {
                            return Err("no space at the end of the claim number".to_string());
                        }
                    };
                },
                State::Delim1 => {
                    match ch {
                        ' ' | '@' => {
                        },
                        '0'..='9' => {
                            buffer.push(ch);
                            state = State::Coord1;
                        },
                        _ => {
                            return Err("no space at the end of the claim number".to_string());
                        }
                    };
                },
                State::Coord1 => {
                    match ch {
                        '0'..='9' => {
                            buffer.push(ch);
                        },
                        ',' => {
                            x = usize::from_str_radix(&buffer, 10)
                                .or(Err("invalid coordinate 1".to_string()))?;
                            buffer.clear();
                            state = State::Coord2;
                        }
                        _ => {
                            return Err("no comma at the end of coordinate 1".to_string());
                        }
                    };
                },
                State::Coord2 => {
                    match ch {
                        '0'..='9' => {
                            buffer.push(ch);
                        },
                        ':' => {
                            y = usize::from_str_radix(&buffer, 10)
                                .or(Err("invalid coordinate 2".to_string()))?;
                            buffer.clear();
                            state = State::Delim2;
                        }
                        _ => {
                            return Err("no colon at the end of coordinate 2".to_string());
                        }
                    };
                },
                State::Delim2 => {
                    match ch {
                        ' ' => {
                        },
                        '0'..='9' => {
                            buffer.push(ch);
                            state = State::Size1;
                        },
                        _ => {
                            return Err("no space before width".to_string());
                        }
                    };
                },
                State::Size1 => {
                    match ch {
                        '0'..='9' => {
                            buffer.push(ch);
                        },
                        'x' => {
                            w = usize::from_str_radix(&buffer, 10)
                                .or(Err("invalid width".to_string()))?;
                            buffer.clear();
                            state = State::Size2;
                        }
                        _ => {
                            return Err("no x at the end of width".to_string());
                        }
                    };
                },
                State::Size2 => {
                    match ch {
                        '0'..='9' => {
                            buffer.push(ch);
                        },
                        _ => {
                            return Err("invalid character at the end of height".to_string());
                        }
                    };
                },
            }
        }

        let h = usize::from_str_radix(&buffer, 10)
            .or(Err("invalid width".to_string()))?;
        buffer.clear();

        Ok(Claim {
            n: n,
            x: x,
            y: y,
            w: w,
            h: h,
        })
    }

    pub fn mark(&self, fabric : &mut Vec<Vec<Vec<usize>>>) {
        for x in self.x .. self.x + self.w {
            for y in self.y .. self.y + self.h {
                fabric[x][y].push(self.n);
            }
        }
    }
}

fn main() {
    let buff = String::from_utf8(std::fs::read("input").unwrap()).unwrap();
    let lines : Vec<&str> = buff.lines().collect();

    let mut fabric : Vec<Vec<Vec<usize>>> = Vec::new();
    for x in 0..1000 {
        let x_axis : Vec<Vec<usize>> = Vec::new();
        fabric.push(x_axis);
        for _ in 0..1000 {
            let y_axis : Vec<usize> = Vec::new();
            fabric[x].push(y_axis);
        }
    }
    let mut claims = Vec::<Claim>::new();
    for line in &lines {
        claims.push(Claim::parse(line).unwrap());
    }

    for claim in claims {
        claim.mark(&mut fabric);
    }

    let mut count = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            if fabric[x][y].len() > 1 {
                count += 1;
            }
        }
    }

    println!("multi-claim sq. in.: {}", count);
}
