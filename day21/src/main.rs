type Word = isize;

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

#[derive(Debug)]
struct Instruction {
    op : OpCode,
    a  : Word,
    b  : Word,
    c  : Word,
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?} {} {} {}", self.op, self.a, self.b, self.c)
    }
}

const NUM_REGISTERS : usize = 6;

enum Part {
    Part1,
    Part2,
}

struct ElfCpu {
    register : [Word; NUM_REGISTERS],
    ip : usize,
    part : Part,
    dupes : std::collections::HashSet<isize>,
    last : Option<isize>,
}

impl ElfCpu {
    pub fn new(r0 : Word, part : Part) -> Self {
        Self {
            register : [r0, 0, 0, 0, 0, 0],
            ip : 0,
            part,
            dupes: std::collections::HashSet::new(),
            last: None,
        }
    }

    fn reg(&self, reg: Word) -> usize {
        assert!(0 <= reg && reg < NUM_REGISTERS as Word);
        reg as usize
    }

    fn rrd(&self, reg: Word) -> Word {
        self.register[self.reg(reg)]
    }

    fn rwr(&mut self, reg: Word, val: Word) {
        self.register[self.reg(reg)] = val;
    }

    fn addr(&mut self, a: Word, b: Word, c: Word) {
        self.rwr(c, self.rrd(a) + self.rrd(b));
    }

    fn addi(&mut self, a: Word, b: Word, c: Word) {
        self.rwr(c, self.rrd(a) + b);
    }

    fn mulr(&mut self, a: Word, b: Word, c: Word) {
        self.rwr(c, self.rrd(a) * self.rrd(b));
    }

    fn muli(&mut self, a: Word, b: Word, c: Word) {
        self.rwr(c, self.rrd(a) * b);
    }

    fn banr(&mut self, a: Word, b: Word, c: Word) {
        self.rwr(c, self.rrd(a) & self.rrd(b));
    }

    fn bani(&mut self, a: Word, b: Word, c: Word) {
        self.rwr(c, self.rrd(a) & b);
    }

    fn borr(&mut self, a: Word, b: Word, c: Word) {
        self.rwr(c, self.rrd(a) | self.rrd(b));
    }

    fn bori(&mut self, a: Word, b: Word, c: Word) {
        self.rwr(c, self.rrd(a) | b);
    }

    fn setr(&mut self, a: Word, _: Word, c: Word) {
        self.rwr(c, self.rrd(a));
    }

    fn seti(&mut self, a: Word, _: Word, c: Word) {
        self.rwr(c, a);
    }

    fn gtir(&mut self, a: Word, b: Word, c: Word) {
        self.rwr(c, if a > self.rrd(b) { 1 } else { 0 });
    }

    fn gtri(&mut self, a: Word, b: Word, c: Word) {
        self.rwr(c, if self.rrd(a) > b { 1 } else { 0 });
    }

    fn gtrr(&mut self, a: Word, b: Word, c: Word) {
        self.rwr(c, if self.rrd(a) > self.rrd(b) { 1 } else { 0 });
    }

    fn eqir(&mut self, a: Word, b: Word, c: Word) {
        self.rwr(c, if a == self.rrd(b) { 1 } else { 0 });
    }

    fn eqri(&mut self, a: Word, b: Word, c: Word) {
        self.rwr(c, if self.rrd(a) == b { 1 } else { 0 });
    }

    fn eqrr(&mut self, a: Word, b: Word, c: Word) {
        self.rwr(c, if self.rrd(a) == self.rrd(b) { 1 } else { 0 });
    }

    fn exec(&mut self, inst : &Instruction) {
        let ops = [
            Self::addr, Self::addi,
            Self::mulr, Self::muli,
            Self::banr, Self::bani,
            Self::borr, Self::bori,
            Self::setr, Self::seti,
            Self::gtir, Self::gtri, Self::gtrr,
            Self::eqir, Self::eqri, Self::eqrr,
        ];

        let op = ops[inst.op as usize];
        op(self, inst.a, inst.b, inst.c);
    }

