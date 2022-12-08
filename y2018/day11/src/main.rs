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

fn find_high_power_region(grid : &Vec<Vec<isize>>, region_size : usize) -> ((isize, isize), isize) {
    let mut max_power = std::isize::MIN;
    let mut top_left = (0, 0);
    for x in 0..=(300 - region_size) {
        for y in 0..=(300 - region_size) {
            let mut power = 0;
            for rx in 0..region_size {
                for ry in 0..region_size {
                    power += grid[x+rx][y+ry];
                }
            }
            if power > max_power {
                max_power = power;
                top_left = (x, y);
            }
        }
    }
    ((top_left.0 as isize + 1, top_left.1 as isize + 1), max_power)
}

fn find_high_power_region2(grid_sn : isize) -> (isize, isize, usize) {
    let grid = gen_power_grid(grid_sn);

    let mut max_power = std::isize::MIN;
    let mut best_top_left = (std::isize::MIN, std::isize::MIN);
    let mut best_region_size = 0;
    for region_size in 1..=300 {
        let (top_left, power) = find_high_power_region(&grid, region_size);
        if power > max_power {
            max_power = power;
            best_top_left = top_left;
            best_region_size = region_size;
        }
    }
    (best_top_left.0, best_top_left.1, best_region_size)
}

fn main() {
    assert_eq!(4, get_power_level(3, 5, 8));
    assert_eq!(-5, get_power_level(122, 79, 57));
    assert_eq!(0, get_power_level(217, 196, 39));
    assert_eq!(4, get_power_level(101, 153, 71));
    assert_eq!((33,45), find_high_power_region(&gen_power_grid(18), 3).0);
    assert_eq!((21,61), find_high_power_region(&gen_power_grid(42), 3).0);

    let grid_sn = 5034;
    let part1 = find_high_power_region(&gen_power_grid(grid_sn), 3).0;
    println!("{},{}", part1.0, part1.1);

    assert_eq!((90,269,16), find_high_power_region2(18));
    assert_eq!((232,251,12), find_high_power_region2(42));

    let part2 = find_high_power_region2(grid_sn);
    println!("{},{},{}", part2.0, part2.1, part2.2);
}
