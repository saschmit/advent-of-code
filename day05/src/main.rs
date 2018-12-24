fn do_react(a : &u8, b : &u8) -> bool {
    (a.is_ascii_uppercase() && b.is_ascii_lowercase() && a.to_ascii_lowercase() == *b) ||
    (b.is_ascii_uppercase() && a.is_ascii_lowercase() && b.to_ascii_lowercase() == *a)
}

fn collapse(mut polymer : Vec<u8>) -> usize {
    let mut i = 0;
    while (i + 1) < polymer.len() {
        if do_react(&polymer[i], &polymer[i+1]) {
            polymer.remove(i+1);
            polymer.remove(i);
            i -= if i > 0 { 1 } else { 0 };
        } else {
            i += 1;
        }
    }
    polymer.len()
}

fn main() {
    assert!(do_react(&b'a', &b'A'));
    assert!(do_react(&b'A', &b'a'));
    assert!(!do_react(&b'A', &b'A'));
    assert!(!do_react(&b'a', &b'a'));

    assert!(!do_react(&b'a', &b'b'));
    assert!(!do_react(&b'b', &b'a'));
    assert!(!do_react(&b'a', &b'B'));
    assert!(!do_react(&b'B', &b'a'));
    assert!(!do_react(&b'A', &b'B'));
    assert!(!do_react(&b'B', &b'A'));
    assert!(!do_react(&b'A', &b'b'));
    assert!(!do_react(&b'b', &b'A'));

    let mut buff = std::fs::read("input").unwrap();
    buff.retain(|&x| x.is_ascii_alphabetic());

    println!("Part 1: {}", collapse(buff.clone()));

    let mut shortest = std::usize::MAX;
    for unit in b'a'..=b'z' {
        let mut polymer = buff.clone();
        polymer.retain(|&x| x != unit && x != unit.to_ascii_uppercase());
        let units = collapse(polymer);
        shortest = std::cmp::min(shortest, units);
    }
    println!("Part 2: {}", shortest);
}