    pub fn run(&mut self, ip_reg : usize, program : Vec<Instruction>) {
        assert!(ip_reg < NUM_REGISTERS);
        loop {
            if self.ip >= program.len() {
                break;
            }
            if self.ip == 28 {
                match self.part {
                    Part::Part1 => {
                        println!("r0 <= r2 ({})", self.register[2]);
                        self.register[0] = self.register[2];
                    },
                    Part::Part2 => {
                        if self.dupes.contains(&self.register[2]) {
                            self.register[0] = self.last.unwrap();
                        } else {
                            self.dupes.insert(self.register[2]);
                            self.last = Some(self.register[2]);
                        }
                    },
                }
            }
            self.register[ip_reg] = self.ip as Word;
            let inst = &program[self.ip];
            if cfg!(feature = "trace") {
                print!("ip={} [{}, {}, {}, {}, {}, {}] {} ",
                    self.ip,
                    self.register[0],
                    self.register[1],
                    self.register[2],
                    self.register[3],
                    self.register[4],
                    self.register[5],
                    inst);
            }
            self.exec(inst);
            if cfg!(feature = "trace") {
                println!("[{}, {}, {}, {}, {}, {}]",
                    self.register[0],
                    self.register[1],
                    self.register[2],
                    self.register[3],
                    self.register[4],
                    self.register[5]);
            }
            self.ip = self.register[ip_reg] as usize;
            self.ip += 1;
        }
    }

    pub fn get_reg(&self, reg : Word) -> Word {
        self.register[self.reg(reg)]
    }
}

fn parse(buff : &[u8]) -> (usize, Vec<Instruction>) {
    let mut offset = 0;
    let mut inst = Instruction {
        op: OpCode::Addi,
        a: 0,
        b: 0,
        c: 0,
    };
    let mut ip_reg = 0;
    let mut out = Vec::new();
    while offset < buff.len() {
        match buff[offset] {
            b'#' => {
                assert_eq!(buff[offset+1], b'i');
                assert_eq!(buff[offset+2], b'p');
                assert_eq!(buff[offset+3], b' ');
                ip_reg = (buff[offset+4] - b'0') as usize;
                offset += 6;
            },
            b'a' | b'b' | b'e' | b'g' | b'm' | b's' => {
                inst.op = match &buff[offset..offset+4] {
                    b"addr" => OpCode::Addr,
                    b"addi" => OpCode::Addi,
                    b"mulr" => OpCode::Mulr,
                    b"muli" => OpCode::Muli,
                    b"banr" => OpCode::Banr,
                    b"bani" => OpCode::Bani,
                    b"borr" => OpCode::Borr,
                    b"bori" => OpCode::Bori,
                    b"setr" => OpCode::Setr,
                    b"seti" => OpCode::Seti,
                    b"gtir" => OpCode::Gtir,
                    b"gtri" => OpCode::Gtri,
                    b"gtrr" => OpCode::Gtrr,
                    b"eqir" => OpCode::Eqir,
                    b"eqri" => OpCode::Eqri,
                    b"eqrr" => OpCode::Eqrr,
                    _ => panic!("Invalid opcode"),
                };

                offset += 5;

                let eol_off = {
                    let mut count = offset .. buff.len();
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

                let mut tokens = buff[offset..eol_off].split(|n| *n == b' ');
                inst.a = isize::from_str_radix(&String::from_utf8_lossy(&tokens.next().unwrap()), 10).unwrap();
                inst.b = isize::from_str_radix(&String::from_utf8_lossy(&tokens.next().unwrap()), 10).unwrap();
                inst.c = isize::from_str_radix(&String::from_utf8_lossy(&tokens.next().unwrap()), 10).unwrap();
                assert_eq!(tokens.next(), None);
                out.push(Instruction { .. inst });
                offset = eol_off + 1;
            },
            _ => panic!("Invalid input"),
        }
        assert_eq!(buff[offset - 1], b'\n');
    }
    (ip_reg, out)
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    let filename = &args[1];
    let buff = std::fs::read(filename).unwrap();
    let (ip_reg, program) = parse(&buff);
    let mut cpu = ElfCpu::new(0, Part::Part1);
    cpu.run(ip_reg, program);
    println!("Part 1: {}", cpu.get_reg(0));

    let (ip_reg, program) = parse(&buff);
    let mut cpu = ElfCpu::new(0, Part::Part2);
    cpu.run(ip_reg, program);
    println!("Part 2: {}", cpu.get_reg(0));
}
