use super::*;

#[test]
fn sample() {
    let depth = 510;
    let target = Pos { x: 10, y: 10 };
    let survey = Survey::new(depth, target);
    assert_eq!(survey.scan[0][0], Region::Rocky);
    assert_eq!(survey.scan[0][1], Region::Wet);
    assert_eq!(survey.scan[1][0], Region::Rocky);
    assert_eq!(survey.scan[1][1], Region::Narrow);
    assert_eq!(survey.scan[10][10], Region::Rocky);
    assert_eq!(survey.get_risk_level(), 114);
}
