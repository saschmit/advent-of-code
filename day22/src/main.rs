#[cfg(test)]
mod tests;

#[derive(Debug,PartialEq,Eq)]
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
    scan : Vec<Vec<Region>>,
}

impl Survey {
    pub fn new(cave_system_depth : usize, target : Pos) -> Self {
        let width = target.x + 1;
        let height = target.y + 1;
        let mut erosion_level = vec![vec![0; width]; height];
        let mut out = Self { scan: Vec::new() };

        for y in 0..height {
            out.scan.push(Vec::new());
            for x in 0..width {
                let pos = Pos { x, y };

                let geologic_index = if pos.x == 0 && pos.y == 0 {
                    0
                } else if pos.x == target.x && pos.y == target.y {
                    0
                } else if pos.y == 0 {
                    pos.x * 16807
                } else if pos.x == 0 {
                    pos.y * 48271
                } else {
                    erosion_level[pos.y][pos.x-1] * erosion_level[pos.y-1][pos.x]
                };
                erosion_level[pos.y][pos.x] = (cave_system_depth + geologic_index) % 20183;

                out.scan[pos.y].push(match erosion_level[pos.y][pos.x] % 3 {
                    0 => Region::Rocky,
                    1 => Region::Wet,
                    2 => Region::Narrow,
                    _ => unreachable!(),
                });
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
}

fn main() {
    let depth = 5355;
    let target = Pos { x: 14, y: 796 };
    let survey = Survey::new(depth, target);
    println!("Part 1: {}", survey.get_risk_level());
}
