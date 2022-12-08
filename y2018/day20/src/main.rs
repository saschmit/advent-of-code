#[cfg(test)]
mod tests;

#[derive(Debug,Clone,Copy,Eq,PartialEq)]
#[repr(u8)]
enum RegexChar {
    Start = b'^',
    North = b'N',
    South = b'S',
    East = b'E',
    West = b'W',
    Open = b'(',
    Pipe = b'|',
    Close = b')',
    End = b'$',
    NL = b'\n',
}

/*
#[derive(Debug)]
enum Dir {
    North,
    South,
    East,
    West
}

#[derive(Debug)]
enum Node {
    Step((Dir, Node)),
    Branch(Vec<Node>),
    End,
}

#[derive(Debug)]
struct Regex {
    regex : Node,
}
*/

#[derive(Debug,Clone)]
enum MapCell {
    Wall,
    Room,
    Door,
    Here,
    Open, // Unknown room
    Dark, // Unknown door/wall
}

struct Map {
    regex : Vec<RegexChar>,
    map : Vec<Vec<MapCell>>,
}

impl Map {
    pub fn new(buff : &[u8]) -> Map {
        let regex = Self::validate(buff);
        let map = Self::draw(&regex);

        Self {
            regex,
            map,
        }
    }

    fn validate(buff : &[u8]) -> Vec<RegexChar> {
        let mut regex = Vec::new();
        for byte in buff {
            regex.push(match byte {
                b'^' => RegexChar::Start,
                b'N' => RegexChar::North,
                b'S' => RegexChar::South,
                b'E' => RegexChar::East,
                b'W' => RegexChar::West,
                b'(' => RegexChar::Open,
                b'|' => RegexChar::Pipe,
                b')' => RegexChar::Close,
                b'$' => RegexChar::End,
                b'\n' => RegexChar::NL,
                _ => panic!("Invalid regex character"),
            });
        }
        if regex.len() > 0 && regex[regex.len()-1] == RegexChar::NL {
            regex.pop();
            regex
        } else {
            regex
        }
    }

    fn draw(regex : &[RegexChar]) -> Vec<Vec<MapCell>> {
        let mut map = vec![
            vec![MapCell::Wall, MapCell::Dark, MapCell::Wall],
            vec![MapCell::Dark, MapCell::Here, MapCell::Dark],
            vec![MapCell::Wall, MapCell::Dark, MapCell::Wall],
        ];
        assert_eq!(regex[0], RegexChar::Start);
        assert_eq!(regex[regex.len()-1], RegexChar::End);
        let regex = &regex[1..regex.len()-1];
        let mut row = 1;
        let mut col = 1;
        let mut stack : Vec<(usize, usize)> = Vec::new();
        for ch in regex {
            //eprintln!("Processing {} at ({}, {})", (*ch as u8) as char, row, col);
            match ch {
                RegexChar::North => {
                    if row == 1 {
                        let new_row = vec![MapCell::Open; map[0].len()];
                        map.insert(0, new_row.clone());
                        map.insert(0, new_row);
                        row += 2;
                        for i in 0..stack.len() {
                            stack[i].0 += 2;
                        }
                    }
                    map = Self::draw_ud(RegexChar::North, row, col, map);
                    row -= 2;
                },
                RegexChar::South => {
                    if row == map.len() - 2 {
                        let new_row = vec![MapCell::Open; map[0].len()];
                        map.push(new_row.clone());
                        map.push(new_row);
                    }
                    map = Self::draw_ud(RegexChar::South, row, col, map);
                    row += 2;
                },
                RegexChar::East => {
                    if col == map[0].len() - 2 {
                        for r in 0..map.len() {
                            map[r].push(MapCell::Open);
                            map[r].push(MapCell::Open);
                        }
                    }
                    map = Self::draw_lr(RegexChar::East, row, col, map);
                    col += 2;
                },
                RegexChar::West => {
                    if col == 1 {
                        for r in 0..map.len() {
                            map[r].insert(0, MapCell::Open);
                            map[r].insert(0, MapCell::Open);
                        }
                        col += 2;
                        for i in 0..stack.len() {
                            stack[i].1 += 2;
                        }
                    }
                    map = Self::draw_lr(RegexChar::West, row, col, map);
                    col -= 2;
                },
                RegexChar::Open => {
                    stack.push((row, col));
                },
                RegexChar::Pipe => {
                    let top = stack.pop().unwrap();
                    row = top.0;
                    col = top.1;
                    stack.push(top);
                },
                RegexChar::Close => {
                    let top = stack.pop().unwrap();
                    row = top.0;
                    col = top.1;
                },
                RegexChar::Start | RegexChar::End | RegexChar::NL =>
                    panic!("Invalid regex"),
            }
            //eprintln!("{}", Map { map: map.clone(), regex: regex.to_vec() });
        }

        // Everything is drawn, so fill in the gaps
        for row in 0..map.len() {
            for col in 0..map[0].len() {
                map[row][col] = match map[row][col] {
                    MapCell::Dark | MapCell::Wall | MapCell::Open => MapCell::Wall,
                    MapCell::Room => MapCell::Room,
                    MapCell::Here => MapCell::Here,
                    MapCell::Door => MapCell::Door,
                };
            }
        }
        eprintln!("{}", Map { map: map.clone(), regex: regex.to_vec() });
        map
    }

