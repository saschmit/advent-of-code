use super::*;

#[test]
fn no_branch() {
    let map = Map::new(b"^WNE$");
    assert_eq!(map.find_furthest_room(), 3);
}

#[test]
fn branch() {
    let map = Map::new(b"^ENWWW(NEEE|SSE(EE|N))$");
    assert_eq!(map.find_furthest_room(), 10);
}

#[test]
fn example1() {
    let map = Map::new(b"^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$");
    assert_eq!(map.find_furthest_room(), 18);
}

#[test]
fn example2() {
    let map = Map::new(b"^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$");
    assert_eq!(map.find_furthest_room(), 23);
}

#[test]
fn example3() {
    let map = Map::new(b"^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$");
    assert_eq!(map.find_furthest_room(), 31);
}
