use aoc_2021::day::day_09::get_day;

const INPUT: &str = r#"
2199943210
3987894921
9856789892
8767896789
9899965678
"#;

#[test]
pub fn part1() {{
    assert_eq!(
        get_day()
            .part1(INPUT)
            .expect("Error"),
        "15".to_string()
    );
}}

#[test]
pub fn part2() {{
    assert_eq!(
        get_day()
            .part2(INPUT)
            .expect("Error"),
        "1134".to_string()
    );
}}
