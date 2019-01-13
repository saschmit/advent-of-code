#[test]
fn pos_ordering() {
    /* These form a grid as such:
     * a  b
     * c  d
     */
    use super::Pos;
    let a = Pos{x:0,y:0};
    let b = Pos{x:1,y:0};
    let c = Pos{x:0,y:1};
    let d = Pos{x:1,y:1};
    assert_eq!(std::cmp::Ordering::Less, a.cmp(&b));
    assert_eq!(std::cmp::Ordering::Less, a.cmp(&c));
    assert_eq!(std::cmp::Ordering::Less, a.cmp(&d));
    assert_eq!(std::cmp::Ordering::Less, b.cmp(&c));
    assert_eq!(std::cmp::Ordering::Less, b.cmp(&d));
    assert_eq!(std::cmp::Ordering::Less, c.cmp(&d));
    assert_eq!(std::cmp::Ordering::Greater, b.cmp(&a));
    assert_eq!(std::cmp::Ordering::Greater, c.cmp(&b));
    assert_eq!(std::cmp::Ordering::Greater, c.cmp(&a));
    assert_eq!(std::cmp::Ordering::Greater, d.cmp(&c));
    assert_eq!(std::cmp::Ordering::Greater, d.cmp(&b));
    assert_eq!(std::cmp::Ordering::Greater, d.cmp(&a));
    assert_eq!(std::cmp::Ordering::Equal, a.cmp(&a));
    assert_eq!(std::cmp::Ordering::Equal, b.cmp(&b));
    assert_eq!(std::cmp::Ordering::Equal, c.cmp(&c));
    assert_eq!(std::cmp::Ordering::Equal, d.cmp(&d));
}


/*
 * battleN & movement tests come from the sample battles provided by AoC.
 * Other tests are courtesy of ShaneMcC
 * from https://github.com/ShaneMcC/aoc-2018/tree/master/15/tests
 */

use super::Team;
fn run_test(data : &[u8], expected : (usize, usize, Team)) {
    let mut game = super::Game::new(data);
    let actual = game.fight();
    assert_eq!(actual.0, expected.0);
    assert_eq!(actual.1, expected.1);
    assert_eq!(actual.2, expected.2);
}

#[test]
fn battle1() {
    run_test(include_bytes!("test-data/battle1/input.txt"),
        (47, 590, Team::Goblin));
}

#[test]
fn battle2() {
    run_test(include_bytes!("test-data/battle2/input.txt"),
        (37, 982, Team::Elf));
}

#[test]
fn battle3() {
    run_test(include_bytes!("test-data/battle3/input.txt"),
        (46, 859, Team::Elf));
}

#[test]
fn battle4() {
    run_test(include_bytes!("test-data/battle4/input.txt"),
        (35, 793, Team::Goblin));
}

#[test]
fn battle5() {
    run_test(include_bytes!("test-data/battle5/input.txt"),
        (54, 536, Team::Goblin));
}

#[test]
fn battle6() {
    run_test(include_bytes!("test-data/battle6/input.txt"),
        (20, 937, Team::Goblin));
}

#[test]
fn move_left() {
    run_test(include_bytes!("test-data/moveLeft/input.txt"),
        (34, 295, Team::Goblin));
}

#[test]
fn movement() {
    run_test(include_bytes!("test-data/movement/input.txt"),
        (18, 1546, Team::Goblin));
}

#[test]
fn move_right() {
    run_test(include_bytes!("test-data/moveRight/input.txt"),
        (34, 301, Team::Goblin));
}

#[test]
fn reddit01() {
    run_test(include_bytes!("test-data/reddit1/input.txt"),
        (67, 200, Team::Goblin));
}

#[test]
fn reddit02() {
    run_test(include_bytes!("test-data/reddit2/input.txt"),
        (71, 197, Team::Goblin));
}

#[test]
fn reddit03() {
    run_test(include_bytes!("test-data/reddit3/input.txt"),
        (35, 295, Team::Goblin));
}

#[test]
fn reddit04() {
    run_test(include_bytes!("test-data/reddit4/input.txt"),
        (37, 292, Team::Elf));
}

#[test]
fn reddit05() {
    run_test(include_bytes!("test-data/reddit5/input.txt"),
        (36, 295, Team::Goblin));
}

#[test]
fn reddit06() {
    run_test(include_bytes!("test-data/reddit6/input.txt"),
        (34, 498, Team::Goblin));
}

#[test]
fn reddit07() {
    run_test(include_bytes!("test-data/reddit7/input.txt"),
        (34, 301, Team::Elf));
}

#[test]
fn reddit08() {
    run_test(include_bytes!("test-data/reddit8/input.txt"),
        (35, 298, Team::Elf));
}

#[test]
fn reddit09() {
    run_test(include_bytes!("test-data/reddit9/input.txt"),
        (24, 531, Team::Goblin));
}

#[test]
fn reddit10() {
    run_test(include_bytes!("test-data/reddit10/input.txt"),
        (20, 737, Team::Goblin));
}

#[test]
fn wall() {
    run_test(include_bytes!("test-data/wall/input.txt"),
        (38, 486, Team::Goblin));
}
