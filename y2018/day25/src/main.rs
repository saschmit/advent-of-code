use std::collections::HashSet;

#[cfg(test)]
mod tests;

fn parse(input : &str) -> Vec<[i64; 4]> {
    let mut coords = Vec::new();
    for line in input.lines() {
        let coord : Vec<i64> = line.split(',').map(|x| x.parse().unwrap()).collect();
        assert_eq!(coord.len(), 4);
        coords.push([coord[0], coord[1], coord[2], coord[3]]);
    }
    coords
}

fn calc_mdist(a : &[i64], b : &[i64]) -> usize {
    use std::cmp::{min,max};
    assert_eq!(a.len(), b.len());

    let mut mdist = 0;
    for i in 0..a.len() {
        mdist += (max(a[i], b[i]) - min(a[i], b[i])) as usize;
    }

    mdist
}

fn count_constellations(coords : &Vec<[i64; 4]>) -> usize {
    use std::cmp::{min,max};
    let mut neighbors = vec![];
    for (n1,point1) in coords.iter().enumerate() {
        neighbors.push(HashSet::new());
        for (n2,point2) in coords.iter().enumerate() {
            if calc_mdist(&point1[..], &point2[..]) <= 3 {
                neighbors[n1].insert((min(n1, n2), max(n1, n2)));
            }
        }
    }
    let mut constellations = vec![];
    while neighbors.len() > 0 {
        let mut collector = neighbors.pop().unwrap();
        let mut i = 0;
        while i < neighbors.len() {
            if collector.is_disjoint(&neighbors[i]) {
                i += 1;
            } else {
                for item in neighbors[i].drain() {
                    collector.insert(item);
                }
                neighbors.swap_remove(i);
                i = 0;
            }
        }
        constellations.push(collector);
    }
    constellations.len()
}

fn main() {
    let input = include_str!("../input");
    let coords = parse(input);
    let part1 = count_constellations(&coords);
    println!("Part 1 = {}", part1);
}
