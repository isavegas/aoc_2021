use aoc_2021::day::day_02::get_day;

const INPUT: &str = r#"forward 5
down 5
forward 8
up 3
down 8
forward 2"#;

#[test]
pub fn part1_1() {
    assert_eq!(
        get_day()
            .part1(INPUT)
            .expect("Error"),
        "150".to_string()
    );
}

#[test]
pub fn part2_1() {
    assert_eq!(
        get_day()
            .part2(INPUT)
            .expect("Error"),
        "900".to_string()
    );
}
