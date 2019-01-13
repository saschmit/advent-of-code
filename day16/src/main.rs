type Register = isize;
type Operand = isize;

#[derive(Debug,Hash,Eq,PartialEq,Copy,Clone)]
#[repr(usize)]
enum OpCode {
    Addr, Addi,
    Mulr, Muli,
    Banr, Bani,
    Borr, Bori,
    Setr, Seti,
    Gtir, Gtri, Gtrr,
    Eqir, Eqri, Eqrr,
}

struct ElfCpu {
    register : [Register; 4],
    opcode : [Option<OpCode>; 16],
}

impl ElfCpu {
    pub fn new() -> Self {
        Self {
            register : [0; 4],
            opcode : [None; 16],
        }
    }

    pub fn with(opcodes : [Option<OpCode>; 16]) -> Self {
        Self {
            register : [0; 4],
            opcode : opcodes,
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

    fn addr(&mut self, a: Operand, b: Operand, c: Operand) {
        self.rwr(c, self.rrd(a) + self.rrd(b));
    }

    fn addi(&mut self, a: Operand, b: Operand, c: Operand) {
        self.rwr(c, self.rrd(a) + b);
    }

    fn mulr(&mut self, a: Operand, b: Operand, c: Operand) {
        self.rwr(c, self.rrd(a) * self.rrd(b));
    }

    fn muli(&mut self, a: Operand, b: Operand, c: Operand) {
        self.rwr(c, self.rrd(a) * b);
    }

    fn banr(&mut self, a: Operand, b: Operand, c: Operand) {
        self.rwr(c, self.rrd(a) & self.rrd(b));
    }

    fn bani(&mut self, a: Operand, b: Operand, c: Operand) {
        self.rwr(c, self.rrd(a) & b);
    }

    fn borr(&mut self, a: Operand, b: Operand, c: Operand) {
        self.rwr(c, self.rrd(a) | self.rrd(b));
    }

    fn bori(&mut self, a: Operand, b: Operand, c: Operand) {
        self.rwr(c, self.rrd(a) | b);
    }

    fn setr(&mut self, a: Operand, _: Operand, c: Operand) {
        self.rwr(c, self.rrd(a));
    }

    fn seti(&mut self, a: Operand, _: Operand, c: Operand) {
        self.rwr(c, a);
    }

    fn gtir(&mut self, a: Operand, b: Operand, c: Operand) {
        self.rwr(c, if a > self.rrd(b) { 1 } else { 0 });
    }

    fn gtri(&mut self, a: Operand, b: Operand, c: Operand) {
        self.rwr(c, if self.rrd(a) > b { 1 } else { 0 });
    }

    fn gtrr(&mut self, a: Operand, b: Operand, c: Operand) {
        self.rwr(c, if self.rrd(a) > self.rrd(b) { 1 } else { 0 });
    }

    fn eqir(&mut self, a: Operand, b: Operand, c: Operand) {
        self.rwr(c, if a == self.rrd(b) { 1 } else { 0 });
    }

    fn eqri(&mut self, a: Operand, b: Operand, c: Operand) {
        self.rwr(c, if self.rrd(a) == b { 1 } else { 0 });
    }

    fn eqrr(&mut self, a: Operand, b: Operand, c: Operand) {
        self.rwr(c, if self.rrd(a) == self.rrd(b) { 1 } else { 0 });
    }

    pub fn check(&mut self, case : &TestCase) -> Vec<OpCode> {
        let opcodes : [OpCode; 16] = [
            OpCode::Addr, OpCode::Addi,
            OpCode::Mulr, OpCode::Muli,
            OpCode::Banr, OpCode::Bani,
            OpCode::Borr, OpCode::Bori,
            OpCode::Setr, OpCode::Seti,
            OpCode::Gtir, OpCode::Gtri, OpCode::Gtrr,
            OpCode::Eqir, OpCode::Eqri, OpCode::Eqrr,
        ];

        let ops = [
            Self::addr, Self::addi,
            Self::mulr, Self::muli,
            Self::banr, Self::bani,
            Self::borr, Self::bori,
            Self::setr, Self::seti,
            Self::gtir, Self::gtri, Self::gtrr,
            Self::eqir, Self::eqri, Self::eqrr,
        ];

        let mut out = Vec::new();
        for (opcode, op) in opcodes.iter().zip(&ops) {
            self.register = case.before;
            op(self, case.oper.a, case.oper.b, case.oper.c);
            if self.register == case.after {
                out.push(*opcode);
            }
        }

        out
    }

    pub fn exec(&mut self, oper : &Operation) {
        let ops = [
            Self::addr, Self::addi,
            Self::mulr, Self::muli,
            Self::banr, Self::bani,
            Self::borr, Self::bori,
            Self::setr, Self::seti,
            Self::gtir, Self::gtri, Self::gtrr,
            Self::eqir, Self::eqri, Self::eqrr,
        ];

        let op = ops[self.opcode[oper.opcode as usize].unwrap() as usize];
        op(self, oper.a, oper.b, oper.c);
    }

    pub fn get_reg(&self, reg : Operand) -> Register {
        self.register[self.reg(reg)]
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
    let mut outputs = Vec::new();
    for input in &inputs {
        match input {
            Input::Part1(case) => {
                let result = cpu.check(&case);
                total += if result.len() >= 3 { 1 } else { 0 };
                outputs.push((case.oper.opcode, result));
            },
            Input::Part2(_) => (),
        }
    }

    println!("Part 1: {}", total);

    let mut opcode_set : [Option<OpCode>; 16] = [None; 16];
    let mut known = std::collections::HashSet::new();
    while opcode_set.iter().any(|x| x.is_none()) {
        for (opcode, results) in &outputs {
            let opcode = *opcode;
            let opcode = opcode as usize;
            if opcode_set[opcode].is_some() {
                continue;
            }
            let results : Vec<OpCode> = results.iter()
                    .filter(|code| !known.contains(*code))
                    .map(|code| *code)
                    .collect();
            if results.len() == 1 {
                opcode_set[opcode] = Some(results[0]);
                known.insert(results[0]);
            }
        }
    }

    let mut cpu = ElfCpu::with(opcode_set);
    for input in &inputs {
        match input {
            Input::Part1(_) => (),
            Input::Part2(op) => cpu.exec(op),
        }
    }

    println!("Part 2: {}", cpu.get_reg(0));
}