    fn draw_ud(dir : RegexChar, mut row : usize, col : usize,
               mut map : Vec<Vec<MapCell>>) -> Vec<Vec<MapCell>> {
        row = match dir {
            RegexChar::North => row - 1,
            RegexChar::South => row + 1,
            _ => panic!("Invalid direction"),
        };
        map[row][col] = match map[row][col] {
            MapCell::Dark | MapCell::Door => MapCell::Door,
            MapCell::Wall => panic!("door from wall?"),
            MapCell::Room => panic!("door from room?"),
            MapCell::Here => panic!("door from here?"),
            MapCell::Open => panic!("door from open?"),
        };
        map[row][col-1] = match map[row][col-1] {
            MapCell::Wall => MapCell::Wall,
            MapCell::Dark => panic!("wall from dark?"),
            MapCell::Door => panic!("wall from door?"),
            MapCell::Room => panic!("wall from room?"),
            MapCell::Here => panic!("wall from here?"),
            MapCell::Open => panic!("wall from open?"),
        };
        map[row][col+1] = match map[row][col+1] {
            MapCell::Wall => MapCell::Wall,
            MapCell::Dark => panic!("wall from dark?"),
            MapCell::Door => panic!("wall from door?"),
            MapCell::Room => panic!("wall from room?"),
            MapCell::Here => panic!("wall from here?"),
            MapCell::Open => panic!("wall from open?"),
        };

        row = match dir {
            RegexChar::North => row - 1,
            RegexChar::South => row + 1,
            _ => panic!("Invalid direction"),
        };
        map[row][col] = match map[row][col] {
            MapCell::Room | MapCell::Open => MapCell::Room,
            MapCell::Here => MapCell::Here,
            MapCell::Dark => panic!("room from dark?"),
            MapCell::Door => panic!("room from door?"),
            MapCell::Wall => panic!("room from wall?"),
        };
        map[row][col-1] = match map[row][col-1] {
            MapCell::Dark | MapCell::Open => MapCell::Dark,
            MapCell::Door => MapCell::Door,
            MapCell::Wall => panic!("dark from wall?"),
            MapCell::Room => panic!("dark from room?"),
            MapCell::Here => panic!("dark from here?"),
        };
        map[row][col+1] = match map[row][col+1] {
            MapCell::Dark | MapCell::Open => MapCell::Dark,
            MapCell::Door => MapCell::Door,
            MapCell::Wall => panic!("dark from wall?"),
            MapCell::Room => panic!("dark from room?"),
            MapCell::Here => panic!("dark from here?"),
        };

        let nrow = match dir {
            RegexChar::North => row - 1,
            RegexChar::South => row + 1,
            _ => panic!("Invalid direction"),
        };
        map[nrow][col] = match map[nrow][col] {
            MapCell::Dark => MapCell::Dark,
            MapCell::Door => MapCell::Door,
            MapCell::Open => MapCell::Dark,
            MapCell::Wall => panic!("dark from wall?"),
            MapCell::Room => panic!("dark from room?"),
            MapCell::Here => panic!("dark from here?"),
        };
        map[nrow][col-1] = match map[nrow][col-1] {
            MapCell::Wall | MapCell::Open => MapCell::Wall,
            MapCell::Dark => panic!("wall from dark?"),
            MapCell::Door => panic!("wall from door?"),
            MapCell::Room => panic!("wall from room?"),
            MapCell::Here => panic!("wall from here?"),
        };
        map[nrow][col+1] = match map[nrow][col+1] {
            MapCell::Wall | MapCell::Open => MapCell::Wall,
            MapCell::Dark => panic!("wall from dark?"),
            MapCell::Door => panic!("wall from door?"),
            MapCell::Room => panic!("wall from room?"),
            MapCell::Here => panic!("wall from here?"),
        };
        map
    }

