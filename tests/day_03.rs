use aoc_2021::day::day_03::Day03;
use aoc_core::AoCDay;

const INPUT: &str = r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"#;

#[test]
pub fn part1_1() {
    assert_eq!(
        Day03 { data_width: 5 }.part1(INPUT).expect("Error"),
        "198".to_string()
    );
}

#[test]
pub fn part2_1() {
    assert_eq!(
        Day03 { data_width: 5 }.part2(INPUT).expect("Error"),
        "230".to_string()
    );
}
