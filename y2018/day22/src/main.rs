#[cfg(test)]
mod tests;

#[derive(Debug,PartialEq,Eq,Clone,Copy)]
enum Region {
    Rocky,
    Narrow,
    Wet
}

#[derive(Debug)]
struct Pos {
    x : usize,
    y : usize,
}

struct Survey {
    depth : usize,
    target : Pos,
    scan : Vec<Vec<Region>>,
    erosion_level : Vec<Vec<usize>>,
}

#[derive(Debug,PartialEq,Eq,Hash,Clone,Copy)]
enum Tool {
    Neither,
    Torch,
    ClimbGear,
}

impl Survey {
    fn calc_pos(&mut self, pos : Pos) {
        let geologic_index = if pos.x == 0 && pos.y == 0 {
            0
        } else if pos.x == self.target.x && pos.y == self.target.y {
            0
        } else if pos.y == 0 {
            pos.x * 16807
        } else if pos.x == 0 {
            pos.y * 48271
        } else {
            self.erosion_level[pos.y][pos.x-1] * self.erosion_level[pos.y-1][pos.x]
        };
        self.erosion_level[pos.y][pos.x] = (self.depth + geologic_index) % 20183;

        self.scan[pos.y][pos.x] = match self.erosion_level[pos.y][pos.x] % 3 {
            0 => Region::Rocky,
            1 => Region::Wet,
            2 => Region::Narrow,
            _ => unreachable!(),
        };
    }

    fn add_row(&mut self) {
        let width = self.scan[0].len();
        self.scan.push(vec![Region::Rocky; width]);
        self.erosion_level.push(vec![0; width]);
        let y = self.scan.len() - 1;
        for x in 0..width {
            self.calc_pos(Pos { x, y });
        }
    }

    fn add_col(&mut self) {
        let height = self.scan.len();
        for y in 0..height {
            self.scan[y].push(Region::Rocky);
            self.erosion_level[y].push(0);
            let x = self.scan[y].len() - 1;
            self.calc_pos(Pos { x, y });
        }
    }

    pub fn new(cave_system_depth : usize, target : Pos) -> Self {
        let width = target.x + 1;
        let height = target.y + 1;
        let mut out = Self {
            depth: cave_system_depth,
            target,
            scan: vec![vec![Region::Rocky; width]; height],
            erosion_level: vec![vec![0; width]; height],
        };

        for y in 0..height {
            for x in 0..width {
                let pos = Pos { x, y };
                out.calc_pos(pos);
            }
        }

        out
    }

    pub fn get_risk_level(&self) -> usize {
        let mut risk = 0;
        for y in 0..self.scan.len() {
            for x in 0..self.scan[0].len() {
                risk += match self.scan[y][x] {
                    Region::Rocky => 0,
                    Region::Wet => 1,
                    Region::Narrow => 2,
                };
            }
        }
        risk
    }

    fn get_neighbors(&mut self, pos : Pos, tool : Tool) -> Vec<(usize, usize, Tool, usize)> {
        let mut out = Vec::new();
        for (x_off, y_off, t_off) in [
            (0,  0, 1),
            (0,  0, 2),
            (0,  1, 0),
            (0, -1, 0),
            ( 1, 0, 0),
            (-1, 0, 0),
        ].iter() {
            if *x_off == -1 && pos.x == 0 || *y_off == -1 && pos.y == 0 {
                continue;
            }
            let x = (pos.x as isize + *x_off) as usize;
            let y = (pos.y as isize + *y_off) as usize;
            if y == self.scan.len() {
                self.add_row();
            }
            if x == self.scan[0].len() {
                self.add_col();
            }
            let new_tool = match (tool, t_off) {
                (Tool::Neither, 0) => Tool::Neither,
                (Tool::Neither, 1) => Tool::Torch,
                (Tool::Neither, 2) => Tool::ClimbGear,
                (Tool::Torch, 0) => Tool::Torch,
                (Tool::Torch, 1) => Tool::ClimbGear,
                (Tool::Torch, 2) => Tool::Neither,
                (Tool::ClimbGear, 0) => Tool::ClimbGear,
                (Tool::ClimbGear, 1) => Tool::Neither,
                (Tool::ClimbGear, 2) => Tool::Torch,
                _ => unreachable!(),
            };
            let compatible = match (self.scan[y][x], new_tool) {
                (Region::Rocky, Tool::Neither) => false,
                (Region::Rocky, Tool::Torch) => true,
                (Region::Rocky, Tool::ClimbGear) => true,
                (Region::Narrow, Tool::Neither) => true,
                (Region::Narrow, Tool::Torch) => true,
                (Region::Narrow, Tool::ClimbGear) => false,
                (Region::Wet, Tool::Neither) => true,
                (Region::Wet, Tool::Torch) => false,
                (Region::Wet, Tool::ClimbGear) => true,
            };
            if ! compatible {
                continue;
            }
            out.push((y, x, new_tool, if *t_off == 0 { 1 } else { 7 }));
        }
        out
    }

    pub fn find_minimum_time_path(&mut self) -> usize {
        let mut todo = std::collections::BinaryHeap::new();
        let mut graph = std::collections::HashMap::new();

        #[derive(Clone,PartialEq,Eq)]
        struct Node(usize, usize, Tool, usize);
        impl std::cmp::PartialOrd for Node {
            fn partial_cmp(&self, other : &Node) -> Option<std::cmp::Ordering> {
                Some(self.3.cmp(&other.3).reverse())
            }
        }
        impl std::cmp::Ord for Node {
            fn cmp(&self, other : &Node) -> std::cmp::Ordering {
                self.3.cmp(&other.3).reverse()
            }
        }

        // Initialize graph & todo list
        todo.push(Node(0, 0, Tool::Torch, 0));
        graph.insert((0, 0, Tool::Torch), 0);

        // Go through the todo list to seek out graph nodes with the shortest path
        let mut target_found = false;
        let mut min_dist_to_tgt = std::usize::MAX;
        loop {
            if let Some(Node(row, col, tool, _)) = todo.pop() {
                for (nrow, ncol, ntool, cost) in self.get_neighbors(Pos { x: col, y: row }, tool) {
                    // Check if we've found our target
                    let nkey = (nrow, ncol, ntool);
                    let alt = graph[&(row, col, tool)] + cost;
                    if self.target.y == nrow && self.target.x == ncol && ntool == Tool::Torch {
                        target_found = true;
                        min_dist_to_tgt = std::cmp::min(alt, min_dist_to_tgt);
                    }
                    if ! graph.contains_key(&nkey) {
                        if ! target_found || alt < min_dist_to_tgt {
                            todo.push(Node(nrow, ncol, ntool, alt));
                        }
                    }
                    let entry = graph.entry(nkey).or_insert(alt);
                    if alt < *entry {
                        *entry = alt;
                    }
                }
            } else {
                break;
            }
        }
        min_dist_to_tgt
    }
}

fn main() {
    let depth = 5355;
    let target = Pos { x: 14, y: 796 };
    let mut survey = Survey::new(depth, target);
    println!("Part 1: {}", survey.get_risk_level());
    println!("Part 2: {}", survey.find_minimum_time_path());
}
