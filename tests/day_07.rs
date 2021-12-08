use aoc_2021::day::day_07::get_day;

const INPUT: &str = r#"16,1,2,0,4,2,7,1,2,14"#;

#[test]
pub fn part1() {
    {
        assert_eq!(get_day().part1(INPUT).expect("Error"), "37".to_string());
    }
}

#[test]
pub fn part2() {
    {
        assert_eq!(get_day().part2(INPUT).expect("Error"), "168".to_string());
    }
}
