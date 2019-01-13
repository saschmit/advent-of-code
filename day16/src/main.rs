type Register = isize;
type Operand = isize;

struct ElfCpu {
    register : [Register; 4],
    //opcode : [Option<fn(Operand, Operand, Operand)>; 16],
}

impl ElfCpu {
    pub fn new() -> Self {
        Self {
            register : [0; 4],
            //opcode : [None; 16],
        }
    }

    fn reg(&self, reg: Operand) -> usize {
        assert!(0 <= reg && reg < 4);
        reg as usize
    }

    fn rrd(&self, reg: Operand) -> Operand {
        self.register[self.reg(reg)]
    }

    fn rwr(&mut self, reg: Operand, val: Operand) {
        self.register[self.reg(reg)] = val;
    }

    pub fn addr(&mut self, a: Operand, b: Operand, c: Operand) {
        self.rwr(c, self.rrd(a) + self.rrd(b));
    }

    pub fn addi(&mut self, a: Operand, b: Operand, c: Operand) {
        self.rwr(c, self.rrd(a) + b);
    }

    pub fn mulr(&mut self, a: Operand, b: Operand, c: Operand) {
        self.rwr(c, self.rrd(a) * self.rrd(b));
    }

    pub fn muli(&mut self, a: Operand, b: Operand, c: Operand) {
        self.rwr(c, self.rrd(a) * b);
    }

    pub fn banr(&mut self, a: Operand, b: Operand, c: Operand) {
        self.rwr(c, self.rrd(a) & self.rrd(b));
    }

    pub fn bani(&mut self, a: Operand, b: Operand, c: Operand) {
        self.rwr(c, self.rrd(a) & b);
    }

    pub fn borr(&mut self, a: Operand, b: Operand, c: Operand) {
        self.rwr(c, self.rrd(a) | self.rrd(b));
    }

    pub fn bori(&mut self, a: Operand, b: Operand, c: Operand) {
        self.rwr(c, self.rrd(a) | b);
    }

    pub fn setr(&mut self, a: Operand, _: Operand, c: Operand) {
        self.rwr(c, self.rrd(a));
    }

    pub fn seti(&mut self, a: Operand, _: Operand, c: Operand) {
        self.rwr(c, a);
    }

    pub fn gtir(&mut self, a: Operand, b: Operand, c: Operand) {
        self.rwr(c, if a > self.rrd(b) { 1 } else { 0 });
    }

    pub fn gtri(&mut self, a: Operand, b: Operand, c: Operand) {
        self.rwr(c, if self.rrd(a) > b { 1 } else { 0 });
    }

    pub fn gtrr(&mut self, a: Operand, b: Operand, c: Operand) {
        self.rwr(c, if self.rrd(a) > self.rrd(b) { 1 } else { 0 });
    }

    pub fn eqir(&mut self, a: Operand, b: Operand, c: Operand) {
        self.rwr(c, if a == self.rrd(b) { 1 } else { 0 });
    }

    pub fn eqri(&mut self, a: Operand, b: Operand, c: Operand) {
        self.rwr(c, if self.rrd(a) == b { 1 } else { 0 });
    }

    pub fn eqrr(&mut self, a: Operand, b: Operand, c: Operand) {
        self.rwr(c, if self.rrd(a) == self.rrd(b) { 1 } else { 0 });
    }

    pub fn check_3plus(&mut self, case : &TestCase) -> bool {
        let ops : [fn(&mut Self, Operand, Operand, Operand); 16] = [
            Self::addr, Self::addi,
            Self::mulr, Self::muli,
            Self::banr, Self::bani,
            Self::borr, Self::bori,
            Self::setr, Self::seti,
            Self::gtir, Self::gtri, Self::gtrr,
            Self::eqir, Self::eqri, Self::eqrr,
        ];

        let mut count = 0;
        for op in &ops {
            self.register = case.before;
            op(self, case.oper.a, case.oper.b, case.oper.c);
            if self.register == case.after {
                count += 1;
            }
        }

        count >= 3
    }
}

#[derive(Debug)]
struct Operation {
    opcode : Operand,
    a      : Operand,
    b      : Operand,
    c      : Operand,
}

