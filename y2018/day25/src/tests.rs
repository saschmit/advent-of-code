use super::*;

#[test]
fn ex1() {
    let input = "0,0,0,0
3,0,0,0
0,3,0,0
0,0,3,0
0,0,0,3
0,0,0,6
9,0,0,0
12,0,0,0
";
    let coords = parse(input);
    assert_eq!(2, count_constellations(&coords));
}

#[test]
fn ex2() {
    let input = "-1,2,2,0
0,0,2,-2
0,0,0,-2
-1,2,0,0
-2,-2,-2,2
3,0,2,-1
-1,3,2,2
-1,0,-1,0
0,2,1,-2
3,0,0,0
";
    let coords = parse(input);
    assert_eq!(4, count_constellations(&coords));
}

#[test]
fn ex3() {
    let input = "1,-1,0,1
2,0,-1,0
3,2,-1,0
0,0,3,1
0,0,-1,-1
2,3,-2,0
-2,2,0,0
2,-2,0,-1
1,-1,0,-1
3,2,0,2
";
    let coords = parse(input);
    assert_eq!(3, count_constellations(&coords));
}

#[test]
fn ex4() {
    let input = "1,-1,-1,-2
-2,-2,0,1
0,2,1,3
-2,3,-2,1
0,2,3,-2
-1,-1,1,-2
0,-2,-1,0
-2,2,3,-1
1,2,2,0
-1,-2,0,-2
";
    let coords = parse(input);
    assert_eq!(8, count_constellations(&coords));
}
