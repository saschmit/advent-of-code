#[derive(PartialEq,Debug,Clone,Copy)]
enum WorkState {
    Idle,
    Working(char),
    Completed(char),
}

#[derive(Debug)]
struct Helper {
    constant_factor : u8,
    time_left : u8,
    state : WorkState,
}

impl Helper {
    pub fn new(constant_factor : u8) -> Helper {
        Helper {
            constant_factor,
            time_left: 0,
            state : WorkState::Idle,
        }
    }

    pub fn give(&mut self, step : char) {
        assert_eq!(self.state, WorkState::Idle);
        self.state = WorkState::Working(step);
        self.time_left = self.constant_factor + step as u8 - 'A' as u8 + 1;
    }

    pub fn get_state(&self) -> WorkState {
        self.state
    }

    fn work(&mut self) -> WorkState {
        match self.get_state() {
            WorkState::Idle => WorkState::Idle,
            WorkState::Working(step) => {
                self.time_left -= 1;
                if self.time_left != 0 {
                    self.state
                } else {
                    self.state = WorkState::Idle;
                    WorkState::Completed(step)
                }
            },
            WorkState::Completed(_) => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Dispatcher {
    all_steps : std::collections::HashSet<char>,
    dependencies : Vec<(char, char)>,
    helpers : Vec<Helper>,
    steps_done : Vec<char>,
}

impl Dispatcher {
    pub fn new(file : &str, num_helpers : usize, constant_factor : u8) -> Dispatcher {
        let buff = std::fs::read(file).unwrap();
        assert_eq!(buff.len() % 49, 0);

        let mut all_steps = std::collections::HashSet::new();
        let mut dependencies = Vec::new();
        for i in 0..buff.len() / 49 {
            let (a, b) = (buff[i*49 + 5] as char, buff[i*49 + 36] as char);
            dependencies.push((a, b));
            all_steps.insert(a);
            all_steps.insert(b);
        }

        let mut d = Dispatcher {
            all_steps,
            dependencies,
            helpers : Vec::with_capacity(num_helpers),
            steps_done : Vec::new(),
        };

        for _ in 0..num_helpers {
            d.helpers.push(Helper::new(constant_factor));
        }

        d
    }

    fn get_next_step(&self) -> Option<char> {
        // stop if we've done all the steps
        if self.steps_done.len() == self.all_steps.len() {
            return None;
        }

        let mut candidates = std::collections::HashSet::new();

        // add any step we haven't already started or finished
        'outer: for step in &self.all_steps {
            if self.steps_done.iter().find(|&&x| x == *step) != None {
                continue;
            }
            for helper in &self.helpers {
                if let WorkState::Working(x) = helper.get_state() {
                    if x == *step {
                        continue 'outer;
                    }
                }
            }
            candidates.insert(*step);
        }

        // remove any step with a prerequisite not already taken
        for (a, b) in &self.dependencies {
            if self.steps_done.iter().find(|&&x| x == *a) != None {
                continue;
            }
            if candidates.iter().find(|&&x| x == *b) != None {
                candidates.remove(b);
            }
        }

        if candidates.len() == 0 {
            return None;
        }

        use std::iter::FromIterator;
        let mut sorted = Vec::from_iter(&candidates);

        // sort the candidates alphabetically
        sorted.sort();

        // take the first
        Some(*sorted[0])
    }

    fn run_step(&mut self) -> bool {
        // Give work to helpers
        let mut some_idle = true;
        let mut next_step = self.get_next_step();
        while some_idle && next_step != None {
            some_idle = false;
            for helper in &mut self.helpers {
                match helper.get_state() {
                    WorkState::Idle => {
                        match next_step {
                            None => some_idle = true,
                            Some(step) => {
                                helper.give(step);
                                next_step = None;
                            },
                        }
                    },
                    WorkState::Working(_step) => (),
                    WorkState::Completed(_) => unreachable!(),
                }
            }
            if some_idle {
                next_step = self.get_next_step();
            }
        }

        // Let the helpers help, collect completed steps
        for helper in &mut self.helpers {
            match helper.work() {
                WorkState::Idle => (),
                WorkState::Working(_step) => (),
                WorkState::Completed(step) => {
                    self.steps_done.push(step);
                },
            }
        }

        // Translate what's going on into a "keep going" boolean
        for helper in &self.helpers {
            if helper.get_state() != WorkState::Idle {
                return true;
            }
        }

        self.steps_done.len() != self.all_steps.len()
    }

    pub fn run(&mut self) -> usize {
        let mut time = 0;
        while self.run_step() {
            time += 1;
        }
        time + 1
    }

    pub fn get_steps(&self) -> &Vec<char> {
        &self.steps_done
    }
}

fn main() {
    let file = std::env::args().nth(1).unwrap();
    let workers = usize::from_str_radix(&std::env::args().nth(2).unwrap(), 10).unwrap();
    let work_factor = u8::from_str_radix(&std::env::args().nth(3).unwrap(), 10).unwrap();
    let mut dispatch = Dispatcher::new(&file, workers, work_factor);
    let elapsed = dispatch.run();
    print!("Steps completed: ");
    for step in dispatch.get_steps() {
        print!("{}", step);
    }
    println!(", which took {}s", elapsed);
}
