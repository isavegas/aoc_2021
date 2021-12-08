use aoc_2021::day::day_01::get_day;

const INPUT: &str = r#"199
200
208
210
200
207
240
269
260
263"#;

#[test]
pub fn part1() {
    assert_eq!(get_day().part1(INPUT).expect("Error"), "7".to_string());
}

#[test]
pub fn part2() {
    assert_eq!(get_day().part2(INPUT).expect("Error"), "5".to_string());
}