#[derive(Debug)]
struct TestCase {
    before : [Register; 4],
    oper   : Operation,
    after  : [Register; 4],
}

#[derive(Debug)]
enum Input {
    Part1(TestCase),
    Part2(Operation),
}

fn parse(buff : &[u8]) -> Vec<Input> {
    enum State {
        Start,
        Before,
        Instr,
        After,
        Sep,
        Sep2,
    }
    let mut offset = 0;
    let mut before = [0; 4];
    let mut after = [0; 4];
    let mut opcode = 0;
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    let mut state = State::Start;
    let mut out = Vec::new();
    while offset < buff.len() {
        match buff[offset] {
            b'A' | b'B' => {
                state = match buff[offset] {
                    b'B' => match state {
                        State::Start | State::Sep => State::Before,
                        _ => panic!("Bad state"),
                    },
                    b'A' => match state {
                        State::Instr => State::After,
                        _ => panic!("Bad state"),
                    },
                    _ => panic!("Bad input"),
                };
                offset += 9;
                let offset2 = {
                    let mut count = offset .. buff.len();
                    loop {
                        let off = count.next();
                        break match off {
                            Some(n) => match buff[n] {
                                b']' => n,
                                _ => continue,
                            },
                            None => panic!("Invalid input"),
                        }
                    }
                };

                let mut tokens = buff[offset..offset2].split(|n| match n {
                    b' ' | b',' => true,
                    _ => false,
                });

                let mut i = 0;
                while let Some(token) = tokens.next() {
                    if ! token.is_empty() {
                        let val = isize::from_str_radix(&String::from_utf8_lossy(&token), 10).unwrap();
                        match state {
                            State::Before => before[i] = val,
                            State::After => after[i] = val,
                            _ => unreachable!(),
                        }
                        i += 1;
                    }
                }
                assert_eq!(i, 4);
                offset = offset2 + 2;
            },
            b'\n' => {
                state = match state {
                    State::After => {
                        out.push(Input::Part1(TestCase {
                            before,
                            oper : Operation { opcode, a, b, c },
                            after
                        }));
                        State::Sep
                    },
                    State::Sep => State::Sep2,
                    State::Sep2 => State::Sep2,
                    _ => panic!("Bad state"),
                };
                offset += 1;
            },
            _ => {
                state = match state {
                    State::Before => State::Instr,
                    State::Instr => {
                        out.push(Input::Part2(Operation { opcode, a, b, c }));
                        State::Instr
                    },
                    State::Sep2 => State::Instr,
                    _ => panic!("Bad state"),
                };
                let offset2 = {
                    let mut count = offset+1 .. buff.len();
                    loop {
                        let off = count.next();
                        break match off {
                            Some(n) => match buff[n] {
                                b'\n' => n,
                                _ => continue,
                            },
                            None => panic!("Invalid input"),
                        }
                    }
                };

                let mut tokens = buff[offset..offset2].split(|n| *n == b' ');
                opcode = isize::from_str_radix(&String::from_utf8_lossy(&tokens.next().unwrap()), 10).unwrap();
                a = isize::from_str_radix(&String::from_utf8_lossy(&tokens.next().unwrap()), 10).unwrap();
                b = isize::from_str_radix(&String::from_utf8_lossy(&tokens.next().unwrap()), 10).unwrap();
                c = isize::from_str_radix(&String::from_utf8_lossy(&tokens.next().unwrap()), 10).unwrap();
                assert_eq!(tokens.next(), None);
                offset = offset2 + 1;
                
            },
        }
        assert_eq!(buff[offset - 1], b'\n');
    }
    out.push(Input::Part2(Operation { opcode, a, b, c }));
    out
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    let filename = &args[1];
    let buff = std::fs::read(filename).unwrap();
    let inputs = parse(&buff);

    let mut cpu = ElfCpu::new();

    let mut total = 0;
    for input in &inputs {
        match input {
            Input::Part1(case) => {
                total += if cpu.check_3plus(&case) { 1 } else { 0 };
            },
            Input::Part2(_op) => (),
        }
    }

    println!("Part 1: {}", total);
}
