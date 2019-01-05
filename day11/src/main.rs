fn get_power_level(x : isize, y : isize, grid_sn : isize) -> isize {
    let rack_id = x + 10;
    (rack_id * y + grid_sn) * rack_id / 100 % 10 - 5
}

fn gen_power_grid(grid_sn : isize) -> Vec<Vec<isize>> {
    let mut grid = Vec::new();
    for x in 0..300 {
        grid.push(Vec::new());
        for y in 0..300 {
            grid[x].push(get_power_level((x + 1) as isize,
                                         (y + 1) as isize, grid_sn));
        }
    }
    grid
}

fn find_high_power_region(grid : Vec<Vec<isize>>) -> (isize, isize) {
    let mut max_power = std::isize::MIN;
    let mut top_left = (0, 0);
    for x in 0..(300-2) {
        for y in 0..(300-2) {
            let power = grid[x+0][y+0] + grid[x+1][y+0] + grid[x+2][y+0] +
                        grid[x+0][y+1] + grid[x+1][y+1] + grid[x+2][y+1] +
                        grid[x+0][y+2] + grid[x+1][y+2] + grid[x+2][y+2];
            if power > max_power {
                max_power = power;
                top_left = (x, y);
            }
        }
    }
    (top_left.0 as isize + 1, top_left.1 as isize + 1)
}

fn main() {
    assert_eq!(4, get_power_level(3, 5, 8));
    assert_eq!(-5, get_power_level(122, 79, 57));
    assert_eq!(0, get_power_level(217, 196, 39));
    assert_eq!(4, get_power_level(101, 153, 71));
    assert_eq!((33,45), find_high_power_region(gen_power_grid(18)));
    assert_eq!((21,61), find_high_power_region(gen_power_grid(42)));

    println!("{:?}", find_high_power_region(gen_power_grid(5034)));
}