    fn draw_lr(dir : RegexChar, row : usize, mut col : usize,
               mut map : Vec<Vec<MapCell>>) -> Vec<Vec<MapCell>> {
        col = match dir {
            RegexChar::East => col + 1,
            RegexChar::West => col - 1,
            _ => panic!("Invalid direction"),
        };
        map[row][col] = match map[row][col] {
            MapCell::Dark | MapCell::Door => MapCell::Door,
            MapCell::Wall => panic!("door from wall?"),
            MapCell::Room => panic!("door from room?"),
            MapCell::Here => panic!("door from here?"),
            MapCell::Open => panic!("door from open?"),
        };
        map[row-1][col] = match map[row-1][col] {
            MapCell::Wall => MapCell::Wall,
            MapCell::Dark => panic!("wall from dark?"),
            MapCell::Door => panic!("wall from door?"),
            MapCell::Room => panic!("wall from room?"),
            MapCell::Here => panic!("wall from here?"),
            MapCell::Open => panic!("wall from open?"),
        };
        map[row+1][col] = match map[row+1][col] {
            MapCell::Wall => MapCell::Wall,
            MapCell::Dark => panic!("wall from dark?"),
            MapCell::Door => panic!("wall from door?"),
            MapCell::Room => panic!("wall from room?"),
            MapCell::Here => panic!("wall from here?"),
            MapCell::Open => panic!("wall from open?"),
        };

        col = match dir {
            RegexChar::East => col + 1,
            RegexChar::West => col - 1,
            _ => panic!("Invalid direction"),
        };
        map[row][col] = match map[row][col] {
            MapCell::Room | MapCell::Open => MapCell::Room,
            MapCell::Here => MapCell::Here,
            MapCell::Dark => panic!("room from dark?"),
            MapCell::Door => panic!("room from door?"),
            MapCell::Wall => panic!("room from wall?"),
        };
        map[row-1][col] = match map[row-1][col] {
            MapCell::Dark | MapCell::Open => MapCell::Dark,
            MapCell::Door => MapCell::Door,
            MapCell::Wall => panic!("dark from wall?"),
            MapCell::Room => panic!("dark from room?"),
            MapCell::Here => panic!("dark from here?"),
        };
        map[row+1][col] = match map[row+1][col] {
            MapCell::Dark | MapCell::Open => MapCell::Dark,
            MapCell::Door => MapCell::Door,
            MapCell::Wall => panic!("dark from wall?"),
            MapCell::Room => panic!("dark from room?"),
            MapCell::Here => panic!("dark from here?"),
        };

        let ncol = match dir {
            RegexChar::East => col + 1,
            RegexChar::West => col - 1,
            _ => panic!("Invalid direction"),
        };
        map[row][ncol] = match map[row][ncol] {
            MapCell::Dark => MapCell::Dark,
            MapCell::Door => MapCell::Door,
            MapCell::Open => MapCell::Dark,
            MapCell::Wall => panic!("dark from wall?"),
            MapCell::Room => panic!("dark from room?"),
            MapCell::Here => panic!("dark from here?"),
        };
        map[row-1][ncol] = match map[row-1][ncol] {
            MapCell::Wall | MapCell::Open => MapCell::Wall,
            MapCell::Dark => panic!("wall from dark?"),
            MapCell::Door => panic!("wall from door?"),
            MapCell::Room => panic!("wall from room?"),
            MapCell::Here => panic!("wall from here?"),
        };
        map[row+1][ncol] = match map[row+1][ncol] {
            MapCell::Wall | MapCell::Open => MapCell::Wall,
            MapCell::Dark => panic!("wall from dark?"),
            MapCell::Door => panic!("wall from door?"),
            MapCell::Room => panic!("wall from room?"),
            MapCell::Here => panic!("wall from here?"),
        };
        map
    }

