#[derive(Debug)]
struct Vector {
    x : i32,
    y : i32,
    dx : i32,
    dy : i32,
}

impl Vector {
    pub fn from_line(line : &str) -> Vector {
        assert!(line.starts_with("position="));
        let (_, remaining) = line.split_at(line.find("=").unwrap() + 2);

        assert_ne!(remaining.find("> velocity=<"), None);
        let (remaining, vel) = remaining.split_at(remaining.find("=").unwrap() + 2);
        let (vel, _) = vel.split_at(vel.len() - 1);

        let (pos, _) = remaining.split_at(remaining.rfind(" ").unwrap() - 1);

        let (x, y) = pos.split_at(pos.find(",").unwrap());
        let (_, y) = y.split_at(1);

        let (dx, dy) = vel.split_at(vel.find(",").unwrap());
        let (_, dy) = dy.split_at(1);

        let x = x.split_whitespace().next().unwrap();
        let y = y.split_whitespace().next().unwrap();
        let dx = dx.split_whitespace().next().unwrap();
        let dy = dy.split_whitespace().next().unwrap();

        Vector {
            x: i32::from_str_radix(x, 10).unwrap(),
            y: i32::from_str_radix(y, 10).unwrap(),
            dx: i32::from_str_radix(dx, 10).unwrap(),
            dy: i32::from_str_radix(dy, 10).unwrap(),
        }
    }
}

struct Sky {
    sky : Vec<Vec<u8>>,
}

impl Sky {
    pub fn new(height : usize, width : usize) -> Sky {
        let mut out = Self {
            sky : Vec::new(),
        };

        for y in 0..height {
            out.sky.push(Vec::new());
            for _ in 0..width {
                out.sky[y].push('.' as u8);
            }
        }
        out
    }
}

impl std::fmt::Display for Sky {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut out = Vec::with_capacity(self.sky.len() * (self.sky[0].len() + 1));
        for row in &self.sky {
            for col in row {
                out.push(*col);
            }
            out.push('\n' as u8);
        }
        out.pop();
        write!(f, "{}", &String::from_utf8_lossy(&out))
    }
}

fn calc_geometry(vecs : &Vec<Vector>) -> (usize, usize, i32, i32) {
    let (left, right, top, bottom) = vecs.iter()
        .fold((std::i32::MAX, std::i32::MIN, std::i32::MAX, std::i32::MIN),
            |(left, right, top, bottom), vec|
                (std::cmp::min(left, vec.x),
                 std::cmp::max(right, vec.x),
                 std::cmp::min(top, vec.y),
                 std::cmp::max(bottom, vec.y)));
    let width = (right - left + 1) as usize;
    let height = (bottom - top + 1) as usize;
    (height, width, top, left)
}

fn main() {
    let input = std::env::args().nth(1).unwrap();
    let data = String::from_utf8(std::fs::read(input).unwrap()).unwrap();
    let mut vecs = Vec::new();
    for line in data.lines() {
        vecs.push(Vector::from_line(line));
    }

    let mut min_area = std::usize::MAX;
    let mut t = 0i32;
    let t = loop {
        let (height, width, _, _) = calc_geometry(&vecs);
        let area = height * width;

        if area > min_area {
            // back up 1 second from where we were
            for mut vec in &mut vecs {
                vec.x -= vec.dx;
                vec.y -= vec.dy;
            }

            break t - 1;
        }

        min_area = std::cmp::min(min_area, area);

        t += 1;
        assert_ne!(t, std::i32::MAX);

        for mut vec in &mut vecs {
            vec.x += vec.dx;
            vec.y += vec.dy;
        }
    };

    let (height, width, min_top, min_left) = calc_geometry(&vecs);

    // Normalize coordinates
    for mut vec in &mut vecs {
        vec.x -= min_left;
        vec.y -= min_top;
    }

    let mut sky = Sky::new(height, width);
    for vec in &vecs {
        if ! (vec.y < 0 || vec.y >= height as i32 || vec.x < 0 || vec.x >= width as i32) {
            sky.sky[vec.y as usize][vec.x as usize] = '#' as u8;
        }
    }

    println!("t = {} s", t);
    println!("{}", sky);
    println!("");

}
