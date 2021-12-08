use aoc_2021::day::day_06::get_day;

const INPUT: &str = r#"3,4,3,1,2"#;

#[test]
pub fn part1_1() {
    assert_eq!(get_day().part1(INPUT).expect("Error"), "5934".to_string());
}

#[test]
pub fn part2_1() {
    assert_eq!(get_day().part2(INPUT).expect("Error"), "26984457539".to_string());
}