    pub fn find_far_rooms(&self) -> (usize, usize) {
        #[derive(Debug)]
        struct Node {
            dist : usize,
            prev : Vec<(usize, usize)>,
        }
        let mut graph = std::collections::HashMap::new();
        let mut todo = Vec::new();

        // Initialize graph & todo list
        for row in 0..self.map.len() {
            for col in 0..self.map[0].len() {
                match self.map[row][col] {
                    MapCell::Room => {
                        todo.push((row, col));
                        graph.insert((row, col), Node {
                            dist : std::usize::MAX,
                            prev : Vec::new(),
                        });
                    },
                    MapCell::Here => {
                        todo.push((row, col));
                        graph.insert((row, col), Node {
                            dist : 0,
                            prev : Vec::new(),
                        });
                    },
                    _ => (),
                }
            }
        }

        // Go through all the graph nodes & find the optimal paths
        loop {
            // Sort by distance such that the last elements have the minimum
            todo.sort_unstable_by(|a, b| {
                let a = &graph.get(a).unwrap().dist;
                let b = &graph.get(b).unwrap().dist;
                b.cmp(a)
            });

            if let Some((row, col)) = todo.pop() {
                for (x_off, y_off) in [
                            (-1, 0), (1, 0), (0, -1), (0, 1),
                        ].iter() {
                    let door = (
                        (row as isize + x_off) as usize,
                        (col as isize + y_off) as usize,
                    );
                    match self.map[door.0][door.1] {
                        MapCell::Door => (),
                        _ => continue,
                    }
                    let next = (
                        (row as isize + x_off * 2) as usize,
                        (col as isize + y_off * 2) as usize,
                    );

                    // If our current node is infinitely far from the source, there are no more
                    // paths
                    if graph[&(row, col)].dist == std::usize::MAX {
                        todo.clear();
                        break;
                    }

                    let alt = graph[&(row, col)].dist + 1;
                    let mut node = graph.get_mut(&next).unwrap();
                    if alt < node.dist {
                        node.dist = alt;
                        node.prev.clear();
                        node.prev.push((row, col));
                    } else if alt == node.dist {
                        node.prev.push((row, col));
                    }
                }
            } else {
                break;
            }
        }

        assert_eq!(0, todo.len());

        let mut furthest = 0;
        let mut far_rooms = 0;
        for (key, value) in graph {
            furthest = std::cmp::max(furthest, value.dist);
            if value.dist >= 1000 {
                far_rooms += 1;
            }
        }
        (furthest, far_rooms)
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f : &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut out = String::new();
        out.push('\n');
        for row in 0..self.map.len() {
            for col in 0..self.map[row].len() {
                out.push(match self.map[row][col] {
                    MapCell::Wall => '#',
                    MapCell::Room => '.',
                    MapCell::Door => if col % 2 == 0 { '|' } else { '-' },
                    MapCell::Here => 'X',
                    MapCell::Open => ' ',
                    MapCell::Dark => '?',
                });
            }
            out.push('\n');
        }
        out.push('\n');
        write!(f, "{}", out)
    }
}

fn main() {
    let buff = include_bytes!("../input");
    let map = Map::new(buff);
    let (part1, part2) = map.find_far_rooms();
    println!("Part 1 = {}", part1);
    println!("Part 2 = {}", part2);
}
