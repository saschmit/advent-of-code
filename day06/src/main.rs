#[derive(Debug)]
struct Location {
    id: Option<usize>,
    dist: usize,
}

#[derive(Debug)]
struct Map {
    dimensions: (usize, usize),
    offset: (usize, usize),
    sites: Vec<(usize, usize)>,
    grid: Vec<Vec<Option<Location>>>,
}

impl Map {
    pub fn load(file : &str) -> Map {
        let buff = String::from_utf8(std::fs::read(file).unwrap()).unwrap();

        let mut m = Map {
            dimensions: (0, 0),
            offset: (0, 0),
            sites: Vec::new(),
            grid: Vec::new(),
        };

        m.parse(buff.lines());
        m.normalize_sites();
        m.init_grid();
        m.color();

        m
    }

    fn parse(&mut self, lines : std::str::Lines) {
        for line in lines {
            let coords : Vec<&str> = line.split(", ").collect();
            assert_eq!(coords.len(), 2);
            let x = usize::from_str_radix(coords[0], 10).unwrap();
            let y = usize::from_str_radix(coords[1], 10).unwrap();
            self.sites.push((y, x));
        }
    }

    fn normalize_sites(&mut self) {
        assert_ne!(self.sites.len(), 0);
        let mut min_x = std::usize::MAX;
        let mut min_y = std::usize::MAX;
        let mut max_x = 0;
        let mut max_y = 0;
        for (x, y) in &self.sites {
            min_x = std::cmp::min(min_x, *x);
            min_y = std::cmp::min(min_y, *y);
            max_x = std::cmp::max(max_x, *x);
            max_y = std::cmp::max(max_y, *y);
        }

        for (x, y) in &mut self.sites {
            *x -= min_x;
            *y -= min_y;
        }
        self.dimensions = (max_x - min_x + 1, max_y - min_y + 1);
        self.offset = (min_x, min_y);
    }

    fn init_grid(&mut self) {
        assert_ne!(self.sites.len(), 0);
        assert_ne!(self.dimensions.0, 0);
        assert_ne!(self.dimensions.1, 0);
        for x in 0..self.dimensions.0 {
            self.grid.push(Vec::new());
            for _ in 0..self.dimensions.1 {
                self.grid[x].push(None);
            }
            assert_eq!(self.grid[x].len(), self.dimensions.1);
        }
        assert_eq!(self.grid.len(), self.dimensions.0);
    }

    fn calc_dist(coord1 : (usize, usize), coord2 : (usize, usize)) -> usize {
        use std::cmp::{min, max};
        max(coord1.0, coord2.0) - min(coord1.0, coord2.0) +
            max(coord1.1, coord2.1) - min(coord1.1, coord2.1)
    }

    fn color(&mut self) {
        assert_ne!(self.grid.len(), 0);
        assert_ne!(self.grid[0].len(), 0);

        for x in 0..self.dimensions.0 {
            for y in 0..self.dimensions.1 {
                for (site_id, site_coord) in self.sites.iter().enumerate() {
                    let dist = Self::calc_dist(*site_coord, (x, y));
                    self.grid[x][y] = match &self.grid[x][y] {
                        None => Some(Location { id: Some(site_id), dist: dist }),
                        Some(loc) => Some(Location {
                            id: if loc.dist == dist { None }
                                else if loc.dist > dist { Some(site_id) }
                                else { loc.id },
                            dist: if loc.dist > dist { dist } else { loc.dist },
                        }),
                    };
                }
            }
        }
    }

    pub fn find_largest_area(&self) -> usize {
        let mut areas = std::collections::HashMap::new();
        for x in 0..self.dimensions.0 {
            for y in 0..self.dimensions.1 {
                match &self.grid[x][y] {
                    None => (),
                    Some(cell) => {
                        match cell.id {
                            None => (),
                            Some(id) => {
                                let entry = areas.entry(id).or_insert(0);
                                *entry += 1;
                            }
                        }
                    }
                }
            }
        }
        for x in 0..self.dimensions.0 {
            for y in 0..self.dimensions.1 {
                if x == 0 || x == self.dimensions.0 - 1 || y == 0 || y == self.dimensions.1 - 1 {
                    match &self.grid[x][y] {
                        None => (),
                        Some(cell) => {
                            match cell.id {
                                None => (),
                                Some(id) => {
                                    areas.remove(&id);
                                }
                            }
                        }
                    }
                }
            }
        }
        let mut max_area = 0;
        for area in areas.values() {
            max_area = std::cmp::max(max_area, *area);
        }
        max_area
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut out = String::new();
        for x in 0..self.dimensions.0 {
            for y in 0..self.dimensions.1 {
                match &self.grid[x][y] {
                    None => out.push('.'),
                    Some(cell) => {
                        match cell.id {
                            None => out.push('.'),
                            Some(id) => out += &String::from_utf8(vec!(
                                if cell.dist == 0 { b'A' } else { b'a' } + (id % 26) as u8)).unwrap(),
                        }
                    }
                }
            }
            out.push('\n');
        }
        write!(f, "{}", out)
    }
}

fn main() {
    let map = Map::load(&std::env::args().nth(1).unwrap());
    println!("Largest area: {}", map.find_largest_area());
}
